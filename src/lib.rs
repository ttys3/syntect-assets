//! `syntect-assets` contains [syntect](https://github.com/trishume/syntect) syntax and theme assets from [bat](https://github.com/sharkdp/bat).
//!
//! The main struct of this crate is `HighlightingAssets` which can be used in `syntect`.
//!
//! "Hello world" example:
//! ```rust
//! use syntect::easy::HighlightLines;
//! use syntect::highlighting::Style;
//! use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
//! use syntect_assets::assets::HighlightingAssets;
//!
//! fn main() {
//!     // Load these once at the start of your program
//!     let assets = HighlightingAssets::from_binary();
//!     let ss = assets.get_syntax_set().unwrap();;
//!     let syntax = ss.find_syntax_by_extension("rs").unwrap();
//!     let theme = assets.get_theme("OneHalfDark");
//!
//!     let mut h = HighlightLines::new(syntax, theme);
//!     let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}\n";
//!     for line in LinesWithEndings::from(s) { // LinesWithEndings enables use of newlines mode
//!         let ranges: Vec<(Style, &str)> = h.highlight_line(line, ss).unwrap();
//!         let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
//!         print!("{}", escaped);
//!     }
//! }
//! ```

#![deny(unsafe_code)]


pub mod assets;
pub mod assets_metadata {
    pub use super::assets::assets_metadata::*;
}

pub(crate) mod syntax_mapping;
mod error;

