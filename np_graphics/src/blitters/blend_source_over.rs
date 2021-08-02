use crate::Blitter;
use crate::Format;
use crate::PixelMap;

use cgmath::Vector2;

/// Blitter that implements the [`SourceOver`] blend mode.
///
/// [`SourceOver`]: `crate::BlendMode::SourceOver`
pub struct BlendSourceOverBlitter<F>
    where F: Format
{
    format: F,
    pixel: F::Pixel,
}

impl<F> BlendSourceOverBlitter<F>
    where F: Format
{
    /// Create a new blitter.
    pub fn new(format: F, pixel: F::Pixel) -> Self
    {
        Self{format, pixel}
    }
}

impl<F> Blitter for BlendSourceOverBlitter<F>
    where F: Format
{
    type Pixel = F::Pixel;

    fn horizontal(
        &self,
        map: &mut PixelMap<F::Pixel>,
        start: Vector2<u32>,
        length: u32,
    )
    {
        let dest = map.horizontal_mut(start, length);
        self.format.blend_source_over(dest, self.pixel);
    }
}
