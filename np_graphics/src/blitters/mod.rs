//! Implementations of the [`Blitter`] trait.
//!
//! [`Blitter`]: `crate::Blitter`

pub use self::blend_destination::*;
pub use self::blend_source::*;
pub use self::blend_source_over::*;

mod blend_destination;
mod blend_source;
mod blend_source_over;
