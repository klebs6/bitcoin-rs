//! Platform‑independent byte‑swap helpers.
//!
//! These functions mirror the semantics of the classic
//! `bswap_16/32/64` intrinsics on Unix‐like systems, while
//! adding structured tracing so that any misuse can be
//! detected immediately in production builds.

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/byteswap.h]

/// Swap the two bytes of a 16‑bit unsigned integer.
///
/// ```text
/// 0xAABB → 0xBBAA
/// ```
#[inline]
pub fn bswap_16(x: u16) -> u16 {
    trace!(target: "compat::byteswap", input = format!("{:#06x}", x), "bswap_16");
    let res = x.swap_bytes();
    trace!(target: "compat::byteswap", output = format!("{:#06x}", res), "bswap_16");
    res
}

/// Swap the four bytes of a 32‑bit unsigned integer.
///
/// ```text
/// 0xAABB_CCDD → 0xDDCC_BBAA
/// ```
#[inline]
pub fn bswap_32(x: u32) -> u32 {
    trace!(target: "compat::byteswap", input = format!("{:#010x}", x), "bswap_32");
    let res = x.swap_bytes();
    trace!(target: "compat::byteswap", output = format!("{:#010x}", res), "bswap_32");
    res
}

/// Swap the eight bytes of a 64‑bit unsigned integer.
///
/// ```text
/// 0x1122_3344_5566_7788 → 0x8877_6655_4433_2211
/// ```
#[inline]
pub fn bswap_64(x: u64) -> u64 {
    trace!(target: "compat::byteswap", input = format!("{:#018x}", x), "bswap_64");
    let res = x.swap_bytes();
    trace!(target: "compat::byteswap", output = format!("{:#018x}", res), "bswap_64");
    res
}
