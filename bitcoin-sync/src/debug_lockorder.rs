crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/sync.cpp]
#[cfg(DEBUG_LOCKORDER)]
pub use debug_lockorder::*;

#[cfg(not(DEBUG_LOCKORDER))]
pub use debug_lockorder_noop::*;

#[cfg(not(DEBUG_LOCKORDER))]
pub mod debug_lockorder_noop {
    use super::*;

    #[inline] pub fn enter_critical<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType,
            try_:     bool)  { }

    #[inline] pub fn leave_critical()  { }

    #[inline] pub fn check_last_critical(
            cs:        *mut c_void,
            lockname:  &mut String,
            guardname: *const u8,
            file:      *const u8,
            line:      i32)  { }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    #[inline] pub fn assert_lock_held_internal<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType)  { }

    #[LOCKS_EXCLUDED(cs)]
    pub fn assert_lock_not_held_internal<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType)  { }

    #[inline] pub fn delete_lock(cs: *mut c_void)  { }

    #[inline] pub fn lock_stack_empty() -> bool {
        
        todo!();
            /*
                return true;
            */
    }
}

#[cfg(DEBUG_LOCKORDER)]
mod debug_lockorder {

    pub fn check_last_critical(
            cs:        *mut c_void,
            lockname:  &mut String,
            guardname: *const u8,
            file:      *const u8,
            line:      i32)  {
        
        todo!();
            /*
            
            */
    }

    pub fn locks_held() -> String {
        
        todo!();
            /*
            
            */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn assert_lock_held_internal<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType)  {

        todo!();
            /*
            
            */
    }

    #[LOCKS_EXCLUDED(cs)]
    pub fn assert_lock_not_held_internal<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType)  {

        todo!();
            /*
            
            */
    }

    pub fn delete_lock(cs: *mut c_void)  {
        
        todo!();
            /*
            
            */
    }

    pub fn lock_stack_empty() -> bool {
        
        todo!();
            /*
            
            */
    }

    /**
      | Call abort() if a potential lock order
      | deadlock bug is detected, instead of
      | just logging information and throwing
      | a logic_error.
      | 
      | Defaults to true, and set to false in
      | DEBUG_LOCKORDER unit tests.
      |
      */
    lazy_static!{
        /*
        extern bool g_debug_lockorder_abort;
        */
    }

    /**
      | Early deadlock detection.
      | Problem being solved:
      |    Thread 1 locks A, then B, then C
      |    Thread 2 locks D, then C, then A
      |     --> may result in deadlock between the two
      |         threads, depending on when they run.
      |
      | Solution implemented here:
      |
      | Keep track of pairs of locks: (A before B), (A
      | before C), etc.
      |
      | Complain if any thread tries to lock in
      | a different order.
      */
    pub struct LockLocation {
        try_:        bool,
        mutex_name:  String,
        source_file: String,
        thread_name: &String,
        source_line: i32,
    }

    impl LockLocation {

        pub fn new(
            psz_name:    *const u8,
            psz_file:    *const u8,
            n_line:      i32,
            try_in:      bool,
            thread_name: &String) -> Self {
        
            todo!();
            /*

                : fTry(fTryIn),
                  mutexName(pszName),
                  sourceFile(pszFile),
                  m_thread_name(thread_name),
                  sourceLine(nLine)
            */
        }
        
        pub fn to_string(&self) -> String {
            
            todo!();
            /*
                return strprintf(
                    "'%s' in %s:%s%s (in thread '%s')",
                    mutexName, sourceFile, sourceLine, (fTry ? " (TRY)" : ""), m_thread_name);
            */
        }
        
        pub fn name(&self) -> String {
            
            todo!();
            /*
                return mutexName;
            */
        }
    }

    pub type LockStackItem = Pair<*mut c_void,LockLocation>;
    pub type LockStack     = Vec<LockStackItem>;
    pub type LockStacks    = HashMap<std::thread::ThreadId,LockStack>;
    pub type LockPair      = Pair<*mut c_void,*mut c_void>;
    pub type LockOrders    = HashMap<LockPair,LockStack>;
    pub type InvLockOrders = HashSet<LockPair>;

    pub struct LockData {
        lock_stacks:   LockStacks,
        lockorders:    LockOrders,
        invlockorders: InvLockOrders,
        dd_mutex:      parking_lot::RawMutex,
    }

    pub fn get_lock_data() -> &mut LockData {
        
        todo!();
            /*
                // This approach guarantees that the object is not destroyed until after its last use.
            // The operating system automatically reclaims all the memory in a program's heap when that program exits.
            // Since the ~LockData() destructor is never called, the LockData class and all
            // its subclasses must have implicitly-defined destructors.
            static LockData& lock_data = *new LockData();
            return lock_data;
            */
    }

    pub fn potential_deadlock_detected(
            mismatch: &LockPair,
            s1:       &LockStack,
            s2:       &LockStack)  {
        
        todo!();
            /*
                LogPrintf("POTENTIAL DEADLOCK DETECTED\n");
            LogPrintf("Previous lock order was:\n");
            for (const LockStackItem& i : s1) {
                std::string prefix{};
                if (i.first == mismatch.first) {
                    prefix = " (1)";
                }
                if (i.first == mismatch.second) {
                    prefix = " (2)";
                }
                LogPrintf("%s %s\n", prefix, i.second.ToString());
            }

            std::string mutex_a, mutex_b;
            LogPrintf("Current lock order is:\n");
            for (const LockStackItem& i : s2) {
                std::string prefix{};
                if (i.first == mismatch.first) {
                    prefix = " (1)";
                    mutex_a = i.second.Name();
                }
                if (i.first == mismatch.second) {
                    prefix = " (2)";
                    mutex_b = i.second.Name();
                }
                LogPrintf("%s %s\n", prefix, i.second.ToString());
            }
            if (g_debug_lockorder_abort) {
                tfm::format(std::cerr, "Assertion failed: detected inconsistent lock order for %s, details in debug log.\n", s2.back().second.ToString());
                abort();
            }
            throw std::logic_error(strprintf("potential deadlock detected: %s -> %s -> %s", mutex_b, mutex_a, mutex_b));
            */
    }

    pub fn double_lock_detected(
            mutex:      *const c_void,
            lock_stack: &LockStack)  {
        
        todo!();
            /*
                LogPrintf("DOUBLE LOCK DETECTED\n");
            LogPrintf("Lock order:\n");
            for (const LockStackItem& i : lock_stack) {
                std::string prefix{};
                if (i.first == mutex) {
                    prefix = " (*)";
                }
                LogPrintf("%s %s\n", prefix, i.second.ToString());
            }
            if (g_debug_lockorder_abort) {
                tfm::format(std::cerr,
                            "Assertion failed: detected double lock for %s, details in debug log.\n",
                            lock_stack.back().second.ToString());
                abort();
            }
            throw std::logic_error("double lock detected");
            */
    }

    pub fn push_lock<MutexType>(
            c:            *mut MutexType,
            locklocation: &LockLocation)  {

        todo!();
            /*
                constexpr bool is_recursive_mutex =
                std::is_base_of<RecursiveMutex, MutexType>::value ||
                std::is_base_of<std::recursive_mutex, MutexType>::value;

            LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);

            LockStack& lock_stack = lockdata.m_lock_stacks[std::this_thread::get_id()];
            lock_stack.emplace_back(c, locklocation);
            for (size_t j = 0; j < lock_stack.size() - 1; ++j) {
                const LockStackItem& i = lock_stack[j];
                if (i.first == c) {
                    if (is_recursive_mutex) {
                        break;
                    }
                    // It is not a recursive mutex and it appears in the stack two times:
                    // at position `j` and at the end (which we added just before this loop).
                    // Can't allow locking the same (non-recursive) mutex two times from the
                    // same thread as that results in an undefined behavior.
                    auto lock_stack_copy = lock_stack;
                    lock_stack.pop_back();
                    double_lock_detected(c, lock_stack_copy);
                    // double_lock_detected() does not return.
                }

                const LockPair p1 = std::make_pair(i.first, c);
                if (lockdata.lockorders.count(p1))
                    continue;

                const LockPair p2 = std::make_pair(c, i.first);
                if (lockdata.lockorders.count(p2)) {
                    auto lock_stack_copy = lock_stack;
                    lock_stack.pop_back();
                    potential_deadlock_detected(p1, lockdata.lockorders[p2], lock_stack_copy);
                    // potential_deadlock_detected() does not return.
                }

                lockdata.lockorders.emplace(p1, lock_stack);
                lockdata.invlockorders.insert(p2);
            }
            */
    }

    pub fn pop_lock()  {
        
        todo!();
            /*
                LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);

            LockStack& lock_stack = lockdata.m_lock_stacks[std::this_thread::get_id()];
            lock_stack.pop_back();
            if (lock_stack.empty()) {
                lockdata.m_lock_stacks.erase(std::this_thread::get_id());
            }
            */
    }

    pub fn enter_critical<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType,
            try_:     bool)  {

        let try_:bool = try_.unwrap_or(false);

        todo!();
            /*
                push_lock(cs, CLockLocation(pszName, pszFile, nLine, fTry, util::ThreadGetInternalName()));
            */
    }

    pub fn check_last_critical(
            cs:        *mut c_void,
            lockname:  &mut String,
            guardname: *const u8,
            file:      *const u8,
            line:      i32)  {
        
        todo!();
            /*
                LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);

            const LockStack& lock_stack = lockdata.m_lock_stacks[std::this_thread::get_id()];
            if (!lock_stack.empty()) {
                const auto& lastlock = lock_stack.back();
                if (lastlock.first == cs) {
                    lockname = lastlock.second.Name();
                    return;
                }
            }

            LogPrintf("INCONSISTENT LOCK ORDER DETECTED\n");
            LogPrintf("Current lock order (least recent first) is:\n");
            for (const LockStackItem& i : lock_stack) {
                LogPrintf(" %s\n", i.second.ToString());
            }
            if (g_debug_lockorder_abort) {
                tfm::format(std::cerr, "%s:%s %s was not most recent critical section locked, details in debug log.\n", file, line, guardname);
                abort();
            }
            throw std::logic_error(strprintf("%s was not most recent critical section locked", guardname));
            */
    }

    pub fn leave_critical()  {
        
        todo!();
            /*
                pop_lock();
            */
    }

    pub fn locks_held() -> String {
        
        todo!();
            /*
                LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);

            const LockStack& lock_stack = lockdata.m_lock_stacks[std::this_thread::get_id()];
            std::string result;
            for (const LockStackItem& i : lock_stack)
                result += i.second.ToString() + std::string("\n");
            return result;
            */
    }

    pub fn lock_held(mutex: *mut c_void) -> bool {
        
        todo!();
            /*
                LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);

            const LockStack& lock_stack = lockdata.m_lock_stacks[std::this_thread::get_id()];
            for (const LockStackItem& i : lock_stack) {
                if (i.first == mutex) return true;
            }

            return false;
            */
    }

    pub fn assert_lock_held_internal<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType)  {

        todo!();
            /*
                if (LockHeld(cs)) return;
            tfm::format(std::cerr, "Assertion failed: lock %s not held in %s:%i; locks held:\n%s", pszName, pszFile, nLine, LocksHeld());
            abort();
            */
    }

    pub fn assert_lock_not_held_internal<MutexType>(
            psz_name: *const u8,
            psz_file: *const u8,
            n_line:   i32,
            cs:       *mut MutexType)  {

        todo!();
            /*
                if (!LockHeld(cs)) return;
            tfm::format(std::cerr, "Assertion failed: lock %s held in %s:%i; locks held:\n%s", pszName, pszFile, nLine, LocksHeld());
            abort();
            */
    }

    pub fn delete_lock(cs: *mut c_void)  {
        
        todo!();
            /*
                LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);
            const LockPair item = std::make_pair(cs, nullptr);
            LockOrders::iterator it = lockdata.lockorders.lower_bound(item);
            while (it != lockdata.lockorders.end() && it->first.first == cs) {
                const LockPair invitem = std::make_pair(it->first.second, it->first.first);
                lockdata.invlockorders.erase(invitem);
                lockdata.lockorders.erase(it++);
            }
            InvLockOrders::iterator invit = lockdata.invlockorders.lower_bound(item);
            while (invit != lockdata.invlockorders.end() && invit->first == cs) {
                const LockPair invinvitem = std::make_pair(invit->second, invit->first);
                lockdata.lockorders.erase(invinvitem);
                lockdata.invlockorders.erase(invit++);
            }
            */
    }

    pub fn lock_stack_empty() -> bool {
        
        todo!();
            /*
                LockData& lockdata = GetLockData();
            std::lock_guard<std::mutex> lock(lockdata.dd_mutex);
            const auto it = lockdata.m_lock_stacks.find(std::this_thread::get_id());
            if (it == lockdata.m_lock_stacks.end()) {
                return true;
            }
            return it->second.empty();
            */
    }

    lazy_static!{
        /*
        bool g_debug_lockorder_abort = true;
        */
    }
}
