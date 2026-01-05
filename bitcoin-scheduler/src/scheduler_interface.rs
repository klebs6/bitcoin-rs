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
