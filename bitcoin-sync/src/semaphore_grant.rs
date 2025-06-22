// ---------------- [ File: bitcoin-sync/src/semaphore_grant.rs ]
crate::ix!();

/// RAII wrapper around a [`Semaphore`] permit.
///
/// A `SemaphoreGrant` **owns** one permit while
/// `have_grant == true`. On drop, any owned permit
/// is automatically returned to the semaphore.
#[derive(Clone)]
pub struct SemaphoreGrant {
    sem:        Arc<Semaphore>,
    have_grant: bool,
}

impl Drop for SemaphoreGrant {
    fn drop(&mut self) {
        self.release();
    }
}

impl From<&SemaphoreGrant> for bool {
    #[inline]
    fn from(sg: &SemaphoreGrant) -> Self {
        sg.have_grant
    }
}

impl SemaphoreGrant {
    /// Acquire a permit, blocking if none available.
    pub fn acquire(&mut self) {
        trace!("SemaphoreGrant::acquire");
        if self.have_grant {
            return;
        }
        self.sem.wait();
        self.have_grant = true;
    }

    /// Release the owned permit, if any.
    pub fn release(&mut self) {
        trace!("SemaphoreGrant::release");
        if !self.have_grant {
            return;
        }
        self.sem.post();
        self.have_grant = false;
    }

    /// Attempt to acquire without blocking.
    ///
    /// Returns `true` on success.
    pub fn try_acquire(&mut self) -> bool {
        trace!("SemaphoreGrant::try_acquire");
        if !self.have_grant && self.sem.try_wait() {
            self.have_grant = true;
        }
        self.have_grant
    }

    /// Transfer ownership of a permit to another `SemaphoreGrant`.
    pub fn move_to(&mut self, target: &mut SemaphoreGrant) {
        trace!("SemaphoreGrant::move_to");
        target.release();
        target.sem        = Arc::clone(&self.sem);
        target.have_grant = self.have_grant;
        self.have_grant   = false;
    }

    /// Construct a new `SemaphoreGrant`.
    ///
    /// If `try_ == true`, attempts a non‑blocking acquire;
    /// otherwise blocks until a permit is obtained.
    pub fn new(sema: Arc<Semaphore>, try_: Option<bool>) -> Self {
        let mut grant = Self {
            sem: sema,
            have_grant: false,
        };
        if try_.unwrap_or(false) {
            grant.try_acquire();
        } else {
            grant.acquire();
        }
        grant
    }
}

/// ---------------- tests
#[cfg(test)]
mod semaphore_grant_tests {
    use super::*;
    use std::thread;
    use std::time::{Duration as StdDuration, Instant};

    #[traced_test]
    fn basic_acquire_release_cycle() {
        let sem  = Arc::new(Semaphore::new(1));
        let mut g1 = SemaphoreGrant::new(Arc::clone(&sem), Some(true));
        assert!(bool::from(&g1), "g1 should have the permit");

        let mut g2 = SemaphoreGrant::new(Arc::clone(&sem), Some(true));
        assert!(!bool::from(&g2), "g2 should not have acquired yet");

        g1.release();
        assert!(!bool::from(&g1), "g1 released");

        assert!(g2.try_acquire(), "g2 now acquires after g1 release");
    }

    #[traced_test]
    fn move_transfers_ownership() {
        let sem  = Arc::new(Semaphore::new(1));
        let mut a = SemaphoreGrant::new(Arc::clone(&sem), None);
        let mut b = SemaphoreGrant::new(Arc::clone(&sem), Some(true));

        assert!(bool::from(&a));
        assert!(!bool::from(&b));

        a.move_to(&mut b);

        assert!(!bool::from(&a));
        assert!(bool::from(&b));
    }

    #[traced_test]
    fn drop_returns_permit() {
        let sem = Arc::new(Semaphore::new(1));
        {
            let _g = SemaphoreGrant::new(Arc::clone(&sem), None);
            // Permit is held here.
            assert!(!sem.try_wait(), "semaphore exhausted while _g alive");
        }
        // After `_g` is dropped the permit is returned.
        assert!(sem.try_wait(), "permit returned after drop");
    }
}
