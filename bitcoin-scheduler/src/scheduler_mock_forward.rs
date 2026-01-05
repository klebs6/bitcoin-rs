// ---------------- [ File: bitcoin-scheduler/src/scheduler_mock_forward.rs ]
crate::ix!();

impl SchedulerMockForward for Scheduler {
    
    /// Mock the scheduler to fast forward in time.
    /// 
    /// Iterates through items on taskQueue and reschedules them to be delta_seconds sooner.
    ///
    fn mock_forward(&mut self, delta_seconds: Duration /* seconds */)  {
        
        todo!();
        /*
            assert(delta_seconds > 0s && delta_seconds <= 1h);

        {
            LOCK(newTaskMutex);

            // use temp_queue to maintain updated schedule
            std::multimap<std::chrono::system_clock::time_point, Function> temp_queue;

            for (const auto& element : taskQueue) {
                temp_queue.emplace_hint(temp_queue.cend(), element.first - delta_seconds, element.second);
            }

            // point taskQueue to temp_queue
            taskQueue = std::move(temp_queue);
        }

        // notify that the taskQueue needs to be processed
        newTaskScheduled.notify_one();
        */
    }
}
