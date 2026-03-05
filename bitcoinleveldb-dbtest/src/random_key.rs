// ---------------- [ File: bitcoinleveldb-dbtest/src/random_key.rs ]
crate::ix!();

/// Invariant: output is entirely determined by the PRNG state reachable from `rnd`.
/// Side effects: consumes PRNG draws in the same conditional structure as the C++ code.
pub fn random_key(rnd: *mut Random) -> String {
    let rnd_ptr_usize: usize = rnd as usize;

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "random_key.entry",
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
        label = "random_key.exit",
        rnd_ptr_usize,
        len,
        out_len = k.len()
    );

    k
}
