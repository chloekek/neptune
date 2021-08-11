use np_graphics::BlendMode;
use np_graphics::Canvas;
use np_graphics::Matrix;
use np_graphics::Paint;
use np_graphics::PixelMap;
use np_graphics::PixelMapCanvas;
use np_graphics::Vector;
use np_graphics::formats::Bgra8888;
use np_shell::RunningApps;
use np_shell::draw_wallpaper;
use np_text::FontFile;
use np_text::Image;
use np_unix::Mmap;
use std::fs::OpenOptions;
use std::io::Result;
use std::os::unix::io::AsRawFd;

fn main() -> Result<()>
{
    let running_apps = RunningApps::new();

    let mut pollfds = Vec::new();

    pollfds.extend(
        running_apps.sockets()
        .map(|fd| libc::pollfd{fd, events: libc::POLLIN, revents: 0})
    );

    let font_file = FontFile::open_mapped("/fonts/FreeSerif.ttf")?;
    let typeface = font_file.typeface(0).unwrap();
    let glyph_h = typeface.glyph(76).unwrap();
    let glyph_a = typeface.glyph(69).unwrap();
    let glyph_l = typeface.glyph(80).unwrap();
    let glyph_o = typeface.glyph(83).unwrap();
    println!("{:?}", glyph_h);
    println!("{:?}", glyph_a);
    println!("{:?}", glyph_l);
    println!("{:?}", glyph_o);

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

    loop {

        let dur = std::time::Duration::from_secs(1);
        std::thread::sleep(dur);

        let mut pixel_map = PixelMap::new(
            unsafe { fb_mmap.as_mut::<[u8; 4]>() },
            1024,
            1024,
            768,
        ).unwrap();

        draw_wallpaper(&mut pixel_map);

        let mut canvas = PixelMapCanvas::new(Bgra8888, pixel_map);

        canvas.rectangle(
            Matrix::from_scale(2.0, 2.0),
            Vector{x: 0.0, y: 0.0},
            Vector{x: 512.0, y: 24.0},
            Paint{
                blend_mode: BlendMode::Source,
                pixel: [0xFF, 0x00, 0xFF, 0xFF],
            },
        );

        let scale = 1.0 / 25.0;
        let mut offset = 100.0;
        for glyph in &[&glyph_h, &glyph_a, &glyph_l, &glyph_l, &glyph_o] {

            let outline = match &glyph.image {
                Image::Outline(outline) => outline,
                _ => panic!("Expected outline glyph"),
            };

            canvas.path(
                Matrix::IDENTITY
                    * Matrix::from_translate(offset, 500.0)
                    * Matrix::from_scale(1.0 * scale, -1.0 * scale),
                outline.instructions(),
                Paint{
                    blend_mode: BlendMode::Source,
                    pixel: [0xFF, 0xFF, 0xFF, 0xFF],
                },
            );

            offset += glyph.advance_x * scale;

        }

    }
}
