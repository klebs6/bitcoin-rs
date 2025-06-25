// ---------------- [ File: bitcoin-compat/tests/bswap.rs ]
use bitcoin_compat::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/test/bswap_tests.cpp]

/// Verify that each `bswap_*` function produces the
/// canonical IEEE little↔big‑endian reversal.
#[traced_test]
fn verify_byteswap_correctness() {
    // --- 16‑bit ------------------------------------------------------------
    let u1: u16 = 0x1234;
    let e1: u16 = 0x3412;
    assert_eq!(bswap_16(u1), e1, "u16 byteswap mismatch");

    // --- 32‑bit ------------------------------------------------------------
    let u2: u32 = 0x5678_9abc;
    let e2: u32 = 0xbc9a_7856;
    assert_eq!(bswap_32(u2), e2, "u32 byteswap mismatch");

    // --- 64‑bit ------------------------------------------------------------
    let u3: u64 = 0xdef0_1234_5678_9abc;
    let e3: u64 = 0xbc9a_7856_3412_f0de;
    assert_eq!(bswap_64(u3), e3, "u64 byteswap mismatch");
}
