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
Name 

+ CreateFilter 
+ KeyMayMatch { }

pub trait CreateFilter {

    /**
      | keys[0,n-1] contains a list of keys
      | (potentially with duplicates) that are
      | ordered according to the user supplied
      | comparator.  Append a filter that summarizes
      | keys[0,n-1] to *dst.
      |
      | Warning: do not change the initial contents
      | of *dst.  Instead, append the newly
      | constructed filter to *dst.
      */
    fn create_filter(&self, 
            keys: *const Slice,
            n:    i32,
            dst:  *mut String);
}

pub trait KeyMayMatch {

    /**
      | "filter" contains the data appended by
      | a preceding call to CreateFilter() on this
      | class.  This method must return true if the
      | key was in the list of keys passed to
      | CreateFilter().
      |
      | This method may return true or false if the
      | key was not on the list, but it should aim to
      | return false with a high probability.
      */
    fn key_may_match(&self, 
            key_:    &Slice,
            filter: &Slice) -> bool;
}

/**
  | Return a new filter policy that uses a bloom
  | filter with approximately the specified number
  | of bits per key.  A good value for bits_per_key
  | is 10, which yields a filter with ~ 1% false
  | positive rate.
  |
  | Callers must delete the result after any
  | database that is using the result has been
  | closed.
  |
  | Note: if you are using a custom comparator that
  | ignores some parts of the keys being compared,
  | you must not use NewBloomFilterPolicy() and
  | must provide your own FilterPolicy that also
  | ignores the corresponding parts of the keys.
  | For example, if the comparator ignores trailing
  | spaces, it would be incorrect to use
  | a FilterPolicy (like NewBloomFilterPolicy) that
  | does not ignore trailing spaces in keys.
  */
pub fn new_bloom_filter_policy(bits_per_key_: i32) -> Box<dyn FilterPolicy> {
    
    todo!();
        /*
        
        */
}
