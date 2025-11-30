// ---------------- [ File: bitcoinleveldb-file/src/sequential_file.rs ]
crate::ix!();

/**
  | A file abstraction for reading sequentially
  | through a file
  |
  */
pub trait SequentialFile: 
SequentialFileRead 
+ SequentialFileSkip 
+ Named { }

pub trait SequentialFileRead {

    /**
      | Read up to "n" bytes from the file. "scratch[0..n-1]"
      | may be written by this routine. Sets
      | "*result" to the data that was read (including
      | if fewer than "n" bytes were successfully
      | read).
      | 
      | May set "*result" to point at data in
      | "scratch[0..n-1]", so "scratch[0..n-1]"
      | must be live when "*result" is used.
      | 
      | If an error was encountered, returns
      | a non-OK status.
      | 
      | REQUIRES: External synchronization
      |
      */
    fn read(&mut self, 
            n:       usize,
            result:  *mut Slice,
            scratch: *mut u8) -> crate::Status;
}

pub trait SequentialFileSkip {

    /**
      | Skip "n" bytes from the file. This is
      | guaranteed to be no slower that reading the
      | same data, but may be faster.
      |
      | If end of file is reached, skipping will stop
      | at the end of the file, and Skip will return
      | OK.
      |
      | REQUIRES: External synchronization
      */
    fn skip(&mut self, n: u64) -> crate::Status;
}
