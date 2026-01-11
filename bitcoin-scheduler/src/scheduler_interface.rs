// ---------------- [ File: bitcoin-scheduler/src/scheduler_interface.rs ]
crate::ix!();

pub trait ScheduleFromNow {

    /// Call f once after the delta has passed
    ///
    fn schedule_from_now(
        &mut self, 
        f:     SchedulerFunction,
        delta: Duration
    );
}

pub trait Stop {

    /// Tell any threads running serviceQueue to
    /// stop as soon as the current task is done
    ///
    fn stop(&mut self);
}

pub trait StopWhenDrained {

    /// Tell any threads running serviceQueue to
    /// stop when there is no work left to be done
    ///
    fn stop_when_drained(&mut self);
}

pub trait ShouldStop {

    fn should_stop(&self) -> bool;
}

pub trait ServiceQueue {

    /// Services the queue 'forever'. Should be
    /// run in a thread.
    ///
    fn service_queue(&mut self);
}
   
pub trait Schedule {
    /// Call func at/after time t
    ///
    fn schedule(
        &mut self, 
        f: SchedulerFunction,
        t: TimePoint
    );
}

pub trait SchedulerMockForward {
    /// Mock the scheduler to fast forward in time.
    /// 
    /// Iterates through items on taskQueue and reschedules them to be delta_seconds sooner.
    ///
    fn mock_forward(&mut self, delta_seconds: Duration /* seconds */);
}

pub trait ScheduleEvery {
    /// Repeat f until the scheduler is stopped. First run is after delta has passed once.
    /// 
    /// The timing is not exact: Every time f is finished, it is rescheduled to run again after
    /// delta. If you need more accurate scheduling, don't use this method.
    ///
    fn schedule_every(
        &mut self, 
        f:     SchedulerFunction,
        delta: Duration
    );
}

pub trait SchedulerGetQueueInfo {

    /// Returns number of tasks waiting to be serviced, and first and last task times
    ///
    fn get_queue_info(&self, 
        first: &mut TimePoint,
        last:  &mut TimePoint) -> usize;
}

pub trait AreThreadsServicingQueue {

    /// Returns true if there are threads actively running in serviceQueue()
    ///
    fn are_threads_servicing_queue(&self) -> bool;
}

#[cfg(test)]
mod scheduler_interface_contract_suite {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[traced_test]
    fn scheduler_can_be_exercised_via_scheduler_interface_trait_bounds() {
        fn run_once_through_interface<S: SchedulerInterface>(s: &mut S) -> usize {
            let counter = Arc::new(AtomicUsize::new(0));
            let counter_cb = counter.clone();

            Schedule::schedule(
                s,
                Box::new(move || {
                    trace!("scheduled through SchedulerInterface bounds");
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
        let ran = run_once_through_interface(&mut scheduler);

        assert_eq!(ran, 1);
    }

    #[traced_test]
    fn scheduler_interface_methods_are_callable_through_a_dyn_scheduler_interface_reference() {
        let mut scheduler = scheduler_for_unit_testing();

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_cb = counter.clone();

        let s: &mut dyn SchedulerInterface = &mut scheduler;

        Schedule::schedule(
            s,
            Box::new(move || {
                trace!("dyn SchedulerInterface scheduled task executed");
                counter_cb.fetch_add(1, Ordering::SeqCst);
            }),
            TimePoint::from_std_instant(std::time::Instant::now())
                - time_point::Duration::from_secs(1),
        );

        StopWhenDrained::stop_when_drained(s);
        ServiceQueue::service_queue(s);

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
