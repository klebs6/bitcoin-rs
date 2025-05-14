// ---------------- [ File: bitcoin-checkqueue/src/control.rs ]
crate::ix!();

/**
  | RAII-style controller object for a
  | CCheckQueue that guarantees the passed
  | queue is finished before continuing.
  |
  */
pub struct CheckQueueControl<T> {
    pqueue: *const CheckQueue<T>,
    done:   bool,
}

impl<T> Drop for CheckQueueControl<T> {
    fn drop(&mut self) {
        todo!();
        /*
            if (!fDone)
                Wait();
            if (pqueue != nullptr) {
                LEAVE_CRITICAL_SECTION(pqueue->m_control_mutex);
            }
        */
    }
}

impl<T> CheckQueueControl<T> {

    pub fn new(pqueue_in: *mut CheckQueue<T>) -> Self {
    
        todo!();
        /*
        : pqueue(pqueueIn),
        : done(false),

            // passed queue is supposed to be unused, or nullptr
            if (pqueue != nullptr) {
                ENTER_CRITICAL_SECTION(pqueue->m_control_mutex);
            }
        */
    }
    
    pub fn wait(&mut self) -> bool {
        
        todo!();
        /*
            if (pqueue == nullptr)
                return true;
            bool fRet = pqueue->Wait();
            fDone = true;
            return fRet;
        */
    }
    
    pub fn add(&mut self, checks: &mut Vec<T>)  {
        
        todo!();
        /*
            if (pqueue != nullptr)
                pqueue->Add(vChecks);
        */
    }
}
