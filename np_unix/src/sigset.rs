use std::io::Error;
use std::io::Result;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

pub struct Sigset
{
    inner: libc::sigset_t,
}

impl Sigset
{
    pub fn sigemptyset() -> Result<Self>
    {
        let mut inner = MaybeUninit::uninit();
        let status = unsafe { libc::sigemptyset(inner.as_mut_ptr()) };
        if status == -1 { return Err(Error::last_os_error()); }
        let inner = unsafe { inner.assume_init() };
        Ok(Self{inner})
    }

    pub fn sigaddset(&mut self, signum: c_int) -> Result<()>
    {
        let status = unsafe { libc::sigaddset(&mut self.inner, signum) };
        if status == -1 { return Err(Error::last_os_error()); }
        Ok(())
    }

    /// Create a [`Sigset`] from a `sigset_t`.
    pub unsafe fn from_raw(inner: libc::sigset_t) -> Self
    {
        Self{inner}
    }

    /// Obtain the `sigset_t` from a [`Sigset`].
    pub fn as_ptr(&self) -> *const libc::sigset_t
    {
        &self.inner
    }
}
