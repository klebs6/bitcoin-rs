// ---------------- [ File: bitcoinleveldbt-util/src/random_key.rs ]
crate::ix!();

/// Invariant: output is entirely determined by the PRNG state reachable from `rnd`.
/// Side effects: consumes PRNG draws in the same conditional structure as the C++ code.
pub fn dbtest_random_key(rnd: *mut Random) -> String {
    let rnd_ptr_usize: usize = rnd as usize;

    trace!(
        target: "bitcoinleveldbt-dbtest",
        label = "dbtest_random_key.entry",
        rnd_ptr_usize
    );

    let len: i32 = unsafe {
        if (*rnd).one_in(3) {
            1 // Short sometimes to encourage collisions
        } else if (*rnd).one_in(100) {
            (*rnd).skewed(10) as i32
        } else {
            (*rnd).uniform(10) as i32
        }
    };

    let k = random_key(rnd, len);

    trace!(
        target: "bitcoinleveldbt-dbtest",
        label = "dbtest_random_key.exit",
        rnd_ptr_usize,
        len,
        out_len = k.len()
    );

    k
}

/**
  | Return a random key with the specified
  | length that may contain interesting
  | characters (e.g. \x00, \xff, etc.).
  |
  */
pub fn random_key(
    rnd: *mut Random,
    len: i32,
) -> String {
    trace!(
        target: "bitcoinleveldbt_util::util",
        event = "random_key_entry",
        rnd_is_null = rnd.is_null(),
        len = len
    );

    // Make sure to generate a wide variety of characters so we
    // test the boundary conditions for short-key optimizations.
    const BITCOINLEVELDB_TEST_UTIL_RANDOM_KEY_TEST_CHARS: [u8; 10] = [
        b'\0',
        b'\x01',
        b'a',
        b'b',
        b'c',
        b'd',
        b'e',
        b'\xfd',
        b'\xfe',
        b'\xff',
    ];

    if rnd.is_null() || len <= 0 {
        trace!(
            target: "bitcoinleveldbt_util::util",
            event = "random_key_exit",
            result_len = 0
        );

        return String::new();
    }

    let mut result = String::new();
    let mut i: i32 = 0i32;

    while i < len {
        let idx = unsafe {
            (&mut *rnd).uniform(BITCOINLEVELDB_TEST_UTIL_RANDOM_KEY_TEST_CHARS.len() as i32)
        } as usize;

        let ch = char::from(BITCOINLEVELDB_TEST_UTIL_RANDOM_KEY_TEST_CHARS[idx]);
        result.push(ch);

        i += 1i32;
    }

    trace!(
        target: "bitcoinleveldbt_util::util",
        event = "random_key_exit",
        result_len = result.len()
    );

    result
}
