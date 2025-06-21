// ---------------- [ File: bitcoin-aes/src/key_setup_transform.rs ]
crate::ix!();

/**
 | Rotate the rows in `s` upward by **one** byte (`RotWord`)
 | and XOR with `r`.
 |
 | With the crate’s **column‑major** lane numbering
 |
 | ```text
 | lane = col · 4 + row
 | ```
 |
 | every 4‑bit nibble encodes one column:
 |
 | ```text
 | bit 0 ≙ row 0   bit 1 ≙ row 1
 | bit 2 ≙ row 2   bit 3 ≙ row 3
 | ```
 |
 | `RotWord` therefore becomes **a right‑rotate by one bit
 | inside every nibble** (a₀ → a₃, a₁ → a₀, …),
 | *not* a left‑rotate.
 */
#[inline(always)]
pub fn key_setup_transform(s: *mut AESState, r: *const AESState) {
    tracing::trace!(
        target: "aes",
        "key_setup_transform – s {:p}, r {:p}",
        s,
        r
    );

    unsafe {
        for bit in 0..8 {
            let v = (*s).slice[bit];

            /* ---- per‑column RotWord (a₀ a₁ a₂ a₃ → a₁ a₂ a₃ a₀) ------ */
            let mut rotated = 0u16;
            for col in 0..4 {
                let shift  = col * 4;
                let nib    = (v >> shift) & 0x000F;                // a₃ a₂ a₁ a₀
                // right‑rotate by 1 bit inside nibble
                let rot    = ((nib >> 1) | (nib << 3)) & 0x000F;   // a₀ a₃ a₂ a₁ → a₁ a₂ a₃ a₀
                rotated   |= rot << shift;
            }

            /* ---- XOR with rcon slice --------------------------------- */
            (*s).slice[bit] = rotated ^ (*r).slice[bit];
        }
    }

    tracing::trace!(target: "aes", "key_setup_transform – exit");
}

#[cfg(test)]
mod key_schedule_transform_validation {
    use super::*;
    use rand::{thread_rng, Rng};

    /// `key_setup_transform` must:
    /// 1. rotate each AES column upward by one byte
    ///    (`RotWord` ≙ right‑rotate *one* bit in the nibble), then
    /// 2. XOR with the corresponding slice of `r`.
    #[traced_test]
    fn rotates_rows_and_xors_once() {
        let mut rng = thread_rng();

        for _ in 0..2_048 {
            let mut s      = AESState::random(&mut rng);
            let     r      = AESState::random(&mut rng);
            let mut expect = AESState::default();

            for bit in 0..8 {
                let v = s.slice()[bit];
                let mut rot = 0u16;
                for col in 0..4 {
                    let shift = col * 4;
                    let nib   = (v >> shift) & 0x000F;
                    let rr    = ((nib >> 1) | (nib << 3)) & 0x000F; // ← NEW: right‑rotate
                    rot      |= rr << shift;
                }
                expect.slice[bit] = rot ^ r.slice()[bit];
            }

            unsafe { key_setup_transform(&mut s as *mut _, &r as *const _) };
            assert_eq!(
                s.slice(),
                expect.slice(),
                "RotWord/XOR mismatch after transform"
            );
        }
    }
}
