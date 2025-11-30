// ---------------- [ File: bitcoinleveldb-posixmmaprfile/src/create.rs ]
crate::ix!();

impl PosixMmapReadableFile {
    /**
      | `mmap_base` [0, length-1] points to the
      | memory-mapped contents of the file. It must
      | be the result of a successful call to
      | mmap(). This instance takes over the
      | ownership of the region.
      |
      | `mmap_limiter` must outlive this instance.
      | The caller must have already acquired the
      | right to use one mmap region, which will be
      | released when this instance is destroyed.
      */
    pub fn new(
        filename:     String,
        mmap_base:    *mut u8,
        length:       usize,
        mmap_limiter: *mut Limiter,
    ) -> Self {
        trace!(
            file    = %filename,
            base    = ?mmap_base,
            len     = length,
            limiter = ?mmap_limiter,
            "PosixMmapReadableFile::new"
        );

        // We *allow* null base with zero length so tests can create degenerate
        // instances safely. Production code should only pass a null base when
        // `length == 0`.
        if length > 0 && mmap_base.is_null() {
            warn!(
                file = %filename,
                "PosixMmapReadableFile::new: non-zero length with null base"
            );
        }

        PosixMmapReadableFileBuilder::default()
            .mmap_base(mmap_base as *const u8)
            .length(length)
            .mmap_limiter(mmap_limiter as *const Limiter)
            .filename(filename)
            .build()
            .unwrap()
    }
}
