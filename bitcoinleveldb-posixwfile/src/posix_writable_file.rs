// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file.rs ]
crate::ix!();

pub const WRITABLE_FILE_BUFFER_SIZE: usize = 65536;

#[derive(Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub(crate)")]
pub struct PosixWritableFile {
    /// buf_[0, pos_ - 1] contains data to be written to fd_.
    buf: [u8; WRITABLE_FILE_BUFFER_SIZE],

    pos: usize,

    /// Underlying POSIX file descriptor.
    fd:  i32,

    /// True if the file's name starts with MANIFEST.
    is_manifest: bool,

    /// Full pathname of the file.
    filename: String,

    /// The directory of filename_.
    dirname: String,

    /// Whether this instance was ever constructed with a valid fd (>= 0).
    ///
    /// This lets us distinguish:
    /// - never-opened / invalid fd at construction → close() should be an IO error
    /// - previously-opened-and-closed → subsequent close() calls are idempotent OK
    ever_valid_fd: bool,
}

impl WritableFile for PosixWritableFile {}

impl Drop for PosixWritableFile {
    fn drop(&mut self) {
        // Mirror the C++ destructor:
        // if (fd_ >= 0) { Close(); } ignoring any potential error.
        if *self.fd() >= 0 {
            trace!(
                file = %self.filename(),
                fd   = *self.fd(),
                "PosixWritableFile::drop: fd still open, calling close()"
            );
            let _ = self.close();
        } else {
            trace!(
                file = %self.filename(),
                fd   = *self.fd(),
                "PosixWritableFile::drop: fd already closed"
            );
        }
    }
}

impl Named for PosixWritableFile {
    fn name(&self) -> Cow<'_,str> {
        // We want a stable tag in logs / debugging.
        Cow::Borrowed("[posix-writable-file]")
    }
}

impl PosixWritableFile {
    pub fn new(filename: String, fd: i32) -> Self {
        trace!(
            file = %filename,
            fd,
            "PosixWritableFile::new"
        );

        let is_manifest   = Self::is_manifest_static(&filename);
        let dirname       = Self::dirname_static(&filename);
        let ever_valid_fd = fd >= 0;

        Self {
            buf: [0u8; WRITABLE_FILE_BUFFER_SIZE],
            pos: 0,
            fd,
            is_manifest,
            filename,
            dirname,
            ever_valid_fd,
        }
    }
}
