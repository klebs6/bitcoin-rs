// ---------------- [ File: bitcoin-checkqueue/src/checkqueue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/checkqueue.h]

pub trait CheckQueueTask: Default {
    fn invoke(&mut self) -> bool;
    fn swap(&mut self, x: &mut Self);
}

/// Queue for verifications that have to
/// be performed.
/// 
/// The verifications are represented
/// by a type T, which must provide an operator(),
/// returning a bool.
/// 
/// One thread (the master) is assumed to
/// push batches of verifications onto
/// the queue, where they are processed
/// by N-1 worker threads. When the master
/// is done adding work, it temporarily
/// joins the worker pool as an N'th worker,
/// until all jobs are done.
/// 
#[derive(Getters,MutGetters,Setters)]
#[getset(get="pub",set="pub",get_mut="pub")]
pub struct CheckQueue<T: CheckQueueTask> {

    /// Mutex to protect the inner state
    /// 
    mutex_:     std::sync::Mutex<CheckQueueInner<T>>,

    /// Worker threads block on this when out
    /// of work
    /// 
    worker_cv: std::sync::Condvar,

    /// Master thread blocks on this when out
    /// of work
    /// 
    master_cv: std::sync::Condvar,

    /// The maximum number of elements to be
    /// processed in one batch
    /// 
    n_batch_size: u32,

    worker_threads: Vec<JoinHandle<()>>,

    /// Mutex to ensure only one concurrent
    /// CCheckQueueControl
    /// 
    control_mutex: RawMutex,
}

impl<T: CheckQueueTask> Drop for CheckQueue<T> {

    fn drop(&mut self) {
        tracing::trace!("CheckQueue::drop");
        assert!(self.worker_threads.is_empty());
    }
}

impl<T: CheckQueueTask> CheckQueue<T> {

    /// Internal function that does bulk of
    /// the verification work.
    /// 
    pub fn loop_(&self, master: bool) -> bool {
        
        let cond: &std::sync::Condvar = if master { &self.master_cv } else { &self.worker_cv };
        let mut v_checks: Vec<T> = Vec::with_capacity(self.n_batch_size as usize);
        let mut n_now: u32 = 0;
        let mut f_ok: bool = true;

        tracing::trace!(
            master,
            n_batch_size = self.n_batch_size,
            "CheckQueue::loop_ enter"
        );

        loop {
            {
                let mut inner = self.mutex_.lock().unwrap();

                // first do the clean-up of the previous loop run (allowing us to do it in the same critsect)
                if n_now != 0 {
                    let all_ok = *inner.all_ok();
                    inner.set_all_ok(all_ok && f_ok);
                    *inner.n_todo_mut() -= n_now;
                    if *inner.n_todo() == 0 && !master {
                        // We processed the last element; inform the master it can exit and return the result
                        self.master_cv.notify_one();
                    }
                } else {
                    // first iteration
                    *inner.n_total_mut() += 1;
                }

                // logically, the do loop starts here
                while inner.queue().is_empty() && !inner.request_stop() {
                    if master && *inner.n_todo() == 0 {
                        *inner.n_total_mut() -= 1;
                        let f_ret: bool = *inner.all_ok();
                        // reset the status for new work later
                        inner.set_all_ok(true);
                        // return the current status
                        tracing::trace!(
                            master,
                            f_ret,
                            "CheckQueue::loop_ master returns"
                        );
                        return f_ret;
                    }

                    *inner.n_idle_mut() += 1;
                    inner = cond.wait(inner).unwrap(); // wait
                    *inner.n_idle_mut() -= 1;
                }

                if *inner.request_stop() {
                    tracing::trace!(master, "CheckQueue::loop_ stop requested");
                    return false;
                }

                // Decide how many work units to process now.
                // * Do not try to do everything at once, but aim for increasingly smaller batches so
                //   all workers finish approximately simultaneously.
                // * Try to account for idle jobs which will instantly start helping.
                // * Don't do batches smaller than 1 (duh), or larger than nBatchSize.
                debug_assert!(*inner.n_total() >= 0);
                debug_assert!(*inner.n_idle() >= 0);

                let denom_i32: i32 = inner.n_total() + inner.n_idle() + 1;
                debug_assert!(denom_i32 > 0);

                let denom_u32: u32 = denom_i32 as u32;
                let qsize_u32: u32 = std::cmp::min(inner.queue().len(), u32::MAX as usize) as u32;

                n_now = std::cmp::max(
                    1u32,
                    std::cmp::min(self.n_batch_size, qsize_u32 / denom_u32),
                );

                v_checks.resize_with(n_now as usize, Default::default);

                for i in 0..(n_now as usize) {
                    // We want the lock on the m_mutex to be as short as possible, so swap jobs from the global
                    // queue to the local batch vector instead of copying.
                    {
                        let back = inner.queue_mut().last_mut().unwrap();
                        v_checks[i].swap(back);
                    }
                    inner.queue_mut().pop();
                }

                // Check whether we need to do work at all
                f_ok = *inner.all_ok();
            }

            // execute work
            for check in v_checks.iter_mut() {
                if f_ok {
                    f_ok = check.invoke();
                }
            }
            v_checks.clear();
        }

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

    /// Create a new check queue
    /// 
    pub fn new(n_batch_size_in: u32) -> Self {
    
        tracing::trace!(n_batch_size_in, "CheckQueue::new");

        Self {
            mutex_: std::sync::Mutex::new(CheckQueueInner {
                queue: Vec::new(),
                n_idle: 0,
                n_total: 0,
                all_ok: true,
                n_todo: 0,
                request_stop: false,
            }),
            worker_cv: std::sync::Condvar::new(),
            master_cv: std::sync::Condvar::new(),
            n_batch_size: n_batch_size_in,
            worker_threads: Vec::new(),
            control_mutex: RawMutex::INIT,
        }
    }

    /// Create a pool of new worker threads.
    /// 
    pub fn start_worker_threads(&mut self, threads_num: i32)
    where
        T: Send,
    {
        tracing::info!(threads_num, "CheckQueue::start_worker_threads");

        {
            let mut inner = self.mutex_.lock().unwrap();
            inner.set_n_idle(0);
            inner.set_n_total(0);
            inner.set_all_ok(true);
        }

        assert!(self.worker_threads.is_empty());

        let this: *const CheckQueue<T> = self as *const CheckQueue<T>;

        for n in 0..threads_num {
            self.worker_threads.push(std::thread::spawn(move || {
                let thread_name = format!("scriptch.{}", n);
                ThreadRename(&thread_name);
                set_syscall_sandbox_policy(SyscallSandboxPolicy::VALIDATION_SCRIPT_CHECK);

                /* worker thread */
                unsafe {
                    (*this).loop_(false);
                }
            }));
        }
    }

    /// Wait until execution finishes, and return
    /// whether all evaluations were successful.
    /// 
    pub fn wait(&self) -> bool {
        
        tracing::trace!("CheckQueue::wait");
        /* master thread */
        self.loop_(true)
    }

    /// Add a batch of checks to the queue
    /// 
    pub fn add(&self, checks: &mut Vec<T>) {
        
        let checks_len = checks.len();
        let mut inner = self.mutex_.lock().unwrap();

        for check in checks.iter_mut() {
            inner.queue_mut().push(T::default());
            let back = inner.queue_mut().last_mut().unwrap();
            check.swap(back);
        }

        *inner.n_todo_mut() += checks_len as u32;

        tracing::trace!(
            checks_len,
            n_todo = inner.n_todo(),
            queue_len = inner.queue().len(),
            "CheckQueue::add"
        );

        if checks_len == 1 {
            self.worker_cv.notify_one();
        } else if checks_len > 1 {
            self.worker_cv.notify_all();
        }
    }

    /// Stop all of the worker threads.
    /// 
    pub fn stop_worker_threads(&mut self)  {
        
        tracing::info!(
            worker_threads = self.worker_threads.len(),
            "CheckQueue::stop_worker_threads"
        );

        {
            let mut inner = self.mutex_.lock().unwrap();
            inner.set_request_stop(true);
        }

        self.worker_cv.notify_all();

        for t in self.worker_threads.drain(..) {
            t.join().unwrap();
        }

        {
            let mut inner = self.mutex_.lock().unwrap();
            inner.set_request_stop(false);
        }
    }
}
