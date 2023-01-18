/*!
  | X86-specific code checking the availability of
  | SSE4.2 instructions.
  |
  | If the compiler supports SSE4.2, it definitely
  | supports X86.
  */

crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_sse42_check.h]

#[cfg(all(HAVE_SSE42,any(_M_X64,__x86_64__)))]
#[cfg(_MSC_VER)]
pub mod crc32c {

    #[inline] pub fn can_use_sse42() -> bool {
        
        todo!();
            /*
                int cpu_info[4];
          __cpuid(cpu_info, 1);
          return (cpu_info[2] & (1 << 20)) != 0;
            */
    }
}

#[cfg(all(HAVE_SSE42,any(_M_X64,__x86_64__)))]
#[cfg(not(_MSC_VER))]
pub mod crc32c {

    #[inline] pub fn can_use_sse42() -> bool {
        
        todo!();
            /*
                unsigned int eax, ebx, ecx, edx;
          return __get_cpuid(1, &eax, &ebx, &ecx, &edx) && ((ecx & (1 << 20)) != 0);
            */
    }
}
