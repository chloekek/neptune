use crate::Sigset;

use std::io::Error;
use std::io::Result;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

pub fn sigprocmask(how: c_int, set: &Sigset) -> Result<Sigset>
{
    let mut oldset = MaybeUninit::uninit();
    let status = unsafe {
        libc::sigprocmask(how, set.as_ptr(), oldset.as_mut_ptr())
    };
    if status == -1 { return Err(Error::last_os_error()); }
    let oldset = unsafe { oldset.assume_init() };
    let oldset = unsafe { Sigset::from_raw(oldset) };
    Ok(oldset)
}
