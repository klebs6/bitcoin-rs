// ---------------- [ File: bitcoin-aes/src/ctaes_mix_columns.rs ]
crate::ix!();

impl AESState {
    /// Apply the (inverse) MixColumns transform in‑place.
    /// * `inv == false` → forward transform
    /// * `inv == true`  → inverse transform
    #[inline(always)]
    pub fn mix_columns(&mut self, inv: bool) {
        /* ---- 1. unpack bit‑sliced state -------------------------------- */
        let mut bytes = [0u8; 16];
        unsafe { crate::save_bytes(bytes.as_mut_ptr(), self as *const _); }

        /* ---- 2. per‑column GF(2⁸) matrix multiply ---------------------- */
        #[inline(always)]
        fn xtime(mut b: u8) -> u8 {
            let hi = b & 0x80;
            b <<= 1;
            if hi != 0 { b ^= 0x1B }
            b
        }
        #[inline(always)]
        fn mul(mut a: u8, mut k: u8) -> u8 {
            let mut res = 0;
            for _ in 0..8 {
                if (k & 1) != 0 { res ^= a; }
                let hi = a & 0x80;
                a = a << 1;
                if hi != 0 { a ^= 0x1B; }
                k >>= 1;
            }
            res
        }

        for col in 0..4 {
            let idx = col * 4;
            let [a0, a1, a2, a3] = [
                bytes[idx    ], bytes[idx + 1],
                bytes[idx + 2], bytes[idx + 3],
            ];

            let (b0, b1, b2, b3) = if !inv {
                // forward MixColumns (02 03 01 01)
                (
                    xtime(a0) ^ (xtime(a1) ^ a1) ^  a2 ^  a3,
                     a0       ^  xtime(a1)       ^ (xtime(a2) ^ a2) ^  a3,
                     a0       ^  a1              ^  xtime(a2)       ^ (xtime(a3) ^ a3),
                    (xtime(a0) ^ a0) ^  a1       ^  a2              ^  xtime(a3),
                )
            } else {
                // inverse MixColumns (0e 0b 0d 09)
                (
                    mul(a0,0x0e) ^ mul(a1,0x0b) ^ mul(a2,0x0d) ^ mul(a3,0x09),
                    mul(a0,0x09) ^ mul(a1,0x0e) ^ mul(a2,0x0b) ^ mul(a3,0x0d),
                    mul(a0,0x0d) ^ mul(a1,0x09) ^ mul(a2,0x0e) ^ mul(a3,0x0b),
                    mul(a0,0x0b) ^ mul(a1,0x0d) ^ mul(a2,0x09) ^ mul(a3,0x0e),
                )
            };

            bytes[idx    ] = b0;
            bytes[idx + 1] = b1;
            bytes[idx + 2] = b2;
            bytes[idx + 3] = b3;
        }

        /* ---- 3. re‑pack into the bit‑sliced state ---------------------- */
        self.slice = [0u16; 8];          // clear before ORing in new bits
        unsafe { crate::load_bytes(self as *mut _, bytes.as_ptr()); }
    }
}

#[cfg(test)]
mod mix_columns_tests {
    use super::*;

    #[traced_test]
    fn forward_then_inverse_is_identity() {
        let mut rng = rand::thread_rng();

        for _ in 0..1_000 {
            let sample: [u16; 8] = [
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            ];

            let mut state = AESStateBuilder::default().slice(sample).build().unwrap();
            state.mix_columns(false); // forward
            state.mix_columns(true);  // inverse
            assert_eq!(state.slice(), &sample);
        }
    }

    /* -------- helpers: pack / unpack <byte[16]> ↔ AESState ---------- */
    fn pack(bytes: &[u8; 16]) -> AESState {
        let mut slice = [0u16; 8];
        for bit in 0..8 {
            let mut w = 0u16;
            for lane in 0..16 {
                if (bytes[lane] >> bit) & 1 == 1 {
                    w |= 1 << lane;
                }
            }
            slice[bit] = w;
        }
        AESState::from_slice(slice)
    }
    fn unpack(state: &AESState) -> [u8; 16] {
        let mut out = [0u8; 16];
        for lane in 0..16 {
            let mut b = 0u8;
            for bit in 0..8 {
                if (state.slice()[bit] >> lane) & 1 == 1 {
                    b |= 1 << bit;
                }
            }
            out[lane] = b;
        }
        out
    }

    /* -------- reference field arithmetic over GF(2^8) --------------- */
    #[inline(always)]
    fn gf_mul(mut a: u8, mut b: u8) -> u8 {
        let mut r = 0;
        for _ in 0..8 {
            if (b & 1) != 0 { r ^= a; }
            let hi = a & 0x80;
            a <<= 1;
            if hi != 0 { a ^= 0x1B; }          // x^8 ⇒ x^4+x^3+x+1
            b >>= 1;
        }
        r
    }

    /* -------- 4×4 MixColumns matrices (column major) ---------------- */
    const FWD: [[u8; 4]; 4] = [
        [0x02, 0x03, 0x01, 0x01],
        [0x01, 0x02, 0x03, 0x01],
        [0x01, 0x01, 0x02, 0x03],
        [0x03, 0x01, 0x01, 0x02],
    ];
    const INV: [[u8; 4]; 4] = [
        [0x0e, 0x0b, 0x0d, 0x09],
        [0x09, 0x0e, 0x0b, 0x0d],
        [0x0d, 0x09, 0x0e, 0x0b],
        [0x0b, 0x0d, 0x09, 0x0e],
    ];

    /* -------- reference scalar MixColumns in column‑major order ------ */
    fn mix_ref(block: &[u8; 16], inv: bool) -> [u8; 16] {
        let m = if inv { &INV } else { &FWD };
        let mut out = [0u8; 16];

        for col in 0..4 {
            // load one column   a0..a3
            let a = [
                block[col * 4 + 0],
                block[col * 4 + 1],
                block[col * 4 + 2],
                block[col * 4 + 3],
            ];
            for row in 0..4 {
                let mut acc = 0u8;
                for k in 0..4 {
                    acc ^= gf_mul(m[row][k], a[k]);
                }
                out[col * 4 + row] = acc;
            }
        }
        out
    }

    /* =================================================================
     *  1. Exhaustive test: 16 lanes × 256 byte values  = 4096 cases
     * ================================================================= */
    #[traced_test]
    fn forward_and_inverse_match_reference_exhaustive() {
        for lane in 0..16 {
            for byte in 0u8..=0xFF {
                let mut input = [0u8; 16];
                input[lane] = byte;

                /* --- FORWARD ---------------------------------------- */
                let expected = mix_ref(&input, false);
                let mut state = pack(&input);
                state.mix_columns(false);          // ← function under test
                assert_eq!(unpack(&state), expected,
                    "forward MC mismatch (lane {lane}, byte 0x{byte:02x})");

                /* --- INVERSE ---------------------------------------- */
                let expected_inv = mix_ref(&input, true);
                let mut state = pack(&input);
                state.mix_columns(true);
                assert_eq!(unpack(&state), expected_inv,
                    "inverse MC mismatch (lane {lane}, byte 0x{byte:02x})");
            }
        }
    }

    /* =================================================================
     *  2. Round‑trip identity on 10 000 random states
     * ================================================================= */
    #[traced_test]
    fn forward_then_inverse_is_identity_randomised() {
        let mut rng = thread_rng();
        for _ in 0..10_000 {
            let mut st = AESState::random(&mut rng);
            let original = st.clone();
            st.mix_columns(false);
            st.mix_columns(true);
            assert_eq!(st.slice(), original.slice(),
                "MC round‑trip failed on random state");
        }
    }
}
