// ---------------- [ File: bitcoinleveldb-comparator/src/comparator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/comparator.h]

/// A bytewise comparator that sorts slices by
/// lexicographic ordering (the default for
/// LevelDB).
#[derive(Debug)]
pub struct BytewiseComparatorImpl {
    // No fields needed; logic is purely functional.
}

impl Default for BytewiseComparatorImpl {
    fn default() -> Self {
        info!("Creating BytewiseComparatorImpl by default");
        Self {}
    }
}

impl SliceComparator for BytewiseComparatorImpl {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        trace!("Returning global bytewise_comparator pointer");
        bytewise_comparator()
    }
}

// -------------------------------------
// Name trait implementation
// -------------------------------------
impl Named for BytewiseComparatorImpl {
    fn name(&self) -> std::borrow::Cow<'_,str> {
        info!("Returning the name of BytewiseComparatorImpl");
        std::borrow::Cow::Borrowed("leveldb.BytewiseComparator")
    }
}

// -------------------------------------
// Compare trait implementation
// -------------------------------------
impl Compare for BytewiseComparatorImpl {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        trace!("BytewiseComparatorImpl::compare invoked");
        let cmp = a.compare(b);
        trace!("compare result: {}", cmp);
        cmp
    }
}

/// Return a pointer to a global, bytewise comparator.
/// This replicates the C++ pattern of `static NoDestructor<BytewiseComparatorImpl>`.
pub fn bytewise_comparator() -> *const dyn SliceComparator {

    static BYTEWISE_COMPARATOR: OnceLock<BytewiseComparatorImpl> = OnceLock::new();

    trace!("bytewise_comparator() invoked");
    let reference = BYTEWISE_COMPARATOR.get_or_init(|| {
        info!("Initializing BytewiseComparatorImpl singleton");
        BytewiseComparatorImpl::default()
    });
    // We cast &BytewiseComparatorImpl -> *const dyn SliceComparator
    reference as *const BytewiseComparatorImpl as *const dyn SliceComparator
}
