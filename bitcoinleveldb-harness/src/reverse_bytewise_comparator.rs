// ---------------- [ File: bitcoinleveldb-harness/src/reverse_bytewise_comparator.rs ]
crate::ix!();

/// Creates a fresh `Arc` for the reverse comparator.
///
/// Invariant: the returned comparator's `name()` is exactly `"leveldb.ReverseBytewiseComparator"`.
pub fn bitcoinleveldb_harness_reverse_bytewise_comparator() -> Arc<dyn SliceComparator> {
    trace!(
        target: "bitcoinleveldb_harness",
        label = "bitcoinleveldb_harness.reverse_bytewise_comparator.arc.create",
    );

    Arc::new(BitcoinLevelDbHarnessReverseBytewiseComparator::default())
}

/// Reverse comparator used by the original LevelDB C++ test harness.
///
/// Invariant: ordering is the lexicographic ordering of the *reversed* byte sequences
/// of the provided `Slice`s. The comparator name must remain stable because tests and
/// harness logic branch on it.
#[derive(Clone, Default)]
pub struct BitcoinLevelDbHarnessReverseBytewiseComparator {}

impl Named for BitcoinLevelDbHarnessReverseBytewiseComparator {
    fn name(&self) -> Cow<'_, str> {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.reverse_bytewise_comparator.name",
        );

        Cow::Borrowed("leveldb.ReverseBytewiseComparator")
    }
}

impl Compare for BitcoinLevelDbHarnessReverseBytewiseComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.reverse_bytewise_comparator.compare.entry",
            a_len = (*a.size()),
            b_len = (*b.size()),
        );

        let a_bytes: &[u8] = unsafe { slice::from_raw_parts(*a.data(), *a.size()) };
        let b_bytes: &[u8] = unsafe { slice::from_raw_parts(*b.data(), *b.size()) };

        let mut ai: usize = a_bytes.len();
        let mut bi: usize = b_bytes.len();

        let result: i32 = loop {
            if ai == 0 || bi == 0 {
                break if ai == 0 && bi == 0 {
                    0
                } else if ai == 0 {
                    -1
                } else {
                    1
                };
            }

            let av: u8 = a_bytes[ai - 1];
            let bv: u8 = b_bytes[bi - 1];

            if av < bv {
                break -1;
            }
            if av > bv {
                break 1;
            }

            ai -= 1;
            bi -= 1;
        };

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.reverse_bytewise_comparator.compare.exit",
            result = result,
        );

        result
    }
}

impl FindShortestSeparator for BitcoinLevelDbHarnessReverseBytewiseComparator {
    fn find_shortest_separator(&self, _start: &mut Vec<u8>, _limit: &[u8]) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.reverse_bytewise_comparator.find_shortest_separator.noop",
        );

        // Invariant: doing nothing preserves correctness; only affects potential space optimizations.
    }
}

impl FindShortSuccessor for BitcoinLevelDbHarnessReverseBytewiseComparator {
    fn find_short_successor(&self, _key: &mut Vec<u8>) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.reverse_bytewise_comparator.find_short_successor.noop",
        );

        // Invariant: doing nothing preserves correctness; only affects potential space optimizations.
    }
}

impl SliceComparator for BitcoinLevelDbHarnessReverseBytewiseComparator {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.reverse_bytewise_comparator.bytewise_comparator",
        );

        bytewise_comparator()
    }
}
