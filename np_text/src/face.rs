use crate::Glyph;
use crate::Image;
use crate::LIBRARY;
use crate::freetype;

use np_freetype_sys::*;
use np_graphics::Bezier;
use np_graphics::Vector;
use std::cell::UnsafeCell;
use std::ffi::CString;
use std::io::Result;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

/// Information about a face,
/// including its glyphs.
pub struct Face
{
    inner: UnsafeCell<freetype::Face<'static>>,
}

impl Face
{
    /// Open a face from a file.
    pub fn open<P>(path: P) -> Result<Self>
        where P: AsRef<Path>
    {
        let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
        let inner = freetype::Face::new(&LIBRARY, &path, 0)?;
        let inner = UnsafeCell::new(inner);
        Ok(Self{inner})
    }

    /// The glyph at the given index.
    pub fn glyph(&self, index: u32) -> Result<Glyph>
    {
        // SAFETY: We will not create multiple aliases.
        let inner = unsafe { &mut *self.inner.get() };
        Self::glyph_inner(inner, index)
    }

    fn glyph_inner(inner: &mut freetype::Face<'static>, index: u32)
        -> Result<Glyph>
    {
        // FIXME: We should probably not do this every time.
        // TODO: We should make these values configurable.
        inner.set_char_size(0, 16 * 64, 300, 300)?;

        inner.load_glyph(index, 0)?;
        let glyph_slot = inner.glyph();

        let image = match glyph_slot.format {
            FT_GLYPH_FORMAT_OUTLINE =>
                // SAFETY: We now know this is an outline image.
                unsafe { image_from_ft_outline(&mut glyph_slot.outline)? },
            _ => todo!("Non-outline glyph formats"),
        };

        Ok(Glyph{image})
    }
}

/// Used as the state for a call to FT_Outline_Decompose.
struct OutlineDecomposeState
{
    start: FT_Vector,
    contours: Vec<Bezier>,
}

unsafe fn image_from_ft_outline(ft_outline: &mut FT_Outline)
    -> Result<Image>
{
    // While FreeType makes a distinction
    // between “line segments” and “Bézier arcs”,
    // the np_graphics library can express
    // line segments as first-order Bézier curves,
    // so this will extract all contours as Bézier curves.

    let mut state = OutlineDecomposeState{
        start: FT_Vector{x: 0, y: 0},
        contours: Vec::with_capacity(ft_outline.n_contours as usize),
    };

    let status = FT_Outline_Decompose(
        ft_outline,
        &FT_Outline_Funcs{
            move_to: Some(image_from_ft_outline_move_to),
            line_to: Some(image_from_ft_outline_line_to),
            conic_to: Some(image_from_ft_outline_conic_to),
            cubic_to: Some(image_from_ft_outline_cubic_to),
            shift: 0,
            delta: 0,
        },
        &mut state as *mut OutlineDecomposeState as *mut c_void,
    );
    freetype::Error::new(status)?;

    Ok(Image::Outline(state.contours))
}

unsafe extern "C" fn image_from_ft_outline_move_to(
    to: *const FT_Vector,
    user: *mut c_void,
) -> c_int
{
    let user = &mut *(user as *mut OutlineDecomposeState);

    user.start = *to;
    0
}

unsafe extern "C" fn image_from_ft_outline_line_to(
    to: *const FT_Vector,
    user: *mut c_void,
) -> c_int
{
    let user = &mut *(user as *mut OutlineDecomposeState);

    let bezier = Bezier::Linear(
        convert_vector(user.start),
        convert_vector(*to),
    );
    user.contours.push(bezier);

    user.start = *to;
    0
}

unsafe extern "C" fn image_from_ft_outline_conic_to(
    control: *const FT_Vector,
    to: *const FT_Vector,
    user: *mut c_void,
) -> c_int
{
    let user = &mut *(user as *mut OutlineDecomposeState);

    let bezier = Bezier::Conic(
        convert_vector(user.start),
        convert_vector(*control),
        convert_vector(*to),
    );
    user.contours.push(bezier);

    user.start = *to;
    0
}

unsafe extern "C" fn image_from_ft_outline_cubic_to(
    control1: *const FT_Vector,
    control2: *const FT_Vector,
    to: *const FT_Vector,
    user: *mut c_void,
) -> c_int
{
    let user = &mut *(user as *mut OutlineDecomposeState);

    let bezier = Bezier::Cubic(
        convert_vector(user.start),
        convert_vector(*control1),
        convert_vector(*control2),
        convert_vector(*to),
    );
    user.contours.push(bezier);

    user.start = *to;
    0
}

fn convert_vector(ft: FT_Vector) -> Vector
{
    Vector{
        x: ft.x as f64,
        y: ft.y as f64,
    }
}
