// ---------------- [ File: bitcoinleveldb-dbtest/src/random.rs ]
crate::ix!();

/// Invariant: output is entirely determined by the PRNG state reachable from `rnd` plus `len`.
/// Side effects: advances `rnd` exactly as `test::dbtest_random_string` does.
pub fn dbtest_random_string(rnd: *mut Random, len: i32) -> String {
    let rnd_ptr_usize: usize = rnd as usize;

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbtest_random_string.entry",
        rnd_ptr_usize,
        len
    );

    let mut r = String::new();
    let _ = bitcoinleveldb_test::random_string(rnd, len, &mut r as *mut String);

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbtest_random_string.exit",
        rnd_ptr_usize,
        len,
        out_len = r.len()
    );

    r
}

/// Invariant: output is entirely determined by the PRNG state reachable from `rnd`.
/// Side effects: consumes PRNG draws in the same conditional structure as the C++ code.
pub fn dbtest_random_key(rnd: *mut Random) -> String {
    let rnd_ptr_usize: usize = rnd as usize;

    trace!(
        target: "bitcoinleveldb-dbtest",
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

    let k = bitcoinleveldb_test::random_key(rnd, len);

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbtest_random_key.exit",
        rnd_ptr_usize,
        len,
        out_len = k.len()
    );

    k
}

/// Deterministic formatting helper for DB tests.
///
/// Invariant: output is exactly `"key"` followed by **six** base-10 digits,
/// zero-padded on the left (matching `printf("key%06d")`).
pub fn key(i: i32) -> String {
    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "dbtest.key.entry",
        i = i
    );

    let s = format!("key{:06}", i);

    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "dbtest.key.exit",
        i = i,
        key = %s
    );

    s
}
