# syntect-assets

[syntect](https://github.com/trishume/syntect) syntax and theme assets from [bat](https://github.com/sharkdp/bat)

## why?

`bat`'s syntect syntax is updated and improved compared to syntect builtin ones

## usage

ref https://docs.rs/syntect/

```rust
/// A simple program that prints its own source code using the syntect-assets library
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightFile;
use std::io::BufRead;
use syntect_assets::assets::HighlightingAssets;

fn main() {
    let assets = HighlightingAssets::from_binary();

    let ss = assets.get_syntax_set().unwrap();
    let theme = assets.get_theme("OneHalfDark");

    let mut highlighter = HighlightFile::new(file!(), ss, theme).unwrap();
    let mut line = String::new();
    while highlighter.reader.read_line(&mut line).unwrap() > 0 {
        {
            let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight_line(&line, &ss).unwrap();
            print!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        } // until NLL this scope is needed so we can clear the buffer after
        line.clear(); // read_line appends so we need to clear between lines
    }
}
```

## supported themes

```yaml
Themes:
- 1337
- Coldark-Cold
- Coldark-Dark
- DarkNeon
- Dracula
- GitHub
- Monokai Extended
- Monokai Extended Bright
- Monokai Extended Light
- Monokai Extended Origin
- Nord
- OneHalfDark
- OneHalfLight
- Solarized (dark)
- Solarized (light)
- Sublime Snazzy
- TwoDark
- Visual Studio Dark+
- ansi
- base16
- base16-256
- gruvbox-dark
- gruvbox-light
- zenburn
```

## supported syntaxes

see [examples/list_syntaxes_and_themes.rs](examples/list_syntaxes_and_themes.rs) example

you can run the example to get the list of supported syntaxes and themes

```bash
cargo run --example list_syntaxes_and_themes
```

## troubleshooting

the latest syntax fixed many bugs, but we can not update to that.

for example, fenced golang now support both `go` and `golang`, prev just support `golang`, but most people use `go` for that.

the latest version fixed the issue:
https://github.com/sublimehq/Packages/blob/master/Markdown/Markdown.sublime-syntax


some issues:

* syntect does not support sublime-syntax from Sublime Text Build 4075
see https://github.com/trishume/syntect/issues/323

* sublimehq packages: Missing mandatory key in YAML file: match
https://github.com/trishume/syntect/issues/461

* zola: Investigate tree-sitter to replace syntect
  > Our syntect syntaxes are stuck on old versions of the grammars because of new features in the Sublime grammar format not supported by Syntect.

  https://github.com/getzola/zola/issues/1787

## related

https://github.com/microsoft/vscode/blob/7a464d6069a39b7d0e63c3da453d43a53eea7495/extensions/markdown-basics/syntaxes/markdown.tmLanguage.json#L1391

https://github.com/microsoft/vscode-textmate

https://github.com/microsoft/vscode-markdown-tm-grammar

Sublime Text

Syntax Definitions https://www.sublimetext.com/docs/syntax.html

Scope Naming https://www.sublimetext.com/docs/scope_naming.html

Themes https://www.sublimetext.com/docs/themes.html

Color Schemes https://www.sublimetext.com/docs/color_schemes.html

TextMate language grammar definition https://macromates.com/manual/en/language_grammars
