//! This crate implements composition of 2D graphics.

#![warn(missing_docs)]

pub use self::blitter::*;
pub use self::canvas::*;
pub use self::format::*;
pub use self::linalg::*;
pub use self::paint::*;
pub use self::pixel::*;

pub mod blitters;
pub mod formats;

mod blitter;
mod canvas;
mod format;
mod linalg;
mod paint;
mod pixel;
