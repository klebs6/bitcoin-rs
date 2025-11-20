// ---------------- [ File: bitcoinleveldb-env/src/start_thread.rs ]
crate::ix!();

pub trait StartThread {

    /**
      | Start a new thread, invoking "function(arg)"
      | within the new thread.
      |
      | When "function(arg)" returns, the thread will
      | be destroyed.
      */
    fn start_thread(&mut self, 
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void);
}
