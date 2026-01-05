// ---------------- [ File: bitcoin-scheduler/src/schedule.rs ]
crate::ix!();

impl Schedule for Scheduler {
    /// Call func at/after time t
    ///
    fn schedule(
        &mut self, 
        f: SchedulerFunction,
        t: TimePoint
    ) {
        todo!();
        /*
            {
            LOCK(newTaskMutex);
            taskQueue.insert(std::make_pair(t, f));
        }
        newTaskScheduled.notify_one();
        */
    }
}
