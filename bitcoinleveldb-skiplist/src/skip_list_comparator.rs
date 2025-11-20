// ---------------- [ File: bitcoinleveldb-skiplist/src/skip_list_comparator.rs ]
crate::ix!();

/// Comparator used by `SkipList`.
///
/// This is intentionally very close to LevelDB's `Comparator` template
/// parameter: it compares two keys and returns `< 0`, `0`, or `> 0`.
pub trait SkipListComparator<K>: Send + Sync {
    fn compare(&self, a: &K, b: &K) -> i32;
}

impl<K, F> SkipListComparator<K> for F
where
    F: Fn(&K, &K) -> i32 + Send + Sync,
{
    #[inline]
    fn compare(&self, a: &K, b: &K) -> i32 {
        self(a, b)
    }
}

#[cfg(test)]
mod comparator_contract_suite {
    use super::*;
    use bitcoin_imports::*;

    #[traced_test]
    fn comparator_orders_values_correctly() {
        info!("comparator_orders_values_correctly: start");

        struct U64Cmp;
        impl SkipListComparator<u64> for U64Cmp {
            #[inline]
            fn compare(&self, a: &u64, b: &u64) -> i32 {
                if a < b { -1 } else if a > b { 1 } else { 0 }
            }
        }

        let cmp = U64Cmp;

        assert_eq!(cmp.compare(&1, &2), -1, "1 < 2 must yield -1");
        assert_eq!(cmp.compare(&5, &5),  0, "5 == 5 must yield 0");
        assert_eq!(cmp.compare(&9, &3),  1, "9 > 3 must yield 1");

        info!("comparator_orders_values_correctly: done");
    }

    #[traced_test]
    fn comparator_plugs_into_skiplist_equal() {
        info!("comparator_plugs_into_skiplist_equal: start");

        use bitcoinleveldb_arena::Arena;

        struct U64Cmp;
        impl SkipListComparator<u64> for U64Cmp {
            #[inline]
            fn compare(&self, a: &u64, b: &u64) -> i32 {
                if a < b { -1 } else if a > b { 1 } else { 0 }
            }
        }

        let mut arena = Arena::default();
        let list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        assert!(list.equal(&42, &42), "equal must reflect comparator equality");
        assert!(!list.equal(&1, &2),  "equal must be false for distinct values");

        info!("comparator_plugs_into_skiplist_equal: done");
    }

    #[traced_test]
    fn comparator_closure_works_and_is_thread_safe() {
        info!("comparator_closure_works_and_is_thread_safe: start");

        use bitcoinleveldb_arena::Arena;

        let cmp = |a: &u64, b: &u64| -> i32 {
            if a < b { -1 } else if a > b { 1 } else { 0 }
        };

        let mut arena = Arena::default();
        let list: SkipList<u64, _> = SkipList::new(cmp, &mut arena as *mut Arena);

        assert!(list.equal(&7, &7), "closure comparator must work in SkipList::equal");
        assert!(!list.equal(&7, &8), "closure comparator must distinguish values");

        info!("comparator_closure_works_and_is_thread_safe: done");
    }
}
