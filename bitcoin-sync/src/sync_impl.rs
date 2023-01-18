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

/**
  | Template mixin that adds -Wthread-safety
  | locking annotations and lock order
  | checking to a subset of the mutex API.
  |
  */
#[LOCKABLE]
pub struct AnnotatedMixin<PARENT> {
    base: PARENT,
}

impl<PARENT> Drop for AnnotatedMixin<PARENT> {
    fn drop(&mut self) {
        todo!();
        /*
            DeleteLock((c_void*)this);
        */
    }
}

pub mod annotated_mixin {
    pub type UniqueLock<PARENT> = super::UniqueLock<PARENT>;
}

impl<PARENT> Not for AnnotatedMixin<PARENT> {
    type Output = AnnotatedMixin<PARENT>;

    /**
      | For negative capabilities in the Clang
      | Thread Safety Analysis.
      |
      | A negative requirement uses the
      | EXCLUSIVE_LOCKS_REQUIRED attribute, in
      | conjunction with the ! operator, to
      | indicate that a mutex should not be held.
      */
    #[inline] fn not(self) -> Self::Output {
        todo!();
        /*
            return *this;
        */
    }
}

impl<PARENT> AnnotatedMixin<PARENT> {

    #[EXCLUSIVE_LOCK_FUNCTION()]
    pub fn lock(&mut self)  {
        
        todo!();
        /*
            PARENT::lock();
        */
    }

    #[UNLOCK_FUNCTION()]
    pub fn unlock(&mut self)  {
        
        todo!();
        /*
            PARENT::unlock();
        */
    }

    #[EXCLUSIVE_TRYLOCK_FUNCTION(true)]
    pub fn try_lock(&mut self) -> bool {
        
        todo!();
        /*
            return PARENT::try_lock();
        */
    }
}

/**
  | Wrapped mutex: supports recursive
  | locking, but no waiting
  | 
  | TODO: We should move away from using
  | the recursive lock by default.
  |
  */
pub type RecursiveMutex<T> = AnnotatedMixin<parking_lot::ReentrantMutex<T>>;

/**
  | Wrapped mutex: supports waiting but
  | not recursive locking
  |
  */
pub type Mutex = AnnotatedMixin<parking_lot::RawMutex>;

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

#[macro_export] macro_rules! reverse_lock {
    ($g:ident) => {
        /*
                typename std::decay<decltype(g)>::type::reverse_lock PASTE2(revlock, __COUNTER__)(g, #g, __FILE__, __LINE__)
        */
    }
}

pub type DebugLock<MutexArg> = UniqueLock<RemoveReference<RemovePointer<MutexArg>>>;

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

///------------------------------
pub struct Semaphore {
    condition: std::sync::Condvar,
    mutex:     parking_lot::RawMutex,
    value:     i32,
}

impl Semaphore {

    pub fn new(init: i32) -> Self {
    
        todo!();
        /*
        : value(init),

        
        */
    }
    
    pub fn wait(&mut self)  {
        
        todo!();
        /*
            std::unique_lock<std::mutex> lock(mutex);
            condition.wait(lock, [&]() { return value >= 1; });
            value--;
        */
    }
    
    pub fn try_wait(&mut self) -> bool {
        
        todo!();
        /*
            std::lock_guard<std::mutex> lock(mutex);
            if (value < 1)
                return false;
            value--;
            return true;
        */
    }
    
    pub fn post(&mut self)  {
        
        todo!();
        /*
            {
                std::lock_guard<std::mutex> lock(mutex);
                value++;
            }
            condition.notify_one();
        */
    }
}

/**
  | RAII-style semaphore lock
  |
  */
#[derive(Clone)]
pub struct SemaphoreGrant {
    sem:        Amo<Semaphore>,
    have_grant: bool,
}

impl Drop for SemaphoreGrant {
    fn drop(&mut self) {
        todo!();
        /*
            Release();
        */
    }
}

impl Into<bool> for &SemaphoreGrant {
    
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return fHaveGrant;
        */
    }
}

impl Default for SemaphoreGrant {

    fn default() -> Self {
        todo!();
        /*
        : sem(nullptr),
        : have_grant(false),

        
        */
    }
}

impl SemaphoreGrant {
    
    pub fn acquire(&mut self)  {
        
        todo!();
        /*
            if (fHaveGrant)
                return;
            sem->wait();
            fHaveGrant = true;
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            if (!fHaveGrant)
                return;
            sem->post();
            fHaveGrant = false;
        */
    }
    
    pub fn try_acquire(&mut self) -> bool {
        
        todo!();
        /*
            if (!fHaveGrant && sem->try_wait())
                fHaveGrant = true;
            return fHaveGrant;
        */
    }
    
    pub fn move_to(&mut self, grant: &mut SemaphoreGrant)  {
        
        todo!();
        /*
            grant.Release();
            grant.sem = sem;
            grant.fHaveGrant = fHaveGrant;
            fHaveGrant = false;
        */
    }
    
    
    pub fn new(
        sema: Amo<Semaphore>,
        try_: Option<bool>) -> Self {
        let try_:bool = try_.unwrap_or(false);
        todo!();
        /*
        : sem(&sema),
        : have_grant(false),

            if (fTry)
                TryAcquire();
            else
                Acquire();
        */
    }
}

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
