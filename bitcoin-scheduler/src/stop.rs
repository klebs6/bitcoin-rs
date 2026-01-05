// ---------------- [ File: bitcoin-scheduler/src/stop.rs ]
crate::ix!();

impl Stop for Scheduler {

    /// Tell any threads running serviceQueue to stop as soon as the current task is done
    ///
    fn stop(&mut self)  {
        
        todo!();
        /*
            
        [&]() { LOCK(newTaskMutex);  stopRequested = true }()
        ;
            newTaskScheduled.notify_all();
            if (m_service_thread.joinable()) m_service_thread.join();
        */
    }
}
