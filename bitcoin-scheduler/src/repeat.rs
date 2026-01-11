// ---------------- [ File: bitcoin-scheduler/src/repeat.rs ]
crate::ix!();

pub fn repeat(
        s:     &mut Scheduler,
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
    
    trace!(delta = ?delta, "repeat: executing callback and rescheduling");

    let mut f = f;

    f();

    let s_ptr: *mut Scheduler = s as *mut Scheduler;
    let mut next_f: Option<SchedulerFunction> = Some(f);

    s.schedule_from_now(Box::new(move || {
        let f = next_f
            .take()
            .expect("repeat scheduled closure invoked more than once");

        unsafe {
            repeat(&mut *s_ptr, f, delta);
        }
    }), delta);

}

#[cfg(test)]
mod repeat_function_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn repeat_invokes_callback_immediately_and_schedules_one_follow_up() {
        let mut scheduler = scheduler_for_unit_testing();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        repeat(
            &mut scheduler,
            Box::new(move || {
                let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
                trace!(n, "repeat user callback invoked");
            }),
            Duration::milliseconds(0),
        );

        assert_eq!(counter.load(Ordering::SeqCst), 1);

        let mut first = TimePoint::from_std_instant(std::time::Instant::now());
        let mut last = TimePoint::from_std_instant(std::time::Instant::now());
        let size = SchedulerGetQueueInfo::get_queue_info(&scheduler, &mut first, &mut last);

        assert_eq!(size, 1);
    }

    #[traced_test]
    fn repeat_follow_up_runs_via_service_queue_and_can_stop_scheduler_after_n_invocations() {
        let mut scheduler = scheduler_for_unit_testing();

        let scheduler_ptr: *mut Scheduler = &mut scheduler;
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        repeat(
            &mut scheduler,
            Box::new(move || {
                let n = counter_cb.fetch_add(1, Ordering::SeqCst) + 1;
                trace!(n, "repeat user callback invoked");
                if n >= 2 {
                    trace!("repeat user callback requesting stop()");
                    unsafe {
                        Stop::stop(&mut *scheduler_ptr);
                    }
                }
            }),
            Duration::milliseconds(0),
        );

        assert_eq!(counter.load(Ordering::SeqCst), 1);

        ServiceQueue::service_queue(&mut scheduler);

        assert_eq!(counter.load(Ordering::SeqCst), 2);
        assert!(!AreThreadsServicingQueue::are_threads_servicing_queue(&scheduler));
    }
}
