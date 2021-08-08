use np_graphics::BlendMode;
use np_graphics::Canvas;
use np_graphics::Matrix;
use np_graphics::Paint;
use np_graphics::PixelMap;
use np_graphics::PixelMapCanvas;
use np_graphics::Vector;
use np_graphics::formats::Bgra8888;
use np_shell::draw_wallpaper;
use np_text::Face;
use np_text::Image;
use np_unix::Mmap;
use std::fs::OpenOptions;
use std::io::Result;
use std::os::unix::io::AsRawFd;

fn main() -> Result<()>
{
    let face = Face::open("/fonts/FreeSerif.ttf")?;
    let glyph_h = face.glyph(76)?;
    let glyph_a = face.glyph(69)?;
    let glyph_l = face.glyph(80)?;
    let glyph_o = face.glyph(83)?;
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

        let scale = 1.0 / 100.0;
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
