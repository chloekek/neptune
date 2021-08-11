use crate::Blitter;
use crate::Format;
use crate::PixelMapMut;

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
        map: &mut PixelMapMut<F::Pixel>,
        start_x: u32,
        start_y: u32,
        length: u32,
    )
    {
        let dest = map.horizontal_mut(start_x, start_y, length);
        self.format.blend_source_over(dest, self.pixel);
    }
}
