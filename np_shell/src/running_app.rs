use crate::AppId;

use np_unix::Mmap;
use std::os::unix::net::UnixDatagram;
use std::process::Child;
use std::time::Instant;

/// Information about a running app.
pub struct RunningApp
{
    /// Identifies which app is running.
    pub app_id: AppId,

    /// Handle to the process of the running app.
    pub process: Child,

    /// The pixel buffer shared with the app.
    pub pixels: Mmap,

    /// The socket for communicating with the app.
    pub socket: UnixDatagram,

    /// When the app was last opened.
    pub last_opened: Instant,
}
