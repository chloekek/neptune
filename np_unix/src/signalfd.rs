use crate::Sigset;

use std::io::Error;
use std::io::Result;
use std::mem::MaybeUninit;
use std::mem::size_of_val;
use std::os::raw::c_int;
use std::os::raw::c_void;

/// Owned file descriptor created with [`libc::signalfd`].
///
/// The [`Drop`] impl for this type will call [`libc::close`].
pub struct Signalfd
{
    inner: c_int,
}

impl Signalfd
{
    pub fn signalfd(mask: &Sigset) -> Result<Self>
    {
        let inner = unsafe {
            libc::signalfd(-1, mask.as_ptr(), libc::SFD_CLOEXEC)
        };
        if inner == -1 { return Err(Error::last_os_error()); }
        Ok(Self{inner})
    }

    pub fn read(&self) -> Result<libc::signalfd_siginfo>
    {
        let mut siginfo = MaybeUninit::uninit();
        let status = unsafe {
            libc::read(
                self.inner,
                siginfo.as_mut_ptr() as *mut c_void,
                size_of_val(&siginfo),
            )
        };
        if status == -1 { return Err(Error::last_os_error()); }
        let siginfo = unsafe { siginfo.assume_init() };
        Ok(siginfo)
    }
}

impl Drop for Signalfd
{
    fn drop(&mut self)
    {
        unsafe { libc::close(self.inner); }
    }
}
