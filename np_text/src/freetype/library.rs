use crate::freetype::Error;
use crate::freetype::Result;

use np_freetype_sys::*;
use std::ptr;

/// FreeType library.
pub struct Library
{
    ft_library: FT_Library,
}

impl Library
{
    /// Initialize the FreeType library.
    pub fn init() -> Result<Self>
    {
        let mut ft_library = ptr::null_mut();
        // SAFETY: FT_Init_FreeType is always safe.
        let status = unsafe { FT_Init_FreeType(&mut ft_library) };
        Error::new(status)?;
        Ok(Self{ft_library})
    }

    /// The wrapped FreeType library.
    pub fn ft_library(&self) -> FT_Library
    {
        self.ft_library
    }
}

impl Drop for Library
{
    fn drop(&mut self)
    {
        // SAFETY: The library is valid by construction.
        unsafe { FT_Done_FreeType(self.ft_library); }
    }
}

// SAFETY: FT_Library can be used across threads with external synchronization.
unsafe impl Send for Library
{
}
