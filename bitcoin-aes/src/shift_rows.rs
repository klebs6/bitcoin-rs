// ---------------- [ File: bitcoin-aes/src/shift_rows.rs ]
crate::ix!();

/// ShiftRows (forward)
#[inline(always)]
pub fn shift_rows(s: *mut AESState) {
    tracing::trace!(target: "aes", "shift_rows – entry {:p}", s);

    unsafe {
        for word in &mut (*s).slice {
            let v = *word;
            *word = (v & bit_range!(0, 4))
                | bit_range_left!(v, 4, 5, 3)
                | bit_range_right!(v, 5, 8, 1)
                | bit_range_left!(v, 8, 10, 2)
                | bit_range_right!(v, 10, 12, 2)
                | bit_range_left!(v, 12, 15, 1)
                | bit_range_right!(v, 15, 16, 3);
        }
    }

    tracing::trace!(target: "aes", "shift_rows – exit");
}

/// Inverse ShiftRows
#[inline(always)]
pub fn inv_shift_rows(s: *mut AESState) {
    tracing::trace!(target: "aes", "inv_shift_rows – entry {:p}", s);

    unsafe {
        for word in &mut (*s).slice {
            let v = *word;
            *word = (v & bit_range!(0, 4))
                | bit_range_left!(v, 4, 7, 1)
                | bit_range_right!(v, 7, 8, 3)
                | bit_range_left!(v, 8, 10, 2)
                | bit_range_right!(v, 10, 12, 2)
                | bit_range_left!(v, 12, 13, 3)
                | bit_range_right!(v, 13, 16, 1);
        }
    }

    tracing::trace!(target: "aes", "inv_shift_rows – exit");
}

#[cfg(test)]
mod shift_rows_validation {
    use super::*;

    // ------------ helper: (un)pack ---------------------------------------

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

    /// **Specification check**: forward `shift_rows` must rotate row *r* left
    /// by *r* bytes (FIPS‑197 §5.1.2).  Verified for 4 096 random states.
    #[traced_test]
    fn forward_matches_spec() {
        let mut rng = thread_rng();

        for _ in 0..4_096 {
            let mut plain = [0u8; 16];
            rng.fill(&mut plain);

            // Expected result via simple matrix rotation.
            let mut expect = [0u8; 16];
            for row in 0..4 {
                for col in 0..4 {
                    let src_idx = col * 4 + row;
                    let dst_idx = row * 4 + ((col + row) % 4);
                    expect[dst_idx] = plain[src_idx];
                }
            }

            let mut state = pack_bytes(&plain);
            unsafe { shift_rows(&mut state as *mut _) };
            let actual = unpack_state(&state);

            assert_eq!(actual, expect, "forward ShiftRows mismatch");
        }
    }

    /// **Specification check**: `inv_shift_rows` must exactly invert
    /// `shift_rows` for arbitrary data.
    #[traced_test]
    fn inverse_inverts_forward() {
        let mut rng = thread_rng();

        for _ in 0..4_096 {
            let mut bytes = [0u8; 16];
            rng.fill(&mut bytes);

            let mut state = pack_bytes(&bytes);
            unsafe { shift_rows(&mut state as *mut _) };
            unsafe { inv_shift_rows(&mut state as *mut _) };

            let out = unpack_state(&state);
            info!(target: "test", ?bytes, ?out);
            assert_eq!(out, bytes, "inv_shift_rows failed to invert shift_rows");
        }
    }
}
