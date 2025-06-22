/*!
   | The Simple Definition:
   |
   | RecursiveMutex mutex;
   |     std::recursive_mutex mutex;
   |
   | LOCK(mutex);
   |     std::unique_lock<std::recursive_mutex> criticalblock(mutex);
   |
   | LOCK2(mutex1, mutex2);
   |     std::unique_lock<std::recursive_mutex> criticalblock1(mutex1);
   |     std::unique_lock<std::recursive_mutex> criticalblock2(mutex2);
   |
   | TRY_LOCK(mutex, name);
   |     std::unique_lock<std::recursive_mutex> name(mutex, std::try_to_lock_t);
   |
   | ENTER_CRITICAL_SECTION(mutex); // no RAII
   |     mutex.lock();
   |
   | LEAVE_CRITICAL_SECTION(mutex); // no RAII
   |     mutex.unlock();
   */

// ---------------- [ File: bitcoin-sync/src/sync_impl.rs ]

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/sync.h]

/* --------- THE ACTUAL IMPLEMENTATION  --------- */

#[macro_export] macro_rules! assert_lock_held {
    ($cs:ident) => {
        /*
                AssertLockHeldInternal(#cs, __FILE__, __LINE__, &cs)
        */
    }
}

#[macro_export] macro_rules! assert_lock_not_held {
    ($cs:ident) => {
        /*
                AssertLockNotHeldInternal(#cs, __FILE__, __LINE__, &cs)
        */
    }
}

#[macro_export] macro_rules! reverse_lock {
    ($g:ident) => {
        /*
                typename std::decay<decltype(g)>::type::reverse_lock PASTE2(revlock, __COUNTER__)(g, #g, __FILE__, __LINE__)
        */
    }
}

#[macro_export] macro_rules! lock {
    ($cs:expr) => {
        /*
                DebugLock<decltype(cs)> PASTE2(criticalblock, __COUNTER__)(cs, #cs, __FILE__, __LINE__)
        */
    }
}

#[macro_export] macro_rules! lock2 {
    ($cs1:expr, $cs2:expr) => {
        /*
        
            DebugLock<decltype(cs1)> criticalblock1(cs1, #cs1, __FILE__, __LINE__); 
            DebugLock<decltype(cs2)> criticalblock2(cs2, #cs2, __FILE__, __LINE__);
        */
    }
}

#[macro_export] macro_rules! try_lock {
    ($cs:expr, $name:expr) => {
        /*
                DebugLock<decltype(cs)> name(cs, #cs, __FILE__, __LINE__, true)
        */
    }
}

#[macro_export] macro_rules! wait_lock {
    ($cs:expr, $name:expr) => {
        /*
                DebugLock<decltype(cs)> name(cs, #cs, __FILE__, __LINE__)
        */
    }
}

#[macro_export] macro_rules! enter_critical_section {
    ($cs:expr) => {
        /*
        
            {                                                         
                EnterCritical(#cs, __FILE__, __LINE__, &cs); 
                (cs).lock();                                          
            }
        */
    }
}

#[macro_export] macro_rules! leave_critical_section {
    ($cs:expr) => {
        /*
        
            {                                                                       
                std::string lockname;                                               
                CheckLastCritical((c_void*)(&cs), lockname, #cs, __FILE__, __LINE__); 
                (cs).unlock();                                                      
                LeaveCritical();                                                    
            }
        */
    }
}

/**
  | Run code while locking a mutex.
  |
  | Examples:
  |
  -------------------------
  |WITH_LOCK(cs, shared_val = shared_val + 1);
  |
  |   int val = WITH_LOCK(cs, return shared_val);
  |
  |
  -------------------------
  | Note:
  |
  | Since the return type deduction follows that
  | of decltype(auto), while the deduced type of:
  |
  |   WITH_LOCK(cs, return {int i = 1; return i;});
  |
  | is int, the deduced type of:
  |
  |   WITH_LOCK(cs, return {int j = 1; return (j);});
  |
  | is &int, a reference to a local variable
  |
  | The above is detectable at compile-time with
  | the -Wreturn-local-addr flag in gcc and the
  | -Wreturn-stack-address flag in clang, both
  | enabled by default.
  */
#[macro_export] macro_rules! with_lock {
    ($cs:expr, $code:expr) => {
        /*
                [&]() -> decltype(auto) { LOCK(cs); code; }()
        */
    }
}
