// ---------------- [ File: bitcoinleveldb-dbtest/src/random_string.rs ]
crate::ix!();

/// Generates a deterministic ASCII string of length `len` using `rnd`.
///
/// Invariants:
/// - If `len <= 0`, returns the empty string and **does not** advance `rnd`.
/// - If `len > 0`, advances `rnd` **exactly `len` times** via `uniform(26)` and
///   emits only `'a'..='z'`.
pub fn random_string(rnd: *mut Random, len: i32) -> String {
    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "db_rand.random_string.entry",
        len = len,
        rnd_is_null = rnd.is_null()
    );

    /*
        std::string r;
        test::RandomString(rnd, len, &r);
        return r;
    */

    if rnd.is_null() {
        tracing::error!(
            target: "bitcoinleveldb.dbtest",
            event = "db_rand.random_string.null_rng",
            len = len
        );
        // C++ would likely crash on null; keep behavior explicit and observable.
        return String::new();
    }

    if len <= 0 {
        tracing::trace!(
            target: "bitcoinleveldb.dbtest",
            event = "db_rand.random_string.len_non_positive",
            len = len
        );

        let out = String::new();

        tracing::trace!(
            target: "bitcoinleveldb.dbtest",
            event = "db_rand.random_string.exit",
            len = len,
            out_len = out.len()
        );

        return out;
    }

    let effective_len: usize = len as usize;

    let rng: &mut Random = unsafe { &mut *rnd };

    let mut out = String::with_capacity(effective_len);

    for _ in 0..effective_len {
        // LevelDB testutil uses alphabetic ASCII; preserve the same character class.
        let r: u32 = rng.uniform(26);
        let ch: char = (b'a' + (r as u8)) as char;
        out.push(ch);
    }

    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "db_rand.random_string.exit",
        len = len,
        out_len = out.len()
    );

    out
}
