crate::ix!();

/// Return a randomization seed for this run.
///
/// Invariant: non-positive or unparsable environment overrides collapse to the
/// stable fallback value `301`, preserving deterministic behavior.
pub fn random_seed() -> i32 {
    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "random_seed_entry"
    );

    let result = unsafe {
        let env_ptr = libc::getenv(b"TEST_RANDOM_SEED\0".as_ptr() as *const c_char);
        if env_ptr.is_null() {
            301
        } else {
            let raw = CStr::from_ptr(env_ptr).to_string_lossy().into_owned();
            match raw.parse::<i32>() {
                Ok(parsed) => {
                    if parsed <= 0 {
                        301
                    } else {
                        parsed
                    }
                }
                Err(_) => 301,
            }
        }
    };

    trace!(
        target: "bitcoinleveldb_test::harness",
        event = "random_seed_exit",
        result = result
    );

    result
}
