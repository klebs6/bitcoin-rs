crate::ix!();

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
        target: "bitcoinleveldb_test::util",
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
            target: "bitcoinleveldb_test::util",
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
        target: "bitcoinleveldb_test::util",
        event = "random_key_exit",
        result_len = result.len()
    );

    result
}
