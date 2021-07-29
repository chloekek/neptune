use std::ffi::CStr;
use std::io::Error;
use std::io::Result;
use std::os::raw::c_ulong;
use std::ptr;

pub fn mount(
    source: &CStr,
    target: &CStr,
    filesystemtype: &CStr,
    mountflags: c_ulong,
    data: Option<&CStr>,
) -> Result<()>
{
    let ok = unsafe {
        libc::mount(
            source.as_ptr(),
            target.as_ptr(),
            filesystemtype.as_ptr(),
            mountflags,
            data.map(|s| s.as_ptr())
                .map(|p| p.cast())
                .unwrap_or(ptr::null()),
        )
    };
    if ok == -1 { return Err(Error::last_os_error()); }
    Ok(())
}
