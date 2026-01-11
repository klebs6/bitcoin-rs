// ---------------- [ File: bitcoin-scheduler/src/scheduler_get_queue_info.rs ]
crate::ix!();

impl SchedulerGetQueueInfo for Scheduler {
    /// Returns number of tasks waiting to be serviced, and first and last task times
    ///
    fn get_queue_info(&self, first: &mut TimePoint, last: &mut TimePoint) -> usize {
        trace!("Scheduler::get_queue_info: acquiring new_task_mutex");

        let mutex_ref = self.new_task_mutex().borrow();
        let inner = mutex_ref.lock();

        let result: usize = inner.task_queue().values().map(|v| v.len()).sum();

        if !inner.task_queue().is_empty() {
            if let Some((t_first, _)) = inner.task_queue().iter().next() {
                *first = *t_first;
            }
            if let Some((t_last, _)) = inner.task_queue().iter().last() {
                *last = *t_last;
            }
        }

        trace!(
            result,
            first = ?*first,
            last = ?*last,
            "Scheduler::get_queue_info: done"
        );

        result
    }
}

#[cfg(test)]
mod scheduler_get_queue_info_contract_suite {
    use super::*;

    #[traced_test]
    fn get_queue_info_returns_zero_and_does_not_clobber_first_last_when_queue_empty() {
        let scheduler = scheduler_for_unit_testing();

        let sentinel_first = TimePoint::from_std_instant(std::time::Instant::now())
            - time_point::Duration::from_secs(123);
        let sentinel_last = TimePoint::from_std_instant(std::time::Instant::now())
            - time_point::Duration::from_secs(456);

        let mut first = sentinel_first;
        let mut last = sentinel_last;

        let size = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(size, 0);
        assert_eq!(first, sentinel_first);
        assert_eq!(last, sentinel_last);
    }

    #[traced_test]
    fn get_queue_info_reports_first_and_last_timepoints_for_non_empty_queue() {
        let mut scheduler = scheduler_for_unit_testing();

        let t0 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(1);
        let t1 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(2);
        let t2 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(3);

        Schedule::schedule(&mut scheduler, Box::new(|| trace!("t1 (not run)")), t1);
        Schedule::schedule(&mut scheduler, Box::new(|| trace!("t2 (not run)")), t2);
        Schedule::schedule(&mut scheduler, Box::new(|| trace!("t0 (not run)")), t0);

        let mut first = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last = TimePoint::from_std_instant(std::time::Instant::now());
        let size = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(size, 3);
        assert_eq!(first, t0);
        assert_eq!(last, t2);
    }
}
