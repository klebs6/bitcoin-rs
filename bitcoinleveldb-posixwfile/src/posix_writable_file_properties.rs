// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_properties.rs ]
crate::ix!();

impl PosixWritableFile {

    /// Returns the directory name in a path pointing
    /// to a file.
    /// 
    /// Returns "." if the path does not contain any
    /// directory separator.
    ///
    pub fn dirname_static(filename: &String) -> String {
        trace!(file = %filename, "PosixWritableFile::dirname");

        if let Some(sep_pos) = filename.rfind('/') {
            // The filename component should not contain another path separator.
            debug_assert!(
                !filename[sep_pos + 1..].contains('/'),
                "PosixWritableFile::dirname: unexpected '/' in filename component: {}",
                filename
            );
            filename[..sep_pos].to_string()
        } else {
            ".".to_string()
        }
    }

    /// Extracts the file name from a path pointing
    /// to a file.
    /// 
    /// The returned Slice points to |filename|'s
    /// data buffer, so it is only valid while
    /// |filename| is alive and unchanged.
    ///
    pub fn basename(filename: &String) -> Slice {
        trace!(file = %filename, "PosixWritableFile::basename");

        if let Some(sep_pos) = filename.rfind('/') {
            debug_assert!(
                !filename[sep_pos + 1..].contains('/'),
                "PosixWritableFile::basename: unexpected '/' in filename component: {}",
                filename
            );

            let ptr = unsafe { filename.as_ptr().add(sep_pos + 1) };
            let len = filename.len() - sep_pos - 1;
            Slice::from_ptr_len(ptr, len)
        } else {
            Slice::from(filename)
        }
    }

    /// True if the given file is a manifest file.
    /// 
    pub fn is_manifest_static(filename: &String) -> bool {
        trace!(file = %filename, "PosixWritableFile::is_manifest");

        let base   = Self::basename(filename);
        let prefix = Slice::from("MANIFEST".as_bytes());
        base.starts_with(&prefix)
    }
}
