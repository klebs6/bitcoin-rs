// ---------------- [ File: bitcoinleveldb-harness/src/slice_comparator_adapter.rs ]
crate::ix!();

/// Builds a `Box<dyn SliceComparator>` from the comparator currently installed in `Options`.
///
/// Invariant: the returned box is independent and may be moved into any constructor.
pub fn bitcoinleveldb_harness_slice_comparator_box_from_options(options: &Options) -> Box<dyn SliceComparator> {
    Box::new(BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter::new(
        options.comparator().clone(),
    ))
}

/// Adapter to bridge an `Arc<dyn Comparator>` into a `Box<dyn SliceComparator>` for constructors.
///
/// Invariant: `compare()` and `name()` are delegated exactly to the wrapped comparator.
#[derive(Clone)]
pub struct BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    comparator: Arc<dyn Comparator>,
}

impl BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    /// Constructs the adapter.
    ///
    /// Invariant: the adapter never mutates or reinterprets comparator behavior.
    pub fn new(comparator: Arc<dyn Comparator>) -> Self {
        Self { comparator }
    }
}

impl SliceComparator for BitcoinLevelDbHarnessArcComparatorSliceComparatorAdapter {
    fn name(&self) -> &str {
        self.comparator.name()
    }

    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        self.comparator.compare(a, b)
    }
}
