use np_unix::c_str;
use np_unix::mount;
use std::io::Result;

/// Mount proc(5) at `/proc`.
pub fn mount_proc() -> Result<()>
{
    mount(
        /* source */ c_str!("proc"),
        /* target */ c_str!("/proc"),
        /* filesystemtype */ c_str!("proc"),
        /* mountflags */ 0,
        /* data */ None,
    )
}

/// Mount sysfs(5) at `/sys`.
pub fn mount_sys() -> Result<()>
{
    mount(
        /* source */ c_str!("sys"),
        /* target */ c_str!("/sys"),
        /* filesystemtype */ c_str!("sysfs"),
        /* mountflags */ 0,
        /* data */ None,
    )
}
