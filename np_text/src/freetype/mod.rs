//! This module provides convenient Rust bindings to the FreeType library.

pub use self::error::*;
pub use self::face::*;
pub use self::library::*;

mod error;
mod face;
mod library;
