// ---------------- [ File: bitcoin-aes/src/key_setup_transform.rs ]
crate::ix!();

/// Rotate the rows in s one position upwards, and xor in r
#[inline(always)]
pub fn key_setup_transform(s: *mut AESState, r: *const AESState) {
    tracing::trace!(
        target: "aes",
        "key_setup_transform – s {:p}, r {:p}",
        s,
        r
    );

    unsafe {
        for b in 0..8 {
            let v = (*s).slice[b];
            (*s).slice[b] = ((v >> 4) | (v << 12)) ^ (*r).slice[b];
        }
    }
}

#[cfg(test)]
mod key_schedule_transform_validation {
    use super::*;

    /// The helper must perform a one‑row upward rotation (nibble‑wise) and then
    /// XOR with `r`, slice‑by‑slice.
    #[traced_test]
    fn rotates_and_xors_exactly_once() {
        let mut rng = thread_rng();

        for _ in 0..2_048 {
            let mut s     = AESState::random(&mut rng);
            let     r     = AESState::random(&mut rng);
            let mut ref_s = AESState::default();

            // Reference transform.
            for b in 0..8 {
                let v = s.slice()[b];
                ref_s.slice[b] = ((v >> 4) | (v << 12)) ^ r.slice()[b];
            }

            // Function under test.
            unsafe { key_setup_transform(&mut s as *mut _, &r as *const _) };

            debug!(?ref_s.slice, ?s.slice);
            assert_eq!(s.slice(), ref_s.slice(), "transform mismatch");
        }
    }
}
