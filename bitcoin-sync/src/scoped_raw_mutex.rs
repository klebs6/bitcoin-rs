// ---------------- [ File: bitcoin-sync/src/scoped_raw_mutex.rs ]
crate::ix!();

//TODO: is this correct? does this work?
//does this create any deadlocks?
//
//I am using this to shim all of the calls to
//CS_MAIN.lock() which the original c++ uses
//throughout
pub struct ScopedRawMutexGuard<'a> {
    lock: &'a ScopedRawMutex,
}

impl<'a> ScopedRawMutexGuard<'a> {
    pub fn new(x: &'a ScopedRawMutex) -> Self {
        Self {
            lock: x
        }
    }
}

impl<'a> Drop for ScopedRawMutexGuard<'a> {
    fn drop(&mut self) {
        unsafe { bitcoin_imports::RawMutexTrait::unlock(&self.lock.0) }
    }
}

//---------------------------------------------
pub struct ScopedRawMutex(RawMutex);

impl Default for ScopedRawMutex {

    fn default() -> Self {
        Self(RawMutex::INIT)
    }
}

impl ScopedRawMutex {
    pub fn lock(&self) -> ScopedRawMutexGuard<'_> {
        bitcoin_imports::RawMutexTrait::lock(&self.0);
        ScopedRawMutexGuard { lock: self }
    }
}
