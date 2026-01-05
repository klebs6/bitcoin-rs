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
        
        todo!();
        /*
            scheduleFromNow([=] { Repeat(*this, f, delta); }, delta);
        */
    }
}
