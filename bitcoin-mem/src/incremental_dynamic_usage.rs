// ---------------- [ File: bitcoin-mem/src/incremental_dynamic_usage.rs ]
crate::ix!();

pub trait IncrementalDynamicUsage {
    fn incremental_dynamic_usage(&self) -> usize;
}

impl<X, Y> IncrementalDynamicUsage for HashSet<X, Y> {
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        let inc = malloc_usage(core::mem::size_of::<StlTreeNode<X>>());
        trace!(
            "IncrementalDynamicUsage<HashSet<{}>> inc_bytes={}",
            core::any::type_name::<X>(),
            inc
        );
        inc
    }
}

impl<X, Y, Z> IncrementalDynamicUsage for HashMap<X, Y, Z> {
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        let inc = malloc_usage(core::mem::size_of::<StlTreeNode<(X, Y)>>());
        trace!(
            "IncrementalDynamicUsage<HashMap<{}, {}>> inc_bytes={}",
            core::any::type_name::<X>(),
            core::any::type_name::<Y>(),
            inc
        );
        inc
    }
}

impl<T> IncrementalDynamicUsage for Arc<T>
where
    T: IncrementalDynamicUsage,
{
    /// Forward the request to the inner value.  
    /// An `Arc` itself never reallocates on `clone`, so the
    /// incremental cost is the same as that of the wrapped
    /// structure.
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        T::incremental_dynamic_usage(&**self)
    }
}

