// ---------------- [ File: bitcoin-indirectmap/src/indirect_map.rs ]
crate::ix!();

/// Map whose keys are `Arc<K>` pointers but are **compared
/// by the dereferenced `K` value**.
///
/// Invariants:
/// * `K: Ord` so the dereferenced value has a total ordering.
/// * Once inserted, a `K` **must not** mutate in a way that
///   changes its ordering.
#[derive(Builder, Getters, MutGetters)]
#[builder(pattern = "owned")]
pub struct IndirectMap<K, V>
where
    K: Ord,
{
    /// Internal storage ordered on the *dereferenced* key.
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    #[builder(default)]
    map: BTreeMap<IndirectKey<K>, V>,
}

impl<K: Ord, V> Default for IndirectMap<K, V> {
    #[inline]
    fn default() -> Self {
        trace!("Creating default `IndirectMap`");
        Self {
            map: BTreeMap::default(),
        }
    }
}

impl<K: Ord, V> IndirectMap<K, V> {

    /// Insert a new `(Arc<K>, V)` pair.  
    /// Returns `true` if the key was newly inserted.
    #[inline]
    pub fn insert(&mut self, key: Arc<K>, value: V) -> bool {
        let inserted = self.map.insert(IndirectKey::new(key), value).is_none();
        info!(inserted, "Insert into IndirectMap");
        inserted
    }

    /// Immutable lookup by **value** of `K`.
    #[inline]
    pub fn find(&self, k: &K) -> Option<(&Arc<K>, &V)> {
        self.map.get_key_value(k).map(|(key, val)| (key.arc(), val))
    }

    /// Mutable lookup by **value** of `K`.
    #[inline]
    pub fn find_mut(&mut self, k: &K) -> Option<(&Arc<K>, &mut V)> {
        self.map.get_mut(k).map(|val| (self.map.get_key_value(k).unwrap().0.arc(), val))
    }

    /// Maximum theoretical size (bounded by `usize::MAX`).
    #[inline]
    pub fn max_size(&self) -> usize {
        usize::MAX
    }

    /// Immutable lookup by **value** of `K`.
    #[inline]
    pub fn get(&self, k: &K) -> Option<(&Arc<K>, &V)> {
        self.map.get(k).map(|v| (self.map.get_key_value(k).unwrap().0.arc(), v))
    }

    /// Mutable lookup by **value** of `K`.
    #[inline]
    pub fn get_mut(&mut self, k: &K) -> Option<(&Arc<K>, &mut V)> {
        self.map
            .get_mut(k)
            .map(|v| (self.map.get_key_value(k).unwrap().0.arc(), v))
    }

    /// Remove by **value** of `K`.  Returns `true` if something was removed.
    #[inline]
    pub fn erase(&mut self, k: &K) -> bool {
        let removed = self.map.remove(k).is_some();
        debug!(removed, "Erase from IndirectMap");
        removed
    }

    /// Number of entries equal to `k` (0 or 1 in a map).
    #[inline]
    pub fn count(&self, k: &K) -> usize {
        usize::from(self.map.contains_key(k))
    }

    /// Return an iterator pointing at the first element
    /// *not* less than `k`.  Equivalent to C++ `lower_bound`.
    #[inline]
    pub fn lower_bound<'a>(&'a self, k: &K) -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a> {
        Box::new(
            self.map
                .range::<K, _>(k..)
                .map(|(indirect, v)| (indirect.arc(), v)),
        )
    }

    /* ---------- STL‑like passthroughs ---------- */

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        info!("Clearing IndirectMap ({} entries)", self.size());
        self.map.clear();
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a V)> + 'a> {
        Box::new(self.map.iter().map(|(k, v)| (k.arc(), v)))
    }

    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = (&'a Arc<K>, &'a mut V)> + 'a> {
        Box::new(self.map.iter_mut().map(|(k, v)| (k.arc(), v)))
    }
}

#[cfg(test)]
mod indirect_map_additional_method_tests {
    use super::*;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct TestKey(i32);

    #[traced_test]
    fn max_size_is_max_usize() {
        let map: IndirectMap<TestKey, u8> = IndirectMap::default();
        assert_eq!(map.max_size(), usize::MAX);
    }

    #[traced_test]
    fn find_exact_match() {
        let mut map = IndirectMap::<TestKey, &str>::default();
        let key = Arc::new(TestKey(100));
        map.insert(key.clone(), "value_100");

        let found = map.find(&TestKey(100));
        assert!(found.is_some());
        let (arc_key, val) = found.unwrap();
        assert_eq!(arc_key.as_ref(), &TestKey(100));
        assert_eq!(*val, "value_100");

        let not_found = map.find(&TestKey(200));
        assert!(not_found.is_none());
    }

    #[traced_test]
    fn find_mut_exact_match() {
        let mut map = IndirectMap::<TestKey, i32>::default();
        let key = Arc::new(TestKey(42));
        map.insert(key.clone(), 500);

        {
            let found_mut = map.find_mut(&TestKey(42));
            assert!(found_mut.is_some());
            let (_arc_key, val_mut) = found_mut.unwrap();
            *val_mut = 1000;
        }

        let (_arc_key, val) = map.find(&TestKey(42)).unwrap();
        assert_eq!(*val, 1000);
    }
}
