// ---------------- [ File: bitcoinleveldb-dbtest/src/random_string.rs ]
crate::ix!();

/// Invariant: output is entirely determined by the PRNG state reachable from `rnd` plus `len`.
/// Side effects: advances `rnd` exactly as `test::random_string` does.
pub fn random_string(rnd: *mut Random, len: i32) -> String {
    let rnd_ptr_usize: usize = rnd as usize;

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "random_string.entry",
        rnd_ptr_usize,
        len
    );

    let mut r = String::new();
    bitcoinleveldb_test::random_string(rnd, len, &mut r);

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "random_string.exit",
        rnd_ptr_usize,
        len,
        out_len = r.len()
    );

    r
}
