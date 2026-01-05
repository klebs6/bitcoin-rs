// ---------------- [ File: bitcoin-scheduler/src/stop_when_drained.rs ]
crate::ix!();

impl StopWhenDrained for Scheduler {

    /// Tell any threads running serviceQueue to stop when there is no work left to be done
    ///
    fn stop_when_drained(&mut self)  {
        
        todo!();
        /*
            
        [&]() { LOCK(newTaskMutex);  stopWhenEmpty = true }()
        ;
            newTaskScheduled.notify_all();
            if (m_service_thread.joinable()) m_service_thread.join();
        */
    }
}
