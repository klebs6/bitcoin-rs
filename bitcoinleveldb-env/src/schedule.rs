// ---------------- [ File: bitcoinleveldb-env/src/schedule.rs ]
crate::ix!();

pub trait Schedule {

    /**
      | Arrange to run "(*function)(arg)" once in
      | a background thread.
      |
      | "function" may run in an unspecified thread.
      | Multiple functions added to the same Env may
      | run concurrently in different threads.
      |
      | I.e., the caller may not assume that
      | background work items are serialized.
      */
    fn schedule(&mut self, 
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void);
}
