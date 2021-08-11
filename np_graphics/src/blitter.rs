use crate::BlendMode;
use crate::Format;
use crate::Paint;
use crate::PixelMapMut;
use crate::blitters::*;

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
        map: &mut PixelMapMut<Self::Pixel>,
        start_x: u32,
        start_y: u32,
        length: u32,
    );

    /// Draw a filled rectangle starting at `start`
    /// and extending `extent` pixels to the bottom right.
    ///
    /// The provided implementation calls [`Blitter::horizontal`]
    /// for each Y coordinate in the start–extent range.
    fn rectangle(
        &self,
        map: &mut PixelMapMut<Self::Pixel>,
        start_x: u32,
        start_y: u32,
        extent_x: u32,
        extent_y: u32,
    )
    {
        let end_y = u32::saturating_add(start_y, extent_y);
        for y in start_y .. end_y {
            self.horizontal(map, start_x, y, extent_x);
        }
    }

    /// Draw a line segment starting at `start` and ending at `end`.
    ///
    /// The provided implementation uses Bresenham’s line algorithm,
    /// calling [`Blitter::horizontal`] with length 1 for each pixel.
    fn line_segment(
        &self,
        map: &mut PixelMapMut<Self::Pixel>,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32
    )
    {
        // Bresenham’s line algorithm.

        let mut x0 = start_x as i64;
        let mut y0 = start_y as i64;
        let x1 = end_x as i64;
        let y1 = end_y as i64;

        let dx = i64::abs(x1 - x0);
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -i64::abs(y1 - y0);
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.horizontal(map, x0 as u32, y0 as u32, 1);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
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
