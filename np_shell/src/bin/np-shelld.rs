use cgmath::Vector2;
use np_graphics::PixelMap;
use np_shell::draw_wallpaper;
use np_unix::Mmap;
use std::fs::OpenOptions;
use std::io::Result;
use std::os::unix::io::AsRawFd;

fn main() -> Result<()>
{
    let fb_file =
        OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/fb0")?;

    let mut fb_mmap =
        Mmap::mmap(
            /* length */ 4 * 1024 * 768,
            /* prot   */ libc::PROT_READ | libc::PROT_WRITE,
            /* flags  */ libc::MAP_SHARED_VALIDATE,
            /* fd     */ fb_file.as_raw_fd(),
            /* offset */ 0,
        )?;

    drop(fb_file);

    let mut pixel_map = PixelMap::new(
        unsafe { fb_mmap.as_mut::<[u8; 4]>() },
        Vector2::new(1024, 768),
    ).unwrap();

    draw_wallpaper(&mut pixel_map);

    Ok(())
}
