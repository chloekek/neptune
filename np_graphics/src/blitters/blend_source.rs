use crate::Blitter;
use crate::PixelMap;

use cgmath::Vector2;

/// Blitter that implements the [`Source`] blend mode.
///
/// [`Source`]: `crate::BlendMode::Source`
pub struct BlendSourceBlitter<T>
{
    pixel: T,
}

impl<T> BlendSourceBlitter<T>
{
    /// Create a new blitter.
    pub fn new(pixel: T) -> Self
    {
        Self{pixel}
    }
}

impl<T> Blitter for BlendSourceBlitter<T>
    where T: Copy
{
    type Pixel = T;

    fn horizontal(
        &self,
        map: &mut PixelMap<T>,
        start: Vector2<u32>,
        length: u32,
    )
    {
        let dest = map.horizontal_mut(start, length);
        dest.fill(self.pixel);
    }
}
