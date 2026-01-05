// ---------------- [ File: bitcoin-scheduler/src/schedule_from_now.rs ]
crate::ix!();

impl ScheduleFromNow for Scheduler {

    /// Call f once after the delta has passed
    ///
    fn schedule_from_now(&mut self, 
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
        
        todo!();
        /*
            schedule(std::move(f), std::chrono::system_clock::now() + delta);
        */
    }
}
