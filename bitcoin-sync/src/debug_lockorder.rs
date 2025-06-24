// ---------------- [ File: bitcoin-sync/src/debug_lockorder.rs ]
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

    use std::{
        collections::{HashMap, HashSet},
        ffi::{c_void, CStr},
        os::raw::c_char,
        sync::atomic::{AtomicBool, Ordering},
        thread::{self, ThreadId},
    };
    use parking_lot::Mutex as ParkingMutex;
    use super::*;

    /// ─────────────────────────────────────────────────────────────────────────
    /// Global behaviour toggle
    /// ─────────────────────────────────────────────────────────────────────────
    /// Call abort() if a potential lock order deadlock bug is detected, instead
    /// of just logging information and throwing a logic_error.
    /// 
    /// Defaults to true, and set to false in DEBUG_LOCKORDER unit tests.
    /// 
    lazy_static! {
        pub static ref G_DEBUG_LOCKORDER_ABORT: AtomicBool = AtomicBool::new(true);
    }

    /// Early deadlock detection.
    /// Problem being solved:
    ///    Thread 1 locks A, then B, then C
    ///    Thread 2 locks D, then C, then A
    ///     --> may result in deadlock between the two
    ///         threads, depending on when they run.
    /// 
    /// Solution implemented here:
    /// 
    /// Keep track of pairs of locks: (A before B), (A
    /// before C), etc.
    /// 
    /// Complain if any thread tries to lock in
    /// a different order.
    ///
    /// ─────────────────────────────────────────────────────────────────────────
    /// Lock‑site metadata
    /// ─────────────────────────────────────────────────────────────────────────
    #[derive(Debug, Clone, Getters, Builder)]
    #[getset(get = "pub")]
    pub struct LockLocation {
        try_:        bool,
        mutex_name:  String,
        source_file: String,
        thread_name: String,
        source_line: i32,
    }

    impl LockLocation {
        /// # Safety
        /// `psz_name` and `psz_file` **must** be valid, NUL‑terminated strings or
        /// `NULL`.  Undefined behaviour otherwise.
        pub unsafe fn new(
            psz_name:    *const u8,
            psz_file:    *const u8,
            n_line:      i32,
            try_in:      bool,
            thread_name: &str,
        ) -> Self {
            let mutex_name = if psz_name.is_null() {
                "<null>".into()
            } else {
                CStr::from_ptr(psz_name as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            };

            let source_file = if psz_file.is_null() {
                "<null>".into()
            } else {
                CStr::from_ptr(psz_file as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            };

            Self {
                try_: try_in,
                mutex_name,
                source_file,
                thread_name: thread_name.into(),
                source_line: n_line,
            }
        }

        pub fn to_string(&self) -> String {
            format!(
                "'{}' in {}:{}{} (in thread '{}')",
                self.mutex_name,
                self.source_file,
                self.source_line,
                if self.try_ { " (TRY)" } else { "" },
                self.thread_name
            )
        }

        #[inline]
        pub fn name(&self) -> &str {
            &self.mutex_name
        }
    }

    pub type LockStackItem = (*mut c_void,LockLocation);
    pub type LockStack     = Vec<LockStackItem>;
    pub type LockStacks    = HashMap<ThreadId,LockStack>;
    pub type LockPair      = (*mut c_void,*mut c_void);
    pub type LockOrders    = HashMap<LockPair,LockStack>;
    pub type InvLockOrders = HashSet<LockPair>;

    #[derive(Default)]
    pub struct LockData {
        lock_stacks:   LockStacks,
        lockorders:    LockOrders,
        invlockorders: InvLockOrders,
    }

    lazy_static! {
        pub static ref LOCK_DATA: ParkingMutex<LockData> = ParkingMutex::new(LockData::default());
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Helper: abort vs. panic
    // ─────────────────────────────────────────────────────────────────────────
    #[inline(always)]
    fn abort_or_panic(msg: &str) -> ! {
        if G_DEBUG_LOCKORDER_ABORT.load(Ordering::Relaxed) {
            error!("{msg}");
            std::process::abort();
        }
        panic!("{msg}");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Diagnostics
    // ─────────────────────────────────────────────────────────────────────────
    pub fn potential_deadlock_detected(
        mismatch: &LockPair,
        prev:     &LockStack,
        current:  &LockStack,
    ) -> ! {
        error!("POTENTIAL DEADLOCK DETECTED");
        error!("Previous lock order was:");
        for (ptr, loc) in prev {
            let tag = if *ptr == mismatch.0 {
                " (1)"
            } else if *ptr == mismatch.1 {
                " (2)"
            } else {
                ""
            };
            error!("{tag} {}", loc.to_string());
        }

        error!("Current lock order is:");
        for (ptr, loc) in current {
            let tag = if *ptr == mismatch.0 {
                " (1)"
            } else if *ptr == mismatch.1 {
                " (2)"
            } else {
                ""
            };
            error!("{tag} {}", loc.to_string());
        }

        let a = current
            .iter()
            .find(|(p, _)| *p == mismatch.0)
            .map(|(_, l)| l.name())
            .unwrap_or("<unknown>");
        let b = current
            .iter()
            .find(|(p, _)| *p == mismatch.1)
            .map(|(_, l)| l.name())
            .unwrap_or("<unknown>");

        abort_or_panic(&format!("potential deadlock detected: {b} -> {a} -> {b}"));
    }

    pub fn double_lock_detected(mutex: *const c_void, stack: &LockStack) -> ! {
        error!("DOUBLE LOCK DETECTED");
        error!("Lock order:");
        for (ptr, loc) in stack {
            let tag = if *ptr == mutex { " (*)" } else { "" };
            error!("{tag} {}", loc.to_string());
        }
        abort_or_panic("double lock detected");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Core bookkeeping
    // ─────────────────────────────────────────────────────────────────────────
    pub fn push_lock<MutexType>(c: *mut MutexType, loc: &LockLocation) {
        let cs_ptr = c as *mut c_void;
        let mut data = LOCK_DATA.lock();
        let tid      = thread::current().id();

        let stack = data.lock_stacks.entry(tid).or_default();
        stack.push((cs_ptr, loc.clone()));

        if stack.len() <= 1 {
            return;
        }

        let snapshot = stack.clone(); // after push
        let head     = snapshot.last().expect("just pushed");

        for (prev_ptr, _) in &snapshot[..snapshot.len() - 1] {
            // 1. Same non‑recursive mutex twice
            if *prev_ptr == cs_ptr {
                let mut copy = snapshot.clone();
                copy.pop(); // remove the most‑recent push
                double_lock_detected(cs_ptr, &copy);
            }

            // 2. Lock‑order inversion
            let p1 = (*prev_ptr, cs_ptr);
            if data.lockorders.contains_key(&p1) {
                continue;
            }

            let p2 = (cs_ptr, *prev_ptr);
            if let Some(prior_stack) = data.lockorders.get(&p2).cloned() {
                let mut copy = snapshot.clone();
                copy.pop();
                potential_deadlock_detected(&p1, &prior_stack, &copy);
            }

            // 3. Record new ordering
            data.lockorders.insert(p1, snapshot.clone());
            data.invlockorders.insert(p2);
        }
    }

    pub fn pop_lock() {
        let mut data = LOCK_DATA.lock();
        let tid = thread::current().id();

        if let Some(stack) = data.lock_stacks.get_mut(&tid) {
            stack.pop();
            if stack.is_empty() {
                data.lock_stacks.remove(&tid);
            }
        }
    }

    pub fn enter_critical<MutexType>(
        psz_name: *const u8,
        psz_file: *const u8,
        n_line:   i32,
        cs:       *mut MutexType,
        try_:     bool,
    ) {
        let thread_name = thread::current().name().unwrap_or("unnamed");
        let loc = unsafe { LockLocation::new(psz_name, psz_file, n_line, try_, thread_name) };
        push_lock(cs, &loc);
    }

    pub fn check_last_critical(
        cs:        *mut c_void,
        lockname:  &mut String,
        guardname: *const u8,
        file:      *const u8,
        line:      i32,
    ) {
        let data = LOCK_DATA.lock();
        let tid  = thread::current().id();

        if let Some(stack) = data.lock_stacks.get(&tid) {
            if let Some((last_ptr, last_loc)) = stack.last() {
                if *last_ptr == cs {
                    *lockname = last_loc.name().into();
                    return;
                }
            }

            error!("INCONSISTENT LOCK ORDER DETECTED");
            error!("Current lock order (least recent first):");
            for (_, loc) in stack {
                error!(" {}", loc.to_string());
            }
        }

        let guard = unsafe {
            if guardname.is_null() {
                "<null>".into()
            } else {
                CStr::from_ptr(guardname as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            }
        };
        let file = unsafe {
            if file.is_null() {
                "<null>".into()
            } else {
                CStr::from_ptr(file as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            }
        };

        abort_or_panic(&format!(
            "{file}:{line} {guard} was not most recent critical section locked"
        ));
    }

    #[inline]
    pub fn leave_critical() {
        pop_lock();
    }

    pub fn locks_held() -> String {
        let data  = LOCK_DATA.lock();
        let tid   = thread::current().id();
        let mut s = String::new();

        if let Some(stack) = data.lock_stacks.get(&tid) {
            for (_, loc) in stack {
                s.push_str(&loc.to_string());
                s.push('\n');
            }
        }
        s
    }

    pub fn lock_held(cs: *mut c_void) -> bool {
        let data = LOCK_DATA.lock();
        let tid  = thread::current().id();
        data
            .lock_stacks
            .get(&tid)
            .map(|s| s.iter().any(|(p, _)| *p == cs))
            .unwrap_or(false)
    }

    pub fn assert_lock_held_internal<MutexType>(
        psz_name: *const u8,
        psz_file: *const u8,
        n_line:   i32,
        cs:       *mut MutexType,
    ) {
        if lock_held(cs as *mut c_void) {
            return;
        }
        let name = unsafe {
            CStr::from_ptr(psz_name as *const c_char)
                .to_string_lossy()
                .into_owned()
        };
        let file = unsafe {
            CStr::from_ptr(psz_file as *const c_char)
                .to_string_lossy()
                .into_owned()
        };
        abort_or_panic(&format!(
            "Assertion failed: lock {name} not held in {file}:{n_line}; locks held:\n{}",
            locks_held()
        ));
    }

    pub fn assert_lock_not_held_internal<MutexType>(
        psz_name: *const u8,
        psz_file: *const u8,
        n_line:   i32,
        cs:       *mut MutexType,
    ) {
        if !lock_held(cs as *mut c_void) {
            return;
        }
        let name = unsafe {
            CStr::from_ptr(psz_name as *const c_char)
                .to_string_lossy()
                .into_owned()
        };
        let file = unsafe {
            CStr::from_ptr(psz_file as *const c_char)
                .to_string_lossy()
                .into_owned()
        };
        abort_or_panic(&format!(
            "Assertion failed: lock {name} **held** in {file}:{n_line}; locks held:\n{}",
            locks_held()
        ));
    }

    pub fn delete_lock(cs: *mut c_void) {
        let mut data = LOCK_DATA.lock();

        data.lockorders
            .retain(|(a, b), _| *a != cs && *b != cs);
        data.invlockorders
            .retain(|(a, b)|        *a != cs && *b != cs);
    }

    pub fn lock_stack_empty() -> bool {
        let data = LOCK_DATA.lock();
        let tid  = thread::current().id();
        data
            .lock_stacks
            .get(&tid)
            .map_or(true, |s| s.is_empty())
    }
}
