use std::fs::File;
use std::io::Result;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::ptr;
use std::thread;
use std::time::Duration;

fn main() -> Result<()>
{
    println!("Hello, world!");

    unsafe {
        let ok = libc::mount(
            b"sysfs\0".as_ptr() as *const libc::c_char,
            b"/sys\0".as_ptr() as *const libc::c_char,
            b"sysfs\0".as_ptr() as *const libc::c_char,
            0,
            ptr::null(),
        );
        println!("{:?}", ok);
    }

    unsafe {
        let ok = libc::mount(
            b"proc\0".as_ptr() as *const libc::c_char,
            b"/proc\0".as_ptr() as *const libc::c_char,
            b"proc\0".as_ptr() as *const libc::c_char,
            0,
            ptr::null(),
        );
        println!("{:?}", ok);
    }

    Command::new("/eudev/bin/udevd")
        .args(&["-d" /* daemon */])
        .output()
        .expect("udevd");

    let ok = Command::new("/bash/bin/bash")
        .env("PATH", "/coreutils/bin")
        .exec();

    println!("{:?}", ok);

    loop {
        let dur = Duration::from_secs(60);
        thread::sleep(dur);
    }
}
