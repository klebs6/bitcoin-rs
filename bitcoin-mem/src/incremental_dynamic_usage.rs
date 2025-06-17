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
