crate::ix!();

// -----------------------------------------------------------------------------
// UPDATED impl DynamicUsage for Amo<T>
// -----------------------------------------------------------------------------
impl<T> DynamicUsage for Amo<T> {
    #[inline]
    fn dynamic_usage(&self) -> usize {
        // Amo<T> is `Arc<RwLock<Option<T>>>`
        let header_bytes = 2 * core::mem::size_of::<usize>();                // Arc header
        let payload      = core::mem::size_of::<parking_lot::RwLock<Option<T>>>(); // lock + option
        let total        = header_bytes + payload;

        trace!(
            "DynamicUsage<Amo<{}>> header={} payload={} total={}",
            core::any::type_name::<T>(),
            header_bytes,
            payload,
            total
        );
        malloc_usage(total)
    }
}

// -----------------------------------------------------------------------------
// UPDATED impl RecursiveDynamicUsage for Amo<X>
// -----------------------------------------------------------------------------
impl<X> RecursiveDynamicUsage for Amo<X>
where
    X: RecursiveDynamicUsage + DynamicUsage,
{
    fn recursive_dynamic_usage(&self) -> usize {
        let own = DynamicUsage::dynamic_usage(self);

        let inner = {
            // work with the outer `Option<X>`
            let guard = self.getopt();                    // &Option<X>
            guard
                .as_ref()
                .map(recursive_dynamic_usage)             // recurse if Some
                .unwrap_or(0)
        };

        trace!(
            "RecursiveDynamicUsage<Amo<{}>> own={} inner={} total={}",
            core::any::type_name::<X>(),
            own,
            inner,
            own + inner
        );
        own + inner
    }
}

#[cfg(test)]
mod memory_usage_tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;
    use tracing::{info, trace};
    use core::mem::size_of;

    // -------------------------------------------------------------------------
    // Helper types
    // -------------------------------------------------------------------------
    #[derive(Default)]
    struct Dummy;

    impl DynamicUsage for Dummy {
        #[inline]
        fn dynamic_usage(&self) -> usize { 0 }
    }
    impl RecursiveDynamicUsage for Dummy {
        fn recursive_dynamic_usage(&self) -> usize { 0 }
    }

    // -------------------------------------------------------------------------
    // Primitive & pointer
    // -------------------------------------------------------------------------
    #[traced_test]
    fn primitive_and_pointer_usage_is_zero() {
        assert_eq!(123_i32.dynamic_usage(), 0);
        assert_eq!(3.14_f64.dynamic_usage(), 0);
        let p: *const i32 = core::ptr::null();
        assert_eq!(p.dynamic_usage(), 0);
    }

    // -------------------------------------------------------------------------
    // malloc_usage rounding
    // -------------------------------------------------------------------------
    #[traced_test]
    fn malloc_usage_alignment() {
        let align = if size_of::<*const ()>() == 8 { 16 } else { 8 };
        for bytes in [1, 8, 15, 32, 63, 128] {
            let r = malloc_usage(bytes);
            assert!(r >= bytes && r % align == 0);
        }
    }

    // -------------------------------------------------------------------------
    // Vec<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn vec_usage_by_capacity() {
        const CAP: usize = 10;
        let v: Vec<u64> = Vec::with_capacity(CAP);
        let expect = malloc_usage(CAP * size_of::<u64>());
        assert_eq!(v.dynamic_usage(), expect);
    }

    // -------------------------------------------------------------------------
    // Box<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn box_usage_exact() {
        let bx = Box::new(42_u32);
        assert_eq!(bx.dynamic_usage(), malloc_usage(size_of::<u32>()));
    }

    // -------------------------------------------------------------------------
    // Arc<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn arc_usage_header_plus_payload() {
        let a = Arc::new(7_u64);
        let expect = malloc_usage(2 * size_of::<usize>() + size_of::<u64>());
        assert_eq!(a.dynamic_usage(), expect);
    }

    // -------------------------------------------------------------------------
    // Amo<T>
    // -------------------------------------------------------------------------
    #[traced_test]
    fn amo_usage_and_recursive_none() {
        let amo: Amo<Dummy> = Amo::default();
        assert_eq!(
            recursive_dynamic_usage(&amo),
            amo.dynamic_usage()
        );
    }

    #[traced_test]
    fn amo_usage_and_recursive_with_inner() {
        let amo: Amo<Arc<Dummy>> = Amo::default();

        // store a value
        {
            let mut g = amo.getopt_mut();
            *g = Some(Arc::new(Dummy::default()));
        }

        let own   = amo.dynamic_usage();
        let inner = {
            // Arc header + Dummy (0)
            malloc_usage(2 * size_of::<usize>() + size_of::<Dummy>())
        };
        assert_eq!(recursive_dynamic_usage(&amo), own + inner);
    }

    // -------------------------------------------------------------------------
    // HashSet / HashMap incremental
    // -------------------------------------------------------------------------
    #[traced_test]
    fn hashset_incremental() {
        let mut s: Arc<HashSet<u32>> = Arc::new(HashSet::new());
        let before = s.dynamic_usage();
        let inc    = s.incremental_dynamic_usage();
        s.insert(1);
        assert_eq!(s.dynamic_usage() - before, inc);
    }

    #[traced_test]
    fn hashmap_incremental() {
        let mut m: Arc<HashMap<u32, u32>> = Arc::new(HashMap::new());
        let before = m.dynamic_usage();
        let inc    = m.incremental_dynamic_usage();
        m.insert(1, 2);
        assert_eq!(m.dynamic_usage() - before, inc);
    }
}
