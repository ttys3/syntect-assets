//! `syntect-assets` is a library to print syntax highlighted content.
//!
//! The main struct of this crate is `PrettyPrinter` which can be used to
//! configure and run the syntax highlighting.
//!
//! If you need more control, you can also use the structs in the submodules
//! (start with `controller::Controller`), but note that the API of these
//! internal modules is much more likely to change. Some or all of these
//! modules might be removed in the future.
//!
//! "Hello world" example:
//! ```
//! use syntect_assets::PrettyPrinter;
//!
//! PrettyPrinter::new()
//!     .input_from_bytes(b"<span style=\"color: #ff00cc\">Hello world!</span>\n")
//!     .language("html")
//!     .print()
//!     .unwrap();
//! ```

#![deny(unsafe_code)]


pub mod assets;
pub mod assets_metadata {
    pub use super::assets::assets_metadata::*;
}

pub(crate) mod syntax_mapping;
mod error;

