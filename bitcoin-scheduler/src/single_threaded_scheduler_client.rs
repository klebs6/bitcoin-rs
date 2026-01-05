// ---------------- [ File: bitcoin-scheduler/src/single_threaded_scheduler_client.rs ]
crate::ix!();

/// Class used by CScheduler clients which may schedule multiple jobs which are required to be run
/// serially.
/// 
/// Jobs may not be run on the same thread, but no two jobs will be executed at the same time and
/// memory will be release-acquire consistent (the scheduler will internally do an acquire before
/// invoking a callback as well as a release at the end).
/// 
/// In practice this means that a callback
/// 
/// B() will be able to observe all of the effects of callback A() which executed before it.
///
pub struct SingleThreadedSchedulerClient {
    pscheduler:            *mut Scheduler,
    cs_callbacks_pending:  parking_lot::ReentrantMutex<SingleThreadedSchedulerClientInner>,
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
