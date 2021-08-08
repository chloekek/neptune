use std::io::Error;
use std::io::Result;
use std::os::unix::io::AsRawFd;

pub fn ftruncate<T>(fd: &T, length: libc::off_t) -> Result<()>
    where T: AsRawFd
{
    let status = unsafe { libc::ftruncate(fd.as_raw_fd(), length) };
    if status == -1 { return Err(Error::last_os_error()); }
    Ok(())
}
