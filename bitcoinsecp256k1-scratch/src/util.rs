// ---------------- [ File: bitcoinsecp256k1-scratch/src/util.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/util.h]

#[cfg(DETERMINISTIC)]
#[macro_export]
macro_rules! test_failure {
    ($msg:expr) => {{
        $crate::util_trigger_test_failure($msg, None, None);
    }};
}

#[cfg(not(DETERMINISTIC))]
#[macro_export]
macro_rules! test_failure {
    ($msg:expr) => {{
        $crate::util_trigger_test_failure($msg, Some(file!()), Some(line!()));
    }};
}

#[cfg(DETERMINISTIC)]
#[macro_export]
macro_rules! check {
    ($cond:expr) => {{
        if !$cond {
            $crate::test_failure!("test condition failed");
        }
    }};
}

#[cfg(not(DETERMINISTIC))]
#[macro_export]
macro_rules! check {
    ($cond:expr) => {{
        if !$cond {
            $crate::test_failure!(concat!("test condition failed: ", stringify!($cond)));
        }
    }};
}

#[inline(always)]
pub(crate) fn util_trigger_test_failure(
    msg: &'static str,
    file: Option<&'static str>,
    line: Option<u32>,
) -> ! {
    match (file, line) {
        (Some(file), Some(line)) => {
            error!(
                target: "bitcoinsecp256k1_scratch::util",
                file,
                line,
                msg,
                "test_failure"
            );
        }
        _ => {
            error!(
                target: "bitcoinsecp256k1_scratch::util",
                msg,
                "test_failure"
            );
        }
    }

    #[cfg(test)]
    panic!("{}", msg);

    #[cfg(not(test))]
    {
        unsafe { libc::abort() }
    }
}

#[cfg(COVERAGE)]
#[macro_export]
macro_rules! VERIFY_CHECK {
    ($cond:expr) => {{}};
    {$cond:expr} => {{}};
}

#[cfg(all(VERIFY, not(COVERAGE)))]
#[macro_export]
macro_rules! VERIFY_CHECK {
    ($cond:expr) => {{
        $crate::check!($cond);
    }};
    {$cond:expr} => {{
        $crate::check!($cond);
    }};
}

#[cfg(all(not(VERIFY), not(COVERAGE)))]
#[macro_export]
macro_rules! VERIFY_CHECK {
    ($cond:expr) => {{
        let _ = $cond;
    }};
    {$cond:expr} => {{
        let _ = $cond;
    }};
}

#[cfg(COVERAGE)]
#[macro_export]
macro_rules! VERIFY_SETUP {
    ($($stmt:tt)*) => {{}};
}

#[cfg(all(VERIFY, not(COVERAGE)))]
#[macro_export]
macro_rules! VERIFY_SETUP {
    ($($stmt:tt)*) => {{
        $($stmt)*
    }};
}

#[cfg(all(not(VERIFY), not(COVERAGE)))]
#[macro_export]
macro_rules! VERIFY_SETUP {
    ($($stmt:tt)*) => {{}};
}

/// Using 16 bytes alignment because common
/// architectures never have alignment
/// requirements above
/// 8 for any of the types we care about. 
///
/// In addition we leave some room because
/// currently we don't care about a few bytes.
/// 
pub const ALIGNMENT: usize = {
    let a = core::mem::align_of::<libc::max_align_t>();
    if a > 16 { a } else { 16 }
};

#[macro_export] macro_rules! round_to_align {
    ($size:expr) => {
        ((($size) + ALIGNMENT - 1) / ALIGNMENT) * ALIGNMENT
    }
}

#[cfg(test)]
mod util_macro_semantics_test_suite {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering};

    #[traced_test]
    fn round_to_align_matches_manual_rounding_for_various_sizes() {
        let sizes: [usize; 14] = [
            0,
            1,
            2,
            15,
            16,
            17,
            31,
            32,
            33,
            ALIGNMENT - 1,
            ALIGNMENT,
            ALIGNMENT + 1,
            (ALIGNMENT * 3) - 1,
            ALIGNMENT * 3,
        ];

        for &s in sizes.iter() {
            let got = round_to_align!(s);
            let want = ((s + ALIGNMENT - 1) / ALIGNMENT) * ALIGNMENT;

            debug!(
                target: "bitcoinsecp256k1_scratch::tests::util",
                s,
                got,
                want,
                "round_to_align check"
            );

            assert_eq!(got, want);
            assert_eq!(got % ALIGNMENT, 0);
            assert!(got >= s);
        }
    }

    #[cfg(all(not(VERIFY), not(COVERAGE)))]
    #[traced_test]
    fn verify_check_evaluates_expression_in_non_verify_non_coverage_build() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        fn side_effect_true() -> bool {
            CALLS.fetch_add(1, Ordering::SeqCst);
            true
        }

        let before = CALLS.load(Ordering::SeqCst);
        VERIFY_CHECK!({ side_effect_true() });
        let after = CALLS.load(Ordering::SeqCst);

        assert_eq!(after, before + 1);
    }

    #[cfg(COVERAGE)]
    #[traced_test]
    fn verify_check_does_not_evaluate_expression_in_coverage_build() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        fn side_effect_true() -> bool {
            CALLS.fetch_add(1, Ordering::SeqCst);
            true
        }

        let before = CALLS.load(Ordering::SeqCst);
        VERIFY_CHECK!({ side_effect_true() });
        let after = CALLS.load(Ordering::SeqCst);

        assert_eq!(after, before);
    }

    #[cfg(all(VERIFY, not(COVERAGE)))]
    #[traced_test]
    fn verify_check_panics_in_tests_on_failure_in_verify_build() {
        let r = std::panic::catch_unwind(|| {
            VERIFY_CHECK!({ false });
        });

        assert!(r.is_err(), "VERIFY_CHECK(false) must fail in VERIFY builds");
    }
}
