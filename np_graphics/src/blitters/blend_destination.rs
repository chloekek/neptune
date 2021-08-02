use crate::Blitter;
use crate::PixelMap;

use cgmath::Vector2;
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
        _map: &mut PixelMap<T>,
        _start: Vector2<u32>,
        _length: u32,
    )
    {
    }

    fn rectangle(
        &self,
        _map: &mut PixelMap<Self::Pixel>,
        _start: Vector2<u32>,
        _extent: Vector2<u32>,
    )
    {
    }
}
