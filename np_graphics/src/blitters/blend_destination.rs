use crate::Blitter;
use crate::PixelMapMut;

use std::marker::PhantomData;

/// Blitter that implements the [`Destination`] blend mode.
///
/// [`Destination`]: `crate::BlendMode::Destination`
#[derive(Clone, Copy, Debug)]
pub struct BlendDestinationBlitter<T>
{
    _pixel: PhantomData<T>,
}

impl<T> BlendDestinationBlitter<T>
{
    /// Create a new blitter.
    pub fn new() -> Self
    {
        Self{_pixel: PhantomData}
    }
}

impl<T> Blitter for BlendDestinationBlitter<T>
{
    type Pixel = T;

    fn horizontal(
        &self,
        _map: &mut PixelMapMut<T>,
        _start_x: u32,
        _start_y: u32,
        _length: u32,
    )
    {
    }

    fn rectangle(
        &self,
        _map: &mut PixelMapMut<Self::Pixel>,
        _start_x: u32,
        _start_y: u32,
        _extent_x: u32,
        _extent_y: u32,
    )
    {
    }
}
