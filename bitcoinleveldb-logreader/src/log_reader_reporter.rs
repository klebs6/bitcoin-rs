// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_reporter.rs ]
crate::ix!();

/**
  | Interface for reporting errors.
  |
  */
pub trait LogReaderReporter {

    /**
      | Some corruption was detected. "size"
      | is the approximate number of bytes dropped
      | due to the corruption.
      |
      */
    fn corruption(&mut self, 
        bytes:  usize,
        status: &Status);

}
