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


## troubleshooting

the latest syntax fixed many bugs, but we can not update to that.

for example, fenced golang now support both `go` and `golang`, prev just support `golang`, but most people use `go` for that.

the latest version fixed the issue:

https://github.com/sublimehq/Packages/blob/master/Markdown/Markdown.sublime-syntax


but syntect does not support sublime-syntax from Sublime Text Build 4075

see https://github.com/trishume/syntect/issues/323

sublimehq packages: Missing mandatory key in YAML file: match
https://github.com/trishume/syntect/issues/461


## related

https://github.com/microsoft/vscode/blob/7a464d6069a39b7d0e63c3da453d43a53eea7495/extensions/markdown-basics/syntaxes/markdown.tmLanguage.json#L1391

https://github.com/microsoft/vscode-textmate

https://github.com/microsoft/vscode-markdown-tm-grammar
