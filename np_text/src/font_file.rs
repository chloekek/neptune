use crate::Typeface;
use crate::Result;

use np_unix::Mmap;
use std::fs::File;
use std::io::Read;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;

/// Font file either stored in memory or memory-mapped.
pub struct FontFile
{
    inner: Inner,
}

enum Inner
{
    Vec(Vec<u8>),
    Mmap(Mmap),
}

impl FontFile
{
    /// Open a font file and read it into memory.
    ///
    /// The font file is not immediately parsed.
    /// Parsing happens whenever features are accessed.
    pub fn open<P>(path: P) -> io::Result<Self>
        where P: AsRef<Path>
    {
        let mut file = File::open(path)?;
        let mut vec = Vec::new();
        file.read_to_end(&mut vec)?;
        Ok(Self{inner: Inner::Vec(vec)})
    }

    /// Open a font file in a memory-mapped fashion.
    ///
    /// Because it is common for multiple processes to use the same font files,
    /// it is more efficient to map them than it is to read them into a buffer.
    /// This way the processes will all share the same memory per font file.
    ///
    /// The font file is not immediately parsed.
    /// Parsing happens whenever features are accessed.
    ///
    /// # Safety
    ///
    /// It is paramount that the font file is not
    /// modified concurrently with the program.
    /// Only open files that are read-only
    /// and will remain read-only.
    pub unsafe fn open_mapped<P>(path: P) -> io::Result<Self>
        where P: AsRef<Path>
    {
        let file = File::open(path)?;
        let length = file.metadata()?.len();
        let mmap = Mmap::mmap(
            length as usize,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            file.as_raw_fd(),
            /* offset */ 0,
        )?;
        Ok(Self{inner: Inner::Mmap(mmap)})
    }

    fn data(&self) -> &[u8]
    {
        match &self.inner {
            Inner::Vec(vec) =>
                vec,
            Inner::Mmap(mmap) =>
                // SAFETY: Guaranteed by safety contract for open_mapped.
                unsafe { mmap.as_ref() },
        }
    }

    /// Typeface at the given index.
    ///
    /// Some font files contain multiple typefaces.
    /// Any of them may be retrieved with this function.
    /// For the common case of one typeface per file,
    /// just pass zero as the index and it will work.
    ///
    /// If the typeface cannot be parsed,
    /// this function returns [`Err`].
    pub fn typeface(&self, index: u32) -> Result<Typeface>
    {
        let inner = ttf_parser::Face::from_slice(self.data(), index)?;
        Ok(Typeface{inner})
    }
}
