// ---------------- [ File: bitcoin-scheduler/src/schedule_every.rs ]
crate::ix!();

impl ScheduleEvery for Scheduler {
    
    /// Repeat f until the scheduler is stopped. First run is after delta has passed once.
    /// 
    /// The timing is not exact: Every time f is finished, it is rescheduled to run again after
    /// delta. If you need more accurate scheduling, don't use this method.
    ///
    fn schedule_every(&mut self, 
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
        
        trace!(delta = ?delta, "Scheduler::schedule_every: scheduling first Repeat after delta");

        let scheduler_ptr: *mut Scheduler = self as *mut Scheduler;
        let mut f_opt: Option<SchedulerFunction> = Some(f);

        self.schedule_from_now(Box::new(move || {
            let f = f_opt
                .take()
                .expect("schedule_every closure invoked more than once");

            unsafe {
                repeat(&mut *scheduler_ptr, f, delta);
            }
        }), delta);
    }
}

#[cfg(test)]
mod schedule_every_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn schedule_every_does_not_invoke_callback_until_service_queue_runs() {
        let mut scheduler = scheduler_for_unit_testing();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        ScheduleEvery::schedule_every(
            &mut scheduler,
            Box::new(move || {
                let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
                trace!(n, "schedule_every callback invoked");
            }),
            Duration::milliseconds(0),
        );

        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn schedule_every_repeats_until_stop_requested_and_can_run_multiple_times_without_waiting() {
        let mut scheduler = scheduler_for_unit_testing();

        let scheduler_ptr: *mut Scheduler = &mut scheduler;

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        ScheduleEvery::schedule_every(
            &mut scheduler,
            Box::new(move || {
                let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
                trace!(n, "schedule_every callback invoked");
                if n >= 3 {
                    trace!("schedule_every callback requesting stop()");
                    unsafe {
                        Stop::stop(&mut *scheduler_ptr);
                    }
                }
            }),
            Duration::milliseconds(0),
        );

        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(counter.load(Ordering::SeqCst), 3);
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }

    #[traced_test]
    fn schedule_every_with_nonzero_delta_can_be_triggered_via_mock_forward_without_sleeping() {
        let mut scheduler = scheduler_for_unit_testing();

        let scheduler_ptr: *mut Scheduler = &mut scheduler;

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        let delta = Duration::seconds(10);

        ScheduleEvery::schedule_every(
            &mut scheduler,
            Box::new(move || {
                let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
                trace!(n, "schedule_every callback invoked");
                trace!("requesting stop() after first invocation");
                unsafe {
                    Stop::stop(&mut *scheduler_ptr);
                }
            }),
            delta,
        );

        assert_eq!(counter.load(Ordering::SeqCst), 0);

        SchedulerMockForward::mock_forward(&mut scheduler, delta);
        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
