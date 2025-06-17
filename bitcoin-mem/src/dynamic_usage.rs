// ---------------- [ File: bitcoin-mem/src/dynamic_usage.rs ]
crate::ix!();

pub trait DynamicUsage {
    fn dynamic_usage(&self) -> usize;
}

impl DynamicUsage for i8  { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<i8>"); 0 } }
impl DynamicUsage for u8  { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<u8>"); 0 } }
impl DynamicUsage for i16 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<i16>"); 0 } }
impl DynamicUsage for u16 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<u16>"); 0 } }
impl DynamicUsage for i32 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<i32>"); 0 } }
impl DynamicUsage for u32 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<u32>"); 0 } }
impl DynamicUsage for i64 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<i64>"); 0 } }
impl DynamicUsage for u64 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<u64>"); 0 } }
impl DynamicUsage for f32 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<f32>"); 0 } }
impl DynamicUsage for f64 { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<f64>"); 0 } }

impl<X> DynamicUsage for *mut X  { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<*mut _>"); 0 } }
impl<X> DynamicUsage for *const X { #[inline] fn dynamic_usage(&self) -> usize { trace!("DynamicUsage<*const _>"); 0 } }

impl<X> DynamicUsage for Vec<X> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let bytes = self.capacity() * core::mem::size_of::<X>();
        trace!(
            "DynamicUsage<Vec<{}>> capacity={} bytes={}",
            core::any::type_name::<X>(),
            self.capacity(),
            bytes
        );
        malloc_usage(bytes)
    }
}

impl<T: Default, const N: usize> DynamicUsage for PreVector<T, N> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        // ❶ `PreVector` keeps up to `N` elements inline; anything beyond that is heap‑allocated.
        let inline_cap = N;
        let heap_cap   = self.capacity();

        if heap_cap <= inline_cap {
            trace!(
                "DynamicUsage<PreVector<{}, {}>> inline_cap={} ‑‑ no heap allocation",
                core::any::type_name::<T>(),
                N,
                inline_cap
            );
            0
        } else {
            let bytes = heap_cap * core::mem::size_of::<T>();
            trace!(
                "DynamicUsage<PreVector<{}, {}>> heap_cap={} elem_size={} bytes={}",
                core::any::type_name::<T>(),
                N,
                heap_cap,
                core::mem::size_of::<T>(),
                bytes
            );
            malloc_usage(bytes)
        }
    }
}

impl<X, Y> DynamicUsage for HashSet<X, Y> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<X>>();
        let bytes = self.len() * node_sz;
        trace!(
            "DynamicUsage<HashSet<{}>> len={} node_sz={} bytes={}",
            core::any::type_name::<X>(),
            self.len(),
            node_sz,
            bytes
        );
        malloc_usage(bytes)
    }
}

impl<X, Y, Z> DynamicUsage for HashMap<X, Y, Z> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let node_sz = core::mem::size_of::<StlTreeNode<(X, Y)>>();
        let bytes = self.len() * node_sz;
        trace!(
            "DynamicUsage<HashMap<{}, {}>> len={} node_sz={} bytes={}",
            core::any::type_name::<X>(),
            core::any::type_name::<Y>(),
            self.len(),
            node_sz,
            bytes
        );
        malloc_usage(bytes)
    }
}

impl<X> DynamicUsage for Box<X> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        let bytes = core::mem::size_of::<X>();
        trace!("DynamicUsage<Box<{}>> bytes={}", core::any::type_name::<X>(), bytes);
        malloc_usage(bytes)
    }
}

impl<X> DynamicUsage for Arc<X> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        // Arc allocates a single block containing a header (2×usize) and the payload.
        let header_bytes = 2 * core::mem::size_of::<usize>();
        let total = header_bytes + core::mem::size_of::<X>();
        trace!(
            "DynamicUsage<Arc<{}>> header={} payload={} total={}",
            core::any::type_name::<X>(),
            header_bytes,
            core::mem::size_of::<X>(),
            total
        );
        malloc_usage(total)
    }
}

// -----------------------------------------------------------------------------
// COMPLETELY NEW impl: DynamicUsage for `Option<T>`
// (an `Option` itself never allocates).
// -----------------------------------------------------------------------------
impl<T> DynamicUsage for Option<T> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        trace!("DynamicUsage<Option<{}>> -> 0", core::any::type_name::<T>());
        0
    }
}

pub struct UnorderedNode<X> {
    base: X,
    ptr:  *mut c_void,
}
