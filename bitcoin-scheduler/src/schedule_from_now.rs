// ---------------- [ File: bitcoin-scheduler/src/schedule_from_now.rs ]
crate::ix!();

impl ScheduleFromNow for Scheduler {
    /// Call f once after the delta has passed
    ///
    fn schedule_from_now(&mut self, f: SchedulerFunction, delta: Duration /* millis */) {
        trace!(
            delta = ?delta,
            "Scheduler::schedule_from_now: scheduling relative task"
        );

        let tp_delta = time_duration_to_time_point_duration(delta);

        self.schedule(
            f,
            TimePoint::from_std_instant(std::time::Instant::now()) + tp_delta,
        );
    }
}

#[cfg(test)]
mod schedule_from_now_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn schedule_from_now_can_be_made_due_via_mock_forward_and_then_drained() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicBool::new(false));
        let ran_cb = ran.clone();

        let delta = Duration::seconds(5);

        ScheduleFromNow::schedule_from_now(
            &mut scheduler,
            Box::new(move || {
                trace!("schedule_from_now task executed");
                ran_cb.store(true, Ordering::SeqCst);
            }),
            delta,
        );

        SchedulerMockForward::mock_forward(&mut scheduler, delta);

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(ran.load(Ordering::SeqCst));
    }

    #[traced_test]
    fn schedule_from_now_with_zero_delta_executes_without_waiting_when_drained() {
        let mut scheduler = scheduler_for_unit_testing();

        let ran = Arc::new(AtomicBool::new(false));
        let ran_cb = ran.clone();

        ScheduleFromNow::schedule_from_now(
            &mut scheduler,
            Box::new(move || {
                trace!("schedule_from_now (zero delta) executed");
                ran_cb.store(true, Ordering::SeqCst);
            }),
            Duration::milliseconds(0),
        );

        StopWhenDrained::stop_when_drained(&mut scheduler);
        ServiceQueue::service_queue(&mut scheduler);

        assert!(ran.load(Ordering::SeqCst));
    }
}
