// ---------------- [ File: bitcoinleveldb-filter/src/filter_policy.rs ]
/*!
  | A database can be configured with a custom
  | FilterPolicy object.  This object is
  | responsible for creating a small filter from
  | a set of keys.  These filters are stored in
  | leveldb and are consulted automatically by
  | leveldb to decide whether or not to read some
  | information from disk. In many cases, a filter
  | can cut down the number of disk seeks form
  | a handful to a single disk seek per DB::Get()
  | call.
  |
  | Most people will want to use the builtin bloom
  | filter support (see NewBloomFilterPolicy()
  | below).
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/filter_policy.h]

pub trait FilterPolicy: 

/*
  | Return the name of this policy.  Note that if
  | the filter encoding changes in an
  | incompatible way, the name returned by this
  | method must be changed.  Otherwise, old
  | incompatible filters may be passed to methods
  | of this type.
  */
Named 

+ CreateFilter 
+ KeyMayMatch { }

pub trait CreateFilter {
    /// Append a filter for [0..n-1] keys to `dst`.  
    /// Each key is given as keys[i].  
    fn create_filter(
        &self,
        keys: *const Slice,
        n: i32,
        dst: &mut Vec<u8>,
    );
}

pub trait KeyMayMatch {
    /// Return true if `key` was in the original `keys` used to build `filter`,
    /// or false with high probability if `key` was not in the `keys`.
    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool;
}

#[cfg(test)]
mod bitcoinleveldb_filter__filter_policy_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_filter__filter_policy_rs__name_matches_builtin_leveldb_identifier() {
        let p = BloomFilterPolicy::new(10);
        assert_eq!(p.name().as_ref(), "leveldb.BuiltinBloomFilter2");
    }

    #[traced_test]
    fn bitcoinleveldb_filter__filter_policy_rs__inserted_key_matches_created_filter() {
        let p = BloomFilterPolicy::new(10);

        let key_buf = b"alpha";
        let key = Slice::from_ptr_len(key_buf.as_ptr(), key_buf.len());
        let keys = vec![key];

        let mut filter = Vec::new();
        p.create_filter(keys.as_ptr(), keys.len() as i32, &mut filter);

        assert!(!filter.is_empty());

        let filter_slice = Slice::from_ptr_len(filter.as_ptr(), filter.len());
        assert!(p.key_may_match(&key, &filter_slice));
    }

    #[traced_test]
    fn bitcoinleveldb_filter__filter_policy_rs__factory_returns_dynamic_policy() {
        let p: Box<dyn FilterPolicy> = new_bloom_filter_policy(10);
        assert_eq!(p.name().as_ref(), "leveldb.BuiltinBloomFilter2");
    }
}
