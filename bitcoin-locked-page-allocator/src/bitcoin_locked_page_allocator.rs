crate::ix!();

/**
  | OS-dependent allocation and deallocation
  | of locked/pinned memory pages.
  | 
  | Abstract base class.
  |
  */
pub trait LockedPageAllocator:
AllocateLocked
+ FreeLocked
+ GetLimit { }

pub trait AllocateLocked {

    /**
      | Allocate and lock memory pages.
      | 
      | If len is not a multiple of the system
      | page size, it is rounded up.
      | 
      | Returns nullptr in case of allocation
      | failure.
      | 
      | If locking the memory pages could not
      | be accomplished it will still return
      | the memory, however the lockingSuccess
      | flag will be false. lockingSuccess
      | is undefined if the allocation fails.
      |
      */
    fn allocate_locked(&mut self, len: usize, locking_success: *mut bool) -> *mut c_void;
}

pub trait FreeLocked {

    /**
      | Unlock and free memory pages.
      | 
      | Clear the memory before unlocking.
      |
      */
    fn free_locked(&mut self, 
            addr: *mut c_void,
            len:  usize);
}

pub trait GetLimit {

    /**
      | Get the total limit on the amount of memory
      | that may be locked by this process, in
      | bytes. Return size_t max if there is
      | no limit or the limit is unknown. Return
      | 0 if no memory can be locked at all.
      |
      */
    fn get_limit(&mut self) -> usize;
}
