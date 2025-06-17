// ---------------- [ File: bitcoin-mem/src/dynamic_usage_tests.rs ]
crate::ix!();

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

    // -------------------------------------------------------------------------
    // Primitive & pointer types
    // -------------------------------------------------------------------------
    #[traced_test]
    fn primitive_and_pointer_dynamic_usage_is_zero() {
        let v_i32   = 123_i32;
        let v_f64   = 3.14_f64;
        let p_mut   = &mut 7_i32 as *mut i32;
        let p_const = core::ptr::null::<i32>();

        info!("Testing DynamicUsage for primitive and raw pointer types");
        assert_eq!(v_i32.dynamic_usage(),   0);
        assert_eq!(v_f64.dynamic_usage(),   0);
        assert_eq!(p_mut.dynamic_usage(),   0);
        assert_eq!(p_const.dynamic_usage(), 0);
    }

    // -------------------------------------------------------------------------
    // malloc_usage helper
    // -------------------------------------------------------------------------
    #[traced_test]
    fn malloc_usage_rounding_rules() {
        let ptr_sz = size_of::<*const ()>();

        // Exact powers of two must round *up* to the allocator bucket.
        for bytes in [1_usize, 8, 16, 24, 32, 64, 128] {
            let rounded = malloc_usage(bytes);
            info!("bytes={} -> rounded={}", bytes, rounded);
            assert!(rounded >= bytes);

            // Alignment guarantees: 16‑byte buckets on 64‑bit, 8‑byte on 32‑bit.
            let align = if ptr_sz == 8 { 16 } else { 8 };
            assert_eq!(rounded % align, 0);
        }
    }

    // -------------------------------------------------------------------------
    // Vec<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn vec_dynamic_usage_matches_capacity_allocation() {
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

    // -------------------------------------------------------------------------
    // PreVector<T, N>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn prevector_inline_allocation_reports_zero() {
        let pv: PreVector<u32, 4> = PreVector::default();
        // Inline capacity == 4, so empty container should allocate nothing.
        assert_eq!(pv.dynamic_usage(), 0);
    }

    #[traced_test]
    fn prevector_heap_allocation_after_growth() {
        let mut pv: PreVector<u32, 4> = PreVector::default();
        // Push 5 items → forces heap growth.
        for i in 0..5 {
            pv.push(i);
        }

        let bytes    = pv.capacity() * size_of::<u32>();
        let expected = malloc_usage(bytes);
        info!(
            "PreVector after growth cap={} elem_size={} expected_bytes={}",
            pv.capacity(),
            size_of::<u32>(),
            expected
        );
        assert_eq!(pv.dynamic_usage(), expected);
    }

    // -------------------------------------------------------------------------
    // Box<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn box_dynamic_usage_exact() {
        let bx = Box::new(0xdead_beef_u64);
        let expected = malloc_usage(size_of::<u64>());
        assert_eq!(bx.dynamic_usage(), expected);
    }

    // -------------------------------------------------------------------------
    // Arc<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn arc_dynamic_usage_header_plus_payload() {
        let arc_val = Arc::new(1234_u16);
        let expected = malloc_usage(
            2 * size_of::<usize>() + size_of::<u16>()
        );
        assert_eq!(arc_val.dynamic_usage(), expected);
    }

    #[traced_test]
    fn arc_recursive_dynamic_usage_includes_inner() {
        let arc_dummy = Arc::new(Dummy::default());
        assert_eq!(
            recursive_dynamic_usage(&arc_dummy),
            arc_dummy.dynamic_usage() // Dummy contributes 0
        );
    }

    // -------------------------------------------------------------------------
    // Amo<T> (Arc<RwLock<Option<T>>>)
    // -------------------------------------------------------------------------
    #[traced_test]
    fn amo_recursive_dynamic_usage_none() {
        let amo: Amo<u8> = Amo::default();          // u8 now implements `RecursiveDynamicUsage`
        assert_eq!(
            recursive_dynamic_usage(&amo),
            amo.dynamic_usage()
        );
    }

    #[traced_test]
    fn amo_recursive_dynamic_usage_with_inner() {
        let amo: Amo<Vec<u32>> = Amo::default();

        // Store a small Vec (inline metadata only, zero capacity)
        {
            let mut guard = amo.getopt_mut();       // write access to `Option<Vec<u32>>`
            *guard = Some(Vec::<u32>::new());
        }

        let own   = amo.dynamic_usage();
        let total = recursive_dynamic_usage(&amo);  // inner vec has capacity 0
        assert_eq!(total, own);
    }

    // -------------------------------------------------------------------------
    // Option<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn option_recursive_dynamic_usage() {
        let opt_some = Some(Dummy::default());
        let opt_none: Option<Dummy> = None;

        assert_eq!(recursive_dynamic_usage(&opt_some), 0);
        assert_eq!(recursive_dynamic_usage(&opt_none), 0);
    }

    #[traced_test]
    fn hashset_incremental() {
        let mut s: HashSet<u32> = HashSet::new();
        let before = s.dynamic_usage();
        let inc    = s.incremental_dynamic_usage();

        s.insert(1);

        assert_eq!(s.dynamic_usage() - before, inc);
    }

    #[traced_test]
    fn hashmap_incremental() {
        let mut m: HashMap<u32, u32> = HashMap::new();
        let before = m.dynamic_usage();
        let inc    = m.incremental_dynamic_usage();

        m.insert(1, 2);

        assert_eq!(m.dynamic_usage() - before, inc);
    }
}
