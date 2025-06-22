crate::ix!();

/**
  | An RAII-style reverse lock. Unlocks
  | on construction and locks on destruction.
  |
  */
pub struct ReverseLock<Base> {
    lock:     Rc<RefCell<UniqueLock<Base>>>,
    templock: UniqueLock<Base>,
    lockname: String,
    file:     String,
    line:     i32,
}

impl<Base> Drop for ReverseLock<Base> {
    fn drop(&mut self) {
        todo!();
        /*
            templock.swap(lock);
                EnterCritical(lockname.c_str(), file.c_str(), line, lock.mutex());
                lock.lock();
        */
    }
}

impl<Base> ReverseLock<Base> {
    
    pub fn new(
        lock:      &mut UniqueLock<Base>,
        guardname: *const u8,
        file:      *const u8,
        line:      i32) -> Self {
    
        todo!();
        /*


            : lock(_lock), file(_file), line(_line) 
                CheckLastCritical((c_void*)lock.mutex(), lockname, _guardname, _file, _line);
                lock.unlock();
                LeaveCritical();
                lock.swap(templock);
        */
    }
}
