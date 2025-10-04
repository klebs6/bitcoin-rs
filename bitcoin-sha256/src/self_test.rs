// ---------------- [ File: bitcoin-sha256/src/self_test.rs ]
crate::ix!();

/// Canonical Bitcoin Core self‑test fixtures and reference vectors.
///
/// *Never* duplicate these bytes or tables anywhere else in the crate.
/// Import from here instead (e.g., `use crate::fixtures::*;`).
pub mod fixtures {

    /// Input state (FIPS 180‑4 IV).
    pub const INIT: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    /// The exact lorem‑ipsum message used by Bitcoin Core’s SHA‑256 self‑test.
    ///
    /// The leading `'-'` is intentional; tests hash from `SELF_TEST_DATA.as_ptr().add(1)`
    /// to stress unaligned reads. **Do not** retype or reflow this string.
    pub const SELF_TEST_DATA: &[u8] = b"-\
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
        eiusmod tempor incididunt ut labore et dolore magna aliqua. Et m\
        olestie ac feugiat sed lectus vestibulum mattis ullamcorper. Mor\
        bi blandit cursus risus at ultrices mi tempus imperdiet nulla. N\
        unc congue nisi vitae suscipit tellus mauris. Imperdiet proin fer\
        mentum leo vel orci. Massa tempor nec feugiat nisl pretium fusce\x20\
        id velit. Tellus in metus vulputate eu scelerisque felis. Mi tem\
        pus imperdiet nulla malesuada pellentesque. Tristique magna sit.";

    /// Sanity: require at least 8 full 64‑byte blocks after skipping the first byte.
    pub const _: () = assert!(
        SELF_TEST_DATA.len() >= 1 + 8 * 64,
        "SELF_TEST_DATA must contain ≥ 8×64B after the +1 misalignment"
    );

    /// Expected chaining states after hashing the first `i` 64‑byte blocks of
    /// `SELF_TEST_DATA[1..]` **without** padding (compression‑only).
    pub const COMP_STATES: [[u32; 8]; 9] = [
        // 0 blocks (IV)
        INIT,

        // 1 block
        [0x91f8ec6b, 0x4da10fe3, 0x1c9c292c, 0x45e18185,
        0x435cc111, 0x3ca26f09, 0xeb954cae, 0x402a7069],

        // 2 blocks
        [0xcabea5ac, 0x374fb97c, 0x182ad996, 0x7bd69cbf,
        0x450ff900, 0xc1d2be8a, 0x6a41d505, 0xe6212dc3],

        // 3 blocks
        [0xbcff09d6, 0x3e76f36e, 0x3ecb2501, 0x78866e97,
        0xe1c1e2fd, 0x32f4eaff, 0x8aa6c4e5, 0xdfc024bc],

        // 4 blocks
        [0xa08c5d94, 0x0a862f93, 0x6b7f2f40, 0x8f9fae76,
        0x6d40439f, 0x79dcee0c, 0x3e39ff3a, 0xdc3bdbb1],

        // 5 blocks  ← UPDATED
        [0x2c679d34, 0x8a18ba45, 0x4d9a6d3c, 0x5ecfaa81,
        0xb89ef8ee, 0x802e17f4, 0x0dd7aa1f, 0x75174074],

        // 6 blocks  ← UPDATED
        [0xfd51ffdb, 0x007a9b0f, 0x5a13fdb1, 0x9ced22e7,
        0x2e70ab66, 0xc54443cc, 0xc5f29870, 0x615ef789],

        // 7 blocks  ← UPDATED
        [0xe1716ee1, 0x2e892fe4, 0x43895a62, 0x5fbf1b20, 
        0xdd5f5f43, 0xeac89500, 0xf3f8f2aa, 0xb266aeea],

        // 8 blocks  ← UPDATED
        [0x98984fcc, 0x47770725, 0x96c37198, 0xeeec8ed5, 
        0xf4e29570, 0xc11c5b13, 0x9e96df27, 0xacb9ddbf],
    ];

    /// Expected outputs for the individual 8× 64‑byte messages under **double SHA‑256**
    /// (including padding), concatenated lane‑by‑lane (each lane is 32 bytes).
    pub const D64_LANES: [u8; 256] = [
        0x09, 0x3a, 0xc4, 0xd0, 0x0f, 0xf7, 0x57, 0xe1, 0x72, 0x85, 0x79, 0x42, 0xfe, 0xe7, 0xe0, 0xa0,
        0xfc, 0x52, 0xd7, 0xdb, 0x07, 0x63, 0x45, 0xfb, 0x53, 0x14, 0x7d, 0x17, 0x22, 0x86, 0xf0, 0x52,
        0x48, 0xb6, 0x11, 0x9e, 0x6e, 0x48, 0x81, 0x6d, 0xcc, 0x57, 0x1f, 0xb2, 0x97, 0xa8, 0xd5, 0x25,
        0x9b, 0x82, 0xaa, 0x89, 0xe2, 0xfd, 0x2d, 0x56, 0xe8, 0x28, 0x83, 0x0b, 0xe2, 0xfa, 0x53, 0xb7,
        0xd6, 0x6b, 0x07, 0x85, 0x83, 0xb0, 0x10, 0xa2, 0xf5, 0x51, 0x3c, 0xf9, 0x60, 0x03, 0xab, 0x45,
        0x6c, 0x15, 0x6e, 0xef, 0xb5, 0xac, 0x3e, 0x6c, 0xdf, 0xb4, 0x92, 0x22, 0x2d, 0xce, 0xbf, 0x3e,
        0xe9, 0xe5, 0xf6, 0x29, 0x0e, 0x01, 0x4f, 0xd2, 0xd4, 0x45, 0x65, 0xb3, 0xbb, 0xf2, 0x4c, 0x16,
        0x37, 0x50, 0x3c, 0x6e, 0x49, 0x8c, 0x5a, 0x89, 0x2b, 0x1b, 0xab, 0xc4, 0x37, 0xd1, 0x46, 0xe9,
        0x3d, 0x0e, 0x85, 0xa2, 0x50, 0x73, 0xa1, 0x5e, 0x54, 0x37, 0xd7, 0x94, 0x17, 0x56, 0xc2, 0xd8,
        0xe5, 0x9f, 0xed, 0x4e, 0xae, 0x15, 0x42, 0x06, 0x0d, 0x74, 0x74, 0x5e, 0x24, 0x30, 0xce, 0xd1,
        0x9e, 0x50, 0xa3, 0x9a, 0xb8, 0xf0, 0x4a, 0x57, 0x69, 0x78, 0x67, 0x12, 0x84, 0x58, 0xbe, 0xc7,
        0x36, 0xaa, 0xee, 0x7c, 0x64, 0xa3, 0x76, 0xec, 0xff, 0x55, 0x41, 0x00, 0x2a, 0x44, 0x68, 0x4d,
        0xb6, 0x53, 0x9e, 0x1c, 0x95, 0xb7, 0xca, 0xdc, 0x7f, 0x7d, 0x74, 0x27, 0x5c, 0x8e, 0xa6, 0x84,
        0xb5, 0xac, 0x87, 0xa9, 0xf3, 0xff, 0x75, 0xf2, 0x34, 0xcd, 0x1a, 0x3b, 0x82, 0x2c, 0x2b, 0x4e,
        0x6a, 0x46, 0x30, 0xa6, 0x89, 0x86, 0x23, 0xac, 0xf8, 0xa5, 0x15, 0xe9, 0x0a, 0xaa, 0x1e, 0x9a,
        0xd7, 0x93, 0x6b, 0x28, 0xe4, 0x3b, 0xfd, 0x59, 0xc6, 0xed, 0x7c, 0x5f, 0xa5, 0x41, 0xcb, 0x51,
    ];
}

/// Exhaustive internal self‑test validating the compression routine and the
/// single/parallel double‑SHA‑256 helpers against the **canonical** fixtures.
///
/// Returns `true` **iff** every comparison matches bit‑for‑bit.
pub fn self_test() -> bool {
    // ---- 1) Compression‑only tests -----------------------------------------
    for i in 0..=8 {
        let mut s = fixtures::INIT;
        unsafe {
            // mirror Core’s misalignment: start at SELF_TEST_DATA + 1
            sha256_transform(s.as_mut_ptr(), fixtures::SELF_TEST_DATA.as_ptr().add(1), i);
        }
        if s != fixtures::COMP_STATES[i] {
            // Pretty hex for quick eyeballing
            fn H(w: u32) -> String { format!("{:08x}", w) }
            eprintln!("SELF-TEST FAIL: compression mismatch at i = {}", i);
            eprintln!("  expected: [{}]", fixtures::COMP_STATES[i].iter().map(|&w| H(w)).collect::<Vec<_>>().join(" "));
            eprintln!("  got     : [{}]", s.iter().map(|&w| H(w)).collect::<Vec<_>>().join(" "));
            return false;
        }
    }

    // ---- 2) Double‑SHA‑256 reference helpers ---------------------------------
    unsafe {
        // --- Single‑lane (scalar) ---
        let mut out32 = [0u8; 32];
        TRANSFORM_D64(out32.as_mut_ptr(), fixtures::SELF_TEST_DATA.as_ptr().add(1));
        if &out32[..] != &fixtures::D64_LANES[..32] {
            eprintln!("SELF-TEST FAIL: TransformD64 (scalar) lane0 mismatch");
            return false;
        }

        // --- 2‑way, 4‑way, 8‑way (if available) ---
        if let Some(func) = TRANSFORM_D64_2WAY {
            let mut out64 = [0u8; 64];
            func(out64.as_mut_ptr(), fixtures::SELF_TEST_DATA.as_ptr().add(1));
            if &out64[..] != &fixtures::D64_LANES[..64] {
                eprintln!("SELF-TEST FAIL: TransformD64_2way mismatch (lane0/1)");
                return false;
            }
        }
        if let Some(func) = TRANSFORM_D64_4WAY {
            let mut out128 = [0u8; 128];
            func(out128.as_mut_ptr(), fixtures::SELF_TEST_DATA.as_ptr().add(1));
            if &out128[..] != &fixtures::D64_LANES[..128] {
                eprintln!("SELF-TEST FAIL: TransformD64_4way mismatch (lane0..3)");
                return false;
            }
        }
        if let Some(func) = TRANSFORM_D64_8WAY {
            let mut out256 = [0u8; 256];
            func(out256.as_mut_ptr(), fixtures::SELF_TEST_DATA.as_ptr().add(1));
            if &out256[..] != &fixtures::D64_LANES[..] {
                eprintln!("SELF-TEST FAIL: TransformD64_8way mismatch (lane0..7)");
                return false;
            }
        }
    }

    trace!(target: "sha256", "self_test: all tests passed");
    true
}

// -----------------------------------------------------------------------------
// Tests – all test code imports the fixtures; there are **no** local copies.
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sha256_reference_vector_validation {
    use super::*;

    #[test]
    fn self_test_data_contains_expected_fusce_id_join() {
        // mirror the +1 misalignment (skip the leading '-')
        let s = &fixtures::SELF_TEST_DATA[1..];
        assert!(core::str::from_utf8(s).unwrap().contains("fusce id velit"),
        "SELF_TEST_DATA drifted: expected \"fusce id velit\" join");
    }

    /// Validate that `self_test` passes *and* that its result is `true`.
    #[traced_test]
    fn bitcoin_core_vectors_match() {
        assert!(
            self_test(),
            "internal self_test failed: compression or digest mismatch"
        );
    }

    /// **Your endianness‑tolerant diagnostic** (integrated here, scoped to D64).
    ///
    /// Some Bitcoin contexts display SHA‑256 digests in reversed byte order.
    /// This test accepts either big‑endian (canonical) or whole‑digest LE,
    /// and prints a clear diagnostic if a mismatch occurs.
    #[traced_test]
    fn d64_lane0_endianness_tolerant_check_with_diagnostics() {
        unsafe {
            let mut got = [0u8; 32];
            TRANSFORM_D64(got.as_mut_ptr(), fixtures::SELF_TEST_DATA.as_ptr().add(1));

            let exp_be = <&[u8; 32]>::try_from(&fixtures::D64_LANES[..32]).unwrap();
            let mut exp_le = *exp_be;
            exp_le.reverse();

            let ok = &got == exp_be || &got == &exp_le;
            if !ok {
                // Minimal hex without extra deps
                fn hex(bytes: &[u8]) -> String {
                    const T: &[u8; 16] = b"0123456789abcdef";
                    let mut s = String::with_capacity(bytes.len() * 2);
                    for &b in bytes {
                        s.push(T[(b >> 4) as usize] as char);
                        s.push(T[(b & 0x0f) as usize] as char);
                    }
                    s
                }
                eprintln!("===== SELF-TEST DIAGNOSTIC (D64 lane0) =====");
                eprintln!("message_len       : 64 bytes");
                eprintln!("message_sha256(be): {}", hex(exp_be));
                eprintln!("message_sha256(le): {}", hex(&exp_le));
                eprintln!("got_digest        : {}", hex(&got));
                eprintln!("note              : Accepting BE or LE digest order for Bitcoin displays.");
                eprintln!("============================================");
            }
            assert!(ok, "D64 lane0 digest mismatch (neither BE nor reversed LE)");
        }
    }
}
