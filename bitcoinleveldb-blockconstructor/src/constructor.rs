// ---------------- [ File: bitcoinleveldb-blockconstructor/src/constructor.rs ]
crate::ix!();

/// Alias used by the test harness to represent
/// the key/value map.
/// 
pub type KVMap = std::collections::HashMap<String, String>;

/// Helper class for tests to unify the interface
/// between BlockBuilder/TableBuilder
/// and Block/Table.
/// 
pub struct Constructor {
    data: KVMap,
}

pub trait ConstructorInterface: ConstructorFinishImpl + ConstructorNewIterator {}

pub trait ConstructorFinishImpl {

    /// Construct the data structure from the
    /// data in "data"
    /// 
    fn finish_impl(
        &mut self, 
        options: &crate::Options,
        data:    &KVMap
    ) -> crate::Status;
}

pub trait ConstructorNewIterator {

    fn new_iterator(&self) -> *mut LevelDBIterator;
}

impl Constructor {

    pub fn new(_cmp: Box<dyn SliceComparator>) -> Self {
        trace!("Constructor::new: initializing KVMap");
        Constructor {
            data: KVMap::default(),
        }
    }

    /// Convenience constructor when the comparator
    /// value is irrelevant to local bookkeeping.
    /// 
    pub fn with_default() -> Self {
        trace!("Constructor::with_default: initializing KVMap with default settings");
        Constructor {
            data: KVMap::default(),
        }
    }
    
    pub fn add(&mut self, key_: &String, value: &Slice) {
        unsafe {
            let ptr: *const u8 = *value.data();
            let len: usize     = *value.size();

            let bytes: &[u8] = core::slice::from_raw_parts(ptr, len);

            // LevelDB’s tests treat these as byte strings; here we go through UTF‑8
            // since the original KVMap uses std::string.
            let s = String::from_utf8_lossy(bytes).to_string();

            trace!(
                "Constructor::add: key_len={}, value_len={}",
                key_.len(),
                s.len()
            );

            self.data.insert(key_.clone(), s);
        }
    }

    /// Finish constructing the data structure with
    /// all the keys that have been added so far.
    ///
    /// Returns the keys in sorted order in "*keys"
    /// and stores the key/value pairs in "*kvmap"
    ///
    pub fn finish(
        &mut self,
        _options: &Options,
        keys:     *mut Vec<String>,
        kvmap:    *mut KVMap,
    ) {
        unsafe {
            assert!(
                !keys.is_null(),
                "Constructor::finish: keys pointer is null"
            );
            assert!(
                !kvmap.is_null(),
                "Constructor::finish: kvmap pointer is null"
            );

            trace!(
                "Constructor::finish: finalizing; local_entries={}",
                self.data.len()
            );

            // Copy data out for the caller.
            *kvmap = self.data.clone();

            // Prepare the caller's keys vector.
            let keys_vec: &mut Vec<String> = &mut *keys;
            keys_vec.clear();

            // Collect keys from the internal map and sort them to match
            // LevelDB's std::map-based test harness semantics.
            let mut collected: Vec<String> = self
                .data
                .keys()
                .cloned()
                .collect();

            collected.sort();

            trace!(
                "Constructor::finish: collected {} keys (sorted)",
                collected.len()
            );

            keys_vec.extend(collected.into_iter());

            // Clear internal map; from here on, the caller owns kvmap.
            self.data.clear();
        }
    }


    pub fn data(&self) -> &KVMap {
        &self.data
    }
}

#[cfg(test)]
mod constructor_kvmap_and_finish_tests {
    use super::*;

    #[traced_test]
    fn constructor_add_populates_internal_map() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default());
        let mut ctor = Constructor::new(cmp_box);

        let value_slice = Slice::from("value-1".as_bytes());
        let key = "k1".to_string();

        trace!("calling Constructor::add");
        ctor.add(&key, &value_slice);

        let data = ctor.data();
        debug!("Constructor::data size after single add = {}", data.len());
        assert_eq!(data.len(), 1);
        assert_eq!(data.get("k1").unwrap(), "value-1");
    }

    #[traced_test]
    fn constructor_finish_clones_map_and_collects_keys() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default());
        let mut ctor = Constructor::new(cmp_box);

        let v1 = Slice::from("value1".as_bytes());
        let v2 = Slice::from("value2".as_bytes());

        ctor.add(&"b".to_string(), &v1);
        ctor.add(&"a".to_string(), &v2);

        let mut keys  = Vec::<String>::new();
        let mut kvmap = KVMap::default();
        let options   = Options::default();

        trace!("calling Constructor::finish");
        ctor.finish(
            &options,
            &mut keys as *mut Vec<String>,
            &mut kvmap as *mut KVMap,
        );

        debug!("after finish: keys={:?}, kvmap={:?}", keys, kvmap);
        assert_eq!(kvmap.len(), 2);
        assert!(kvmap.contains_key("a"));
        assert!(kvmap.contains_key("b"));

        assert!(ctor.data().is_empty());

        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }
}
