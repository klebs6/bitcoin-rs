// ---------------- [ File: bitcoin-crc32c/src/arm64_check.rs ]
/*!
  | ARM-specific code checking for the
  | availability of CRC32C instructions.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64_check.h]

/**
   getauxval() is not available on Android until
   API level 20. Link it as a weak symbol.
  */
#[cfg(HAVE_ARM64_CRC32C)]
#[cfg(__linux__)]
#[cfg(HAVE_WEAK_GETAUXVAL)]
extern "C" {

    #[__attribute__((weak))]
    pub fn getauxval(ty: u64) -> u64 {
        
        todo!();
            /*
            
            */
    }
}

#[cfg(HAVE_ARM64_CRC32C)]
#[cfg(__linux__)]
#[cfg(HAVE_WEAK_GETAUXVAL)]
pub const AT_HWCAP: usize = 16;

#[cfg(HAVE_ARM64_CRC32C)]
pub mod crc32c {

    #[inline] pub fn can_use_arm_64crc32() -> bool {
        
        todo!();
            /*
                #if defined (__linux__) && (HAVE_STRONG_GETAUXVAL || HAVE_WEAK_GETAUXVAL)
          // From 'arch/arm64/include/uapi/asm/hwcap.h' in Linux kernel source code.
          constexpr unsigned long kHWCAP_PMULL = 1 << 4;
          constexpr unsigned long kHWCAP_CRC32 = 1 << 7;
          unsigned long hwcap =
        #if HAVE_STRONG_GETAUXVAL
              // Some compilers warn on (&getauxval != nullptr) in the block below.
              getauxval(AT_HWCAP);
        #elif HAVE_WEAK_GETAUXVAL
              (&getauxval != nullptr) ? getauxval(AT_HWCAP) : 0;
        #else
        #error This is supposed to be nested inside a check for HAVE_*_GETAUXVAL.
        #endif  // HAVE_STRONG_GETAUXVAL
          return (hwcap & (kHWCAP_PMULL | kHWCAP_CRC32)) ==
                 (kHWCAP_PMULL | kHWCAP_CRC32);
        #elif defined(__APPLE__)
          int val = 0;
          size_t len = sizeof(val);
          return sysctlbyname("hw.optional.armv8_crc32", &val, &len, nullptr, 0) == 0
                     && val != 0;
        #else
          return false;
        #endif  // HAVE_STRONG_GETAUXVAL || HAVE_WEAK_GETAUXVAL
            */
    }
}
