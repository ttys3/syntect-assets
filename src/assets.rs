use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use once_cell::unsync::OnceCell;

use syntect::highlighting::Theme;
use syntect::parsing::{SyntaxReference, SyntaxSet};

use crate::syntax_mapping::ignored_suffixes::IgnoredSuffixes;
use crate::syntax_mapping::{MappingTarget, SyntaxMapping};

use lazy_theme_set::LazyThemeSet;

use serialized_syntax_set::*;

use crate::error::*;

pub(crate) mod assets_metadata;
mod lazy_theme_set;
mod serialized_syntax_set;

#[derive(Debug)]
pub struct HighlightingAssets {
    syntax_set_cell: OnceCell<SyntaxSet>,
    serialized_syntax_set: SerializedSyntaxSet,

    theme_set: LazyThemeSet,
    fallback_theme: Option<&'static str>,
}

#[derive(Debug)]
pub struct SyntaxReferenceInSet<'a> {
    pub syntax: &'a SyntaxReference,
    pub syntax_set: &'a SyntaxSet,
}

/// Lazy-loaded syntaxes are already compressed, and we don't want to compress
/// already compressed data.
pub(crate) const COMPRESS_SYNTAXES: bool = false;

/// We don't want to compress our [LazyThemeSet] since the lazy-loaded themes
/// within it are already compressed, and compressing another time just makes
/// performance suffer
pub(crate) const COMPRESS_THEMES: bool = false;

/// Compress for size of ~40 kB instead of ~200 kB without much difference in
/// performance due to lazy-loading
pub(crate) const COMPRESS_LAZY_THEMES: bool = true;

/// Compress for size of ~10 kB instead of ~120 kB
pub(crate) const COMPRESS_ACKNOWLEDGEMENTS: bool = true;

impl HighlightingAssets {
    fn new(serialized_syntax_set: SerializedSyntaxSet, theme_set: LazyThemeSet) -> Self {
        HighlightingAssets {
            syntax_set_cell: OnceCell::new(),
            serialized_syntax_set,
            theme_set,
            fallback_theme: None,
        }
    }

    /// The default theme.
    ///
    /// ### Windows and Linux
    ///
    /// Windows and most Linux distributions has a dark terminal theme by
    /// default. On these platforms, this function always returns a theme that
    /// looks good on a dark background.
    ///
    /// ### macOS
    ///
    /// On macOS the default terminal background is light, but it is common that
    /// Dark Mode is active, which makes the terminal background dark. On this
    /// platform, the default theme depends on
    /// ```bash
    /// defaults read -globalDomain AppleInterfaceStyle
    /// ```
    /// To avoid the overhead of the check on macOS, simply specify a theme
    /// explicitly via `--theme`, `BAT_THEME`, or `~/.config/syntect-assets`.
    ///
    /// See <https://github.com/sharkdp/bat/issues/1746> and
    /// <https://github.com/sharkdp/bat/issues/1928> for more context.
    pub fn default_theme() -> &'static str {
        #[cfg(not(target_os = "macos"))]
        {
            Self::default_dark_theme()
        }
    }

    /**
     * The default theme that looks good on a dark background.
     */
    fn default_dark_theme() -> &'static str {
        "Monokai Extended"
    }

    /**
     * The default theme that looks good on a light background.
     */
    #[cfg(target_os = "macos")]
    fn default_light_theme() -> &'static str {
        "Monokai Extended Light"
    }

    pub fn from_cache(cache_path: &Path) -> Result<Self> {
        Ok(HighlightingAssets::new(
            SerializedSyntaxSet::FromFile(cache_path.join("syntaxes.bin")),
            asset_from_cache(&cache_path.join("themes.bin"), "theme set", COMPRESS_THEMES)?,
        ))
    }

    pub fn from_binary() -> Self {
        HighlightingAssets::new(
            SerializedSyntaxSet::FromBinary(get_serialized_integrated_syntaxset()),
            get_integrated_themeset(),
        )
    }

    pub fn set_fallback_theme(&mut self, theme: &'static str) {
        self.fallback_theme = Some(theme);
    }

    /// Return the collection of syntect syntax definitions.
    pub fn get_syntax_set(&self) -> Result<&SyntaxSet> {
        self.syntax_set_cell
            .get_or_try_init(|| self.serialized_syntax_set.deserialize())
    }

    pub fn get_syntaxes(&self) -> Result<&[SyntaxReference]> {
        Ok(self.get_syntax_set()?.syntaxes())
    }

    fn get_theme_set(&self) -> &LazyThemeSet {
        &self.theme_set
    }

    pub fn themes(&self) -> impl Iterator<Item = &str> {
        self.get_theme_set().themes()
    }

    /// Detect the syntax based on, in order:
    ///  1. Syntax mappings with [MappingTarget::MapTo] and [MappingTarget::MapToUnknown]
    ///     (e.g. `/etc/profile` -> `Bourne Again Shell (bash)`)
    ///  2. The file name (e.g. `Dockerfile`)
    ///  3. Syntax mappings with [MappingTarget::MapExtensionToUnknown]
    ///     (e.g. `*.conf`)
    ///  4. The file name extension (e.g. `.rs`)
    ///
    /// When detecting syntax based on syntax mappings, the full path is taken
    /// into account. When detecting syntax based on file name, no regard is
    /// taken to the path of the file. Only the file name itself matters. When
    /// detecting syntax based on file name extension, only the file name
    /// extension itself matters.
    ///
    /// Returns [Error::UndetectedSyntax] if it was not possible detect syntax
    /// based on path/file name/extension (or if the path was mapped to
    /// [MappingTarget::MapToUnknown] or [MappingTarget::MapExtensionToUnknown]).
    /// In this case it is appropriate to fall back to other methods to detect
    /// syntax. Such as using the contents of the first line of the file.
    ///
    /// Returns [Error::UnknownSyntax] if a syntax mapping exist, but the mapped
    /// syntax does not exist.
    pub fn get_syntax_for_path(
        &self,
        path: impl AsRef<Path>,
        mapping: &SyntaxMapping,
    ) -> Result<SyntaxReferenceInSet> {
        let path = path.as_ref();

        let syntax_match = mapping.get_syntax_for(path);

        if let Some(MappingTarget::MapToUnknown) = syntax_match {
            return Err(crate::error::Error::UndetectedSyntax(path.to_string_lossy().into()));
        }

        if let Some(MappingTarget::MapTo(syntax_name)) = syntax_match {
            return self
                .find_syntax_by_name(syntax_name)?
                .ok_or_else(|| crate::error::Error::UnknownSyntax(syntax_name.to_owned()));
        }

        let file_name = path.file_name().unwrap_or_default();

        match (
            self.get_syntax_for_file_name(file_name, &mapping.ignored_suffixes)?,
            syntax_match,
        ) {
            (Some(syntax), _) => Ok(syntax),

            (_, Some(MappingTarget::MapExtensionToUnknown)) => {
                Err(crate::error::Error::UndetectedSyntax(path.to_string_lossy().into()))
            }

            _ => self
                .get_syntax_for_file_extension(file_name, &mapping.ignored_suffixes)?
                .ok_or_else(|| crate::error::Error::UndetectedSyntax(path.to_string_lossy().into())),
        }
    }

    /// Look up a syntect theme by name.
    pub fn get_theme(&self, theme: &str) -> &Theme {
        match self.get_theme_set().get(theme) {
            Some(theme) => theme,
            None => {
                if theme == "ansi-light" || theme == "ansi-dark" {
                    log::warn!("Theme '{}' is deprecated, using 'ansi' instead.", theme);
                    return self.get_theme("ansi");
                }
                if !theme.is_empty() {
                    log::warn!("Unknown theme '{}', using default.", theme)
                }
                self.get_theme_set()
                    .get(self.fallback_theme.unwrap_or_else(Self::default_theme))
                    .expect("something is very wrong if the default theme is missing")
            }
        }
    }


    pub(crate) fn find_syntax_by_name(
        &self,
        syntax_name: &str,
    ) -> Result<Option<SyntaxReferenceInSet>> {
        let syntax_set = self.get_syntax_set()?;
        Ok(syntax_set
            .find_syntax_by_name(syntax_name)
            .map(|syntax| SyntaxReferenceInSet { syntax, syntax_set }))
    }

    fn find_syntax_by_extension(&self, e: Option<&OsStr>) -> Result<Option<SyntaxReferenceInSet>> {
        let syntax_set = self.get_syntax_set()?;
        let extension = e.and_then(|x| x.to_str()).unwrap_or_default();
        Ok(syntax_set
            .find_syntax_by_extension(extension)
            .map(|syntax| SyntaxReferenceInSet { syntax, syntax_set }))
    }

    fn get_syntax_for_file_name(
        &self,
        file_name: &OsStr,
        ignored_suffixes: &IgnoredSuffixes,
    ) -> Result<Option<SyntaxReferenceInSet>> {
        let mut syntax = self.find_syntax_by_extension(Some(file_name))?;
        if syntax.is_none() {
            syntax =
                ignored_suffixes.try_with_stripped_suffix(file_name, |stripped_file_name| {
                    // Note: recursion
                    self.get_syntax_for_file_name(stripped_file_name, ignored_suffixes)
                })?;
        }
        Ok(syntax)
    }

    fn get_syntax_for_file_extension(
        &self,
        file_name: &OsStr,
        ignored_suffixes: &IgnoredSuffixes,
    ) -> Result<Option<SyntaxReferenceInSet>> {
        let mut syntax = self.find_syntax_by_extension(Path::new(file_name).extension())?;
        if syntax.is_none() {
            syntax =
                ignored_suffixes.try_with_stripped_suffix(file_name, |stripped_file_name| {
                    // Note: recursion
                    self.get_syntax_for_file_extension(stripped_file_name, ignored_suffixes)
                })?;
        }
        Ok(syntax)
    }

}

pub(crate) fn get_serialized_integrated_syntaxset() -> &'static [u8] {
    include_bytes!("../assets/syntaxes.bin")
}

pub(crate) fn get_integrated_themeset() -> LazyThemeSet {
    from_binary(include_bytes!("../assets/themes.bin"), COMPRESS_THEMES)
}

pub fn get_acknowledgements() -> String {
    from_binary(
        include_bytes!("../assets/acknowledgements.bin"),
        COMPRESS_ACKNOWLEDGEMENTS,
    )
}

pub(crate) fn from_binary<T: serde::de::DeserializeOwned>(v: &[u8], compressed: bool) -> T {
    asset_from_contents(v, "n/a", compressed)
        .expect("data integrated in binary is never faulty, but make sure `compressed` is in sync!")
}

fn asset_from_contents<T: serde::de::DeserializeOwned>(
    contents: &[u8],
    description: &str,
    compressed: bool,
) -> Result<T> {
    if compressed {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(contents))
    } else {
        bincode::deserialize_from(contents)
    }
    .map_err(|_| format!("Could not parse {}", description).into())
}

fn asset_from_cache<T: serde::de::DeserializeOwned>(
    path: &Path,
    description: &str,
    compressed: bool,
) -> Result<T> {
    let contents = fs::read(path).map_err(|_| {
        format!(
            "Could not load cached {} '{}'",
            description,
            path.to_string_lossy()
        )
    })?;
    asset_from_contents(&contents[..], description, compressed)
        .map_err(|_| format!("Could not parse cached {}", description).into())
}