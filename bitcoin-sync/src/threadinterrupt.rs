// ---------------- [ File: bitcoin-sync/src/threadinterrupt.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/threadinterrupt.h]
//-------------------------------------------[.cpp/bitcoin/src/threadinterrupt.cpp]

/**
  | A helper class for interruptible sleeps.
  | Calling operator() will interrupt
  | any current sleep, and after that point
  | operator bool() will return true until
  | reset.
  |
  */
pub struct ThreadInterrupt {
    cond: std::sync::Condvar,
    mut_: parking_lot::RawMutex,
    flag: AtomicBool,
}

unsafe impl Send for ThreadInterrupt {}
unsafe impl Sync for ThreadInterrupt {}

impl ThreadInterrupt {

    pub fn as_bool(&self) -> bool {
        todo!();
        /*
            return flag.load(std::memory_order_acquire);
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : flag(false),
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            flag.store(false, std::memory_order_release);
        */
    }
    
    pub fn invoke(&mut self)  {
        
        todo!();
        /*
            {
            LOCK(mut);
            flag.store(true, std::memory_order_release);
        }
        cond.notify_all();
        */
    }

    pub fn sleep_for(&mut self, rel_time: Duration) -> bool {
        todo!();
    }
    
    /*
    pub fn sleep_for_millis(&mut self, rel_time: Milliseconds) -> bool {
        
        todo!();
        /*
            WAIT_LOCK(mut, lock);
        return !cond.wait_for(lock, rel_time, [this]() { return flag.load(std::memory_order_acquire); });
        */
    }
    
    pub fn sleep_for_seconds(&mut self, rel_time: Seconds) -> bool {
        
        todo!();
        /*
            return sleep_for(std::chrono::duration_cast<std::chrono::milliseconds>(rel_time));
        */
    }
    
    pub fn sleep_for_minutes(&mut self, rel_time: Minutes) -> bool {
        
        todo!();
        /*
            return sleep_for(std::chrono::duration_cast<std::chrono::milliseconds>(rel_time));
        */
    }
    */
}
