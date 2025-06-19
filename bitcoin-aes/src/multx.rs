// ---------------- [ File: bitcoin-aes/src/multx.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/ctaes.c]

/// Multiply the cells in s by x, as polynomials over GF(2) mod x^8 + x^4 + x^3 + x + 1
///
/// Multiply state by x in GF(2⁸) mod x⁸ + x⁴ + x³ + x + 1.
///
#[inline(always)]
pub fn multx(s: *mut AESState) {
    tracing::trace!(target: "aes", "multx – entry {:p}", s);

    unsafe {
        let top = (*s).slice[7];
        (*s).slice[7] = (*s).slice[6];
        (*s).slice[6] = (*s).slice[5];
        (*s).slice[5] = (*s).slice[4];
        (*s).slice[4] = (*s).slice[3] ^ top;
        (*s).slice[3] = (*s).slice[2] ^ top;
        (*s).slice[2] = (*s).slice[1];
        (*s).slice[1] = (*s).slice[0] ^ top;
        (*s).slice[0] = top;
    }

    tracing::trace!(target: "aes", "multx – exit");
}

#[cfg(test)]
mod multx_validation {
    use super::*;

    // ------------ helper: (un)pack ---------------------------------------

    /// Pack 16 bytes into the bit‑sliced [`AESState`] format.
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

    /// Unpack a bit‑sliced [`AESState`] into its canonical 16‑byte form.
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

    /// Per‑byte GF(2⁸) multiplication by `x` (⟨02⟩) with
    /// modulus `x⁸ + x⁴ + x³ + x + 1` (0x1B).
    #[inline(always)]
    fn xtime(b: u8) -> u8 {
        let hi = b & 0x80;
        let mut out = b << 1;
        if hi != 0 {
            out ^= 0x1B;
        }
        out
    }

    /// Exhaustive byte‑wise verification of the **bitsliced** `multx`
    /// routine for 10 000 random states.
    #[traced_test]
    fn bitsliced_xtime_matches_scalar_reference() {
        let mut rng = thread_rng();

        for _ in 0..10_000 {
            // ----- random plaintext ------------------------------------
            let mut plain = [0u8; 16];
            rng.fill(&mut plain);

            // ----- expected ciphertext ---------------------------------
            let mut expected = [0u8; 16];
            for (&b, out) in plain.iter().zip(expected.iter_mut()) {
                *out = xtime(b);
            }

            // ----- bitsliced transform ---------------------------------
            let mut state = pack_bytes(&plain);
            unsafe { multx(&mut state as *mut _) };
            let actual = unpack_state(&state);

            debug!(target: "test", ?plain, ?expected, ?actual);
            assert_eq!(actual, expected, "GF(2⁸) xtime mismatch");
        }
    }
}
