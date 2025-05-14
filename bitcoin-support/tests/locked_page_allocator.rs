// ---------------- [ File: bitcoin-support/tests/locked_page_allocator.rs ]
crate::ix!();

/**
  | Mock LockedPageAllocator for testing
  |
  */
pub struct TestLockedPageAllocator {
    count:       i32,
    lockedcount: i32,
}

impl LockedPageAllocator for TestLockedPageAllocator {

}

impl TestLockedPageAllocator {
    pub fn new(
        count_in:       i32,
        lockedcount_in: i32) -> Self {
    
        todo!();
        /*
        : count(count_in),
        : lockedcount(lockedcount_in),
        */
    }
}
    
impl AllocateLocked for TestLockedPageAllocator {
    fn allocate_locked(&mut self, 
        len:             usize,
        locking_success: *mut bool)  {
        
        todo!();
        /*
            *lockingSuccess = false;
            if (count > 0) {
                --count;

                if (lockedcount > 0) {
                    --lockedcount;
                    *lockingSuccess = true;
                }

                return reinterpret_cast<c_void*>(uint64_t{static_cast<uint64_t>(0x08000000) + (count << 24)}); // Fake address, do not actually use this memory
            }
            return nullptr;
        */
    }
}
    
impl FreeLocked for TestLockedPageAllocator {
    fn free_locked(&mut self, 
        addr: *mut c_void,
        len:  usize)  {
        
        todo!();
        /*
        
        */
    }
}
    
impl GetLimit for TestLockedPageAllocator {
    fn get_limit(&mut self) -> usize {
        
        todo!();
        /*
            return std::numeric_limits<size_t>::max();
        */
    }
}
