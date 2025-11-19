// ---------------- [ File: bitcoinleveldb-comparator/src/interface.rs ]
crate::ix!();

/// A trait for three-way comparison of slices.
///
/// Returns:
///  - < 0 if `a < b`
///  - 0 if `a == b`
///  - > 0 if `a > b`
pub trait Compare {
    fn compare(&self, a: &Slice, b: &Slice) -> i32;
}

/// A trait for modifying a key in `[start, limit)`
/// so that it's "shorter" yet still in that range.
///
/// In original C++ code, `start` and `limit` were
/// `std::string`. But that can hold arbitrary
/// bytes (including `0xFF`), whereas `String` in
/// Rust must be UTF-8. Therefore, we accept
/// `&mut Vec<u8>` and `&[u8]` to handle any bytes.
pub trait FindShortestSeparator {
    /// If *start < limit, changes *start to a short
    /// array in `[start, limit)`. May leave *start
    /// unchanged if no better separator is found.
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]);
}

/// A trait for adjusting a key to be a short
/// successor >= its current value, using the same
/// byte-based logic.
pub trait FindShortSuccessor {
    /// Changes *key to a short array >= *key.
    /// May leave *key unchanged if no better
    /// "successor" can be found.
    fn find_short_successor(&self, key: &mut Vec<u8>);
}

/// A unifying trait for a full "SliceComparator,"
/// combining the previous traits.
pub trait SliceComparator:
    Compare + Named + FindShortestSeparator + FindShortSuccessor
{
    /// Return a pointer to a built-in comparator
    /// that uses lexicographic byte-wise ordering.
    /// The pointer is valid for the program lifetime.
    fn bytewise_comparator(&self) -> *const dyn SliceComparator;
}
