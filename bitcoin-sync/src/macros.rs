// ---------------- [ File: bitcoin-sync/src/macros.rs ]
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

#[macro_export]
macro_rules! assert_lock_held {
    ($cs:expr) => {{
        unsafe {
            $crate::assert_lock_held_internal(
                concat!(stringify!($cs), "\0").as_ptr(),
                concat!(file!(),            "\0").as_ptr(),
                line!() as i32,
                &($cs) as *const _ as *mut _,
            );
        }
    }};
}

#[macro_export]
macro_rules! assert_lock_not_held {
    ($cs:expr) => {{
        unsafe {
            $crate::assert_lock_not_held_internal(
                concat!(stringify!($cs), "\0").as_ptr(),
                concat!(file!(),            "\0").as_ptr(),
                line!() as i32,
                &($cs) as *const _ as *mut _,
            );
        }
    }};
}

#[macro_export]
macro_rules! reverse_lock {
    ($g:ident) => {
        paste! {
            let mut [<__revlock_  line!() _ column!()>] =
                $crate::ReverseLock::new(&mut $g);
        }
    };
}

#[macro_export]
macro_rules! lock {
    ($cs:expr) => {
        paste! {
            let [<__criticalblock_  line!() _ column!()>] =
                $crate::UniqueLock::new(
                    &$cs,
                    stringify!($cs),
                    file!(),
                    line!(),
                    None,
                );
        }
    };
}

#[macro_export]
macro_rules! lock2 {
    ($cs1:expr, $cs2:expr) => {
        paste! {
            let [<__criticalblock1_  line!() _ column!()>] =
                $crate::UniqueLock::new(
                    &$cs1,
                    stringify!($cs1),
                    file!(),
                    line!(),
                    None,
                );
            let [<__criticalblock2_  line!() _ column!()>] =
                $crate::UniqueLock::new(
                    &$cs2,
                    stringify!($cs2),
                    file!(),
                    line!(),
                    None,
                );
        }
    };
}

#[macro_export]
macro_rules! try_lock {
    ($cs:expr, $name:ident) => {
        let mut $name = $crate::UniqueLock::new(
            &$cs,
            stringify!($cs),
            file!(),
            line!(),
            Some(true),
        );
    };
}

#[macro_export]
macro_rules! wait_lock {
    ($cs:expr, $name:ident) => {
        let mut $name = $crate::UniqueLock::new(
            &$cs,
            stringify!($cs),
            file!(),
            line!(),
            None,
        );
    };
}

#[macro_export]
macro_rules! enter_critical_section {
    ($cs:expr) => {{
        unsafe {
            $crate::enter_critical(
                concat!(stringify!($cs), "\0").as_ptr(),
                concat!(file!(),            "\0").as_ptr(),
                line!() as i32,
                &($cs) as *const _ as *mut _,
                false,
            );
        }
        ($cs).lock();
    }};
}

#[macro_export]
macro_rules! leave_critical_section {
    ($cs:expr) => {{
        let mut lockname = String::new();
        unsafe {
            $crate::check_last_critical(
                &($cs) as *const _ as *mut core::ffi::c_void,
                &mut lockname,
                concat!(stringify!($cs), "\0").as_ptr(),
                concat!(file!(),            "\0").as_ptr(),
                line!() as i32,
            );
        }
        ($cs).unlock();
        $crate::leave_critical();
    }};
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
#[macro_export]
macro_rules! with_lock {
    ($cs:expr, $code:expr) => {{
        (|| {
            lock!($cs);
            $code
        })()
    }};
}

// ---------------- [ File: bitcoin-sync/src/macros.rs ] (added exhaustive test suite)

#[cfg(test)]
mod macros_contract_tests {
    use super::*;
    use parking_lot::RawMutex;

    type AM = AnnotatedMixin<RawMutex>;

    #[traced_test]
    fn lock_macro_acquires_and_releases() {
        let m = AM::default();
        {
            lock!(m);
            assert!(!m.try_lock(), "mutex should be held inside lock! scope");
        }
        assert!(m.try_lock(), "mutex should be released after lock! scope");
        m.unlock();
    }

    #[traced_test]
    fn lock2_macro_acquires_two() {
        let m1 = AM::default();
        let m2 = AM::default();
        {
            lock2!(m1, m2);
            assert!(!m1.try_lock(), "m1 should be held inside lock2! scope");
            assert!(!m2.try_lock(), "m2 should be held inside lock2! scope");
        }
        assert!(m1.try_lock(), "m1 released after scope"); m1.unlock();
        assert!(m2.try_lock(), "m2 released after scope"); m2.unlock();
    }

    #[traced_test]
    fn reverse_lock_temporarily_releases() {
        let m = AM::default();
        let mut g = UniqueLock::new(&m, "m", file!(), line!(), None);
        assert!(g.owns_lock(), "guard must start owning the lock");

        {
            reverse_lock!(g); // releases lock
            assert!(m.try_lock(), "lock released while ReverseLock alive");
            m.unlock();
        }
        // ReverseLock dropped → `g` must have re‑acquired
        assert!(!m.try_lock(), "lock re‑acquired by guard after ReverseLock drop");
        g.unlock();
        assert!(m.try_lock(), "lock fully released after guard unlock");
        m.unlock();
    }

    #[traced_test]
    fn try_lock_macro_success_and_failure() {
        // success path
        let m = AM::default();
        try_lock!(m, t_success);
        assert!(bool::from(&t_success), "try_lock! should own when mutex free");
        t_success.unlock();

        // failure path
        let m2 = AM::default();
        m2.lock(); // pre‑lock so try fails
        try_lock!(m2, t_fail);
        assert!(!bool::from(&t_fail), "try_lock! must fail when mutex already held");
        m2.unlock();
    }

    #[traced_test]
    fn wait_lock_macro_acquires() {
        let m = AM::default();
        {
            wait_lock!(m, guard);
            assert!(!m.try_lock(), "mutex held inside wait_lock! guard");
        }
        assert!(m.try_lock(), "mutex released after wait_lock! scope");
        m.unlock();
    }

    #[traced_test]
    fn enter_leave_critical_section_lifecycle() {
        let m = AM::default();
        enter_critical_section!(m);
        assert!(!m.try_lock(), "mutex locked by enter_critical_section!");
        leave_critical_section!(m);
        assert!(m.try_lock(), "mutex unlocked by leave_critical_section!");
        m.unlock();
    }

    #[traced_test]
    fn with_lock_macro_returns_value() {
        let m = AM::default();
        let mut val = 0;
        let out = with_lock!(m, { val += 1; val });
        assert_eq!(out, 1, "with_lock! must return inner expression value");
        assert!(m.try_lock(), "mutex released after with_lock! evaluation");
        m.unlock();
    }

    #[traced_test]
    fn assert_lock_macros_compile_and_run() {
        let m = AM::default();
        m.lock();
        assert_lock_held!(m);     // should be a no‑op in non‑DEBUG build
        m.unlock();
        assert_lock_not_held!(m); // likewise
    }
}
