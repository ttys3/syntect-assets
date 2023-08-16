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
