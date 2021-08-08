use np_graphics::PathBuf;

/// Information about a glyph in a face.
#[derive(Debug)]
pub struct Glyph
{
    /// The horizontal advance of the glyph.
    pub advance_x: f64,

    /// What the glyph looks like.
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
