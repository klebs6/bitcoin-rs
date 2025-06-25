// ---------------- [ File: bitcoin-crc32c/src/platform.rs ]
crate::ix!();

/// X86-specific code checking the availability of
/// SSE4.2 instructions.
/// 
/// If the compiler supports SSE4.2, it definitely
/// supports X86.
///
/// Query the CPU *once*; subsequent calls are
/// just a cheap load from a `OnceLock`.
pub fn can_use_sse42() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        use std::sync::OnceLock;
        static FLAG: OnceLock<bool> = OnceLock::new();
        *FLAG.get_or_init(|| std::arch::is_x86_feature_detected!("sse4.2"))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// ARM-specific code checking for the availability of CRC32C instructions.
///
/// Detect AArch64 CRC32/PMULL support.
pub fn can_use_arm64_crc32() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        use std::sync::OnceLock;
        static FLAG: OnceLock<bool> = OnceLock::new();
        *FLAG.get_or_init(|| std::arch::is_aarch64_feature_detected!("crc"))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        false
    }
}

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_sse42_check.h]
//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64_check.h]
