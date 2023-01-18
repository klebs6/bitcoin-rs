crate::ix!();

pub fn repeat(
        s:     &mut Scheduler,
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
    
    todo!();
        /*
            f();
        s.scheduleFromNow([=, &s] { Repeat(s, f, delta); }, delta);
        */
}

//-------------------------------------------[.cpp/bitcoin/src/scheduler.h]
//-------------------------------------------[.cpp/bitcoin/src/scheduler.cpp]

/**
  | Simple class for background tasks that
  | should be run periodically or once "after
  | a while"
  | 
  | Usage:
  | 
  | -----------
  | @code
  | 
  | CScheduler* s = new CScheduler();
  | s->scheduleFromNow(doSomething, std::chrono::milliseconds{11}); // Assuming a: c_void doSomething() { }
  | s->scheduleFromNow([=] { this->func(argument); }, std::chrono::milliseconds{3});
  | std::thread* t = new std::thread([&] { s->serviceQueue(); });
  |  
  | ... then at program shutdown, make sure to call stop() to clean up the thread(s) running serviceQueue:
  | s->stop();
  | t->join();
  | delete t;
  | delete s; // Must be done after thread is interrupted/joined.
  |
  */
pub struct Scheduler {
    service_thread:     Thread,
    new_task_mutex:     RefCell<Mutex<SchedulerInner>>,
    new_task_scheduled: Condvar,
}

pub struct SchedulerInner {
    task_queue:                MultiMap<TimePoint,SchedulerFunction>,
    n_threads_servicing_queue: i32, // default = { 0 }
    stop_requested:            bool, // default = { false }
    stop_when_empty:           bool, // default = { false }
}

pub type SchedulerFunction = Box<dyn FnMut() -> ()>;

impl Drop for Scheduler {
    fn drop(&mut self) {
        todo!();
        /*
            assert(nThreadsServicingQueue == 0);
        if (stopWhenEmpty) assert(taskQueue.empty());
        */
    }
}

impl Scheduler {

    /**
      | Call f once after the delta has passed
      |
      */
    pub fn schedule_from_now(&mut self, 
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
        
        todo!();
        /*
            schedule(std::move(f), std::chrono::system_clock::now() + delta);
        */
    }

    /**
      | Tell any threads running serviceQueue
      | to stop as soon as the current task is
      | done
      |
      */
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            
        [&]() { LOCK(newTaskMutex);  stopRequested = true }()
        ;
            newTaskScheduled.notify_all();
            if (m_service_thread.joinable()) m_service_thread.join();
        */
    }

    /**
      | Tell any threads running serviceQueue
      | to stop when there is no work left to be
      | done
      |
      */
    pub fn stop_when_drained(&mut self)  {
        
        todo!();
        /*
            
        [&]() { LOCK(newTaskMutex);  stopWhenEmpty = true }()
        ;
            newTaskScheduled.notify_all();
            if (m_service_thread.joinable()) m_service_thread.join();
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(newTaskMutex)]
    pub fn should_stop(&self) -> bool {
        
        todo!();
        /*
            return stopRequested || (stopWhenEmpty && taskQueue.empty());
        */
    }
    
    /**
      | Services the queue 'forever'. Should
      | be run in a thread.
      |
      */
    pub fn service_queue(&mut self)  {
        
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
    
    /**
      | Call func at/after time t
      |
      */
    pub fn schedule(&mut self, 
        f: SchedulerFunction,
        t: TimePoint)  {
        
        todo!();
        /*
            {
            LOCK(newTaskMutex);
            taskQueue.insert(std::make_pair(t, f));
        }
        newTaskScheduled.notify_one();
        */
    }
    
    /**
      | Mock the scheduler to fast forward in
      | time.
      | 
      | Iterates through items on taskQueue
      | and reschedules them to be delta_seconds
      | sooner.
      |
      */
    pub fn mock_forward(&mut self, delta_seconds: Duration /* seconds */)  {
        
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
    
    /**
      | Repeat f until the scheduler is stopped.
      | First run is after delta has passed once.
      | 
      | The timing is not exact: Every time f
      | is finished, it is rescheduled to run
      | again after delta. If you need more accurate
      | scheduling, don't use this method.
      |
      */
    pub fn schedule_every(&mut self, 
        f:     SchedulerFunction,
        delta: Duration /* millis */)  {
        
        todo!();
        /*
            scheduleFromNow([=] { Repeat(*this, f, delta); }, delta);
        */
    }
    
    /**
      | Returns number of tasks waiting to be
      | serviced, and first and last task times
      |
      */
    pub fn get_queue_info(&self, 
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
    
    /**
      | Returns true if there are threads actively
      | running in serviceQueue()
      |
      */
    pub fn are_threads_servicing_queue(&self) -> bool {
        
        todo!();
        /*
            LOCK(newTaskMutex);
        return nThreadsServicingQueue;
        */
    }
}

/**
  | Class used by CScheduler clients which
  | may schedule multiple jobs which are
  | required to be run serially.
  | 
  | Jobs may not be run on the same thread,
  | but no two jobs will be executed at the
  | same time and memory will be release-acquire
  | consistent (the scheduler will internally
  | do an acquire before invoking a callback
  | as well as a release at the end).
  | 
  | In practice this means that a callback
  | 
  | B() will be able to observe all of the
  | effects of callback A() which executed
  | before it.
  |
  */
pub struct SingleThreadedSchedulerClient {
    pscheduler:            *mut Scheduler,
    cs_callbacks_pending:  parking_lot::ReentrantMutex<single_threaded_scheduler_client::Inner>,
}

pub mod single_threaded_scheduler_client {

    use super::*;

    pub struct Inner {
        callbacks_pending:     LinkedList<fn() -> ()>,
        are_callbacks_running: bool, // default = false
    }
}

impl SingleThreadedSchedulerClient {
    
    pub fn new(pscheduler_in: *mut Scheduler) -> Self {
    
        todo!();
        /*
        : pscheduler(pschedulerIn),
        */
    }

    pub fn maybe_schedule_process_queue(&mut self)  {
        
        todo!();
        /*
            {
            LOCK(m_cs_callbacks_pending);
            // Try to avoid scheduling too many copies here, but if we
            // accidentally have two ProcessQueue's scheduled at once its
            // not a big deal.
            if (m_are_callbacks_running) return;
            if (m_callbacks_pending.empty()) return;
        }
        m_pscheduler->schedule(std::bind(&SingleThreadedSchedulerClient::ProcessQueue, this), std::chrono::system_clock::now());
        */
    }
    
    pub fn process_queue(&mut self)  {
        
        todo!();
        /*
            std::function<c_void()> callback;
        {
            LOCK(m_cs_callbacks_pending);
            if (m_are_callbacks_running) return;
            if (m_callbacks_pending.empty()) return;
            m_are_callbacks_running = true;

            callback = std::move(m_callbacks_pending.front());
            m_callbacks_pending.pop_front();
        }

        // RAII the setting of fCallbacksRunning and calling MaybeScheduleProcessQueue
        // to ensure both happen safely even if callback() throws.
        struct RAIICallbacksRunning {
            SingleThreadedSchedulerClient* instance;
            explicit RAIICallbacksRunning(SingleThreadedSchedulerClient* _instance) : instance(_instance) {}
            ~RAIICallbacksRunning()
            {
                {
                    LOCK(instance->m_cs_callbacks_pending);
                    instance->m_are_callbacks_running = false;
                }
                instance->MaybeScheduleProcessQueue();
            }
        } raiicallbacksrunning(this);

        callback();
        */
    }
    
    /**
      | Add a callback to be executed. Callbacks
      | are executed serially and memory is
      | release-acquire consistent between
      | callback executions.
      | 
      | Practically, this means that callbacks
      | can behave as if they are executed in
      | order by a single thread.
      |
      */
    pub fn add_to_process_queue(&mut self, func: fn() -> ())  {
        
        todo!();
        /*
            assert(m_pscheduler);

        {
            LOCK(m_cs_callbacks_pending);
            m_callbacks_pending.emplace_back(std::move(func));
        }
        MaybeScheduleProcessQueue();
        */
    }
    
    /**
      | Processes all remaining queue members
      | on the calling thread, blocking until
      | queue is empty
      | 
      | Must be called after the CScheduler
      | has no remaining processing threads!
      |
      */
    pub fn empty_queue(&mut self)  {
        
        todo!();
        /*
            assert(!m_pscheduler->AreThreadsServicingQueue());
        bool should_continue = true;
        while (should_continue) {
            ProcessQueue();
            LOCK(m_cs_callbacks_pending);
            should_continue = !m_callbacks_pending.empty();
        }
        */
    }
    
    pub fn callbacks_pending(&mut self) -> usize {
        
        todo!();
        /*
            LOCK(m_cs_callbacks_pending);
        return m_callbacks_pending.size();
        */
    }
}
