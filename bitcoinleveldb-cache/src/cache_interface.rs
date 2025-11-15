// ---------------- [ File: bitcoinleveldb-cache/src/cache_interface.rs ]
crate::ix!();

/*!
  | A Cache is an interface that maps keys to
  | values.  It has internal synchronization and
  | may be safely accessed concurrently from
  | multiple threads.  It may automatically evict
  | entries to make room for new entries.  Values
  | have a specified charge against the cache
  | capacity.  For example, a cache where the
  | values are variable length strings, may use the
  | length of the string as the charge for the
  | string.
  |
  | A builtin cache implementation with
  | a least-recently-used eviction policy is
  | provided.  Clients may use their own
  | implementations if they want something more
  | sophisticated (like scan-resistance, a custom
  | eviction policy, variable cache sizing, etc.)
  */
pub trait CacheInterface:
CacheInsert
+ CacheLookup
+ CacheRelease
+ CacheValue
+ CacheErase
+ CacheNewId
+ CachePrune
+ CacheTotalCharge {}

impl<T> CacheInterface for T where
    T: CacheInsert
        + CacheLookup
        + CacheRelease
        + CacheValue
        + CacheErase
        + CacheNewId
        + CachePrune
        + CacheTotalCharge
{
}

pub type CacheDeleterFn = fn(key_: &Slice, value: *mut c_void) -> c_void;

pub trait CacheInsert {
    /**
      | Insert a mapping from key->value into the
      | cache and assign it the specified charge
      | against the total cache capacity.
      |
      | Returns a handle that corresponds to the
      | mapping.  The caller must call
      | this->Release(handle) when the returned
      | mapping is no longer needed.
      |
      | When the inserted entry is no longer needed,
      | the key and value will be passed to
      | "deleter".
      */
    fn insert(
        &mut self,
        key_: &Slice,
        value: *mut c_void,
        charge: usize,
        deleter: CacheDeleterFn,
    ) -> *mut CacheHandle;
}

pub trait CacheLookup {

    /**
      | If the cache has no mapping for "key",
      | returns nullptr.
      |
      | Else return a handle that corresponds to the
      | mapping.  The caller must call
      | this->Release(handle) when the returned
      | mapping is no longer needed.
      */
    fn lookup(&mut self, key_: &Slice) -> *mut CacheHandle;
}

pub trait CacheRelease {

    /**
      | Release a mapping returned by a previous
      | Lookup().
      | 
      | REQUIRES: handle must not have been
      | released yet.
      | 
      | REQUIRES: handle must have been returned
      | by a method on *this.
      |
      */
    fn release(&mut self, handle: *mut CacheHandle);
}

pub trait CacheValue {

    /**
      | Return the value encapsulated in a handle
      | returned by a successful Lookup().
      |
      | REQUIRES: handle must not have been released
      | yet.
      |
      | REQUIRES: handle must have been returned by
      | a method on *this.
      */
    fn value(&mut self, handle: *mut CacheHandle);
}

pub trait CacheErase {

    /**
      | If the cache contains entry for key, erase
      | it.  Note that the underlying entry will be
      | kept around until all existing handles to it
      | have been released.
      */
    fn erase(&mut self, key_: &Slice);
}

pub trait CacheNewId {

    /**
      | Return a new numeric id.  May be used by
      | multiple clients who are sharing the same
      | cache to partition the key space.  Typically
      | the client will allocate a new id at startup
      | and prepend the id to its cache keys.
      */
    fn new_id(&mut self) -> u64;
}

pub trait CachePrune {

    /**
      | Remove all cache entries that are not
      | actively in use.  Memory-constrained
      | applications may wish to call this method to
      | reduce memory usage.
      |
      | Default implementation of Prune() does
      | nothing.  Subclasses are strongly encouraged
      | to override the default implementation.
      | A future release of leveldb may change
      | Prune() to a pure abstract method.
      */
    fn prune(&mut self);
}

pub trait CacheTotalCharge {

    /**
      | Return an estimate of the combined charges
      | of all elements stored in the cache.
      |
      */
    fn total_charge(&self) -> usize;
}
