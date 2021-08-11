use std::marker::PhantomData;
use std::slice;

/// Mutable reference to a 2D array.
pub struct PixelMapMut<'a, T>
{
    _slice: PhantomData<&'a mut [T]>,
    pixels: *mut T,
    pitch: u32,
    extent_x: u32,
    extent_y: u32,
}

unsafe impl<'a, T> Send for PixelMapMut<'a, T>
    where T: Send
{
}

impl<'a, T> PixelMapMut<'a, T>
{
    /// Create a pixel map from a slice of pixels.
    ///
    /// The value of `pitch` is the number of pixels per row.
    /// This may be longer than the value of `extent_x`,
    /// in case a rectangular slice of the pixel buffer is taken.
    /// If the number of pixels `pitch * extent_y`
    /// does not match the number of elements in `pixels`,
    /// this function returns `None`.
    pub fn new(
        pixels: &'a mut [T],
        pitch: u32,
        extent_x: u32,
        extent_y: u32,
    ) -> Option<Self>
    {
        // Check that the pitch is not shorter than the extent.
        // That would be very bad and cause out of bounds access.
        if extent_x > pitch {
            return None;
        }

        // Check that the length of the slice
        // corresponds to the width and height.
        let expected_len = u32::checked_mul(pitch, extent_y)? as usize;
        let len_as_expected = pixels.len() == expected_len;
        if !len_as_expected { return None; }

        // Construct slice blitter.
        let pixels = pixels.as_mut_ptr();
        Some(Self{_slice: PhantomData, pixels, pitch, extent_x, extent_y})
    }
}

impl<'a, T> PixelMapMut<'a, T>
{
    /// The width and height of the pixel map.
    pub fn extent(&self) -> (u32, u32)
    {
        (self.extent_x, self.extent_y)
    }

    /// Slice of a line segment starting at `start`
    /// and extending `length` pixels to the right.
    ///
    /// The returned slice may be shorter than `length`
    /// if the line segment is (partially) out of bounds.
    /// This function does not panic on out of bounds conditions.
    pub fn horizontal_mut(&mut self, start_x: u32, start_y: u32, length: u32)
        -> &mut [T]
    {
        // Check that the starting vertex is in bounds.
        if start_x >= self.extent_x { return &mut []; }
        if start_y >= self.extent_y { return &mut []; }

        // Shorten the length to fit within the bounds.
        let length = u32::min(length, self.extent_x - start_x);

        // Compute the start and end offsets of the vertices.
        let start_index = start_x + start_y * self.pitch;
        let end_index = start_index + length;

        // SAFETY: The indices are in bounds as per the above checks.
        unsafe {
            debug_assert!(
                end_index >= start_index,
                "{:?} >= {:?}",
                end_index,
                start_index,
            );
            slice::from_raw_parts_mut(
                self.pixels.add(start_index as usize),
                end_index as usize - start_index as usize,
            )
        }
    }
}
