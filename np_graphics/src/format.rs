/// Characteristics of pixels in an image.
///
/// This trait provides methods for querying and manipulating pixels.
/// The implementation chooses the in-memory format of the pixels
/// and thereby of the images those pixels make up.
/// The exact behavior of drawing operations depends on the format,
/// and the format is free to pick any restrictions on pixel values.
pub trait Format
{
    /// Data type for a single pixel.
    ///
    /// Values of this type make up the images onto which shapes are drawn,
    /// as well as intermediate pixel values passed to drawing operations.
    /// This data type should contain each color component of the pixel
    /// as well as any alpha component if applicable.
    /// The [`Copy`] bound makes it convenient to work with,
    /// and let’s be honest, types that don’t implement [`Copy`]
    /// really aren’t suitable for use as pixels in images.
    type Pixel: Copy;

    /// Whether the given pixel is visible.
    ///
    /// Pixels with an alpha value of zero are not visible.
    /// Pixels with a non-zero alpha value are visible.
    fn is_visible(&self, pixel: Self::Pixel) -> bool;

    /// Whether the given pixel is opaque.
    ///
    /// Pixels with an alpha value of one are opaque.
    /// Pixels with a non-one alpha value are not opaque.
    fn is_opaque(&self, pixel: Self::Pixel) -> bool;

    /// Blend `source` over each pixel in `dest`
    /// using the formula for [`SourceOver`].
    ///
    /// This operation is presented using a slice
    /// so that it can be easily vectorized.
    /// Nonetheless, the implementation must behave _as if_
    /// it were called for each destination pixel separately.
    ///
    /// [`SourceOver`]: `crate::BlendMode::SourceOver`
    fn blend_source_over(&self, dest: &mut [Self::Pixel], source: Self::Pixel);
}

impl<'a, F> Format for &'a F
    where F: Format
{
    type Pixel = F::Pixel;

    fn is_visible(&self, pixel: Self::Pixel) -> bool
    {
        (**self).is_visible(pixel)
    }

    fn is_opaque(&self, pixel: Self::Pixel) -> bool
    {
        (**self).is_opaque(pixel)
    }

    fn blend_source_over(&self, dest: &mut [Self::Pixel], source: Self::Pixel)
    {
        (**self).blend_source_over(dest, source)
    }
}
