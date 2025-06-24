// ---------------- [ File: bitcoin-sync/src/unique_lock.rs ]
crate::ix!();

/// RAII guard that mirrors `std::unique_lock` semantics.
///
/// * Holds **no** poisoning state – matches Bitcoin Core.
/// * Integrates with `AnnotatedMixin` so future lock‑order checks
///   can inject hooks transparently.
pub struct UniqueLock<'a, M: LockApi + ?Sized> {
    mutex:   &'a M,
    owns:    bool,
    name:    &'static str,
    file:    &'static str,
    line:    u32,
}

impl<'a, M: LockApi + ?Sized> UniqueLock<'a, M> {
    /// Construct; lock immediately or try‑lock depending on `try_`.
    pub fn new(
        mutex:   &'a M,
        name:    &'static str,
        file:    &'static str,
        line:    u32,
        try_:    Option<bool>,
    ) -> Self {
        let mut ul = Self {
            mutex,
            owns: false,
            name,
            file,
            line,
        };
        if try_.unwrap_or(false) {
            ul.owns = mutex.try_lock();
            trace!(
                "UniqueLock::new TRY — owns = {} ({name} @ {file}:{line})",
                ul.owns
            );
        } else {
            trace!("UniqueLock::new LOCK — ({name} @ {file}:{line})");
            mutex.lock();
            ul.owns = true;
        }
        ul
    }

    #[inline]
    pub fn enter(&mut self) {
        if !self.owns {
            trace!("UniqueLock::enter — ({})", self.name);
            self.mutex.lock();
            self.owns = true;
        }
    }

    #[inline]
    pub fn try_enter(&mut self) -> bool {
        if !self.owns {
            self.owns = self.mutex.try_lock();
            trace!(
                "UniqueLock::try_enter — success = {} ({})",
                self.owns, self.name
            );
        }
        self.owns
    }

    #[inline] pub fn unlock(&mut self) {
        if self.owns {
            trace!("UniqueLock::unlock — ({})", self.name);
            self.mutex.unlock();
            self.owns = false;
        }
    }

    /// Mirror `std::unique_lock::owns_lock`.
    #[inline] pub fn owns_lock(&self) -> bool { self.owns }
}

impl<'a, M: LockApi + ?Sized> Drop for UniqueLock<'a, M> {
    fn drop(&mut self) {
        if self.owns {
            debug!("UniqueLock::drop — unlocking ({})", self.name);
            self.mutex.unlock();
        }
    }
}

impl<'a, M: LockApi + ?Sized> From<&UniqueLock<'a, M>> for bool {
    #[inline] fn from(ul: &UniqueLock<'a, M>) -> Self { ul.owns }
}

/// ---------------- tests
#[cfg(test)]
mod unique_lock_tests {
    use super::*;
    use parking_lot::RawMutex;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    type AM = AnnotatedMixin<RawMutex>;

    #[traced_test]
    fn lock_unlock_cycle() {
        let am = AM::default();
        let mut ul = UniqueLock::new(&am, "m", file!(), line!(), None);
        assert!(ul.owns_lock());
        ul.unlock();
        assert!(!ul.owns_lock());
        ul.enter();
        assert!(ul.owns_lock());
    }

    #[traced_test]
    fn try_lock_behavior() {
        let am = Arc::new(AM::default());
        let am2 = Arc::clone(&am);

        let _t = thread::spawn(move || {
            am2.lock();
            thread::sleep(Duration::from_millis(150));
            am2.unlock();
        });

        thread::sleep(Duration::from_millis(10));
        let mut ul = UniqueLock::new(&*am, "m", file!(), line!(), Some(true));
        assert!(!ul.owns_lock(), "try_lock should fail while other thread holds it");
    }
}
