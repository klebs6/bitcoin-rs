// ---------------- [ File: bitcoin-mem/src/incremental_dynamic_usage.rs ]
crate::ix!();

/// Trait for computing the additional heap allocation that **one more**
/// element would cause inside a container (mirrors the C++ memusage logic).
pub trait IncrementalDynamicUsage {
    fn incremental_dynamic_usage(&self) -> usize;
}

impl<K, S> IncrementalDynamicUsage for HashSet<K, S>
where
    S: BuildHasher,
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<K>>())
    }
}

impl<K, V, S> IncrementalDynamicUsage for HashMap<K, V, S>
where
    S: BuildHasher,
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<(K, V)>>())
    }
}

impl<T> IncrementalDynamicUsage for Arc<T>
where
    T: IncrementalDynamicUsage + ?Sized,
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        (**self).incremental_dynamic_usage()
    }
}
