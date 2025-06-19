// ---------------- [ File: bitcoin-aes/src/ctaes_sub_bytes.rs ]
crate::ix!();

/// Apply the Boyar–Peralta depth‑16 S‑box to all 16 parallel bytes of `state`.
///
/// * `inverse == false` → forward AES S‑box  
/// * `inverse == true`  → inverse AES S‑box
///
/// The algorithm is a **bit‑for‑bit** translation of the reference C
/// implementation published in ePrint 2011/332.
///
/// based on the gate logic from:
///
/// Joan Boyar and Rene Peralta, A depth-16
/// circuit for the AES S-box. https://eprint.iacr.org/2011/332.pdf
///
pub(crate) fn sub_bytes(state: &mut AESState, inverse: bool) {
    trace!(inverse, "sub_bytes – entry");

    /* ---------------------------------------------------------------------
     * Input unpacking
     * ------------------------------------------------------------------ */
    let (u0, u1, u2, u3, u4, u5, u6, u7) = {
        let s = state.slice;
        (s[7], s[6], s[5], s[4], s[3], s[2], s[1], s[0])
    };

    /* ---------------------------------------------------------------------
     * Linear pre‑/post‑processing
     * ------------------------------------------------------------------ */
    #[allow(non_snake_case, clippy::many_single_char_names)]
    #[rustfmt::skip]
    let (
        mut T1,  mut T2,  mut T3,  mut T4,  mut T5,  mut T6,  mut T7,
        mut T8,  mut T9,  mut T10, mut T11, mut T12, mut T13, mut T14,
        mut T15, mut T16, mut T17, mut T18, mut T19, mut T20, mut T21,
        mut T22, mut T23, mut T24, mut T25, mut T26, mut T27, mut D
    ) = (
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16   // ← 28 zeros
    );

    if inverse {
        /* ---- undo linear post‑processing (EXACT copy of ePrint 2011/332) -- */
        T23 =  u0 ^  u3;
        T22 = !(u1 ^  u3);
        T2  = !(u0 ^  u1);
        T1  =  u3 ^  u4;
        T24 = !(u4 ^  u7);

        let R5  =  u6 ^  u7;
        T8  = !(u1 ^ T23);
        T19 =  T22 ^ R5;
        T9  = !(u7 ^  T1);
        T10 =  T2  ^  T24;
        T13 =  T2  ^  R5;
        T3  =  T1  ^  R5;
        T25 = !(u2 ^  T1);

        let R13 =  u1 ^  u6;
        T17 = !(u2 ^  T19);
        T20 =  T24 ^  R13;
        T4  =  u4 ^  T8;

        let R17 = !(u2 ^  u5);
        let R18 = !(u5 ^  u6);
        let R19 = !(u2 ^  u4);

        D   =  u0 ^  R17;
        T6  =  T22 ^  R17;
        T16 =  R13 ^  R19;
        T27 =  T1  ^  R18;
        T15 =  T10 ^  T27;
        T14 =  T10 ^  R18;
        T26 =  T3  ^  T16;
    } else {
        /* ---- linear pre‑processing -------------------------------------- */
        T1  = u0 ^ u3;
        T2  = u0 ^ u5;
        T3  = u0 ^ u6;
        T4  = u3 ^ u5;
        T5  = u4 ^ u6;
        T6  = T1 ^ T5;
        T7  = u1 ^ u2;
        T8  = u7 ^ T6;
        T9  = u7 ^ T7;
        T10 = T6 ^ T7;
        T11 = u1 ^ u5;
        T12 = u2 ^ u5;
        T13 = T3 ^ T4;
        T14 = T6 ^ T11;
        T15 = T5 ^ T11;
        T16 = T5 ^ T12;
        T17 = T9 ^ T16;
        T18 = u3 ^ u7;
        T19 = T7 ^ T18;
        T20 = T1 ^ T19;
        T21 = u6 ^ u7;
        T22 = T7 ^ T21;
        T23 = T2 ^ T22;
        T24 = T2 ^ T10;
        T25 = T20 ^ T17;
        T26 = T3 ^ T16;
        T27 = T1 ^ T12;
        D   = u7;
    }

    /* ---------------------------------------------------------------------
     * Shared non‑linear core
     * ------------------------------------------------------------------ */
    #[allow(non_snake_case, clippy::many_single_char_names)]
    #[rustfmt::skip]
    let (
        M1,  M6,  M11, M13, M15, M20, M21, M22, M23, M25, M37, M38, M39, M40,
        M41, M42, M43, M44, M45, M46, M47, M48, M49, M50, M51, M52, M53, M54,
        M55, M56, M57, M58, M59, M60, M61, M62, M63
    ) = {
        let m1  = T13 & T6;
        let m6  = T3  & T16;
        let m11 = T1  & T15;
        let m13 = (T4 & T27) ^ m11;
        let m15 = (T2 & T10) ^ m11;
        let m20 = T14 ^ m1 ^ (T23 & T8) ^ m13;
        let m21 = (T19 & D) ^ m1 ^ T24 ^ m15;
        let m22 = T26 ^ m6 ^ (T22 & T9) ^ m13;
        let m23 = (T20 & T17) ^ m6 ^ m15 ^ T25;
        let m25 = m22 & m20;
        let m37 = m21 ^ ((m20 ^ m21) & (m23 ^ m25));
        let m38 = m20 ^ m25 ^ (m21 | (m20 & m23));
        let m39 = m23 ^ ((m22 ^ m23) & (m21 ^ m25));
        let m40 = m22 ^ m25 ^ (m23 | (m21 & m22));
        let m41 = m38 ^ m40;
        let m42 = m37 ^ m39;
        let m43 = m37 ^ m38;
        let m44 = m39 ^ m40;
        let m45 = m42 ^ m41;
        let m46 = m44 & T6;
        let m47 = m40 & T8;
        let m48 = m39 & D;
        let m49 = m43 & T16;
        let m50 = m38 & T9;
        let m51 = m37 & T17;
        let m52 = m42 & T15;
        let m53 = m45 & T27;
        let m54 = m41 & T10;
        let m55 = m44 & T13;
        let m56 = m40 & T23;
        let m57 = m39 & T19;
        let m58 = m43 & T3;
        let m59 = m38 & T22;
        let m60 = m37 & T20;
        let m61 = m42 & T1;
        let m62 = m45 & T4;
        let m63 = m41 & T2;
        (
            m1,  m6,  m11, m13, m15, m20, m21, m22, m23, m25, m37, m38, m39, m40,
            m41, m42, m43, m44, m45, m46, m47, m48, m49, m50, m51, m52, m53, m54,
            m55, m56, m57, m58, m59, m60, m61, m62, m63
        )
    };

    /* ---------------------------------------------------------------------
     * Final linear layer (direction‑dependent)
     * ------------------------------------------------------------------ */
    #[allow(non_snake_case, clippy::many_single_char_names)]
    if inverse {
        /* ---- undo linear preprocessing ---------------------------------- */
        let p0  = M52 ^ M61;
        let p1  = M58 ^ M59;
        let p2  = M54 ^ M62;
        let p3  = M47 ^ M50;
        let p4  = M48 ^ M56;
        let p5  = M46 ^ M51;
        let p6  = M49 ^ M60;
        let p7  = p0 ^ p1;
        let p8  = M50 ^ M53;
        let p9  = M55 ^ M63;
        let p10 = M57 ^ p4;
        let p11 = p0 ^ p3;
        let p12 = M46 ^ M48;
        let p13 = M49 ^ M51;
        let p14 = M49 ^ M62;
        let p15 = M54 ^ M59;
        let p16 = M57 ^ M61;
        let p17 = M58 ^ p2;
        let p18 = M63 ^ p5;
        let p19 = p2 ^ p3;
        let p20 = p4 ^ p6;
        let p22 = p2 ^ p7;
        let p23 = p7 ^ p8;
        let p24 = p5 ^ p7;
        let p25 = p6 ^ p10;
        let p26 = p9 ^ p11;
        let p27 = p10 ^ p18;
        let p28 = p11 ^ p25;
        let p29 = p15 ^ p20;

        state.slice = [
            p9  ^ p16,  /* bit‑slice 0 */
            p14 ^ p23,  /* bit‑slice 1 */
            p19 ^ p24,  /* bit‑slice 2 */
            p23 ^ p27,  /* bit‑slice 3 */
            p12 ^ p22,  /* bit‑slice 4 */
            p17 ^ p28,  /* bit‑slice 5 */
            p26 ^ p29,  /* bit‑slice 6 */
            p13 ^ p22,  /* bit‑slice 7 */
        ];
    } else {
        /* ---- linear post‑processing ------------------------------------- */
        let l0  = M61 ^ M62;
        let l1  = M50 ^ M56;
        let l2  = M46 ^ M48;
        let l3  = M47 ^ M55;
        let l4  = M54 ^ M58;
        let l5  = M49 ^ M61;
        let l6  = M62 ^ l5;
        let l7  = M46 ^ l3;
        let l8  = M51 ^ M59;
        let l9  = M52 ^ M53;
        let l10 = M53 ^ l4;
        let l11 = M60 ^ l2;
        let l12 = M48 ^ M51;
        let l13 = M50 ^ l0;
        let l14 = M52 ^ M61;
        let l15 = M55 ^ l1;
        let l16 = M56 ^ l0;
        let l17 = M57 ^ l1;
        let l18 = M58 ^ l8;
        let l19 = M63 ^ l4;
        let l20 = l0 ^ l1;
        let l21 = l1 ^ l7;
        let l22 = l3 ^ l12;
        let l23 = l18 ^ l2;
        let l24 = l15 ^ l9;
        let l25 = l6  ^ l10;
        let l26 = l7  ^ l9;
        let l27 = l8  ^ l10;
        let l28 = l11 ^ l14;
        let l29 = l11 ^ l17;

        state.slice = [
            !(l6 ^ l23),  /* bit‑slice 0 */
            !(l13 ^ l27), /* bit‑slice 1 */
            l25 ^ l29,    /* bit‑slice 2 */
            l20 ^ l22,    /* bit‑slice 3 */
            l6  ^ l21,    /* bit‑slice 4 */
            !(l19 ^ l28), /* bit‑slice 5 */
            !(l16 ^ l26), /* bit‑slice 6 */
            l6  ^ l24,    /* bit‑slice 7 */
        ];
    }

    debug!(?state.slice, "sub_bytes – exit");
}

#[cfg(test)]
mod aes_sbox_exhaustive_tests {
    use super::*;

    /// Randomised sanity check: forward → inverse must be the identity.
    #[traced_test]
    fn random_roundtrip_identity() {
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let original = AESState::random(&mut rng);
            let mut state = original.clone();
            sub_bytes(&mut state, false);
            sub_bytes(&mut state, true);
            assert_eq!(state.slice(), original.slice(), "random state failed");
        }
    }

    /// Forward AES S‑box (FIPS‑197, §5.1.1).
    const SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];

    /// Inverse AES S‑box.
    const INV_SBOX: [u8; 256] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
        0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde,
        0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42,
        0xfa, 0xc3, 0x4e, 0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
        0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c,
        0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15,
        0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, 0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7,
        0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
        0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc,
        0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, 0x96, 0xac, 0x74, 0x22, 0xe7, 0xad,
        0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d,
        0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
        0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, 0x1f, 0xdd, 0xa8,
        0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, 0x60, 0x51,
        0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0,
        0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
        0x7d,
    ];

    /* ------------------------- helper: pack / unpack -------------------- */

    /// Convert 16 bytes into a bit‑sliced `AESState`.
    fn pack_bytes(bytes: &[u8; 16]) -> AESState {
        let mut slice = [0u16; 8];
        for bit in 0..8 {
            let mut word = 0u16;
            for lane in 0..16 {
                if (bytes[lane] >> bit) & 1 == 1 {
                    word |= 1 << lane;
                }
            }
            slice[bit] = word;
        }
        AESState::from_slice(slice)
    }

    /// Convert a bit‑sliced `AESState` back to its 16 bytes.
    fn unpack_state(state: &AESState) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        for lane in 0..16 {
            let mut value = 0u8;
            for bit in 0..8 {
                if (state.slice()[bit] >> lane) & 1 == 1 {
                    value |= 1 << bit;
                }
            }
            bytes[lane] = value;
        }
        bytes
    }

    /* ----------------------------- exhaustive tests ---------------------- */

    #[traced_test]
    fn forward_matches_reference() {
        for x in 0u8..=255 {
            let input = [x; 16];
            let mut state = pack_bytes(&input);
            sub_bytes(&mut state, false); // forward S‑box
            for &out in &unpack_state(&state) {
                assert_eq!(out, SBOX[x as usize], "forward S‑box({x:#04x})");
            }
        }
    }

    #[traced_test]
    fn inverse_matches_reference() {
        for x in 0u8..=255 {
            let input = [x; 16];
            let mut state = pack_bytes(&input);
            sub_bytes(&mut state, true); // inverse S‑box
            for &out in &unpack_state(&state) {
                assert_eq!(out, INV_SBOX[x as usize], "inverse S‑box({x:#04x})");
            }
        }
    }

    #[traced_test]
    fn forward_then_inverse_is_identity() {
        for x in 0u8..=255 {
            let input = [x; 16];
            let mut state = pack_bytes(&input);
            sub_bytes(&mut state, false);
            sub_bytes(&mut state, true);
            assert_eq!(
                unpack_state(&state),
                input,
                "round‑trip failed for byte {x:#04x}"
            );
        }
    }

    #[traced_test]
    fn inverse_then_forward_is_identity() {
        for x in 0u8..=255 {
            let input = [x; 16];
            let mut state = pack_bytes(&input);
            sub_bytes(&mut state, true);
            sub_bytes(&mut state, false);
            assert_eq!(
                unpack_state(&state),
                input,
                "round‑trip failed for byte {x:#04x}"
            );
        }
    }
}
