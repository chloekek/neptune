use std::ffi::CStr;
use std::io::Error;
use std::io::Result;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_uint;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;

// memfd_create is unfortunately not yet in the libc crate.
extern "C"
{
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
}

/// Owned file descriptor created with `libc::memfd_create`.
///
/// The [`Drop`] impl for this type will call [`libc::close`].
pub struct Memfd
{
    inner: c_int,
}

impl Memfd
{
    pub fn memfd_create(name: &CStr, flags: c_uint) -> Result<Self>
    {
        let inner = unsafe { memfd_create(name.as_ptr(), flags) };
        if inner == -1 { return Err(Error::last_os_error()); }
        Ok(Self{inner})
    }
}

impl Drop for Memfd
{
    fn drop(&mut self)
    {
        unsafe { libc::close(self.inner); }
    }
}

impl AsRawFd for Memfd
{
    fn as_raw_fd(&self) -> RawFd
    {
        self.inner
    }
}
