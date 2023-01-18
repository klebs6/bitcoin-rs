crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_prefetch.h]

pub mod crc32c {

    /**
      | Ask the hardware to prefetch the data
      | at the given address into the L1 cache.
      |
      */
    #[inline] pub fn request_prefetch(address: *const u8)  {
        
        todo!();
            /*
                #if HAVE_BUILTIN_PREFETCH
              // Clang and GCC implement the __builtin_prefetch non-standard extension,
              // which maps to the best instruction on the target architecture.
              __builtin_prefetch(reinterpret_cast<const char*>(address), 0 /* Read only. */,
                                 0 /* No temporal locality. */);
            #elif HAVE_MM_PREFETCH
              // Visual Studio doesn't implement __builtin_prefetch, but exposes the
              // PREFETCHNTA instruction via the _mm_prefetch intrinsic.
              _mm_prefetch(reinterpret_cast<const char*>(address), _MM_HINT_NTA);
            #else
              // No prefetch support. Silence compiler warnings.
              (c_void)address;
            #endif  // HAVE_BUILTIN_PREFETCH
            */
    }
}
