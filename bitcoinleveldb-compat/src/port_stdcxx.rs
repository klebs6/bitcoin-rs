// ---------------- [ File: bitcoinleveldb-compat/src/port_stdcxx.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/port/port_stdcxx.h]

pub mod port {

    use super::*;

    use std::os::raw::c_void;
    use std::sync::{
        Condvar as StdCondvar,
        Mutex as StdMutex,
        atomic::{AtomicBool, Ordering},
    };

    /**
       | The following boolean constant must be true on
       | a little-endian machine and false otherwise.
       |
       | or some other expression
       |
      */
    pub const LITTLE_ENDIAN: bool = !crate::port::LEVELDB_IS_BIG_ENDIAN;

    /**
      | Thin wrapper around a low‑level mutex primitive.
      |
      | The API mirrors the original LevelDB `port::Mutex`
      | (explicit `lock()` / `unlock()` calls and an
      | `assert_held()` helper) while the implementation
      | uses `parking_lot::RawMutex` under the hood.
      */
    #[LOCKABLE]
    pub struct Mutex {
        mu:        parking_lot::RawMutex,
        is_locked: AtomicBool,
    }

    unsafe impl Send for Mutex {}
    unsafe impl Sync for Mutex {}

    impl Default for Mutex {
        #[instrument(level = "trace", skip_all)]
        fn default() -> Self {
            trace!("constructing port::Mutex with unlocked state");
            Self {
                mu:        parking_lot::RawMutex::INIT,
                is_locked: AtomicBool::new(false),
            }
        }
    }

    impl Mutex {
        #[instrument(level = "trace", skip_all)]
        pub fn new() -> Self {
            Self::default()
        }

        #[EXCLUSIVE_LOCK_FUNCTION]
        #[instrument(level = "trace", skip_all)]
        pub fn lock(&mut self) {
            trace!("port::Mutex::lock requested");
            self.mu.lock();
            self.is_locked.store(true, Ordering::SeqCst);
            trace!("port::Mutex::lock acquired");
        }

        #[UNLOCK_FUNCTION]
        #[instrument(level = "trace", skip_all)]
        pub fn unlock(&mut self) {
            trace!("port::Mutex::unlock requested");
            self.is_locked.store(false, Ordering::SeqCst);
            unsafe {
                self.mu.unlock();
            }
            trace!("port::Mutex::unlock completed");
        }

        #[ASSERT_EXCLUSIVE_LOCK]
        #[instrument(level = "trace", skip_all)]
        pub fn assert_held(&mut self) {
            let held = self.is_locked.load(Ordering::SeqCst);
            trace!(held, "port::Mutex::assert_held");
            debug_assert!(
                held,
                "port::Mutex::assert_held: mutex is expected to be locked"
            );
        }
    }
      
    /**
      | Condition variable wired to a `port::Mutex`.
      |
      | Semantics:
      |   - `wait()` must be called with the associated
      |     mutex locked; it atomically:
      |       * registers the current thread as a waiter,
      |       * releases the mutex,
      |       * sleeps, and then
      |       * re‑acquires the mutex before returning.
      |   - `signal()` wakes a single waiter if any.
      |   - `signal_all()` wakes all waiters.
      |
      | Internally this is implemented in terms of
      | `std::sync::Condvar` plus a small waiter count
      | protected by an internal `StdMutex`.
      */
    pub struct CondVar {
        cv:      StdCondvar,
        mu:      *const Mutex,
        waiters: StdMutex<usize>,
    }

    unsafe impl Send for CondVar {}
    unsafe impl Sync for CondVar {}

    impl CondVar {
        #[instrument(level = "trace", skip_all)]
        pub fn new(mu: *mut Mutex) -> Self {
            debug_assert!(
                !mu.is_null(),
                "port::CondVar::new requires a non‑null mutex pointer"
            );
            trace!(ptr = ?mu, "constructing port::CondVar");
            Self {
                cv:      StdCondvar::new(),
                mu,
                waiters: StdMutex::new(0),
            }
        }

        #[instrument(level = "trace", skip_all)]
        pub fn wait(&mut self) {
            unsafe {
                debug_assert!(
                    !self.mu.is_null(),
                    "port::CondVar::wait called with null mutex pointer"
                );

                let mu_ref: &mut Mutex = &mut *(self.mu as *mut Mutex);

                // Register as a waiter while still holding the
                // external mutex.  This mirrors the usual
                // “check predicate under lock, then wait()”
                // pattern and avoids lost wake‑ups.
                let mut guard = match self.waiters.lock() {
                    Ok(g) => g,
                    Err(poisoned) => {
                        warn!(
                            "port::CondVar waiter mutex was poisoned; \
                             continuing with recovered state"
                        );
                        poisoned.into_inner()
                    }
                };

                *guard += 1;
                trace!(waiters = *guard, "port::CondVar::wait registering waiter");

                // Release the external mutex before sleeping.
                mu_ref.unlock();

                // Sleep; this releases the internal waiter mutex
                // while blocked and re‑acquires it on wake‑up.
                guard = match self.cv.wait(guard) {
                    Ok(g) => g,
                    Err(poisoned) => {
                        warn!(
                            "port::CondVar wait guard was poisoned; \
                             continuing with recovered state"
                        );
                        poisoned.into_inner()
                    }
                };

                *guard -= 1;
                trace!(waiters = *guard, "port::CondVar::wait woke up");
                drop(guard);

                // Re‑acquire the external mutex before returning
                // to user code.
                mu_ref.lock();
            }
        }

        #[instrument(level = "trace", skip_all)]
        pub fn signal(&mut self) {
            let guard = match self.waiters.lock() {
                Ok(g) => g,
                Err(poisoned) => {
                    warn!(
                        "port::CondVar::signal waiter mutex poisoned; \
                         continuing with recovered state"
                    );
                    poisoned.into_inner()
                }
            };
            let waiters = *guard;
            if waiters > 0 {
                trace!(waiters, "port::CondVar::signal notifying one waiter");
                self.cv.notify_one();
            } else {
                trace!("port::CondVar::signal called with no registered waiters");
            }
        }

        #[instrument(level = "trace", skip_all)]
        pub fn signal_all(&mut self) {
            let guard = match self.waiters.lock() {
                Ok(g) => g,
                Err(poisoned) => {
                    warn!(
                        "port::CondVar::signal_all waiter mutex poisoned; \
                         continuing with recovered state"
                    );
                    poisoned.into_inner()
                }
            };
            let waiters = *guard;
            if waiters > 0 {
                trace!(waiters, "port::CondVar::signal_all notifying all waiters");
                self.cv.notify_all();
            } else {
                trace!(
                    "port::CondVar::signal_all called with no registered waiters"
                );
            }
        }
    }

    //-------------------------------------------[.cpp/bitcoin/src/leveldb/port/port_example.h]

    /*
       | This file contains the specification, but not
       | the implementations, of the
       | types/operations/etc. that should be defined by
       | a platform specific port_<platform>.h file.
       | Use this file as a reference for how to port
       | this package to a new platform.
       |
       | TODO(jorlow): Many of these belong more in the
       |               environment class rather than
       |               here. We should try moving them
       |               and see if it affects perf.
       */

    // ------------------ Miscellaneous -------------------
}
