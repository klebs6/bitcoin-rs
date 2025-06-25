// ---------------- [ File: bitcoin-compat/tests/endian.rs ]
//! Round‑trip and numeric tests for host/endian helpers.

use bitcoin_compat::*;
use bitcoin_imports::*;

/// Ensure each helper performs a correct round‑trip
/// conversion for representative values.
#[traced_test]
fn verify_endian_roundtrips() {
    // 16‑bit ----------------------------------------------------------------
    let v16: u16 = 0x1234;
    assert_eq!(be_16toh(htobe16(v16)), v16, "u16 BE round‑trip failed");
    assert_eq!(le_16toh(htole16(v16)), v16, "u16 LE round‑trip failed");

    // 32‑bit ----------------------------------------------------------------
    let v32: u32 = 0x89ab_cdef;
    assert_eq!(be_32toh(htobe32(v32)), v32, "u32 BE round‑trip failed");
    assert_eq!(le_32toh(htole32(v32)), v32, "u32 LE round‑trip failed");

    // 64‑bit ----------------------------------------------------------------
    let v64: u64 = 0x0123_4567_89ab_cdef;
    assert_eq!(be_64toh(htobe64(v64)), v64, "u64 BE round‑trip failed");
    assert_eq!(le_64toh(htole64(v64)), v64, "u64 LE round‑trip failed");
}

/// Verify that the numeric output matches the
/// canonical little‑endian expectations when run
/// on little‑endian hosts. Big‑endian hosts perform
/// an equivalent but architecture‑specific check.
#[traced_test]
fn verify_numeric_conversion() {
    // ---- 16‑bit ----------------------------------------------------------
    #[cfg(target_endian = "little")]
    {
        assert_eq!(htobe16(0x1234), 0x3412);
        assert_eq!(htole16(0x1234), 0x1234);
    }
    #[cfg(target_endian = "big")]
    {
        assert_eq!(htobe16(0x1234), 0x1234);
        assert_eq!(htole16(0x1234), 0x3412);
    }

    // ---- 32‑bit ----------------------------------------------------------
    #[cfg(target_endian = "little")]
    {
        assert_eq!(htobe32(0x89ab_cdef), 0xefcd_ab89);
        assert_eq!(htole32(0x89ab_cdef), 0x89ab_cdef);
    }
    #[cfg(target_endian = "big")]
    {
        assert_eq!(htobe32(0x89ab_cdef), 0x89ab_cdef);
        assert_eq!(htole32(0x89ab_cdef), 0xefcd_ab89);
    }

    // ---- 64‑bit ----------------------------------------------------------
    #[cfg(target_endian = "little")]
    {
        assert_eq!(htobe64(0x0123_4567_89ab_cdef), 0xefcd_ab89_6745_2301);
        assert_eq!(htole64(0x0123_4567_89ab_cdef), 0x0123_4567_89ab_cdef);
    }
    #[cfg(target_endian = "big")]
    {
        assert_eq!(htobe64(0x0123_4567_89ab_cdef), 0x0123_4567_89ab_cdef);
        assert_eq!(htole64(0x0123_4567_89ab_cdef), 0xefcd_ab89_6745_2301);
    }
}
