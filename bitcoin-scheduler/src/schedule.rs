// ---------------- [ File: bitcoin-scheduler/src/schedule.rs ]
crate::ix!();

impl Schedule for Scheduler {
    /// Call func at/after time t
    ///
    fn schedule(&mut self, f: SchedulerFunction, t: TimePoint) {
        trace!(t = ?t, "Scheduler::schedule: inserting task");

        {
            let mutex_ref = self.new_task_mutex().borrow();
            let mut inner = mutex_ref.lock();

            inner.task_queue_mut().entry(t).or_default().push(f);

            trace!(
                queued_tasks = inner.task_queue().values().map(|v| v.len()).sum::<usize>(),
                queued_timepoints = inner.task_queue().len(),
                "Scheduler::schedule: task enqueued"
            );
        }

        self.new_task_scheduled().notify_one();
        trace!("Scheduler::schedule: notified one waiter");
    }
}

#[cfg(test)]
mod schedule_method_contract_suite {
    use super::*;
    use std::sync::{Arc, Mutex as StdMutex};

    #[traced_test]
    fn schedule_updates_queue_info_first_last_and_size_with_out_of_order_insertions() {
        let mut scheduler = scheduler_for_unit_testing();

        let t0 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(1);
        let t1 = TimePoint::from_std_instant(std::time::Instant::now())
            + time_point::Duration::from_secs(10);

        Schedule::schedule(&mut scheduler, Box::new(|| trace!("task t1 (not run)")), t1);
        Schedule::schedule(&mut scheduler, Box::new(|| trace!("task t0 (not run)")), t0);

        let mut first = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last = TimePoint::from_std_instant(std::time::Instant::now());
        let size = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(size, 2);
        assert_eq!(first, t0);
        assert_eq!(last, t1);
    }

    #[traced_test]
    fn schedule_preserves_insertion_order_for_equal_timepoints_when_serviced() {
        let mut scheduler = scheduler_for_unit_testing();

        let log: Arc<StdMutex<Vec<u32>>> = Arc::new(StdMutex::new(Vec::new()));
        let log1 = log.clone();
        let log2 = log.clone();
        let log3 = log.clone();

        let t = TimePoint::from_std_instant(std::time::Instant::now())
            - time_point::Duration::from_secs(1);

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("running task 1");
                log1.lock().unwrap().push(1);
            }),
            t,
        );

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("running task 2");
                log2.lock().unwrap().push(2);
            }),
            t,
        );

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("running task 3");
                log3.lock().unwrap().push(3);
            }),
            t,
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        let observed = log.lock().unwrap().clone();
        assert_eq!(observed, vec![1, 2, 3]);
    }

    #[traced_test]
    fn schedule_allows_tasks_scheduled_in_the_past_to_run_immediately_when_drained() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran: Arc<StdMutex<bool>> = Arc::new(StdMutex::new(false));
        let ran_cb = ran.clone();

        Schedule::schedule(
            &mut scheduler,
            Box::new(move || {
                trace!("past-due task executed");
                *ran_cb.lock().unwrap() = true;
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(*ran.lock().unwrap());
    }
}
