// ---------------- [ File: bitcoin-scheduler/src/scheduler_get_queue_info.rs ]
crate::ix!();

impl SchedulerGetQueueInfo for Scheduler {

    /// Returns number of tasks waiting to be serviced, and first and last task times
    ///
    fn get_queue_info(&self, 
        first: &mut TimePoint,
        last:  &mut TimePoint) -> usize {
        
        todo!();
        /*
            LOCK(newTaskMutex);
        size_t result = taskQueue.size();
        if (!taskQueue.empty()) {
            first = taskQueue.begin()->first;
            last = taskQueue.rbegin()->first;
        }
        return result;
        */
    }
}
