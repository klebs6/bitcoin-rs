crate::ix!();

#[cfg(test)]
mod indirect_map_behaviour_tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct IntBox(i32);

    #[traced_test]
    fn insert_and_get_roundtrip() {
        let mut map = IndirectMap::<IntBox, &str>::default();
        let key42   = Arc::new(IntBox(42));

        assert!(map.insert(key42.clone(), "meaning of life"));
        assert!(!map.insert(key42.clone(), "overwritten"));

        let (arc_key, val) = map.find(&IntBox(42)).expect("key present");
        assert_eq!(arc_key.as_ref(), &IntBox(42));
        assert_eq!(*val, "overwritten");
    }

    #[traced_test]
    fn erase_and_count() {
        let mut map = IndirectMap::<IntBox, i32>::default();
        let keys = [1, 2, 3].into_iter().map(|n| Arc::new(IntBox(n))).collect::<Vec<_>>();

        for k in &keys {
            assert!(map.insert(k.clone(), k.0));
        }

        assert_eq!(map.size(), 3);
        assert_eq!(map.count(&IntBox(2)), 1);
        assert!(map.erase(&IntBox(2)));
        assert_eq!(map.count(&IntBox(2)), 0);
        assert_eq!(map.size(), 2);
    }

    #[traced_test]
    fn lower_bound_behaviour() {
        let mut map = IndirectMap::<IntBox, i32>::default();
        for &n in &[10, 20, 30] {
            assert!(map.insert(Arc::new(IntBox(n)), n));
        }

        let collected: Vec<_> = map.lower_bound(&IntBox(15)).map(|(_, v)| *v).collect();
        assert_eq!(collected, vec![20, 30]);
    }

    #[traced_test]
    fn dynamic_usage_is_positive() {
        let mut map = IndirectMap::<IntBox, [u8; 32]>::default();
        map.insert(Arc::new(IntBox(7)), [0u8; 32]);

        assert!(map.dynamic_usage() > 0);
        assert_eq!(map.dynamic_usage(), map.size() * map.incremental_dynamic_usage());
    }
}
