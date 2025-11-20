// ---------------- [ File: bitcoinleveldb-env/src/new_logger.rs ]
crate::ix!();

pub trait NewLogger {

    /**
       Create and return a log file for storing
       informational messages.
      */
    fn new_logger(&mut self, 
            fname:  &String,
            result: *mut *mut Box<dyn Logger>) -> crate::Status;
}
