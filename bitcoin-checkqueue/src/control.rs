// ---------------- [ File: bitcoin-checkqueue/src/control.rs ]
crate::ix!();

/// RAII-style controller object for a
/// CCheckQueue that guarantees the passed
/// queue is finished before continuing.
/// 
pub struct CheckQueueControl<T: CheckQueueTask> {
    pqueue: *const CheckQueue<T>,
    done:   bool,
}

impl<T: CheckQueueTask> Drop for CheckQueueControl<T> {
    fn drop(&mut self) {
        tracing::trace!("CheckQueueControl::drop");

        if !self.done {
            let _ = self.wait();
        }
        if !self.pqueue.is_null() {
            unsafe {
                // Match the C++: LEAVE_CRITICAL_SECTION(pqueue->m_control_mutex);
                self.pqueue.as_ref().unwrap().control_mutex().unlock();
            }
        }
    }
}

impl<T: CheckQueueTask> CheckQueueControl<T> {

    pub fn new(pqueue_in: *mut CheckQueue<T>) -> Self {
    
        let pqueue: *const CheckQueue<T> = pqueue_in as *const CheckQueue<T>;
        let done: bool = false;

        // passed queue is supposed to be unused, or nullptr
        if !pqueue.is_null() {
            unsafe {
                // Match the C++: ENTER_CRITICAL_SECTION(pqueue->m_control_mutex);
                pqueue.as_ref().unwrap().control_mutex().lock();
            }
        }

        Self { pqueue, done }
    }
    
    pub fn wait(&mut self) -> bool {
        
        if self.pqueue.is_null() {
            return true;
        }

        let f_ret: bool = unsafe { self.pqueue.as_ref().unwrap().wait() };
        self.done = true;
        f_ret
    }
    
    pub fn add(&mut self, checks: &mut Vec<T>)  {
        
        if !self.pqueue.is_null() {
            unsafe {
                self.pqueue.as_ref().unwrap().add(checks);
            }
        }
    }
}
