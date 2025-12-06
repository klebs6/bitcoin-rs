crate::ix!();

pub struct AtomicCounter {
    mu:    Mutex<AtomicCounterInner>,

}

pub struct AtomicCounterInner {
    count: i32,
}

impl Default for AtomicCounter {
    
    fn default() -> Self {
        todo!();
        /*
        : count(0),

        
        */
    }
}

impl AtomicCounter {

    pub fn increment(&mut self)  {
        
        todo!();
        /*
            IncrementBy(1);
        */
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn increment_by(&mut self, count: i32)  {
        
        todo!();
        /*
            MutexLock l(&mu_);
        count_ += count;
        */
    }
    
    #[LOCKS_EXCLUDED(mu_)]
    pub fn read(&mut self) -> i32 {
        
        todo!();
        /*
            MutexLock l(&mu_);
        return count_;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            MutexLock l(&mu_);
        count_ = 0;
        */
    }
}
