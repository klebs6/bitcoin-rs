// ---------------- [ File: bitcoin-scheduler/src/service_queue.rs ]
crate::ix!();

impl ServiceQueue for Scheduler {
    
    /// Services the queue 'forever'. Should be run in a thread.
    ///
    fn service_queue(&mut self)  {
        
        todo!();
        /*
            SetSyscallSandboxPolicy(SyscallSandboxPolicy::SCHEDULER);
        WAIT_LOCK(newTaskMutex, lock);
        ++nThreadsServicingQueue;

        // newTaskMutex is locked throughout this loop EXCEPT
        // when the thread is waiting or when the user's function
        // is called.
        while (!shouldStop()) {
            try {
                while (!shouldStop() && taskQueue.empty()) {
                    // Wait until there is something to do.
                    newTaskScheduled.wait(lock);
                }

                // Wait until either there is a new task, or until
                // the time of the first item on the queue:

                while (!shouldStop() && !taskQueue.empty()) {
                    std::chrono::system_clock::time_point timeToWaitFor = taskQueue.begin()->first;
                    if (newTaskScheduled.wait_until(lock, timeToWaitFor) == std::cv_status::timeout) {
                        break; // Exit loop after timeout, it means we reached the time of the event
                    }
                }

                // If there are multiple threads, the queue can empty while we're waiting (another
                // thread may service the task we were waiting on).
                if (shouldStop() || taskQueue.empty())
                    continue;

                SchedulerFunction f = taskQueue.begin()->second;
                taskQueue.erase(taskQueue.begin());

                {
                    // Unlock before calling f, so it can reschedule itself or another task
                    // without deadlocking:
                    REVERSE_LOCK(lock);
                    f();
                }
            } catch (...) {
                --nThreadsServicingQueue;
                throw;
            }
        }
        --nThreadsServicingQueue;
        newTaskScheduled.notify_one();
        */
    }
}
