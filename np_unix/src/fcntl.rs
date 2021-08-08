use std::io::Error;
use std::io::Result;
use std::os::raw::c_int;
use std::os::unix::io::AsRawFd;

pub fn fcntl_f_add_seals<T>(fd: &T, arg: c_int) -> Result<()>
    where T: AsRawFd
{
    let status = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_ADD_SEALS, arg) };
    if status == -1 { return Err(Error::last_os_error()); }
    Ok(())
}

pub fn fcntl_f_setfd<T>(fd: &T, arg: c_int) -> Result<()>
    where T: AsRawFd
{
    let status = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, arg) };
    if status == -1 { return Err(Error::last_os_error()); }
    Ok(())
}
