// ---------------- [ File: bitcoinleveldb-dbtest/src/key.rs ]
crate::ix!();

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
