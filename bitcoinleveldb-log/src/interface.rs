// ---------------- [ File: bitcoinleveldb-log/src/interface.rs ]
crate::ix!();
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging.cc]

/**
  | An interface for writing log messages.
  |
  */
pub trait Logger: Logv { }

pub trait Logv {

    /**
      | Write an entry to the log file with the
      | specified format.
      |
      */
    fn logv(&mut self, 
            format: *const u8,
            ap:     &[&str]);

}
