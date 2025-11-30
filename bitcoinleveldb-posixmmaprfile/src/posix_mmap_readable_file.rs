// ---------------- [ File: bitcoinleveldb-posixmmaprfile/src/posix_mmap_readable_file.rs ]
crate::ix!();

/**
  | Implements random read access in a file using
  | mmap().
  |
  | Instances of this class are thread-safe, as
  | required by the RandomAccessFile API. Instances
  | are immutable and Read() only calls thread-safe
  | library functions.
  */
#[derive(Getters, Builder, Debug)]
#[getset(get = "pub")] // generate public getters instead of exposing fields
pub struct PosixMmapReadableFile {
    /// Start of the mmap()‑ed region.
    mmap_base:    *const u8,
    /// Length in bytes of the mapped region.
    length:       usize,
    /// Limiter that tracks how many mmaps are in use.
    /// The pointer must outlive this object; we own
    /// exactly one "slot" which is released on Drop.
    mmap_limiter: *const Limiter,
    /// File name used only for diagnostics / errors.
    filename:     String,
}

impl RandomAccessFile for PosixMmapReadableFile { }

impl Named for PosixMmapReadableFile {

    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.filename)
    }
}
