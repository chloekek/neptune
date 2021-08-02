use crate::BlendMode;
use crate::Format;
use crate::Paint;
use crate::PixelMap;
use crate::blitters::*;

use cgmath::Vector2;

/// Specialized code for drawing pixels onto a pixel map.
///
/// Drawing pixels is an operation that happens a lot,
/// and depending on the blend mode and pixel value
/// different implementations may be used.
/// The [`with_blitter`] function will
/// automatically select a suitable implementation,
/// but you can instantiate a specific implementation if desired.
pub trait Blitter
{
    /// Data type for a single pixel.
    ///
    /// The pixel type a blitter is suitable for
    /// is the same as the pixel type of the pixel maps
    /// that can be passed to the methods of the blitter.
    /// See [`Format::Pixel`] for more information on pixel types.
    type Pixel;

    /// Draw a line segment starting at `start`
    /// and extending `length` pixels to the right.
    fn horizontal(
        &self,
        map: &mut PixelMap<Self::Pixel>,
        start: Vector2<u32>,
        length: u32,
    );

    /// Draw a filled rectangle starting at `start`
    /// and extending `extent` pixels to the bottom right.
    ///
    /// The default implementation calls [`Blitter::horizontal`]
    /// for each Y coordinate in the start–extent range.
    fn rectangle(
        &self,
        map: &mut PixelMap<Self::Pixel>,
        start: Vector2<u32>,
        extent: Vector2<u32>,
    )
    {
        let end_y = u32::saturating_add(start.y, extent.y);
        for y in start.y .. end_y {
            self.horizontal(map, Vector2::new(start.x, y), extent.x);
        }
    }
}

/// Common implementation for [`create_blitter`] and [`with_blitter`].
///
/// I couldn’t find another way to share this code
/// that would satisfy the Rust type checker,
/// due to its polymorphic nature.
/// A macro will have to do.
macro_rules! select_blitter
{
    ($format:expr, $paint:expr, $f:expr) => {
        {
            let format = $format;
            let Paint{blend_mode, pixel} = $paint;
            match blend_mode {
                BlendMode::Source =>
                    $f(BlendSourceBlitter::new(pixel)),
                BlendMode::SourceOver =>
                    if !format.is_visible(pixel) {
                        $f(BlendDestinationBlitter::new())
                    } else if format.is_opaque(pixel) {
                        $f(BlendSourceBlitter::new(pixel))
                    } else {
                        $f(BlendSourceOverBlitter::new(format, pixel))
                    },
                BlendMode::Destination =>
                    $f(BlendDestinationBlitter::new()),
            }
        }
    };
}

/// Choose the optimal blitter for a paint.
///
/// In some cases a more optimal blitter can be chosen for a given paint
/// than the canonical one for that paint’s blend mode.
/// This function replaces the blend mode
/// with a less computationally intensive one
/// if this would have no visible effect in the final image.
pub fn create_blitter<'a, F>(format: F, paint: Paint<F::Pixel>)
    -> Box<dyn 'a + Blitter<Pixel=F::Pixel>>
    where F: 'a + Format
{
    select_blitter!(format, paint, move |b| Box::new(b))
}

/// Choose the optimal blitter for a paint.
///
/// This function is equivalent to [`create_blitter`],
/// but it allocates the blitter on the stack rather than on the heap,
/// and passes it to the given function rather than returning it.
pub fn with_blitter<F, R>(
    format: F,
    paint: Paint<F::Pixel>,
    f: impl FnOnce(&mut dyn Blitter<Pixel=F::Pixel>) -> R,
) -> R
    where F: Format
{
    select_blitter!(format, paint, move |mut b| f(&mut b))
}
