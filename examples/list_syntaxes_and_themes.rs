/// A simple program that lists all supported syntaxes and themes.
use syntect_assets::assets::HighlightingAssets;

fn main() {
    let assets = HighlightingAssets::from_binary();

    println!("Syntaxes:");
    for syntax in assets.get_syntaxes().unwrap() {
        println!("- {} ({})", syntax.name, syntax.file_extensions.join(", "));
    }

    println!();

    println!("Themes:");
    for theme in assets.themes() {
        println!("- {}", theme);
    }
}
