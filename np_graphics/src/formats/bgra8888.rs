use crate::Format;

/// BGRA format storing each component in an 8-bit integer.
///
/// The components are stored in the following order,
/// starting at low addresses and ending at high addresses:
/// blue, green, red, alpha.
#[derive(Clone, Copy, Debug)]
pub struct Bgra8888;

impl Format for Bgra8888
{
    type Pixel = [u8; 4];

    fn is_visible(&self, pixel: Self::Pixel) -> bool
    {
        pixel[3] > 0
    }

    fn is_opaque(&self, pixel: Self::Pixel) -> bool
    {
        pixel[3] == 255
    }

    fn blend_source_over(&self, dest: &mut [Self::Pixel], source: Self::Pixel)
    {
        for dest in dest.iter_mut() {

            let [rd, gd, bd, ad] = *dest;
            let [rs, gs, bs, az] = source;

            let [rd, gd, bd, ad] = [rd as u32, gd as u32, bd as u32, ad as u32];
            let [rs, gs, bs, az] = [rs as u32, gs as u32, bs as u32, az as u32];

            let ar = az + (ad * (255 - az) >> 8);
            if ar == 0 {
                *dest = [0, 0, 0, 0];
                continue;
            }

            let rr = ((az * rs << 8) + ad * rd * (255 - az) >> 16) / ar;
            let gr = ((az * gs << 8) + ad * gd * (255 - az) >> 16) / ar;
            let br = ((az * bs << 8) + ad * bd * (255 - az) >> 16) / ar;

            *dest = [rr as u8, gr as u8, br as u8, ar as u8];

        }
    }
}
