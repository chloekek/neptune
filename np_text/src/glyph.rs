use np_graphics::Bezier;

/// Information about a glyph in a face.
#[derive(Debug)]
pub struct Glyph
{
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
    /// hence the use of Infallible.
    Bitmap(std::convert::Infallible),

    /// Vector image of a glyph,
    /// given by its outline path.
    Outline(Vec<Bezier>),
}
