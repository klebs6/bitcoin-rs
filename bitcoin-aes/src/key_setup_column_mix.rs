// ---------------- [ File: bitcoin-aes/src/key_setup_column_mix.rs ]
crate::ix!();

/// Key‑schedule column mix (`column_c1(r) |= (column_0(s) ^= column_c2(a))`)
#[inline(always)]
pub fn key_setup_column_mix(
    s:  *mut AESState,
    r:  *mut AESState,
    a:  *const AESState,
    c1: i32,
    c2: i32,
) {
    tracing::trace!(
        target: "aes",
        "key_setup_column_mix – s {:p}, r {:p}, a {:p}, c1 = {}, c2 = {}",
        s, r, a, c1, c2
    );

    debug_assert!(
        (0..=3).contains(&c1) && (0..=3).contains(&c2),
        "column indices must be 0‑3 (got c1 = {c1}, c2 = {c2})"
    );

    unsafe {
        let shift_src  = (c2 * 4) as u32;  // column‑major nibble size
        let shift_dest = (c1 * 4) as u32;

        for b in 0..8 {
            let tmp  = ((*a).slice[b] >> shift_src) & 0x000F;  // source column in low nibble
            (*s).slice[b] ^= tmp;                              // XOR into working state
            let low  = (*s).slice[b] & 0x000F;                 // keep column 0 nibble only
            (*r).slice[b] |= low << shift_dest;                // write into target column
        }
    }
}

#[cfg(test)]
mod key_schedule_column_mix_validation {
    use super::*;

    /// Exhaustive sanity: for **every** pair (c1,c2) ∈ 0‑3, the helper must
    /// satisfy the documented bit‑twiddle semantics under **randomised**
    /// inputs for both `s` and `r`.
    #[traced_test]
    fn mixes_columns_according_to_spec() {
        let mut rng = thread_rng();

        // iterate through all column pairs once per random instance
        for _case in 0..1_024 {
            let a = AESState::random(&mut rng);

            for c1 in 0..4 {
                for c2 in 0..4 {
                    let mut s      = AESState::random(&mut rng);
                    let mut r      = AESState::random(&mut rng);
                    let mut ref_s  = s.clone();
                    let mut ref_r  = r.clone();

                    /* ---------- reference transform (safe Rust) ---------- */
                    for b in 0..8 {
                        let tmp = (a.slice()[b] >> (c2 * 4)) & 0x000F;
                        ref_s.slice[b] ^= tmp;
                        let low = ref_s.slice[b] & 0x000F;
                        ref_r.slice[b] |= low << (c1 * 4);
                    }

                    /* ---------- function under test --------------------- */
                    unsafe {
                        key_setup_column_mix(
                            &mut s as *mut _,
                            &mut r as *mut _,
                            &a  as *const _,
                            c1,
                            c2,
                        );
                    }

                    trace!(
                        target: "test",
                        c1, c2,
                        ?ref_s.slice, ?s.slice,
                        ?ref_r.slice, ?r.slice,
                        "column‑mix comparison"
                    );
                    assert_eq!(s.slice(), ref_s.slice(), "state `s` mismatch for (c1={c1}, c2={c2})");
                    assert_eq!(r.slice(), ref_r.slice(), "state `r` mismatch for (c1={c1}, c2={c2})");
                }
            }
        }
    }
}
