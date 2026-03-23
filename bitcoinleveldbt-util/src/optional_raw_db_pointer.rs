// ---------------- [ File: bitcoinleveldbt-util/src/optional_raw_db_pointer.rs ]
crate::ix!();

/// Invariant: returns `true` iff the optional raw DB pointer carries no live allocation.
pub trait DBTestOptionalRawDbPointerExt {
    /// Precondition: none.
    /// Postcondition: reports the closed/open state without touching ownership.
    fn is_null(&self) -> bool;
}

impl DBTestOptionalRawDbPointerExt for Option<*mut dyn DB> {
    #[inline]
    fn is_null(&self) -> bool {
        match self {
            Some(_) => false,
            None => true,
        }
    }
}
