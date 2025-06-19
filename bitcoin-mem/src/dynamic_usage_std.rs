crate::ix!();

use core::hash::BuildHasher;
use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet};

// -----------------------------------------------------------------------------
// NEW – std‑collections variants
//   The `bitcoin_imports::*` re‑exports use the hashbrown back‑end, which is a
//   *different* type from `std::collections`.  Our tests construct the standard
//   library containers directly, so we must implement `DynamicUsage` for them
//   as well.
// -----------------------------------------------------------------------------

impl<T, S> DynamicUsage for StdHashSet<T, S>
where
    S: BuildHasher,
{
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<T>>();
        let bytes   = self.len() * node_sz;
        trace!(
            "DynamicUsage<std::collections::HashSet<{}>> len={} node_sz={} bytes={}",
            core::any::type_name::<T>(),
            self.len(),
            node_sz,
            bytes
        );
        malloc_usage(bytes)
    }
}

impl<K, V, S> DynamicUsage for StdHashMap<K, V, S>
where
    S: BuildHasher,
{
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<(K, V)>>();
        let bytes   = self.len() * node_sz;
        trace!(
            "DynamicUsage<std::collections::HashMap<{}, {}>> len={} node_sz={} bytes={}",
            core::any::type_name::<K>(),
            core::any::type_name::<V>(),
            self.len(),
            node_sz,
            bytes
        );
        malloc_usage(bytes)
    }
}

// -----------------------------------------------------------------------------
// NEW – incremental allocation cost for the *standard‑library* hash containers.
// -----------------------------------------------------------------------------

impl<T, S> IncrementalDynamicUsage for StdHashSet<T, S>
where
    S: BuildHasher,
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<T>>())
    }
}

impl<K, V, S> IncrementalDynamicUsage for StdHashMap<K, V, S>
where
    S: BuildHasher,
{
    #[inline]
    fn incremental_dynamic_usage(&self) -> usize {
        malloc_usage(core::mem::size_of::<StlTreeNode<(K, V)>>())
    }
}

