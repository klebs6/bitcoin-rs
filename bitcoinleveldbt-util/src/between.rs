// ---------------- [ File: bitcoinleveldb-testutil/src/between.rs ]
crate::ix!();

/// Range predicate used by approximate-size tests.
///
/// Invariants:
/// - Returns `true` iff `low <= val <= high`.
/// - If the predicate is `false`, emits a diagnostic side-effect (stderr + tracing)
///   and still returns `false` (no panic, no short-circuit).
pub fn between(val: u64, low: u64, high: u64) -> bool {
    tracing::trace!(
        target: "bitcoinleveldb.dbtest",
        event = "dbtest.between.entry",
        val = val,
        low = low,
        high = high
    );

    let result: bool = (val >= low) && (val <= high);

    match result {
        true => {
            tracing::trace!(
                target: "bitcoinleveldb.dbtest",
                event = "dbtest.between.exit_ok",
                val = val,
                low = low,
                high = high
            );
        }
        false => {
            // Preserve the diagnostic side-effect.
            eprintln!("Value {} is not in range [{}, {}]", val, low, high);

            tracing::error!(
                target: "bitcoinleveldb.dbtest",
                event = "dbtest.between.out_of_range",
                val = val,
                low = low,
                high = high
            );
        }
    }

    result
}
