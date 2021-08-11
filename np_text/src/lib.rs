//! This crate implements laying out and rendering text.

#![warn(missing_docs)]

pub use self::error::*;
pub use self::font_file::*;
pub use self::glyph::*;
pub use self::typeface::*;

mod error;
mod font_file;
mod glyph;
mod typeface;
