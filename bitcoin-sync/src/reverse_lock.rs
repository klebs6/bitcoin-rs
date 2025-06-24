// ---------------- [ File: bitcoin-sync/src/reverse_lock.rs ]
crate::ix!();

/// RAII helper: unlocks the passed `UniqueLock` on
/// construction, re‑locks on drop.
pub struct ReverseLock<'guard, 'lock, M: LockApi + ?Sized> {
    guard:    &'guard mut UniqueLock<'lock, M>,
    relocked: bool,
}

impl<'guard, 'lock, M: LockApi + ?Sized> ReverseLock<'guard, 'lock, M> {
    /// Temporarily release the lock held by `guard`.
    pub fn new(guard: &'guard mut UniqueLock<'lock, M>) -> Self {
        trace!("ReverseLock::new — temporarily releasing");
        guard.unlock();
        Self { guard, relocked: false }
    }
}

impl<'guard, 'lock, M: LockApi + ?Sized> Drop for ReverseLock<'guard, 'lock, M> {
    fn drop(&mut self) {
        if !self.relocked {
            trace!("ReverseLock::drop — re‑acquiring");
            self.guard.enter();
            self.relocked = true;
        }
    }
}

#[cfg(test)]
mod reverse_lock_tests {
    use super::*;
    use parking_lot::RawMutex;

    #[traced_test]
    fn reverse_lock_round_trip() {
        let am  = AnnotatedMixin::<RawMutex>::default();
        let mut ul = UniqueLock::new(&am, "m", file!(), line!(), None);

        {
            let _rev = ReverseLock::new(&mut ul); // unlocks …
        } // …and is dropped here, re‑locking

        assert!(ul.owns_lock(), "lock must be reacquired after ReverseLock");
    }
}
