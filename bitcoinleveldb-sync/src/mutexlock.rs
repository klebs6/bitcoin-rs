// ---------------- [ File: bitcoinleveldb-sync/src/mutexlock.rs ]
/**
  | Helper class that locks a mutex on construction
  | and unlocks the mutex when the destructor
  | of the MutexLock object is invoked.
  | 
  | Typical usage:
  | 
  | -----------
  | @code
  | 
  | c_void MyClass::MyMethod() {
  |   MutexLock l(&mu_);       // mu_ is an instance variable
  |   ... some complex code, possibly with multiple return paths ...
  | }
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/mutexlock.h]

#[SCOPED_LOCKABLE]
pub struct MutexLock {
    mu: *const parking_lot::RawMutex,
}

impl Drop for MutexLock {

    #[UNLOCK_FUNCTION]
    fn drop(&mut self) {
        if !self.mu.is_null() {
            unsafe { (*self.mu).unlock(); }
        }
    }
}

impl MutexLock {

    #[EXCLUSIVE_LOCK_FUNCTION(mu)]
    pub fn new(mu: *mut parking_lot::RawMutex) -> Self {

        if mu.is_null() {
            panic!("must provide valid pointer to this constructor");
        }

        let mut x = Self { mu };

        unsafe { (*x.mu).lock(); }

        x
    }
}
