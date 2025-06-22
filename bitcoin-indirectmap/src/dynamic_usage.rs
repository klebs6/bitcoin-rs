// ---------------- [ File: bitcoin-indirectmap/src/dynamic_usage.rs ]
crate::ix!();

impl<K: Ord, V> DynamicUsage for IndirectMap<K, V> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let entry_bytes = mem::size_of::<IndirectKey<K>>() + mem::size_of::<V>();
        let total       = entry_bytes * self.map().len();
        debug!(entries = self.map().len(), entry_bytes, total, "Computed dynamic usage for IndirectMap");
        total
    }
}

impl<K: Ord, V> IncrementalDynamicUsage for IndirectMap<K, V> {
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        mem::size_of::<IndirectKey<K>>() + mem::size_of::<V>()
    }
}
