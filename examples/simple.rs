/// A simple program that prints its own source code using the syntect-assets library
use bat::PrettyPrinter;

fn main() {
    PrettyPrinter::new().input_file(file!()).print().unwrap();
}
