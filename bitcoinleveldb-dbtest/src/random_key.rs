// ---------------- [ File: bitcoinleveldb-dbtest/src/random_key.rs ]
crate::ix!();

/// Generates a random key with the same length-selection distribution as the C++ test.
///
/// Invariants:
/// - Uses the exact decision tree:
///   `OneIn(3) -> len=1` else `OneIn(100) -> Skewed(10)` else `Uniform(10)`.
/// - After choosing `len`, delegates to `random_string(rnd, len)` and therefore
///   advances the RNG for character generation **exactly `len` times**.
pub fn random_key(rnd: *mut Random) -> String {
    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "db_rand.random_key.entry",
        rnd_is_null = rnd.is_null()
    );

    /*
        int len =
          (rnd->OneIn(3) ? 1  // Short sometimes to encourage collisions
                         : (rnd->OneIn(100) ? rnd->Skewed(10) : rnd->Uniform(10)));
        return test::RandomKey(rnd, len);
    */

    if rnd.is_null() {
        tracing::error!(
            target: "bitcoinleveldb.dbtest",
            event = "db_rand.random_key.null_rng"
        );
        // C++ would likely crash on null; keep behavior explicit and observable.
        return String::new();
    }

    let rng: &mut Random = unsafe { &mut *rnd };

    let len: i32 = if rng.one_in(3) {
        1
    } else if rng.one_in(100) {
        rng.skewed(10) as i32
    } else {
        rng.uniform(10) as i32
    };

    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "db_rand.random_key.len_chosen",
        len = len
    );

    let k: String = random_string(rnd, len);

    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "db_rand.random_key.exit",
        len = len,
        key_len = k.len()
    );

    k
}
