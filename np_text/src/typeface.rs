use crate::Glyph;
use crate::Image;

use np_graphics::PathBuf;
use np_graphics::Vector;

/// Information about a typeface, including its glyphs.
///
/// To obtain a typeface, call [`FontFile::typeface`] after opening a font file.
///
/// [`FontFile::typeface`]: `crate::FontFile::typeface`
pub struct Typeface<'a>
{
    pub (crate) inner: ttf_parser::Face<'a>,
}

impl<'a> Typeface<'a>
{
    /// Glyph with the given glyph identifier.
    ///
    /// If the glyph cannot be parsed,
    /// this function returns [`None`].
    pub fn glyph(&self, glyph_id: u16) -> Option<Glyph>
    {
        let glyph_id = ttf_parser::GlyphId(glyph_id);
        let advance_x = self.inner.glyph_hor_advance(glyph_id)? as f64;
        let image = self.glyph_image(glyph_id)?;
        Some(Glyph{advance_x, image})
    }

    fn glyph_image(&self, glyph_id: ttf_parser::GlyphId) -> Option<Image>
    {
        let mut builder = OutlineBuilder::new();
        self.inner.outline_glyph(glyph_id, &mut builder)?;
        Some(Image::Outline(builder.path))
    }
}

struct OutlineBuilder
{
    path: PathBuf,
}

impl OutlineBuilder
{
    fn new() -> Self
    {
        Self{path: PathBuf::new()}
    }
}

impl ttf_parser::OutlineBuilder for OutlineBuilder
{
    fn move_to(&mut self, x: f32, y: f32)
    {
        let to = Vector{x: x as f64, y: y as f64};
        self.path.push_move(to);
    }

    fn line_to(&mut self, x: f32, y: f32)
    {
        let p1 = Vector{x: x as f64, y: y as f64};
        self.path.push_linear(p1);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32)
    {
        let p1 = Vector{x: x1 as f64, y: y1 as f64};
        let p2 = Vector{x: x as f64, y: y as f64};
        self.path.push_quadratic(p1, p2);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32)
    {
        let p1 = Vector{x: x1 as f64, y: y1 as f64};
        let p2 = Vector{x: x2 as f64, y: y2 as f64};
        let p3 = Vector{x: x as f64, y: y as f64};
        self.path.push_cubic(p1, p2, p3);
    }

    fn close(&mut self)
    {
    }
}
