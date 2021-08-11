use crate::Blitter;
use crate::PixelMapMut;

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
        map: &mut PixelMapMut<T>,
        start_x: u32,
        start_y: u32,
        length: u32,
    )
    {
        let dest = map.horizontal_mut(start_x, start_y, length);
        dest.fill(self.pixel);
    }
}
