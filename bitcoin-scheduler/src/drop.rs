// ---------------- [ File: bitcoin-scheduler/src/drop.rs ]
crate::ix!();

impl Drop for Scheduler {
    fn drop(&mut self) {
        trace!("Scheduler::drop: verifying shutdown invariants");

        let mutex_ref = self.new_task_mutex().borrow();
        let inner = mutex_ref.lock();

        assert!(inner.n_threads_servicing_queue() == 0);
        if inner.stop_when_empty() {
            assert!(inner.task_queue().is_empty());
        }
    }
}

#[cfg(test)]
mod scheduler_lifecycle_contract_suite {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn scheduler_drop_allows_non_draining_shutdown_with_pending_tasks() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_cb = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("pending task ran unexpectedly");
                ran_cb.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                + time_point::Duration::from_secs(60),
        );

        assert_eq!(ran.load(Ordering::SeqCst), 0);

        trace!("dropping Scheduler (stop_when_empty=false, pending tasks allowed)");
        drop(scheduler);

        assert_eq!(ran.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn scheduler_drop_enforces_empty_queue_when_stop_when_empty_set() {
        let mut scheduler = scheduler_for_unit_testing();

        Schedule::schedule(
            &mut scheduler,
            Box::new(|| {
                trace!("this task should never run in this test");
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                + time_point::Duration::from_secs(60),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);

        let result = catch_unwind(AssertUnwindSafe(|| {
            trace!("dropping Scheduler (stop_when_empty=true, queue non-empty => must panic)");
            drop(scheduler);
        }));

        assert!(result.is_err());
    }

    #[traced_test]
    fn scheduler_interface_bounds_allow_generic_usage_without_touching_impl_details() {
        fn schedule_and_drain_once_via_interface<S: SchedulerInterface>(s: &mut S) -> usize {
            let counter = Arc::new(AtomicUsize::new(0));
            let counter_cb = counter.clone();

            Schedule::schedule(
                s,
                Box::new(move || {
                    trace!("interface-scheduled task executed");
                    counter_cb.fetch_add(1, Ordering::SeqCst);
                }),
                TimePoint::from_std_instant(std::time::Instant::now())
                    - time_point::Duration::from_secs(1),
            );

            StopWhenDrained::stop_when_drained(s);
            ServiceQueue::service_queue(s);

            counter.load(Ordering::SeqCst)
        }

        let mut scheduler = scheduler_for_unit_testing();
        let ran = schedule_and_drain_once_via_interface(&mut scheduler);

        assert_eq!(ran, 1);
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }
}
