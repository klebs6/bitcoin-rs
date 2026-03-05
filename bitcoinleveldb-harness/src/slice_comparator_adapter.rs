// ---------------- [ File: bitcoinleveldb-harness/src/slice_comparator_adapter.rs ]
crate::ix!();

/// Builds a `Box<dyn SliceComparator>` from the comparator currently installed in `Options`.
///
/// Invariant: the returned box is independent and may be moved into any constructor.
pub fn bitcoinleveldb_harness_slice_comparator_box_from_options(options: &Options) -> Box<dyn SliceComparator> {
    let comparator_name: Cow<'_, str> = options.comparator().name();

    trace!(
        target: "bitcoinleveldb_harness",
        label = "bitcoinleveldb_harness.slice_comparator_adapter.box_from_options.entry",
        comparator_name = comparator_name.as_ref(),
    );

    let boxed: Box<dyn SliceComparator> = Box::new(BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter::new(
        options.comparator().clone(),
    ));

    trace!(
        target: "bitcoinleveldb_harness",
        label = "bitcoinleveldb_harness.slice_comparator_adapter.box_from_options.exit",
    );

    boxed
}

/// Adapter to bridge an `Arc<dyn SliceComparator>` into a `Box<dyn SliceComparator>` for constructors.
///
/// Invariant: all required comparator behaviors (`compare`, `name`, separator/successor transforms,
/// and access to the built-in bytewise comparator) are delegated exactly to the wrapped comparator.
#[derive(Clone)]
pub struct BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    comparator: Arc<dyn SliceComparator>,
}

impl BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    /// Constructs the adapter.
    ///
    /// Invariant: the adapter never mutates or reinterprets comparator behavior.
    pub fn new(comparator: Arc<dyn SliceComparator>) -> Self {
        let comparator_name: Cow<'_, str> = comparator.name();

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.slice_comparator_adapter.new",
            comparator_name = comparator_name.as_ref(),
        );

        Self { comparator }
    }
}

impl Compare for BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        self.comparator.compare(a, b)
    }
}

impl Named for BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    fn name(&self) -> Cow<'_, str> {
        self.comparator.name()
    }
}

impl FindShortestSeparator for BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        self.comparator.find_shortest_separator(start, limit)
    }
}

impl FindShortSuccessor for BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    fn find_short_successor(&self, key: &mut Vec<u8>) {
        self.comparator.find_short_successor(key)
    }
}

impl SliceComparator for BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        self.comparator.bytewise_comparator()
    }
}
