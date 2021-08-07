use np_freetype_sys::*;
use std::error;
use std::ffi::CStr;
use std::fmt;
use std::io;
use std::result;

/// Result type for FreeType operations.
pub type Result<T> = result::Result<T, Error>;

/// FreeType error.
#[derive(Debug)]
pub struct Error
{
    ft_error: FT_Error,
}

impl Error
{
    /// Wrap a FreeType error, or return [`Ok`] if there was no error.
    pub fn new(ft_error: FT_Error) -> Result<()>
    {
        if ft_error == 0 {
            Ok(())
        } else {
            Err(Self{ft_error})
        }
    }
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // SAFETY: FT_Error_String is always safe.
        let string = unsafe { FT_Error_String(self.ft_error) };
        if string.is_null() {
            write!(f, "FT_Error: {}", self.ft_error)
        } else {
            // SAFETY: FT_Error_String returns a null-terminated string.
            let string = unsafe { CStr::from_ptr(string) };
            let string = string.to_string_lossy();
            write!(f, "{}", string)
        }
    }
}

impl error::Error for Error
{
}

impl From<Error> for io::Error
{
    fn from(other: Error) -> Self
    {
        Self::new(io::ErrorKind::Other, other)
    }
}
