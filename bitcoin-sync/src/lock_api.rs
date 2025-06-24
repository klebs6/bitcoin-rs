crate::ix!();

/// Minimal trait representing the operations we need
/// from a raw, non‑poisoning mutex.
pub trait LockApi {
    fn lock(&self);
    fn unlock(&self);
    fn try_lock(&self) -> bool;
}

impl LockApi for parking_lot::RawMutex {
    #[inline]
    fn lock(&self) {
        // call the low‑level `RawMutexTrait` implementation explicitly
        bitcoin_imports::RawMutexTrait::lock(self)
    }

    #[inline]
    fn unlock(&self) {
        unsafe { bitcoin_imports::RawMutexTrait::unlock(self) }
    }

    #[inline]
    fn try_lock(&self) -> bool {
        bitcoin_imports::RawMutexTrait::try_lock(self)
    }
}
