// ---------------- [ File: bitcoinleveldb-harness/src/reverse_bytewise_comparator.rs ]
crate::ix!();

/// Creates a fresh `Arc` for the reverse comparator.
///
/// Invariant: the returned comparator's `name()` is exactly `"leveldb.ReverseBytewiseComparator"`.
pub fn bitcoinleveldb_harness_reverse_bytewise_comparator() -> Arc<dyn Comparator> {
    Arc::new(BitcoinLevelDbHarnessReverseBytewiseComparator::default())
}

/// Reverse comparator used by the original LevelDB C++ test harness.
///
/// Invariant: ordering is the lexicographic ordering of the *reversed* byte sequences
/// of the provided `Slice`s. The comparator name must remain stable because tests and
/// harness logic branch on it.
#[derive(Clone, Default)]
pub struct BitcoinLevelDbHarnessReverseBytewiseComparator {}

impl Comparator for BitcoinLevelDbHarnessReverseBytewiseComparator {
    fn name(&self) -> &str {
        "leveldb.ReverseBytewiseComparator"
    }

    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        let a_bytes: &[u8] = unsafe { slice::from_raw_parts(a.data(), a.size()) };
        let b_bytes: &[u8] = unsafe { slice::from_raw_parts(b.data(), b.size()) };

        let mut ai: usize = a_bytes.len();
        let mut bi: usize = b_bytes.len();

        while ai > 0 && bi > 0 {
            let av: u8 = a_bytes[ai - 1];
            let bv: u8 = b_bytes[bi - 1];

            if av < bv {
                return -1;
            }
            if av > bv {
                return 1;
            }

            ai -= 1;
            bi -= 1;
        }

        if ai == 0 && bi == 0 {
            0
        } else if ai == 0 {
            -1
        } else {
            1
        }
    }

    fn find_shortest_separator(&self, _start: *mut String, _limit: &Slice) {
        // Invariant: doing nothing preserves correctness; only affects potential space optimizations.
    }

    fn find_short_successor(&self, _key: *mut String) {
        // Invariant: doing nothing preserves correctness; only affects potential space optimizations.
    }
}
