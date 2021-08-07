use crate::freetype::Error;
use crate::freetype::Library;
use crate::freetype::Result;

use np_freetype_sys::*;
use std::ffi::CStr;
use std::ptr;
use std::sync::Mutex;

/// FreeType face.
pub struct Face<'a>
{
    library: &'a Mutex<Library>,
    ft_face: FT_Face,
}

impl<'a> Face<'a>
{
    /// Create a FreeType face from a file.
    ///
    /// The given mutex will be locked during creation,
    /// because this operation may not occur without synchronization.
    /// The mutex will also be locked whilst the face is being dropped.
    pub fn new(
        library: &'a Mutex<Library>,
        filepathname: &CStr,
        face_index: FT_Long,
    ) -> Result<Self>
    {
        let mut ft_face = ptr::null_mut();
        let status = {
            let lock = library.lock().unwrap();
            // SAFETY: The library mutex is now locked.
            // SAFETY: The path is null-terminated.
            unsafe {
                FT_New_Face(
                    lock.ft_library(),
                    filepathname.as_ptr(),
                    face_index,
                    &mut ft_face,
                )
            }
        };
        Error::new(status)?;
        Ok(Self{library, ft_face})
    }

    /// Set the nominal size in points.
    pub fn set_char_size(
        &mut self,
        char_width: FT_F26Dot6,
        char_height: FT_F26Dot6,
        horz_resolution: FT_UInt,
        vert_resolution: FT_UInt,
    ) -> Result<()>
    {
        // SAFETY: The face is valid by construction.
        let status = unsafe {
            FT_Set_Char_Size(
                self.ft_face,
                char_width,
                char_height,
                horz_resolution,
                vert_resolution,
            )
        };
        Error::new(status)?;
        Ok(())
    }

    /// Load a glyph into the glyph slot of the face.
    pub fn load_glyph(
        &mut self,
        glyph_index: FT_UInt,
        load_flags: FT_Int32,
    ) -> Result<()>
    {
        // SAFETY: The face is valid by construction.
        // SAFETY: FreeType will handle out of bounds access safely.
        // SAFETY: The load flags do not incur any unsafety.
        let status = unsafe {
            FT_Load_Glyph(self.ft_face, glyph_index, load_flags)
        };
        Error::new(status)?;
        Ok(())
    }

    /// The glyph slot of the face.
    pub fn glyph(&mut self) -> &mut FT_GlyphSlotRec_
    {
        // SAFETY: The face is valid by construction.
        unsafe { &mut *(*self.ft_face).glyph }
    }
}

impl<'a> Drop for Face<'a>
{
    fn drop(&mut self)
    {
        let _lock = self.library.lock().unwrap();
        // SAFETY: The library mutex is now locked.
        // SAFETY: The face is valid by construction.
        unsafe { FT_Done_Face(self.ft_face); }
    }
}

// SAFETY: The Drop impl locks the library prior to calling FT_Done_Face.
unsafe impl<'a> Send for Face<'a>
{
}
