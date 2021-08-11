use crate::Format;
use crate::Matrix;
use crate::Paint;
use crate::PathBuf;
use crate::PixelMap;
use crate::Vector;
use crate::path::Instruction;
use crate::path::bezier_cubic;
use crate::path::bezier_quadratic;
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

    /// Draw a filled path starting at the origin
    /// transformed with the given matrix.
    ///
    /// To start the path elsewhere, provide a translation matrix
    /// or make sure the path begins with a move instruction.
    fn path(
        &mut self,
        matrix: Matrix,
        path: &PathBuf,
        paint: Paint<Self::Pixel>,
    );
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

        // Transform start and extent.
        let mut t_start = matrix * start;
        let mut t_extent = matrix * (start + extent) - t_start;

        // Make sure start comes before end.
        if t_extent.x < 0.0 {
            t_start.x += t_extent.x;
            t_extent.x = -t_extent.x;
        }
        if t_extent.y < 0.0 {
            t_start.y += t_extent.y;
            t_extent.y = -t_extent.y;
        }

        // Borrow fields separately.
        let format = &self.format;
        let pixel_map = &mut self.pixel_map;

        // Draw rectangle using blitter.
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

    fn path(
        &mut self,
        matrix: Matrix,
        path: &PathBuf,
        paint: Paint<Self::Pixel>,
    )
    {
        // TODO: Fill the path instead of drawing the outline only. :P

        // Borrow fields separately.
        let format = &self.format;
        let pixel_map = &mut self.pixel_map;

        // Draw using the blitter for this paint.
        with_blitter(format, paint, |blitter| {

            // The Bézier curves will be divided into line segments.
            // The line segments are drawn using the blitter.
            let mut line_segment = |p0: Vector, p1: Vector| {
                blitter.line_segment(
                    pixel_map,
                    p0.x as u32,
                    p0.y as u32,
                    p1.x as u32,
                    p1.y as u32,
                );
            };

            // Keep track of the p0 point across iterations.
            // After each iteraiton, this is moved to
            // the end point of each Bézier curve.
            let mut p0 = Vector{x: 0.0, y: 0.0};

            // Perform each instruction.
            for instruction in path.instructions() {

                // Apply the matrix to each point in the instruction.
                // As far as I know this works as expected
                // with Bézier control points.
                let instruction = matrix * instruction;

                match instruction {
                    Instruction::Move(to) => {
                        p0 = to;
                    },
                    Instruction::Linear(p1) => {
                        line_segment(p0, p1);
                        p0 = p1;
                    },
                    Instruction::Quadratic(p1, p2) => {
                        // TODO: Smarter heuristics to generate line segments.
                        let half = bezier_quadratic(p0, p1, p2, 0.5);
                        line_segment(p0, half);
                        line_segment(half, p2);
                        p0 = p2;
                    },
                    Instruction::Cubic(p1, p2, p3) => {
                        // TODO: Smarter heuristics to generate line segments.
                        let half = bezier_cubic(p0, p1, p2, p3, 0.5);
                        line_segment(p0, half);
                        line_segment(half, p3);
                        p0 = p3;
                    },
                }
            }

        });
    }
}
