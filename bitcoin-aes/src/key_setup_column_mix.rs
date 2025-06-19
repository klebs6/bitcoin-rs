// ---------------- [ File: bitcoin-aes/src/key_setup.rs ]
crate::ix!();

/// Key‑schedule column mix (`column_c1(r) |= (column_0(s) ^= column_c2(a))`)
#[inline(always)]
pub fn key_setup_column_mix(
    s: *mut AESState,
    r: *mut AESState,
    a: *const AESState,
    c1: i32,
    c2: i32,
) {
    tracing::trace!(
        target: "aes",
        "key_setup_column_mix – s {:p}, r {:p}, a {:p}, c1 = {}, c2 = {}",
        s,
        r,
        a,
        c1,
        c2
    );

    unsafe {
        for b in 0..8 {
            let tmp = ((*a).slice[b] >> c2 as u32) & 0x1111;
            (*s).slice[b] ^= tmp;
            (*r).slice[b] |= ((*s).slice[b] & 0x1111) << c1 as u32;
        }
    }
}

#[cfg(test)]
mod key_schedule_column_mix_validation {
    use super::*;

    /// Property‑based check: the low‑level word updates performed by
    /// `key_setup_column_mix` must exactly follow its documented bit‑twiddling
    /// logic for **all** combinations of `c1`/`c2` in 0‑3.
    #[traced_test]
    fn mixes_columns_according_to_spec() {
        let mut rng = thread_rng();

        for _ in 0..2_048 {
            let mut s     = AESState::random(&mut rng);
            let mut r     = AESState::random(&mut rng);
            let     a     = AESState::random(&mut rng);
            let mut s_ref = s.clone();
            let mut r_ref = r.clone();

            let c1 = rng.gen_range(0..4);
            let c2 = rng.gen_range(0..4);

            // Reference implementation in safe Rust.
            for b in 0..8 {
                let tmp = (a.slice()[b] >> c2) & 0x1111;
                s_ref.slice[b] ^= tmp;
                r_ref.slice[b] |= (s_ref.slice[b] & 0x1111) << c1;
            }

            // Function under test.
            unsafe {
                key_setup_column_mix(
                    &mut s as *mut _,
                    &mut r as *mut _,
                    &a  as *const _,
                    c1,
                    c2,
                );
            }

            debug!(target: "test", c1, c2, ?s_ref.slice, ?s.slice, ?r_ref.slice, ?r.slice);
            assert_eq!(s.slice(), s_ref.slice(), "state `s` mismatch");
            assert_eq!(r.slice(), r_ref.slice(), "state `r` mismatch");
        }
    }
}
