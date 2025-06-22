// ---------------- [ File: bitcoin-indirectmap/src/indirect_key.rs ]
crate::ix!();

/// Key wrapper that orders **by the dereferenced value** of the `Arc`.
#[derive(Clone)]
pub struct IndirectKey<K: Ord>(Arc<K>);

impl<K: Ord> IndirectKey<K> {
    /// Construct a new wrapper around an `Arc<K>`.
    #[inline]
    pub(crate) fn new(key: Arc<K>) -> Self {
        trace!("Creating new `IndirectKey`");
        Self(key)
    }

    /// Borrow the inner `Arc` (needed by the map façade).
    #[inline]
    pub(crate) fn arc(&self) -> &Arc<K> {
        &self.0
    }
}

impl<K: Ord> Borrow<K> for IndirectKey<K> {
    #[inline] fn borrow(&self) -> &K { &*self.0 }
}

impl<K: Ord> PartialEq for IndirectKey<K> { #[inline] fn eq(&self, o: &Self) -> bool { *self.0 == *o.0 } }
impl<K: Ord> Eq for IndirectKey<K> {}
impl<K: Ord> PartialOrd for IndirectKey<K> { #[inline] fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }
impl<K: Ord> Ord for IndirectKey<K> { #[inline] fn cmp(&self, o: &Self) -> Ordering { (*self.0).cmp(&*o.0) } }
