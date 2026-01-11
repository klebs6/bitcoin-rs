// ---------------- [ File: bitcoin-scheduler/src/should_stop.rs ]
crate::ix!();

impl ShouldStop for Scheduler {
    #[EXCLUSIVE_LOCKS_REQUIRED(newTaskMutex)]
    fn should_stop(&self) -> bool {
        trace!("Scheduler::should_stop: acquiring new_task_mutex");

        let mutex_ref = self.new_task_mutex().borrow();
        let inner = mutex_ref.lock();

        let result = inner.stop_requested()
            || (inner.stop_when_empty() && inner.task_queue().is_empty());

        trace!(
            stop_requested = inner.stop_requested(),
            stop_when_empty = inner.stop_when_empty(),
            queue_empty = inner.task_queue().is_empty(),
            result,
            "Scheduler::should_stop: done"
        );

        result
    }
}

#[cfg(test)]
mod should_stop_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn should_stop_is_false_for_fresh_scheduler_with_no_flags_and_empty_queue() {
        let scheduler = scheduler_for_unit_testing();
        assert!(!ShouldStop::should_stop(&scheduler));
    }

    #[traced_test]
    fn should_stop_is_true_after_stop_even_if_queue_is_not_empty() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicUsize::new(0));
        let ran_cb = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("task should not run");
                ran_cb.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        assert!(!ShouldStop::should_stop(&scheduler));

        Stop::stop(&mut scheduler);

        assert!(ShouldStop::should_stop(&scheduler));

        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(ran.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn should_stop_tracks_stop_when_empty_and_queue_state_transitions() {
        let mut scheduler = scheduler_for_unit_testing();

        Schedule::schedule(
            &mut scheduler,
            Box::new(|| {
                trace!("drainable task executed");
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);

        assert!(!ShouldStop::should_stop(&scheduler));

        ServiceQueue::service_queue(&mut scheduler);

        assert!(ShouldStop::should_stop(&scheduler));
    }
}
