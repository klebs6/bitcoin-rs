crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/cleanse.h]
//-------------------------------------------[.cpp/bitcoin/src/support/cleanse.cpp]

/**
  | Secure overwrite a buffer (possibly
  | containing secret data) with zero-bytes.
  | The write operation will not be optimized
  | out by the compiler.
  |
  */
pub fn memory_cleanse(
        _ptr: *mut c_void,
        _len: usize)  {
    
    todo!();
        /*
            #if defined(_MSC_VER)
        /* SecureZeroMemory is guaranteed not to be optimized out by MSVC. */
        SecureZeroMemory(ptr, len);
    #else
        std::memset(ptr, 0, len);

        /* Memory barrier that scares the compiler away from optimizing out the memset.
         *
         * Quoting Adam Langley <agl@google.com> in commit ad1907fe73334d6c696c8539646c21b11178f20f
         * in BoringSSL (ISC License):
         *    As best as we can tell, this is sufficient to break any optimisations that
         *    might try to eliminate "superfluous" memsets.
         * This method is used in memzero_explicit() the Linux kernel, too. Its advantage is that it
         * is pretty efficient because the compiler can still implement the memset() efficiently,
         * just not remove it entirely. See "Dead Store Elimination (Still) Considered Harmful" by
         * Yang et al. (USENIX Security 2017) for more background.
         */
        __asm__ __volatile__("" : : "r"(ptr) : "memory");
    #endif
        */
}
