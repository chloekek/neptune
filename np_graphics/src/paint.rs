/// Pair of blend mode and pixel value.
///
/// This is just a convenient type as the two are often passed together.
/// Any combination of values is valid, so this type has no smart constructor.
#[derive(Clone, Copy)]
pub struct Paint<T>
{
    /// The blend mode of the paint.
    pub blend_mode: BlendMode,

    /// The pixel value of the paint.
    pub pixel: T,
}

/// How to combine two pixels when one is drawn on top of the other.
///
/// The documentation for each variant shows the associated formulae
/// used for applying the blend mode when combining (_blending_) pixels.
/// In these formulae,
/// \\( a \\) refers to the alpha component of the pixel,
/// \\( c \\) refers to each color component of the pixel,
/// \\( s \\) refers to the source pixel,
/// \\( d \\) refers to the destination pixel, and
/// \\( r \\) refers to the pixel after blending.
/// The formulae assume a value range of \\( [ 0, 1 ] \\) for each component,
/// but the implementations of the corresponding [`Format`] methods
/// may use any component representation
/// and the formulae should be scaled accordingly.
///
/// [`Format`]: `crate::Format`
#[derive(Clone, Copy, Debug)]
pub enum BlendMode
{
    /// <table style="width: auto;">
    ///     <tr>
    ///         <th>Alpha</th>
    ///         <td>\( a_r = a_s \)</td>
    ///     </tr>
    ///     <tr>
    ///         <th>Color</th>
    ///         <td>\( c_r = c_s \)</td>
    ///     </tr>
    /// </table>
    Source,

    /// <table style="width: auto;">
    ///     <tr>
    ///         <th>Alpha</th>
    ///         <td>\( a_r = a_s + a_d(1 - a_s) \)</td>
    ///     </tr>
    ///     <tr>
    ///         <th>Color</th>
    ///         <td>\( c_r = \frac{a_s c_s + a_d c_d (1 - a_s)}{a_r} \)</td>
    ///     </tr>
    /// </table>
    SourceOver,

    /// <table style="width: auto;">
    ///     <tr>
    ///         <th>Alpha</th>
    ///         <td>\( a_r = a_d \)</td>
    ///     </tr>
    ///     <tr>
    ///         <th>Color</th>
    ///         <td>\( c_r = c_d \)</td>
    ///     </tr>
    /// </table>
    Destination,
}
