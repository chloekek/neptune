use crate::Bezier;
use crate::Format;
use crate::Matrix;
use crate::Paint;
use crate::PixelMap;
use crate::Vector;
use crate::with_blitter;

/// High-level interface for drawing shapes.
pub trait Canvas
{
    /// Data type for a single pixel.
    ///
    /// While canvases don’t necessarily write pixels
    /// (perhaps they generate vector graphics instead),
    /// this type gives them an idea of color and alpha values.
    /// See [`Format::Pixel`] for more information on pixel types.
    type Pixel: Copy;

    /// Draw a filled rectangle starting at `start`
    /// and extending `extent` units to the bottom right,
    /// transformed with the given matrix.
    fn rectangle(
        &mut self,
        matrix: Matrix,
        start: Vector,
        extent: Vector,
        paint: Paint<Self::Pixel>,
    );

    /// Draw a single Bézier curve,
    /// transformed with the given matrix.
    ///
    /// The provided implementation samples the Bézier curve
    /// and draws a 1×1 rectangle at each sample.
    fn bezier(
        &mut self,
        matrix: Matrix,
        bezier: Bezier,
        paint: Paint<Self::Pixel>,
    )
    {
        // FIXME: This algorithm is very bad.
        let samples = 100;
        for i in 0 .. samples {
            let t = i as f64 / samples as f64;
            let point = bezier.evaluate(t);
            self.rectangle(
                matrix,
                point,
                Vector{x: 1.0, y: 1.0},
                paint,
            );
        }
    }
}

/// Canvas that draws onto a pixel map.
///
/// The canvas operations will automatically construct suitable blitters.
/// Only the format and the pixel map are configurable.
pub struct PixelMapCanvas<'a, F>
    where F: Format
{
    format: F,
    pixel_map: PixelMap<'a, F::Pixel>,
}

impl<'a, F> PixelMapCanvas<'a, F>
    where F: Format
{
    /// Create a new canvas.
    pub fn new(format: F, pixel_map: PixelMap<'a, F::Pixel>) -> Self
    {
        Self{format, pixel_map}
    }
}

impl<'a, F> Canvas for PixelMapCanvas<'a, F>
    where F: Format
{
    type Pixel = F::Pixel;

    fn rectangle(
        &mut self,
        matrix: Matrix,
        start: Vector,
        extent: Vector,
        paint: Paint<Self::Pixel>,
    )
    {
        // TODO: Draw rotated and skewed rectangles correctly.

        let t_start = matrix * start;
        let t_extent = matrix * (start + extent) - t_start;

        let format = &self.format;
        let pixel_map = &mut self.pixel_map;

        with_blitter(format, paint, |blitter| {
            blitter.rectangle(
                pixel_map,
                t_start.x as u32,
                t_start.y as u32,
                t_extent.x as u32,
                t_extent.y as u32,
            );
        });
    }
}
