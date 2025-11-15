// ---------------- [ File: bitcoinleveldb-limiter/src/limiter.rs ]
crate::ix!();

/**
  | Helper class to limit resource usage to avoid
  | exhaustion.
  |
  | Currently used to limit read-only file
  | descriptors and mmap file usage so that we do
  | not run out of file descriptors or virtual
  | memory, or run into kernel performance problems
  | for very large databases.
  */
#[derive(Debug)]
pub struct Limiter {

    /**
      | The number of available resources.
      |
      | This is a counter and is not tied to the
      | invariants of any other class, so it can be
      | operated on safely using relaxed atomics.
      */
    acquires_allowed: Atomic<i32>,

    /**
      | Maximum number of resources that can ever be
      | acquired.
      |
      | Used to mirror the original LevelDB debug
      | assertions that detect over-release bugs.
      */
    max_acquires: i32,
}

impl Limiter {

    /**
      | Limit maximum number of resources to
      | max_acquires|.
      |
      */
    pub fn new(max_acquires: i32) -> Self {
        trace!(max_acquires, "Limiter::new → constructing");

        debug_assert!(
            max_acquires >= 0,
            "Limiter::new: max_acquires must be non-negative"
        );

        if max_acquires < 0 {
            warn!(
                max_acquires,
                "Limiter::new: negative max_acquires, clamping to zero"
            );
        }

        let initial = if max_acquires < 0 { 0 } else { max_acquires };

        let limiter = Self {
            acquires_allowed: Atomic::new(initial),
            max_acquires:     initial,
        };

        debug!(
            initial,
            "Limiter::new → initialized acquires_allowed and max_acquires"
        );

        limiter
    }

    /**
      | If another resource is available, acquire it
      | and return true.
      |
      | Else return false.
      */
    pub fn acquire(&self) -> bool {
        trace!("Limiter::acquire → start");

        let old_acquires_allowed =
            self.acquires_allowed.fetch_sub(1, atomic::Ordering::Relaxed);

        debug!(
            old_acquires_allowed,
            "Limiter::acquire → post fetch_sub"
        );

        if old_acquires_allowed > 0 {
            trace!(
                old_acquires_allowed,
                "Limiter::acquire → success (resource acquired)"
            );
            true
        } else {
            let pre_increment_acquires_allowed =
                self.acquires_allowed.fetch_add(1, atomic::Ordering::Relaxed);

            debug!(
                pre_increment_acquires_allowed,
                "Limiter::acquire → no capacity; counter restored"
            );

            debug_assert!(
                pre_increment_acquires_allowed < self.max_acquires,
                "Limiter::acquire: Release() was called more times than acquire()"
            );

            trace!(
                pre_increment_acquires_allowed,
                "Limiter::acquire → failure (no resource available)"
            );
            false
        }
    }

    /**
      | Release a resource acquired by a previous
      | call to acquire() that returned true.
      |
      */
    pub fn release(&self) {
        trace!("Limiter::release → start");

        let pre_increment_acquires_allowed =
            self.acquires_allowed.fetch_add(1, atomic::Ordering::Relaxed);

        debug!(
            pre_increment_acquires_allowed,
            "Limiter::release → post fetch_add"
        );

        debug_assert!(
            pre_increment_acquires_allowed < self.max_acquires,
            "Limiter::release: called more times than successful acquire()"
        );

        trace!(
            pre_increment_acquires_allowed,
            "Limiter::release → resource released"
        );
    }
}
