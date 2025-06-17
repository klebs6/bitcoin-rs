// ---------------- [ File: bitcoin-mem/src/dynamic_usage.rs ]
crate::ix!();

pub trait DynamicUsage {
    fn dynamic_usage(&self) -> usize;
}

pub trait IncrementalDynamicUsage {
    fn incremental_dynamic_usage(&self) -> usize;
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

// STL data structures

pub struct StlTreeNode<X> {
    color:  i32,
    parent: *mut c_void,
    left:   *mut c_void,
    right:  *mut c_void,
    x:      X,
}

pub struct StlSharedCounter
{
    /**
      | Various platforms use different sized
      | counters here.
      | 
      | Conservatively assume that they won't
      | be larger than size_t.
      |
      */
    class_type: *mut c_void,

    use_count:  usize,
    weak_count: usize,
}

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
        let bytes = self.allocated_memory();
        trace!(
            "DynamicUsage<PreVector<{}, {}>> bytes={}",
            core::any::type_name::<T>(),
            N,
            bytes
        );
        malloc_usage(bytes)
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

pub struct UnorderedNode<X> {
    base: X,
    ptr:  *mut c_void,
}

lazy_static!{
    /*
        impl<X, Y> DynamicUsage for HashSet<X,Y> {

            #[inline] fn dynamic_usage(&self) -> usize {

                todo!();
                    /*
                        return MallocUsage(sizeof(unordered_node<X>)) * s.size() + MallocUsage(sizeof(c_void*) * s.bucket_count());
                    */
            }
        }

        impl<X, Y, Z> DynamicUsage for HashMap<X,Y,Z> {

            #[inline] fn dynamic_usage(&self) -> usize {

                todo!();
                    /*
                        return MallocUsage(sizeof(unordered_node<std::pair<const X, Y> >)) * m.size() + MallocUsage(sizeof(c_void*) * m.bucket_count());
                    */
            }
        }
    */
}

#[cfg(test)]
mod memory_usage_tests {
    use super::*;

    /// Dummy value that consumes no additional dynamic memory.
    #[derive(Default)]
    struct Dummy;

    impl DynamicUsage for Dummy {
        #[inline]
        fn dynamic_usage(&self) -> usize {
            trace!("DynamicUsage<Dummy> -> 0");
            0
        }
    }

    impl RecursiveDynamicUsage for Dummy {
        fn recursive_dynamic_usage(&self) -> usize {
            trace!("RecursiveDynamicUsage<Dummy> -> 0");
            0
        }
    }

    // ------------------------------------------------------------
    // ‑‑ Primitive and pointer types
    // ------------------------------------------------------------

    #[traced_test]
    fn primitive_dynamic_usage_is_zero() {
        let int_val = 123_i32;
        let float_val = 3.14_f64;
        let ptr: *const i32 = core::ptr::null();

        info!("Testing DynamicUsage for primitive and pointer types");
        assert_eq!(int_val.dynamic_usage(), 0);
        assert_eq!(float_val.dynamic_usage(), 0);
        assert_eq!(ptr.dynamic_usage(), 0);
    }

    // ------------------------------------------------------------
    // ‑‑ Vec<T>
    // ------------------------------------------------------------

    #[traced_test]
    fn vec_dynamic_usage_matches_malloc_usage() {
        const CAP: usize = 10;
        let vec: Vec<u64> = Vec::with_capacity(CAP);

        let expected = malloc_usage(CAP * size_of::<u64>());
        info!(
            "Vec capacity={}, element_size={}, expected_bytes={}",
            CAP,
            size_of::<u64>(),
            expected
        );
        assert_eq!(vec.dynamic_usage(), expected);
    }

    // ------------------------------------------------------------
    // ‑‑ Box<T>
    // ------------------------------------------------------------

    #[traced_test]
    fn box_dynamic_usage_is_exact() {
        let bx = Box::new(42_u32);
        let expected = malloc_usage(size_of::<u32>());

        info!(
            "Box payload_size={}, expected_bytes={}",
            size_of::<u32>(),
            expected
        );
        assert_eq!(bx.dynamic_usage(), expected);
    }

    // ------------------------------------------------------------
    // ‑‑ Arc<T>
    // ------------------------------------------------------------

    #[traced_test]
    fn arc_dynamic_usage_includes_header_and_payload() {
        let arc_val = Arc::new(7_u64);
        let header_bytes = 2 * size_of::<usize>();
        let total_bytes = malloc_usage(header_bytes + size_of::<u64>());

        info!(
            "Arc header_bytes={}, payload_size={}, total_expected={}",
            header_bytes,
            size_of::<u64>(),
            total_bytes
        );
        assert_eq!(arc_val.dynamic_usage(), total_bytes);
    }

    #[traced_test]
    fn arc_recursive_dynamic_usage_adds_inner() {
        let arc_dummy = Arc::new(Dummy::default());

        let own = arc_dummy.dynamic_usage();
        let total = recursive_dynamic_usage(&arc_dummy);

        info!("Arc own={}, recursive_total={}", own, total);
        assert_eq!(own, total); // Dummy has zero inner usage
    }

    // ------------------------------------------------------------
    // ‑‑ HashSet<T>
    // ------------------------------------------------------------

    #[traced_test]
    fn hashset_incremental_dynamic_usage_matches_growth() {
        let mut set: HashSet<u32> = HashSet::new();
        let before = set.dynamic_usage();
        let inc = set.incremental_dynamic_usage();

        set.insert(1);
        let after = set.dynamic_usage();

        info!(
            "HashSet node_inc={}, before={}, after={}, delta={}",
            inc,
            before,
            after,
            after - before
        );
        assert_eq!(after - before, inc);
    }

    // ------------------------------------------------------------
    // ‑‑ HashMap<K, V>
    // ------------------------------------------------------------

    #[traced_test]
    fn hashmap_incremental_dynamic_usage_matches_growth() {
        let mut map: HashMap<u32, u64> = HashMap::new();
        let before = map.dynamic_usage();
        let inc = map.incremental_dynamic_usage();

        map.insert(1, 100);
        let after = map.dynamic_usage();

        info!(
            "HashMap node_inc={}, before={}, after={}, delta={}",
            inc,
            before,
            after,
            after - before
        );
        assert_eq!(after - before, inc);
    }
}
