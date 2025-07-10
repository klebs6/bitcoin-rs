// ---------------- [ File: bitcoin-sha256/src/self_test.rs ]
crate::ix!();

/// Exhaustive internal self‑test validating the compression routine and
/// single/parallel double‑SHA‑256 helpers against the official Bitcoin Core
/// reference vectors.
///
/// Returns `true` **iff** every comparison matches bit‑for‑bit.
pub fn self_test() -> bool {
    // ---- Reference vectors (verbatim from Bitcoin Core) --------------------
    // Input state (equal to the initial Sha256 state)
    const INIT: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // Some random input data to test with -- Intentionally not aligned
    const DATA: &[u8] = b"-\
        Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do \
        eiusmod tempor incididunt ut labore et dolore magna aliqua. Et m\
        olestie ac feugiat sed lectus vestibulum mattis ullamcorper. Mor\
        bi blandit cursus risus at ultrices mi tempus imperdiet nulla. N\
        unc congue nisi vita suscipit tellus mauris. Imperdiet proin fer\
        mentum leo vel orci. Massa tempor nec feugiat nisl pretium fusce\
         id velit. Telus in metus vulputate eu scelerisque felis. Mi tem\
        pus imperdiet nulla malesuada pellentesque. Tristique magna sit.";

    // Expected output state for hashing the i*64 first input bytes above (excluding Sha256 padding).
    const RESULT: [[u32; 8]; 9] = [
        INIT,
        [0x91f8ec6b, 0x4da10fe3, 0x1c9c292c, 0x45e18185, 0x435cc111, 0x3ca26f09, 0xeb954cae, 0x402a7069],
        [0xcabea5ac, 0x374fb97c, 0x182ad996, 0x7bd69cbf, 0x450ff900, 0xc1d2be8a, 0x6a41d505, 0xe6212dc3],
        [0xbcff09d6, 0x3e76f36e, 0x3ecb2501, 0x78866e97, 0xe1c1e2fd, 0x32f4eaff, 0x8aa6c4e5, 0xdfc024bc],
        [0xa08c5d94, 0x0a862f93, 0x6b7f2f40, 0x8f9fae76, 0x6d40439f, 0x79dcee0c, 0x3e39ff3a, 0xdc3bdbb1],
        [0x216a0895, 0x9f1a3662, 0xe99946f9, 0x87ba4364, 0x0fb5db2c, 0x12bed3d3, 0x6689c0c7, 0x292f1b04],
        [0xca3067f8, 0xbc8c2656, 0x37cb7e0d, 0x9b6b8b0f, 0x46dc380b, 0xf1287f57, 0xc42e4b23, 0x3fefe94d],
        [0x3e4c4039, 0xbb6fca8c, 0x6f27d2f7, 0x301e44a4, 0x8352ba14, 0x5769ce37, 0x48a1155f, 0xc0e1c4c6],
        [0xfe2fa9dd, 0x69d0862b, 0x1ae0db23, 0x471f9244, 0xf55c0145, 0xc30f9c3b, 0x40a84ea0, 0x5b8a266c],
    ];

        // Expected output for each of the individual 8 64-byte messages under full double Sha256 (including padding).
    const RESULT_D64: [u8; 256] = [
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

    // ---- 1. Compression‑only tests ----------------------------------------
    // Test Transform() for 0 through 8 transformations.
    for i in 0..=8 {
        let mut s = INIT;
        unsafe {
            sha256_transform(s.as_mut_ptr(), DATA.as_ptr().add(1), i);
        }
        if s != RESULT[i] {
            trace!(target: "sha256", i, expected = ?RESULT[i], got = ?s, "self_test: compression mismatch");
            return false;
        }
    }

    // ---- 2. Double‑SHA‑256 reference helpers --------------------------------
    unsafe {
        // --- Single‑block (scalar) ---
        let mut out32 = [0u8; 32];
        TRANSFORM_D64(out32.as_mut_ptr(), DATA.as_ptr().add(1));
        if &out32[..] != &RESULT_D64[..32] {
            trace!(target: "sha256", expected = &RESULT_D64[..32], got = &out32[..], "self_test: TransformD64 mismatch");
            return false;
        }

        // --- 2‑way, 4‑way, 8‑way (if available) ---
        // Test TransformD64_2way, if available.
        if let Some(func) = TRANSFORM_D64_2WAY {
            let mut out64 = [0u8; 64];
            func(out64.as_mut_ptr(), DATA.as_ptr().add(1));
            if &out64[..] != &RESULT_D64[..64] {
                trace!(target: "sha256", "self_test: TransformD64_2way mismatch");
                return false;
            }
        }
        // Test TransformD64_4way, if available.
        if let Some(func) = TRANSFORM_D64_4WAY {
            let mut out128 = [0u8; 128];
            func(out128.as_mut_ptr(), DATA.as_ptr().add(1));
            if &out128[..] != &RESULT_D64[..128] {
                trace!(target: "sha256", "self_test: TransformD64_4way mismatch");
                return false;
            }
        }
        // Test TransformD64_8way, if available.
        if let Some(func) = TRANSFORM_D64_8WAY {
            let mut out256 = [0u8; 256];
            func(out256.as_mut_ptr(), DATA.as_ptr().add(1));
            if &out256[..] != &RESULT_D64[..] {
                trace!(target: "sha256", "self_test: TransformD64_8way mismatch");
                return false;
            }
        }
    }

    trace!(target: "sha256", "self_test: all tests passed");
    true
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sha256_reference_vector_validation {
    use super::*;

    /// Validate that `self_test` passes *and* that its result is `true`.
    #[traced_test]
    fn bitcoin_core_vectors_match() {
        assert!(
            self_test(),
            "internal self_test failed: compression or digest mismatch"
        );
    }
}
