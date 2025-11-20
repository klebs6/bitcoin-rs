// ---------------- [ File: bitcoinleveldb-file/src/random_access_file.rs ]
crate::ix!();

/**
  | A file abstraction for randomly reading
  | the contents of a file.
  |
  */
pub trait RandomAccessFile: 
RandomAccessFileRead 
+ GetName {}

pub trait RandomAccessFileRead {

    /**
      | Read up to "n" bytes from the file starting
      | at "offset".  "scratch[0..n-1]" may be
      | written by this routine.  Sets "*result" to
      | the data that was read (including if fewer
      | than "n" bytes were successfully read).  May
      | set "*result" to point at data in
      | "scratch[0..n-1]", so "scratch[0..n-1]" must
      | be live when "*result" is used.  If an error
      | was encountered, returns a non-OK status.
      |
      | Safe for concurrent use by multiple threads.
      */
    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status;
}
