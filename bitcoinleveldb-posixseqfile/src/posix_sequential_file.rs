// ---------------- [ File: bitcoinleveldb-posixseqfile/src/posix_sequential_file.rs ]
crate::ix!();

/**
  | Implements sequential read access in a file
  | using read().
  |
  | Instances of this class are thread-friendly but
  | not thread-safe, as required by the
  | SequentialFile API.
  */
pub struct PosixSequentialFile {
    pub(crate) fd:       i32,
    pub(crate) filename: String,
}

impl SequentialFile for PosixSequentialFile {}

impl Named for PosixSequentialFile {

    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.filename)
    }
}

impl PosixSequentialFile {

    pub fn new(filename: String, fd: i32) -> Self {
        trace!(
            file = %filename,
            fd,
            "PosixSequentialFile::new: constructing"
        );
        Self { fd, filename }
    }
}
