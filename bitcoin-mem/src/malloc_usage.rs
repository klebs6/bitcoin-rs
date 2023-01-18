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
#[inline] pub fn malloc_usage(alloc: usize) -> usize {
    
    todo!();
        /*
        // Measured on libc6 2.19 on Linux.
        if (alloc == 0) {
            return 0;
        } else if (sizeof(c_void*) == 8) {
            return ((alloc + 31) >> 4) << 4;
        } else if (sizeof(c_void*) == 4) {
            return ((alloc + 15) >> 3) << 3;
        } else {
            assert(0);
        }
        */
}
