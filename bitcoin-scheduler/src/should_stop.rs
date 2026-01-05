// ---------------- [ File: bitcoin-scheduler/src/should_stop.rs ]
crate::ix!();

impl ShouldStop for Scheduler {

    #[EXCLUSIVE_LOCKS_REQUIRED(newTaskMutex)]
    fn should_stop(&self) -> bool {
        
        todo!();
        /*
            return stopRequested || (stopWhenEmpty && taskQueue.empty());
        */
    }
}
