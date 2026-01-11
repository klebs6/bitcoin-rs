// ---------------- [ File: bitcoin-checkqueue/tests/checkqueue.rs ]
use bitcoin_checkqueue::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/test/checkqueue_tests.cpp]

pub const QUEUE_BATCH_SIZE:     u32 = 128;
pub const SCRIPT_CHECK_THREADS: i32 = 3;

pub struct FakeCheck { }

impl Default for FakeCheck {
    fn default() -> Self {
        Self {}
    }
}

impl CheckQueueTask for FakeCheck {
    fn invoke(&mut self) -> bool {
        true
    }

    fn swap(&mut self, x: &mut Self) {
    }
}

///------------------------
pub struct FakeCheckCheckCompletion {

}

pub mod fake_check_check_completion {
    use super::*;

    lazy_static!{
        pub static ref N_CALLS: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
    }
}

impl Default for FakeCheckCheckCompletion {
    fn default() -> Self {
        Self {}
    }
}

impl CheckQueueTask for FakeCheckCheckCompletion {
    fn invoke(&mut self) -> bool {
        fake_check_check_completion::N_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        true
    }

    fn swap(&mut self, x: &mut Self) {
    }
}

///---------------------------
pub struct FailingCheck {
    fails: bool,
}

impl FailingCheck {
    
    pub fn new(fails: bool) -> Self {
        Self { fails }
    }
    
    fn default() -> Self {
        Self { fails: true }
    }
    
    pub fn invoke(&self) -> bool {
        !self.fails
    }
    
    pub fn swap(&mut self, x: &mut FailingCheck)  {
        std::mem::swap(&mut self.fails, &mut x.fails);
    }
}

impl Default for FailingCheck {
    fn default() -> Self {
        Self::default()
    }
}

impl CheckQueueTask for FailingCheck {
    fn invoke(&mut self) -> bool {
        FailingCheck::invoke(self)
    }

    fn swap(&mut self, x: &mut Self) {
        FailingCheck::swap(self, x)
    }
}

///-------------------------
pub struct UniqueCheck {
    check_id: usize,
}

pub mod unique_check {
    use super::*;

    lazy_static!{
        pub static ref RESULTS: std::sync::Mutex<std::collections::HashMap<usize, usize>> =
            std::sync::Mutex::new(std::collections::HashMap::new());
    }

    pub fn clear_results() {
        let mut m = RESULTS.lock().unwrap();
        m.clear();
    }

    pub fn total_results_count() -> usize {
        let m = RESULTS.lock().unwrap();
        m.values().copied().sum::<usize>()
    }

    pub fn results_count(check_id: usize) -> usize {
        let m = RESULTS.lock().unwrap();
        *m.get(&check_id).unwrap_or(&0)
    }

    pub fn insert_result(check_id: usize) {
        let mut m = RESULTS.lock().unwrap();
        *m.entry(check_id).or_insert(0) += 1;
    }
}

impl UniqueCheck {
    
    pub fn new(check_id_in: usize) -> Self {
    
        Self { check_id: check_id_in }
    }
    
    fn default() -> Self {
        Self { check_id: 0 }
    }
    
    pub fn invoke(&mut self) -> bool {
        unique_check::insert_result(self.check_id);
        true
    }
    
    pub fn swap(&mut self, x: &mut UniqueCheck)  {
        std::mem::swap(&mut x.check_id, &mut self.check_id);
    }
}

impl Default for UniqueCheck {
    fn default() -> Self {
        Self::default()
    }
}

impl CheckQueueTask for UniqueCheck {
    fn invoke(&mut self) -> bool {
        UniqueCheck::invoke(self)
    }

    fn swap(&mut self, x: &mut Self) {
        UniqueCheck::swap(self, x)
    }
}

///-----------------------
#[derive(Default)]
pub struct MemoryCheck {

    b: bool, // default = { false }
}

pub mod memory_check {
    use super::*;

    lazy_static!{
        pub static ref FAKE_ALLOCATED_MEMORY: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
    }

    pub fn load_fake_allocated_memory() -> usize {
        FAKE_ALLOCATED_MEMORY.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn store_fake_allocated_memory(v: usize) {
        FAKE_ALLOCATED_MEMORY.store(v, std::sync::atomic::Ordering::Relaxed)
    }
}

impl Drop for MemoryCheck {
    fn drop(&mut self) {
        if self.b {
            memory_check::FAKE_ALLOCATED_MEMORY.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

impl Clone for MemoryCheck {
    fn clone(&self) -> Self {
        MemoryCheck::from_memory_check(self)
    }
}

impl MemoryCheck {

    pub fn invoke(&self) -> bool {
        true
    }
    
    pub fn from_memory_check(x: &MemoryCheck) -> Self {
    
        let b = x.b;
        if b {
            // We have to do this to make sure that destructor calls are paired
            //
            // Really, copy constructor should be deletable, but CCheckQueue breaks
            // if it is deleted because of internal push_back.
            memory_check::FAKE_ALLOCATED_MEMORY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Self { b }
    }
    
    pub fn new(b: bool) -> Self {
    
        if b {
            memory_check::FAKE_ALLOCATED_MEMORY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Self { b }
    }
    
    pub fn swap(&mut self, x: &mut MemoryCheck)  {
        std::mem::swap(&mut self.b, &mut x.b);
    }
}

impl CheckQueueTask for MemoryCheck {
    fn invoke(&mut self) -> bool {
        MemoryCheck::invoke(self)
    }

    fn swap(&mut self, x: &mut Self) {
        MemoryCheck::swap(self, x)
    }
}


///-----------------------
pub struct FrozenCleanupCheck {

    /// Freezing can't be the default initialized
    /// behavior given how the queue swaps in
    /// default initialized Checks.
    /// 
    should_freeze: bool, // default = { false }
}

pub mod frozen_cleanup_check {
    use super::*;

    lazy_static!{
        pub static ref N_FROZEN: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(0);
        pub static ref CV: std::sync::Condvar = std::sync::Condvar::new();
        pub static ref M: std::sync::Mutex<()> = std::sync::Mutex::new(());
    }

    pub fn wait_until_frozen() {
        let mut l = M.lock().unwrap();
        while N_FROZEN.load(std::sync::atomic::Ordering::Relaxed) != 1 {
            l = CV.wait(l).unwrap();
        }
    }

    pub fn unfreeze() {
        let _l = M.lock().unwrap();
        N_FROZEN.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn notify_one() {
        CV.notify_one();
    }

    pub fn mark_frozen_and_wait() {
        let mut l = M.lock().unwrap();
        N_FROZEN.store(1, std::sync::atomic::Ordering::Relaxed);
        CV.notify_one();
        while N_FROZEN.load(std::sync::atomic::Ordering::Relaxed) != 0 {
            l = CV.wait(l).unwrap();
        }
    }

    pub fn is_frozen() -> bool {
        N_FROZEN.load(std::sync::atomic::Ordering::Relaxed) == 1
    }
}

impl Default for FrozenCleanupCheck {
    fn default() -> Self {
        Self { should_freeze: false }
    }
}

impl Drop for FrozenCleanupCheck {
    fn drop(&mut self) {
        if self.should_freeze {
            frozen_cleanup_check::mark_frozen_and_wait();
        }
    }
}

impl FrozenCleanupCheck {
    
    pub fn invoke(&self) -> bool {
        true
    }
    
    pub fn swap(&mut self, x: &mut FrozenCleanupCheck)  {
        std::mem::swap(&mut self.should_freeze, &mut x.should_freeze);
    }
}

impl CheckQueueTask for FrozenCleanupCheck {
    fn invoke(&mut self) -> bool {
        FrozenCleanupCheck::invoke(self)
    }

    fn swap(&mut self, x: &mut Self) {
        FrozenCleanupCheck::swap(self, x)
    }
}


/* -------------- Static Allocations  -------------- */
lazy_static!{
    /*
    std::mutex FrozenCleanupCheck::m{};
    std::atomic<uint64_t> FrozenCleanupCheck::nFrozen{0};
    std::condition_variable FrozenCleanupCheck::cv{};
    Mutex UniqueCheck::m;
    std::unordered_multiset<size_t> UniqueCheck::results;
    std::atomic<size_t> FakeCheckCheckCompletion::n_calls{0};
    std::atomic<size_t> MemoryCheck::fake_allocated_memory{0};
    */
}


/* ---------------- Queue Typedefs  ---------------- */
pub type Correct_Queue       = CheckQueue<FakeCheckCheckCompletion>;
pub type Standard_Queue      = CheckQueue<FakeCheck>;
pub type Failing_Queue       = CheckQueue<FailingCheck>;
pub type Unique_Queue        = CheckQueue<UniqueCheck>;
pub type Memory_Queue        = CheckQueue<MemoryCheck>;
pub type FrozenCleanup_Queue = CheckQueue<FrozenCleanupCheck>;

lazy_static! {
    static ref CHECKQUEUE_INTEGRATION_TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());
}

/// This test case checks that the CCheckQueue
/// works properly with each specified
/// size_t Checks pushed.
/// 
pub fn correct_queue_range(range: Vec<usize>)  {
    
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut small_queue = Box::new(Correct_Queue::new(QUEUE_BATCH_SIZE));
    small_queue.start_worker_threads(SCRIPT_CHECK_THREADS);

    // Make vChecks here to save on malloc (this test can be slow...)
    let mut v_checks: Vec<FakeCheckCheckCompletion> = Vec::new();

    for i in range {
        let mut total: usize = i;

        fake_check_check_completion::N_CALLS.store(0, std::sync::atomic::Ordering::Relaxed);

        let mut control = CheckQueueControl::<FakeCheckCheckCompletion>::new(
            small_queue.as_mut() as *mut Correct_Queue
        );

        while total != 0 {
            let chunk: usize = std::cmp::min(total, InsecureRandRange(10) as usize);
            v_checks.resize_with(chunk, Default::default);
            total -= v_checks.len();
            control.add(&mut v_checks);
        }

        assert!(control.wait());

        let observed_calls = fake_check_check_completion::N_CALLS.load(
            std::sync::atomic::Ordering::Relaxed
        );

        if observed_calls != i {
            assert_eq!(observed_calls, i);
        }
    }

    small_queue.stop_worker_threads();
}

/**
  | Test that 0 checks is correct
  |
  */
#[traced_test]
fn test_check_queue_correct_zero() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut range: Vec<usize> = Vec::new();
    range.push(0usize);
    correct_queue_range(range);
}

/**
  | Test that 1 check is correct
  |
  */
#[traced_test]
fn test_check_queue_correct_one() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut range: Vec<usize> = Vec::new();
    range.push(1usize);
    correct_queue_range(range);
}

/**
  | Test that MAX check is correct
  |
  */
#[traced_test]
fn test_check_queue_correct_max() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut range: Vec<usize> = Vec::new();
    range.push(100000usize);
    correct_queue_range(range);
}

/**
  | Test that random numbers of checks are
  | correct
  |
  */
#[traced_test]
fn test_check_queue_correct_random() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut range: Vec<usize> = Vec::with_capacity(100000usize / 1000usize);

    let mut i: usize = 2;
    while i < 100000usize {
        range.push(i);
        let remaining: usize = 100000usize - i;
        let step_max: usize = std::cmp::min(1000usize, remaining);
        let step: usize = std::cmp::max(1usize, InsecureRandRange(step_max as u64) as usize);
        i += step;
    }

    correct_queue_range(range);
}

/**
  | Test that failing checks are caught
  |
  */
#[traced_test]
fn test_check_queue_catches_failure() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut fail_queue = Box::new(Failing_Queue::new(QUEUE_BATCH_SIZE));
    fail_queue.start_worker_threads(SCRIPT_CHECK_THREADS);

    for i in 0..1001usize {
        let mut control = CheckQueueControl::<FailingCheck>::new(
            fail_queue.as_mut() as *mut Failing_Queue
        );

        let mut remaining: usize = i;
        while remaining != 0 {
            let r: usize = InsecureRandRange(10) as usize;

            let mut v_checks: Vec<FailingCheck> = Vec::new();
            v_checks.reserve(r);

            let mut k: usize = 0;
            while k < r && remaining != 0 {
                v_checks.push(FailingCheck::new(remaining == 1));
                k += 1;
                remaining -= 1;
            }

            control.add(&mut v_checks);
        }

        let success = control.wait();
        if i > 0 {
            assert!(!success);
        } else {
            assert!(success);
        }
    }

    fail_queue.stop_worker_threads();
}

/**
  | Test that a block validation which fails
  | does not interfere with future blocks,
  | ie, the bad state is cleared.
  |
  */
#[traced_test]
fn test_check_queue_recovers_from_failure() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut fail_queue = Box::new(Failing_Queue::new(QUEUE_BATCH_SIZE));
    fail_queue.start_worker_threads(SCRIPT_CHECK_THREADS);

    for _times in 0..10 {
        for end_fails in [true, false] {
            let mut control = CheckQueueControl::<FailingCheck>::new(
                fail_queue.as_mut() as *mut Failing_Queue
            );

            {
                let mut v_checks: Vec<FailingCheck> = Vec::with_capacity(100);
                for _ in 0..100 {
                    v_checks.push(FailingCheck::new(false));
                }
                v_checks[99].fails = end_fails;
                control.add(&mut v_checks);
            }

            let r = control.wait();
            assert!(r != end_fails);
        }
    }

    fail_queue.stop_worker_threads();
}

/**
  | Test that unique checks are actually all called
  | individually, rather than just one check being
  | called repeatedly. Test that checks are not
  | called more than once as well
  */
#[traced_test]
fn test_check_queue_unique() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    unique_check::clear_results();

    let mut queue = Box::new(Unique_Queue::new(QUEUE_BATCH_SIZE));
    queue.start_worker_threads(SCRIPT_CHECK_THREADS);

    let count: usize = 100000;
    let mut total: usize = count;

    {
        let _control = CheckQueueControl::<UniqueCheck>::new(queue.as_mut() as *mut Unique_Queue);

        while total != 0 {
            let r: usize = InsecureRandRange(10) as usize;
            let mut v_checks: Vec<UniqueCheck> = Vec::new();

            let mut k: usize = 0;
            while k < r && total != 0 {
                total -= 1;
                v_checks.push(UniqueCheck::new(total));
                k += 1;
            }

            // control drops and Wait()s at end of scope; we mirror that by explicit drop ordering.
            // To keep the exact C++ shape (Add inside the scope), create a control each loop chunk.
            //
            // In the original test, control lives for the whole scope and multiple Adds happen.
            // We'll match that exactly by creating it once and keeping it mutable:
            //
            // (We keep it in a separate block below for line-for-line structure.)
            //
            // (No-op here.)
            drop(v_checks);
        }
    }

    // Re-run with proper lifetime of control, preserving original Add loop exactly.
    total = count;
    unique_check::clear_results();
    {
        let mut control = CheckQueueControl::<UniqueCheck>::new(queue.as_mut() as *mut Unique_Queue);
        while total != 0 {
            let r: usize = InsecureRandRange(10) as usize;
            let mut v_checks: Vec<UniqueCheck> = Vec::new();

            let mut k: usize = 0;
            while k < r && total != 0 {
                total -= 1;
                v_checks.push(UniqueCheck::new(total));
                k += 1;
            }

            control.add(&mut v_checks);
        }
    }

    {
        let total_results = unique_check::total_results_count();
        assert_eq!(total_results, count);

        let mut r_ok = true;
        for i in 0..count {
            r_ok = r_ok && unique_check::results_count(i) == 1;
        }
        assert!(r_ok);
    }

    queue.stop_worker_threads();
}

/**
  | Test that blocks which might allocate lots of
  | memory free their memory aggressively.
  |
  | This test attempts to catch a pathological case
  | where by lazily freeing checks might mean
  | leaving a check un-swapped out, and decreasing
  | by 1 each time could leave the data hanging
  | across a sequence of blocks.
  */
#[traced_test]
fn test_check_queue_memory() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut queue = Box::new(Memory_Queue::new(QUEUE_BATCH_SIZE));
    queue.start_worker_threads(SCRIPT_CHECK_THREADS);

    for i in 0..1000usize {
        let mut total: usize = i;

        {
            let mut control = CheckQueueControl::<MemoryCheck>::new(queue.as_mut() as *mut Memory_Queue);
            while total != 0 {
                let r: usize = InsecureRandRange(10) as usize;
                let mut v_checks: Vec<MemoryCheck> = Vec::new();

                let mut k: usize = 0;
                while k < r && total != 0 {
                    total -= 1;
                    // Each iteration leaves data at the front, back, and middle
                    // to catch any sort of deallocation failure
                    v_checks.push(MemoryCheck::new(total == 0 || total == i || total == i / 2));
                    k += 1;
                }

                control.add(&mut v_checks);
            }
        }

        assert_eq!(memory_check::load_fake_allocated_memory(), 0usize);
    }

    queue.stop_worker_threads();
}

/// Test that a new verification cannot
/// occur until all checks have been destructed
/// 
#[traced_test]
fn test_check_queue_frozen_cleanup() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut queue = Box::new(FrozenCleanup_Queue::new(QUEUE_BATCH_SIZE));
    queue.start_worker_threads(SCRIPT_CHECK_THREADS);

    let queue_ptr: *mut FrozenCleanup_Queue = queue.as_mut() as *mut FrozenCleanup_Queue;

    let t0 = std::thread::spawn(move || {
        let mut control = CheckQueueControl::<FrozenCleanupCheck>::new(queue_ptr);

        let mut v_checks: Vec<FrozenCleanupCheck> = vec![FrozenCleanupCheck::default(); 1];

        // Freezing can't be the default initialized behavior given how the queue
        // swaps in default initialized Checks (otherwise freezing destructor
        // would get called twice).
        v_checks[0].should_freeze = true;

        control.add(&mut v_checks);

        let wait_result = control.wait(); // Hangs here
        assert!(wait_result);
    });

    // Wait until the queue has finished all jobs and frozen
    frozen_cleanup_check::wait_until_frozen();
    assert!(frozen_cleanup_check::is_frozen());

    // Try to get control of the queue a bunch of times (interface-level: try to construct a control)
    let acquired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let acquired2 = acquired.clone();

    let queue_ptr2: *mut FrozenCleanup_Queue = queue.as_mut() as *mut FrozenCleanup_Queue;
    let t_try = std::thread::spawn(move || {
        let _control = CheckQueueControl::<FrozenCleanupCheck>::new(queue_ptr2);
        acquired2.store(true, std::sync::atomic::Ordering::SeqCst);
        // Ensure we actually exercise Wait path on drop in the same way
        // (no outstanding work here; it should return quickly).
    });

    let mut fails = false;
    for _x in 0..100 {
        if acquired.load(std::sync::atomic::Ordering::SeqCst) {
            fails = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // Unfreeze (we need lock in case of spurious wakeup)
    frozen_cleanup_check::unfreeze();

    // Awaken frozen destructor
    frozen_cleanup_check::notify_one();

    // Wait for control to finish
    t0.join().unwrap();
    t_try.join().unwrap();

    assert!(!fails);
    queue.stop_worker_threads();
}

/// Test that CCheckQueueControl is threadsafe
/// 
#[traced_test]
fn test_check_queue_control_locks() {
    let _guard = CHECKQUEUE_INTEGRATION_TEST_MUTEX.lock().unwrap();

    let mut queue = Box::new(Standard_Queue::new(QUEUE_BATCH_SIZE));

    {
        let mut tg: Vec<std::thread::JoinHandle<()>> = Vec::new();
        let n_threads = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
        let fails = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));

        for _i in 0..3usize {
            let queue_ptr: *mut Standard_Queue = queue.as_mut() as *mut Standard_Queue;
            let n_threads2 = n_threads.clone();
            let fails2 = fails.clone();

            tg.push(std::thread::spawn(move || {
                let _control = CheckQueueControl::<FakeCheck>::new(queue_ptr);
                // While sleeping, no other thread should execute to this point
                let observed = n_threads2.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;

                std::thread::sleep(std::time::Duration::from_millis(10));

                let now = n_threads2.load(std::sync::atomic::Ordering::SeqCst);
                if observed != now {
                    fails2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }));
        }

        for thread in tg {
            thread.join().unwrap();
        }

        assert_eq!(fails.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    {
        let mut tg: Vec<std::thread::JoinHandle<()>> = Vec::new();

        let m = std::sync::Arc::new(std::sync::Mutex::new(()));
        let cv = std::sync::Arc::new(std::sync::Condvar::new());

        let has_lock = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let has_tried = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let done_ack = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let queue_ptr: *mut Standard_Queue = queue.as_mut() as *mut Standard_Queue;

        {
            let m2 = m.clone();
            let cv2 = cv.clone();
            let has_lock2 = has_lock.clone();
            let has_tried2 = has_tried.clone();
            let done2 = done.clone();
            let done_ack2 = done_ack.clone();

            tg.push(std::thread::spawn(move || {
                let _control = CheckQueueControl::<FakeCheck>::new(queue_ptr);

                let mut ll = m2.lock().unwrap();
                has_lock2.store(true, std::sync::atomic::Ordering::SeqCst);
                cv2.notify_one();

                while !has_tried2.load(std::sync::atomic::Ordering::SeqCst) {
                    ll = cv2.wait(ll).unwrap();
                }

                done2.store(true, std::sync::atomic::Ordering::SeqCst);
                cv2.notify_one();

                // Wait until the done is acknowledged
                while !done_ack2.load(std::sync::atomic::Ordering::SeqCst) {
                    ll = cv2.wait(ll).unwrap();
                }
            }));

            // Wait for thread to get the lock
            let mut l = m.lock().unwrap();
            while !has_lock.load(std::sync::atomic::Ordering::SeqCst) {
                l = cv.wait(l).unwrap();
            }

            // Interface-level equivalent of try_lock: spawn a thread that attempts to create a control,
            // and ensure it does not succeed while the first control is alive.
            let acquired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
            let acquired2 = acquired.clone();

            let queue_ptr2: *mut Standard_Queue = queue.as_mut() as *mut Standard_Queue;
            let t_try = std::thread::spawn(move || {
                let _control = CheckQueueControl::<FakeCheck>::new(queue_ptr2);
                acquired2.store(true, std::sync::atomic::Ordering::SeqCst);
            });

            let mut fails = false;
            for _x in 0..100 {
                if acquired.load(std::sync::atomic::Ordering::SeqCst) {
                    fails = true;
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }

            has_tried.store(true, std::sync::atomic::Ordering::SeqCst);
            cv.notify_one();

            while !done.load(std::sync::atomic::Ordering::SeqCst) {
                l = cv.wait(l).unwrap();
            }

            // Acknowledge the done
            done_ack.store(true, std::sync::atomic::Ordering::SeqCst);
            cv.notify_one();

            assert!(!fails);

            t_try.join().unwrap();
        }

        for thread in tg {
            thread.join().unwrap();
        }
    }
}
