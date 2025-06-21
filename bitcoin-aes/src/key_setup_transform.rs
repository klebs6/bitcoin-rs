// ---------------- [ File: bitcoin-aes/src/key_setup_transform.rs ]
crate::ix!();

/// Rotate the rows in `s` one position upwards (`RotWord`) and XOR with `r`.
#[inline(always)]
pub fn key_setup_transform(s: *mut AESState, r: *const AESState) {
    tracing::trace!(
        target: "aes",
        "key_setup_transform – s {:p}, r {:p}",
        s, r
    );

    unsafe {
        for b in 0..8 {
            let v = (*s).slice[b];
            /*  --------  CHANGE  ---------
             * RotWord is a **left** rotation by one byte (= 4 lanes),
             * so we must rotate the 16‑bit word left by 4 bits.
             */
            (*s).slice[b] = ((v << 4) | (v >> 12)) ^ (*r).slice[b];
        }
    }
}

#[cfg(test)]
mod key_schedule_transform_validation {
    use super::*;

    /// `key_setup_transform` = left‑rotate by one nibble **then** XOR with `r`.
    #[traced_test]
    fn rotates_and_xors_exactly_once() {
        let mut rng = thread_rng();

        for _ in 0..2_048 {
            let mut s     = AESState::random(&mut rng);
            let     r     = AESState::random(&mut rng);
            let mut ref_s = AESState::default();

            for b in 0..8 {
                let v = s.slice()[b];
                ref_s.slice[b] = ((v << 4) | (v >> 12)) ^ r.slice()[b];
            }

            unsafe { key_setup_transform(&mut s as *mut _, &r as *const _) };

            assert_eq!(s.slice(), ref_s.slice(), "transform mismatch");
        }
    }
}
