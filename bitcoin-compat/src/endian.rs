//! Host ↔ Endian conversion helpers.
//
//! The original C++ code provided a large matrix of
//! `htobe*`, `htole*`, `be*toh`, and `le*toh` helpers
//! hidden behind a thicket of pre‑processor guards.
//! In Rust the same behaviour is achievable in a
//! single, portable implementation using the
//! intrinsic `[u{16,32,64}::to_be / to_le / from_be / from_le]`
//! conversions.
//!
//! All helpers are instrumented with `tracing` so any
//! misuse is surfaced immediately in production
//! deployments.
// ---------------- [ File: bitcoin-compat/src/endian.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/endian.h]

/// Convert a host‑order `u16` to big‑endian.
#[inline]
pub fn htobe16(x: u16) -> u16 {
    trace!(target: "compat::endian", func = "htobe16", input = format!("{:#06x}", x));
    let res = x.to_be();
    trace!(target: "compat::endian", func = "htobe16", output = format!("{:#06x}", res));
    res
}

/// Convert a host‑order `u16` to little‑endian.
#[inline]
pub fn htole16(x: u16) -> u16 {
    trace!(target: "compat::endian", func = "htole16", input = format!("{:#06x}", x));
    let res = x.to_le();
    trace!(target: "compat::endian", func = "htole16", output = format!("{:#06x}", res));
    res
}

/// Convert a big‑endian `u16` to host order.
#[inline]
pub fn be_16toh(x: u16) -> u16 {
    trace!(target: "compat::endian", func = "be_16toh", input = format!("{:#06x}", x));
    let res = u16::from_be(x);
    trace!(target: "compat::endian", func = "be_16toh", output = format!("{:#06x}", res));
    res
}

/// Convert a little‑endian `u16` to host order.
#[inline]
pub fn le_16toh(x: u16) -> u16 {
    trace!(target: "compat::endian", func = "le_16toh", input = format!("{:#06x}", x));
    let res = u16::from_le(x);
    trace!(target: "compat::endian", func = "le_16toh", output = format!("{:#06x}", res));
    res
}

/// Convert a host‑order `u32` to big‑endian.
#[inline]
pub fn htobe32(x: u32) -> u32 {
    trace!(target: "compat::endian", func = "htobe32", input = format!("{:#010x}", x));
    let res = x.to_be();
    trace!(target: "compat::endian", func = "htobe32", output = format!("{:#010x}", res));
    res
}

/// Convert a host‑order `u32` to little‑endian.
#[inline]
pub fn htole32(x: u32) -> u32 {
    trace!(target: "compat::endian", func = "htole32", input = format!("{:#010x}", x));
    let res = x.to_le();
    trace!(target: "compat::endian", func = "htole32", output = format!("{:#010x}", res));
    res
}

/// Convert a big‑endian `u32` to host order.
#[inline]
pub fn be_32toh(x: u32) -> u32 {
    trace!(target: "compat::endian", func = "be_32toh", input = format!("{:#010x}", x));
    let res = u32::from_be(x);
    trace!(target: "compat::endian", func = "be_32toh", output = format!("{:#010x}", res));
    res
}

/// Convert a little‑endian `u32` to host order.
#[inline]
pub fn le_32toh(x: u32) -> u32 {
    trace!(target: "compat::endian", func = "le_32toh", input = format!("{:#010x}", x));
    let res = u32::from_le(x);
    trace!(target: "compat::endian", func = "le_32toh", output = format!("{:#010x}", res));
    res
}

/// Convert a host‑order `u64` to big‑endian.
#[inline]
pub fn htobe64(x: u64) -> u64 {
    trace!(target: "compat::endian", func = "htobe64", input = format!("{:#018x}", x));
    let res = x.to_be();
    trace!(target: "compat::endian", func = "htobe64", output = format!("{:#018x}", res));
    res
}

/// Convert a host‑order `u64` to little‑endian.
#[inline]
pub fn htole64(x: u64) -> u64 {
    trace!(target: "compat::endian", func = "htole64", input = format!("{:#018x}", x));
    let res = x.to_le();
    trace!(target: "compat::endian", func = "htole64", output = format!("{:#018x}", res));
    res
}

/// Convert a big‑endian `u64` to host order.
#[inline]
pub fn be_64toh(x: u64) -> u64 {
    trace!(target: "compat::endian", func = "be_64toh", input = format!("{:#018x}", x));
    let res = u64::from_be(x);
    trace!(target: "compat::endian", func = "be_64toh", output = format!("{:#018x}", res));
    res
}

/// Convert a little‑endian `u64` to host order.
#[inline]
pub fn le_64toh(x: u64) -> u64 {
    trace!(target: "compat::endian", func = "le_64toh", input = format!("{:#018x}", x));
    let res = u64::from_le(x);
    trace!(target: "compat::endian", func = "le_64toh", output = format!("{:#018x}", res));
    res
}
