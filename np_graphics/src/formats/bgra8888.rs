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

            let [bd, gd, rd, ad] = *dest;
            let [bs, gs, rs, az] = source;

            let [bd, gd, rd, ad] = [bd as u32, gd as u32, rd as u32, ad as u32];
            let [bs, gs, rs, az] = [bs as u32, gs as u32, rs as u32, az as u32];

            let ar = az + (ad * (255 - az) / 255);
            if ar == 0 {
                *dest = [0, 0, 0, 0];
                continue;
            }

            let br = (az * bs + (ad * bd * (255 - az) / 255)) / ar;
            let gr = (az * gs + (ad * gd * (255 - az) / 255)) / ar;
            let rr = (az * rs + (ad * rd * (255 - az) / 255)) / ar;

            *dest = [br as u8, gr as u8, rr as u8, ar as u8];

        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    use proptest::proptest;
    use std::slice;

    #[test]
    fn test_blend_source_over_examples()
    {
        let examples = &[
            // dest         source         expected
            (0x00_00_00_00, 0x00_00_00_00, 0x00_00_00_00),
            (0xFF_FF_FF_FF, 0xFF_FF_FF_FF, 0xFF_FF_FF_FF),
            (0xFF_00_00_FF, 0xFF_FF_00_7F, 0xFF_7F_00_FF),
        ];

        for &(dest, source, expected) in examples {
            let mut dest = u32::to_be_bytes(dest);
            let source   = u32::to_be_bytes(source);
            let expected = u32::to_be_bytes(expected);
            Bgra8888.blend_source_over(slice::from_mut(&mut dest), source);
            assert_eq!(dest, expected);
        }
    }

    proptest! {

        #[test]
        fn test_blend_source_over_invisible(
            dest   in 0x00_00_00_00u32 .. 0xFF_FF_FF_FFu32,
            source in 0x00_00_00_00u32 .. 0xFF_FF_FF_FFu32,
        )
        {
            let orig_dest = u32::to_be_bytes(dest);
            let mut dest  = u32::to_be_bytes(dest);
            let source    = u32::to_be_bytes(source & 0xFF_FF_FF_00);
            Bgra8888.blend_source_over(slice::from_mut(&mut dest), source);
            if dest[3] == 0 {
                // All transparent pixels are the same,
                // so test them with a different assertion.
                assert_eq!(orig_dest[3], 0);
            } else {
                assert_eq!(dest, orig_dest);
            }
        }

        #[test]
        fn test_blend_source_over_opaque(
            dest   in 0x00_00_00_00u32 .. 0xFF_FF_FF_FFu32,
            source in 0x00_00_00_00u32 .. 0xFF_FF_FF_FFu32,
        )
        {
            let mut dest = u32::to_be_bytes(dest);
            let source   = u32::to_be_bytes(source | 0x00_00_00_FF);
            Bgra8888.blend_source_over(slice::from_mut(&mut dest), source);
            assert_eq!(dest, source);
        }

    }
}
