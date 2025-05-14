// ---------------- [ File: bitcoin-compat/src/cpuid.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/cpuid.h]

/**
  | We can't use cpuid.h's __get_cpuid
  | as it does not support subleafs.
  |
  */
#[cfg(have_getcpuid)]
#[inline] pub fn getcpuid(
        leaf:    u32,
        subleaf: u32,
        a:       &mut u32,
        b:       &mut u32,
        c:       &mut u32,
        d:       &mut u32)  {
    
    todo!();
        /*
            #ifdef __GNUC__
        __cpuid_count(leaf, subleaf, a, b, c, d);
    #else
      __asm__ ("cpuid" : "=a"(a), "=b"(b), "=c"(c), "=d"(d) : "0"(leaf), "2"(subleaf));
    #endif
        */
}
