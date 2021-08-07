use np_graphics::Blitter;
use np_graphics::PixelMap;
use np_graphics::blitters::BlendSourceBlitter;

/// Cover the entire screen with the wallpaper.
pub fn draw_wallpaper(pixel_map: &mut PixelMap<[u8; 4]>)
{
    let pixel = [0xC2, 0x48, 0x1D, 0xFF];
    let blitter = BlendSourceBlitter::new(pixel);
    let extent = pixel_map.extent();
    blitter.rectangle(pixel_map, 0, 0, extent.0, extent.1);
}
