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

    #[UNLOCK_FUNCTION()]
    fn drop(&mut self) {
        todo!();
        /*
            this->mu_->Unlock();
        */
    }
}

impl MutexLock {

    #[EXCLUSIVE_LOCK_FUNCTION(mu)]
    pub fn new(mu: *mut parking_lot::RawMutex) -> Self {
    
        todo!();
        /*
        : mu(mu),

            this->mu_->Lock();
        */
    }
}
