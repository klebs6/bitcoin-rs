// ---------------- [ File: bitcoin-poly1305/src/ct_select_limbs.rs ]
crate::ix!();

/// In constant time replace *h* with *g* **iff** `select_mask == 0xFFFF_FFFF`.
///
/// `select_mask` is typically derived from the sign‑bit of the last limb
/// produced by `compute_g_plus5_minus_p`.
#[inline(always)]
pub fn ct_select_limbs(h: &mut LimbArr5, g: &LimbArr5, select_mask: u32) {
    let nmask = !select_mask;
    h[0] = (h[0] & nmask) | (g[0] & select_mask);
    h[1] = (h[1] & nmask) | (g[1] & select_mask);
    h[2] = (h[2] & nmask) | (g[2] & select_mask);
    h[3] = (h[3] & nmask) | (g[3] & select_mask);
    h[4] = (h[4] & nmask) | (g[4] & select_mask);
}

#[cfg(test)]
mod tests_ct_select_limbs {
    use super::*;
    use proptest::prelude::*;

    // deterministic sanity
    #[traced_test]
    fn select_all_or_none() {
        let g = [1,2,3,4,5];
        let mut h = [9,9,9,9,9];

        // keep h (mask=0)
        ct_select_limbs(&mut h, &g, 0x0000_0000);
        assert_eq!(h, [9,9,9,9,9]);

        // overwrite h (mask=all 1s)
        ct_select_limbs(&mut h, &g, 0xFFFF_FFFF);
        assert_eq!(h, g);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(300))]

        /// Behaviour must match a naïve branchy implementation.
        #[traced_test]
        fn prop_equivalence_to_branch(
            mut h in proptest::array::uniform5(0u32..(1u32<<26)),
            g in proptest::array::uniform5(0u32..(1u32<<26)),
            use_g in any::<bool>(),
        ) {
            let mask = if use_g { 0xFFFF_FFFF } else { 0 };
            let mut h_ref = h;

            // reference behaviour
            if use_g { h_ref = g; }

            // function under test
            ct_select_limbs(&mut h, &g, mask);

            prop_assert_eq!(h, h_ref);
        }
    }
}
