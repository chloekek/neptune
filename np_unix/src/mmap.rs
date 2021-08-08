use std::io::Error;
use std::io::Result;
use std::mem::size_of;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::ptr;
use std::slice;

/// Owned memory mapped with [`libc::mmap`].
///
/// The [`Drop`] impl for this type will call [`libc::munmap`].
pub struct Mmap
{
    addr: *mut c_void,
    length: libc::size_t,
}

impl Mmap
{
    pub fn mmap(
        length: libc::size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> Result<Self>
    {
        let addr = unsafe {
            libc::mmap(ptr::null_mut(), length, prot, flags, fd, offset)
        };
        if addr == libc::MAP_FAILED { return Err(Error::last_os_error()); }
        Ok(Self{addr, length})
    }

    /// The mapped memory as a slice.
    ///
    /// # Safety
    ///
    ///  - The mapped memory must be suitably aligned for `T`.
    ///    This is almost always the case as page alignment is huge.
    ///  - The data must be in a suitable representation of `T`.
    ///  - Beware that the data may be changed from other processes
    ///    without any synchronization.
    pub unsafe fn as_mut<T>(&mut self) -> &mut [T]
    {
        slice::from_raw_parts_mut(
            self.addr as *mut T,
            self.length / size_of::<T>(),
        )
    }
}

impl Drop for Mmap
{
    fn drop(&mut self)
    {
        unsafe { libc::munmap(self.addr, self.length); }
    }
}

// SAFETY: There is nothing restricting you from using
// SAFETY: mapped memory from a different thread.
unsafe impl Send for Mmap
{
}
