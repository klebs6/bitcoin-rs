// ---------------- [ File: bitcoinleveldb-versionsetinterface/src/file_number_interface.rs ]
crate::ix!();

pub trait ManifestFileNumber {

    /**
      | Return the current manifest file number
      |
      */
    fn manifest_file_number(&self) -> u64;
}

pub trait NewFileNumber {

    /**
      | Allocate and return a new file number
      |
      */
    fn new_file_number(&mut self) -> u64;
}

pub trait ReuseFileNumber {

    /**
      | Arrange to reuse "file_number" unless a newer
      | file number has already been allocated.
      |
      | REQUIRES: "file_number" was returned by
      | a call to NewFileNumber().
      */
    fn reuse_file_number(&mut self, file_number: u64);
}

pub trait GetCurrentLogFileNumber {

    /**
      | Return the current log file number.
      |
      */
    fn log_number(&self) -> u64;
}

pub trait GetPrevLogFileNumber {

    /**
      | Return the log file number for the log
      | file that is currently being compacted,
      | or zero if there is no such log file.
      |
      */
    fn prev_log_number(&self) -> u64;
}


pub trait MarkFileNumberUsed {

    /**
      | Mark the specified file number as used.
      |
      */
    fn mark_file_number_used(&mut self, number: u64);
}
