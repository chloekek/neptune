#![allow(improper_ctypes)] // bindgen emits u128 for long double.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!("../bindgen.rs");

pub const fn FT_CURVE_TAG(flag: u32) -> u32
{
    flag & 0x03
}

#[link(name = "freetype")]
extern "C"
{
}
