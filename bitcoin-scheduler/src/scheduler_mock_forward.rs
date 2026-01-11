// ---------------- [ File: bitcoin-scheduler/src/scheduler_mock_forward.rs ]
crate::ix!();

impl SchedulerMockForward for Scheduler {
    /// Mock the scheduler to fast forward in time.
    ///
    /// Iterates through items on taskQueue and reschedules them to be delta_seconds sooner.
    ///
    fn mock_forward(&mut self, delta_seconds: Duration /* seconds */) {
        trace!(delta_seconds = ?delta_seconds, "Scheduler::mock_forward: begin");

        assert!(
            delta_seconds > Duration::seconds(0) && delta_seconds <= Duration::hours(1),
            "delta_seconds out of range"
        );

        let delta_tp = time_duration_to_time_point_duration(delta_seconds);

        {
            let mutex_ref = self.new_task_mutex().borrow();
            let mut inner = mutex_ref.lock();

            // use temp_queue to maintain updated schedule
            let mut temp_queue: std::collections::BTreeMap<TimePoint, Vec<SchedulerFunction>> =
                Default::default();

            for (t, tasks) in std::mem::take(inner.task_queue_mut()).into_iter() {
                temp_queue.insert(t - delta_tp, tasks);
            }

            // point taskQueue to temp_queue
            *inner.task_queue_mut() = temp_queue;
        }

        // notify that the taskQueue needs to be processed
        self.new_task_scheduled().notify_one();

        trace!("Scheduler::mock_forward: notified one waiter");
    }
}

#[cfg(test)]
mod scheduler_mock_forward_contract_suite {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[traced_test]
    fn mock_forward_panics_on_zero_delta_and_on_delta_greater_than_one_hour() {
        let mut scheduler = scheduler_for_unit_testing();

        let zero = catch_unwind(AssertUnwindSafe(|| {
            SchedulerMockForward::mock_forward(&mut scheduler, Duration::seconds(0));
        }));
        assert!(zero.is_err());

        let too_large = catch_unwind(AssertUnwindSafe(|| {
            SchedulerMockForward::mock_forward(&mut scheduler, Duration::seconds(60 * 60 + 1));
        }));
        assert!(too_large.is_err());
    }

    #[traced_test]
    fn mock_forward_moves_all_tasks_earlier_by_exact_delta_and_preserves_queue_size() {
        let mut scheduler = scheduler_for_unit_testing();

        let t1 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(10);
        let t2 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(20);

        Schedule::schedule(&mut scheduler, Box::new(|| trace!("t1 (not run)")), t1);
        Schedule::schedule(&mut scheduler, Box::new(|| trace!("t2 (not run)")), t2);

        let mut first_before = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last_before = TimePoint::from_std_instant(std::time::Instant::now());
        let size_before =
            SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first_before, &mut last_before);

        assert_eq!(size_before, 2);
        assert_eq!(first_before, t1);
        assert_eq!(last_before, t2);

        let delta = Duration::seconds(5);
        SchedulerMockForward::mock_forward(&mut scheduler, delta);

        let mut first_after = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last_after = TimePoint::from_std_instant(std::time::Instant::now());
        let size_after =
            SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first_after, &mut last_after);

        let delta_tp = time_point::Duration::from_secs(5);

        assert_eq!(size_after, 2);
        assert_eq!(first_after, t1 - delta_tp);
        assert_eq!(last_after, t2 - delta_tp);
    }

    #[traced_test]
    fn mock_forward_enables_immediate_execution_without_sleeping_when_used_with_schedule_from_now() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ran_cb = ran.clone();

        let delta = Duration::seconds(7);

        ScheduleFromNow::schedule_from_now(
            &mut scheduler,
            Box::new(move || {
                trace!("scheduled task executed after mock_forward");
                ran_cb.store(true, std::sync::atomic::Ordering::SeqCst);
            }),
            delta,
        );

        SchedulerMockForward::mock_forward(&mut scheduler, delta);

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(ran.load(std::sync::atomic::Ordering::SeqCst));
    }
}
