// ---------------- [ File: bitcoin-sync/src/annotated_mixin.rs ]
crate::ix!();

/// Mixin that forwards to the underlying mutex **and**
/// provides future hooks for lock‑order tracking.
pub struct AnnotatedMixin<Parent: LockApi> {
    parent: Parent,
}

impl<Parent: LockApi> AnnotatedMixin<Parent> {
    #[inline]
    pub const fn new(parent: Parent) -> Self { Self { parent } }

    #[inline]
    pub fn as_ptr(&self) -> *const Self { self as *const _ }
}

impl<Parent: LockApi> LockApi for AnnotatedMixin<Parent> {
    #[inline] fn lock(&self)             { self.parent.lock() }
    #[inline] fn unlock(&self)           { self.parent.unlock() }
    #[inline] fn try_lock(&self) -> bool { self.parent.try_lock() }
}

/// Generic `Drop` (no specialisation).
impl<Parent: LockApi> Drop for AnnotatedMixin<Parent> {
    fn drop(&mut self) {
        trace!("AnnotatedMixin dropped @ {:p}", self.as_ptr());
    }
}

/// Provide `Default` only for the concrete `RawMutex` case.
impl Default for AnnotatedMixin<parking_lot::RawMutex> {
    fn default() -> Self { Self::new(parking_lot::RawMutex::INIT) }
}

#[cfg(test)]
mod annotated_mixin_tests {
    use super::*;

    #[traced_test]
    fn basic_lock_cycle() {
        let m = AnnotatedMixin::<parking_lot::RawMutex>::default();
        assert!(m.try_lock());
        m.unlock();
        m.lock();
        m.unlock();
    }
}
