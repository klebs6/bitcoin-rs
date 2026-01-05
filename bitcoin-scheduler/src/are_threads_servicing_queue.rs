// ---------------- [ File: bitcoin-scheduler/src/are_threads_servicing_queue.rs ]
crate::ix!();

impl AreThreadsServicingQueue for Scheduler {
    
    /// Returns true if there are threads actively running in serviceQueue()
    ///
    fn are_threads_servicing_queue(&self) -> bool {
        
        todo!();
        /*
            LOCK(newTaskMutex);
        return nThreadsServicingQueue;
        */
    }
}
