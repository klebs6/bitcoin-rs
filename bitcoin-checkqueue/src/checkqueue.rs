// ---------------- [ File: bitcoin-checkqueue/src/checkqueue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/checkqueue.h]

/**
  | Queue for verifications that have to
  | be performed.
  | 
  | The verifications are represented
  | by a type T, which must provide an operator(),
  | returning a bool.
  | 
  | One thread (the master) is assumed to
  | push batches of verifications onto
  | the queue, where they are processed
  | by N-1 worker threads. When the master
  | is done adding work, it temporarily
  | joins the worker pool as an N'th worker,
  | until all jobs are done.
  |
  */
pub struct CheckQueue<T> {

    /**
      | Mutex to protect the inner state
      |
      */
    mutex_:     std::sync::Mutex<CheckQueueInner<T>>,

    /**
      | Worker threads block on this when out
      | of work
      |
      */
    worker_cv: std::sync::Condvar,

    /**
      | Master thread blocks on this when out
      | of work
      |
      */
    master_cv: std::sync::Condvar,

    /**
      | The maximum number of elements to be
      | processed in one batch
      |
      */
    n_batch_size: u32,

    worker_threads: Vec<Thread>,

    /**
      | Mutex to ensure only one concurrent
      | CCheckQueueControl
      |
      */
    control_mutex: RawMutex,
}

impl<T> Drop for CheckQueue<T> {

    fn drop(&mut self) {
        todo!();
        /*
            assert(m_worker_threads.empty());
        */
    }
}

impl<T> CheckQueue<T> {

    /**
      | Internal function that does bulk of
      | the verification work.
      |
      */
    pub fn loop_(&mut self, master: bool) -> bool {
        
        todo!();
        /*
            std::condition_variable& cond = fMaster ? m_master_cv : m_worker_cv;
            std::vector<T> vChecks;
            vChecks.reserve(nBatchSize);
            unsigned int nNow = 0;
            bool fOk = true;
            do {
                {
                    WAIT_LOCK(m_mutex, lock);
                    // first do the clean-up of the previous loop run (allowing us to do it in the same critsect)
                    if (nNow) {
                        fAllOk &= fOk;
                        nTodo -= nNow;
                        if (nTodo == 0 && !fMaster)
                            // We processed the last element; inform the master it can exit and return the result
                            m_master_cv.notify_one();
                    } else {
                        // first iteration
                        nTotal++;
                    }
                    // logically, the do loop starts here
                    while (queue.empty() && !m_request_stop) {
                        if (fMaster && nTodo == 0) {
                            nTotal--;
                            bool fRet = fAllOk;
                            // reset the status for new work later
                            fAllOk = true;
                            // return the current status
                            return fRet;
                        }
                        nIdle++;
                        cond.wait(lock); // wait
                        nIdle--;
                    }
                    if (m_request_stop) {
                        return false;
                    }

                    // Decide how many work units to process now.
                    // * Do not try to do everything at once, but aim for increasingly smaller batches so
                    //   all workers finish approximately simultaneously.
                    // * Try to account for idle jobs which will instantly start helping.
                    // * Don't do batches smaller than 1 (duh), or larger than nBatchSize.
                    nNow = std::max(1U, std::min(nBatchSize, (unsigned int)queue.size() / (nTotal + nIdle + 1)));
                    vChecks.resize(nNow);
                    for (unsigned int i = 0; i < nNow; i++) {
                        // We want the lock on the m_mutex to be as short as possible, so swap jobs from the global
                        // queue to the local batch vector instead of copying.
                        vChecks[i].swap(queue.back());
                        queue.pop_back();
                    }
                    // Check whether we need to do work at all
                    fOk = fAllOk;
                }
                // execute work
                for (T& check : vChecks)
                    if (fOk)
                        fOk = check();
                vChecks.clear();
            } while (true);
        */
    }

    /**
      | Create a new check queue
      |
      */
    pub fn new(n_batch_size_in: u32) -> Self {
    
        todo!();
        /*
        : n_batch_size(nBatchSizeIn),

        
        */
    }

    /**
      | Create a pool of new worker threads.
      |
      */
    pub fn start_worker_threads(&mut self, threads_num: i32)  {
        
        todo!();
        /*
            {
                LOCK(m_mutex);
                nIdle = 0;
                nTotal = 0;
                fAllOk = true;
            }

            assert(m_worker_threads.empty());

            for (int n = 0; n < threads_num; ++n) {

                m_worker_threads.emplace_back([this, n]() {
                    util::ThreadRename(strprintf("scriptch.%i", n));
                    SetSyscallSandboxPolicy(SyscallSandboxPolicy::VALIDATION_SCRIPT_CHECK);

                    /* worker thread */
                    Loop(false );
                });
            }
        */
    }

    /**
      | Wait until execution finishes, and
      | return whether all evaluations were
      | successful.
      |
      */
    pub fn wait(&mut self) -> bool {
        
        todo!();
        /*
            /* master thread */
            return Loop(true );
        */
    }

    /**
      | Add a batch of checks to the queue
      |
      */
    pub fn add(&mut self, checks: &mut Vec<T>)  {
        
        todo!();
        /*
            LOCK(m_mutex);
            for (T& check : vChecks) {
                queue.push_back(T());
                check.swap(queue.back());
            }
            nTodo += vChecks.size();
            if (vChecks.size() == 1)
                m_worker_cv.notify_one();
            else if (vChecks.size() > 1)
                m_worker_cv.notify_all();
        */
    }

    /**
      | Stop all of the worker threads.
      |
      */
    pub fn stop_worker_threads(&mut self)  {
        
        todo!();
        /*
            
        [&]() { LOCK(m_mutex);  m_request_stop = true }()
        ;
            m_worker_cv.notify_all();
            for (std::thread& t : m_worker_threads) {
                t.join();
            }
            m_worker_threads.clear();
            
        [&]() { LOCK(m_mutex);  m_request_stop = false }()
        ;
        */
    }
}
