// ---------------- [ File: bitcoin-sync/src/unique_lock.rs ]
crate::ix!();

pub type MutexUniqueLock = Broken;

/**
  | Wrapper around std::unique_lock style
  | lock for Mutex.
  |
  */
#[SCOPED_LOCKABLE]
#[derive(Default)]
pub struct UniqueLock<Base = MutexUniqueLock> {
    base: Base,
}

pub mod unique_lock {
    use super::*;

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
}

impl<Base> Drop for UniqueLock<Base> {

    #[UNLOCK_FUNCTION()]
    fn drop(&mut self) {
        todo!();
        /*
            if (Base::owns_lock())
                LeaveCritical();
        */
    }
}

impl<Base> Into<bool> for UniqueLock<Base> {
    
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return Base::owns_lock();
        */
    }
}

impl<Base> UniqueLock<Base> {

    pub fn enter(&mut self, 
        psz_name: *const u8,
        psz_file: *const u8,
        n_line:   i32)  {
        
        todo!();
        /*
            EnterCritical(pszName, pszFile, nLine, Base::mutex());
            if (Base::try_lock()) return;
            LOG_TIME_MICROS_WITH_CATEGORY(strprintf("lock contention %s, %s:%d", pszName, pszFile, nLine), BCLog::LOCK);
            Base::lock();
        */
    }
    
    pub fn try_enter(&mut self, 
        psz_name: *const u8,
        psz_file: *const u8,
        n_line:   i32) -> bool {
        
        todo!();
        /*
            EnterCritical(pszName, pszFile, nLine, Base::mutex(), true);
            Base::try_lock();
            if (!Base::owns_lock()) {
                LeaveCritical();
            }
            return Base::owns_lock();
        */
    }

    #[EXCLUSIVE_LOCK_FUNCTION(mutexIn)]
    pub fn new(
        mutex_in: &mut Mutex,
        psz_name: *const u8,
        psz_file: *const u8,
        n_line:   i32,
        try_:     Option<bool>) -> Self {
        let try_: bool = try_.unwrap_or(false);
        todo!();
        /*
        : base(mutexIn, std::defer_lock),

            if (fTry)
                TryEnter(pszName, pszFile, nLine);
            else
                Enter(pszName, pszFile, nLine);
        */
    }
}
