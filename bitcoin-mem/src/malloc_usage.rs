// ---------------- [ File: bitcoin-mem/src/malloc_usage.rs ]
crate::ix!();

/**
  | Compute the total memory used by allocating
  | alloc bytes.
  |
  -------------------
  | Compute the memory used for dynamically
  | allocated but owned data structures.
  | 
  | For generic data types, this is *not*
  | recursive. DynamicUsage(vector<vector<int>
  | >) will compute the memory used for the
  | vector<int>'s, but not for the ints
  | inside.
  | 
  | This is for efficiency reasons, as these
  | functions are intended to be fast. If
  | application data structures require
  | more accurate inner accounting, they
  | should iterate themselves, or use more
  | efficient caching + updating on modification.
  |
  */
#[inline]
pub fn malloc_usage(alloc: usize) -> usize {
    if alloc == 0 {
        trace!("malloc_usage(0) -> 0");
        return 0;
    }
    let ptr_sz = core::mem::size_of::<*const ()>();
    let result = match ptr_sz {
        8 => ((alloc + 31) >> 4) << 4, // 16‑byte buckets on 64‑bit
        4 => ((alloc + 15) >> 3) << 3, // 8‑byte buckets on 32‑bit
        _ => panic!("Unsupported pointer size: {}", ptr_sz),
    };
    trace!("malloc_usage({}) -> {} (ptr_sz={})", alloc, result, ptr_sz);
    result
}
