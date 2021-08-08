//! This crate provides an interface to Unix system calls.
//!
//! Unlike the `libc` crate, this crate provides safe interfaces.
//! Functions are named after the underlying system calls,
//! so no further documentation beyond the man pages is necessary.
//!
//! For convenience, this crate also provides a C string literal macro.
//! This can be used for creating literal string arguments to system calls.

pub use self::fcntl::*;
pub use self::ftruncate::*;
pub use self::memfd::*;
pub use self::mmap::*;
pub use self::mount::*;
pub use self::signalfd::*;
pub use self::sigprocmask::*;
pub use self::sigset::*;

mod fcntl;
mod ftruncate;
mod memfd;
mod mmap;
mod mount;
mod signalfd;
mod sigprocmask;
mod sigset;

/// Create a `&'static CStr` from a string literal.
///
/// # Example
///
/// ```
/// # use np_unix::c_str;
/// # use std::ffi::CStr;
/// let c_str: &'static CStr = c_str!("hello");
/// assert_eq!(c_str.to_bytes(), b"hello");
/// ```
#[macro_export]
macro_rules! c_str
{
    ($lit:expr) => {
        unsafe {
            std::ffi::CStr::from_ptr(
                concat!($lit, "\0").as_ptr()
                    as *const std::os::raw::c_char
            )
        }
    };
}
