use np_graphics::PathBuf;

/// Information about a glyph in a typeface.
///
/// A glyph is a visual representation of a character in a typeface.
/// This structure contains information parsed from a typeface.
/// To obtain a glyph, call [`Typeface::glyph`] after obtaining a typeface.
///
/// [`Typeface::glyph`]: `crate::Typeface::glyph`
#[derive(Debug)]
pub struct Glyph
{
    /// The horizontal advance of the glyph.
    pub advance_x: f64,

    /// What the glyph looks like.
    ///
    /// Whether this is a bitmap or an outline depends on the typeface;
    /// this crate does not perform a conversion
    /// from one representation into the other.
    pub image: Image,
}

/// What a glyph looks like.
#[derive(Debug)]
pub enum Image
{
    /// Bitmap image of a glyph.
    ///
    /// This is not yet implemented,
    /// hence the use of `Infallible`.
    Bitmap(std::convert::Infallible),

    /// Vector image of a glyph,
    /// given by its outline path.
    Outline(PathBuf),
}
