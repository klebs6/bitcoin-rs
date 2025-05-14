// ---------------- [ File: bitcoin-bitstream/src/count.rs ]
crate::ix!();

/**
  | Return the smallest number n such that
  | (x >> n) == 0 (or 64 if the highest bit in
  | x is set.
  |
  */
#[inline] pub fn count_bits(x: u64) -> u64 {
    
    todo!();
        /*
            #if HAVE_BUILTIN_CLZL
        if (sizeof(unsigned long) >= sizeof(uint64_t)) {
            return x ? 8 * sizeof(unsigned long) - __builtin_clzl(x) : 0;
        }
    #endif
    #if HAVE_BUILTIN_CLZLL
        if (sizeof(unsigned long long) >= sizeof(uint64_t)) {
            return x ? 8 * sizeof(unsigned long long) - __builtin_clzll(x) : 0;
        }
    #endif
        int ret = 0;
        while (x) {
            x >>= 1;
            ++ret;
        }
        return ret;
        */
}
