// ---------------- [ File: bitcoinleveldb-dbimplinner/src/condvar_signal_ext.rs ]
crate::ix!();

pub trait CondvarSignalExt {
    fn signal(&self);
    fn signal_all(&self);
}

impl CondvarSignalExt for Condvar {
    /// Invariant:
    /// - `signal()` must be observationally equivalent to `parking_lot::Condvar::notify_one()`.
    /// - This method must not perform allocation.
    /// - This method must never panic.
    #[inline]
    fn signal(&self) {
        #[cfg(any(test, debug_assertions))]
        {
            tracing::trace!(
                target: "bitcoinleveldb.dbimplinner.compat",
                label = "condvar.signal.notify_one",
                condvar_ptr = (self as *const Condvar) as usize
            );
        }

        self.notify_one();
    }

    /// Invariant:
    /// - `signal_all()` must be observationally equivalent to `parking_lot::Condvar::notify_all()`.
    /// - This method must not perform allocation.
    /// - This method must never panic.
    #[inline]
    fn signal_all(&self) {
        #[cfg(any(test, debug_assertions))]
        {
            tracing::trace!(
                target: "bitcoinleveldb.dbimplinner.compat",
                label = "condvar.signal.notify_all",
                condvar_ptr = (self as *const Condvar) as usize
            );
        }

        self.notify_all();
    }
}

#[cfg(test)]
mod condvar_signal_ext_wiring_contract_suite_20260303 {
    use super::*;

    struct CondvarSignalExtHandshakeState_20260303 {
        ready: bool,
        go: bool,
    }

    struct CondvarSignalAllExtHandshakeState_20260303 {
        ready_count: u32,
        go: bool,
    }

    /// Invariant:
    /// - `CondvarSignalExt::signal()` must cause a waiting thread to observe `timed_out=false`
    ///   on `Condvar::wait_for(...)` when the waiter is known to be waiting.
    #[traced_test]
    fn condvar_signal_ext_signal_wakes_waiter_without_timeout_20260303() {
        tracing::info!(
            target: "bitcoinleveldb.dbimplinner.compat.test",
            label = "condvar_signal_ext.signal.wakes_waiter",
            case = "signal"
        );

        let state: Arc<Mutex<CondvarSignalExtHandshakeState_20260303>> = Arc::new(Mutex::new(
            CondvarSignalExtHandshakeState_20260303 {
                ready: false,
                go: false,
            },
        ));

        let cv_ready: Arc<Condvar> = Arc::new(Condvar::new());
        let cv_go: Arc<Condvar> = Arc::new(Condvar::new());

        let waiter_timed_out: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let state_waiter = state.clone();
        let cv_ready_waiter = cv_ready.clone();
        let cv_go_waiter = cv_go.clone();
        let waiter_timed_out_waiter = waiter_timed_out.clone();

        let waiter = thread::spawn(move || {
            tracing::trace!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal.waiter_thread.entry",
                role = "waiter"
            );

            let mut g = state_waiter.lock();

            g.ready = true;
            cv_ready_waiter.signal();

            let wait_res = cv_go_waiter.wait_for(&mut g, Duration::from_millis(1500));

            if wait_res.timed_out() {
                tracing::error!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal.waiter_thread.wait_for.timed_out",
                    role = "waiter"
                );
                waiter_timed_out_waiter.store(true, Ordering::Release);
                return;
            }

            if !g.go {
                tracing::error!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal.waiter_thread.woke_but_predicate_false",
                    role = "waiter"
                );
                waiter_timed_out_waiter.store(true, Ordering::Release);
                return;
            }

            tracing::trace!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal.waiter_thread.exit_ok",
                role = "waiter"
            );
        });

        let mut g = state.lock();

        let mut main_wait_timed_out: bool = false;
        while !g.ready {
            let wait_res = cv_ready.wait_for(&mut g, Duration::from_millis(1500));
            if wait_res.timed_out() {
                main_wait_timed_out = true;
                break;
            }
        }

        if main_wait_timed_out {
            tracing::error!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal.main_thread.ready_wait.timed_out",
                case = "signal"
            );
            panic!();
        }

        if !g.ready {
            tracing::error!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal.main_thread.ready_predicate_false_after_wait",
                case = "signal"
            );
            panic!();
        }

        g.go = true;
        cv_go.signal();

        drop(g);

        match waiter.join() {
            Ok(()) => {}
            Err(_panic_payload) => {
                tracing::error!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal.waiter_thread.panicked",
                    case = "signal"
                );
                panic!();
            }
        }

        if waiter_timed_out.load(Ordering::Acquire) {
            tracing::error!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal.failure_flag_set",
                case = "signal"
            );
            panic!();
        }

        tracing::info!(
            target: "bitcoinleveldb.dbimplinner.compat.test",
            label = "condvar_signal_ext.signal.ok",
            case = "signal"
        );
    }

    /// Invariant:
    /// - `CondvarSignalExt::signal_all()` must cause all waiting threads to observe `timed_out=false`
    ///   on `Condvar::wait_for(...)` when the waiters are known to be waiting.
    #[traced_test]
    fn condvar_signal_ext_signal_all_wakes_all_waiters_without_timeout_20260303() {
        tracing::info!(
            target: "bitcoinleveldb.dbimplinner.compat.test",
            label = "condvar_signal_ext.signal_all.wakes_all",
            case = "signal_all",
            waiters = 2u64
        );

        let state: Arc<Mutex<CondvarSignalAllExtHandshakeState_20260303>> = Arc::new(Mutex::new(
            CondvarSignalAllExtHandshakeState_20260303 {
                ready_count: 0,
                go: false,
            },
        ));

        let cv_ready: Arc<Condvar> = Arc::new(Condvar::new());
        let cv_go: Arc<Condvar> = Arc::new(Condvar::new());

        let timed_out_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));

        let make_waiter = |waiter_id: u32| {
            let st = state.clone();
            let r = cv_ready.clone();
            let g = cv_go.clone();
            let to = timed_out_count.clone();

            thread::spawn(move || {
                tracing::trace!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal_all.waiter_thread.entry",
                    role = "waiter",
                    waiter_id = waiter_id as u64
                );

                let mut guard = st.lock();

                guard.ready_count = guard.ready_count.saturating_add(1);
                r.signal_all();

                let wait_res = g.wait_for(&mut guard, Duration::from_millis(1500));

                if wait_res.timed_out() {
                    tracing::error!(
                        target: "bitcoinleveldb.dbimplinner.compat.test",
                        label = "condvar_signal_ext.signal_all.waiter_thread.wait_for.timed_out",
                        role = "waiter",
                        waiter_id = waiter_id as u64
                    );
                    to.fetch_add(1, Ordering::AcqRel);
                    return;
                }

                if !guard.go {
                    tracing::error!(
                        target: "bitcoinleveldb.dbimplinner.compat.test",
                        label = "condvar_signal_ext.signal_all.waiter_thread.woke_but_predicate_false",
                        role = "waiter",
                        waiter_id = waiter_id as u64
                    );
                    to.fetch_add(1, Ordering::AcqRel);
                    return;
                }

                tracing::trace!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal_all.waiter_thread.exit_ok",
                    role = "waiter",
                    waiter_id = waiter_id as u64
                );
            })
        };

        let w1 = make_waiter(1);
        let w2 = make_waiter(2);

        let mut guard = state.lock();

        let mut main_ready_wait_timed_out: bool = false;
        while guard.ready_count < 2 {
            let wait_res = cv_ready.wait_for(&mut guard, Duration::from_millis(1500));
            if wait_res.timed_out() {
                main_ready_wait_timed_out = true;
                break;
            }
        }

        if main_ready_wait_timed_out {
            tracing::error!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal_all.main_thread.ready_wait.timed_out",
                case = "signal_all",
                observed_ready_count = guard.ready_count as u64
            );
            panic!();
        }

        if guard.ready_count < 2 {
            tracing::error!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal_all.main_thread.ready_predicate_false_after_wait",
                case = "signal_all",
                observed_ready_count = guard.ready_count as u64
            );
            panic!();
        }

        guard.go = true;
        cv_go.signal_all();

        drop(guard);

        match w1.join() {
            Ok(()) => {}
            Err(_panic_payload) => {
                tracing::error!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal_all.waiter_thread.panicked",
                    waiter_id = 1u64
                );
                panic!();
            }
        }

        match w2.join() {
            Ok(()) => {}
            Err(_panic_payload) => {
                tracing::error!(
                    target: "bitcoinleveldb.dbimplinner.compat.test",
                    label = "condvar_signal_ext.signal_all.waiter_thread.panicked",
                    waiter_id = 2u64
                );
                panic!();
            }
        }

        let to = timed_out_count.load(Ordering::Acquire);
        if to != 0 {
            tracing::error!(
                target: "bitcoinleveldb.dbimplinner.compat.test",
                label = "condvar_signal_ext.signal_all.timeout_count_nonzero",
                case = "signal_all",
                timed_out_count = to as u64
            );
            panic!();
        }

        tracing::info!(
            target: "bitcoinleveldb.dbimplinner.compat.test",
            label = "condvar_signal_ext.signal_all.ok",
            case = "signal_all",
            waiters = 2u64
        );
    }
}
