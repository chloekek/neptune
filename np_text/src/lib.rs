//! This crate implements laying out and rendering text.

#![warn(missing_docs)]

pub use self::face::*;
pub use self::glyph::*;

use lazy_static::lazy_static;
use std::sync::Mutex;

mod face;
mod freetype;
mod glyph;

lazy_static! {
    /// FreeType has this library type you need to keep around an object of.
    /// It’s pretty inconvenient and I don’t want to expose it from np_text,
    /// so we’ll just make it global (yikes) and behind a mutex.
    /// The lock only has to be taken for a couple of infrequent operations.
    static ref LIBRARY: Mutex<freetype::Library> =
        Mutex::new(freetype::Library::init().unwrap());
}
