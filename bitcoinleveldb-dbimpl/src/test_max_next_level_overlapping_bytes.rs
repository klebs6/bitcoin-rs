// ---------------- [ File: bitcoinleveldb-dbimpl/src/test_max_next_level_overlapping_bytes.rs ]
crate::ix!();

impl DBImpl {
    
    /// Return the maximum overlapping data (in bytes) at next level for any file at a level >= 1.
    pub fn test_max_next_level_overlapping_bytes(&mut self) -> i64 { 
        todo!(); 
        /*
        self.mutex.lock();
        let v = unsafe { (*self.versions).max_next_level_overlapping_bytes() };
        self.mutex.unlock();
        v
                                                                     */
    }
}
