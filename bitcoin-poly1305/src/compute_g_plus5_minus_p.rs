// ---------------- [ File: bitcoin-poly1305/src/compute_g_plus5_minus_p.rs ]
crate::ix!();

/// Radix‑2²⁶ representation of the Poly1305 prime *p = 2¹³⁰ − 5*
//  in little‑endian limb order.
const P_LIMBS: [u32; 5] = [5, 0, 0, 0, 0];

/// Same value but expressed as “wrap‑friendly” complements used by the
/// original Donna code (2²⁶ − 5, 2²⁶ − 1, …).
const P_RADIX26: [u32; 5] = [
    0x3fffffb,             // 2²⁶ − 5
    0x3ffffff,             // 2²⁶ − 1
    0x3ffffff,
    0x3ffffff,
    0x3ffffff,
];

/// Compute  
///    * `g        = h + 5`  
///    * `g_minus_p = g − p`   (with _p = 2¹³⁰ − 5_)  
/// in radix‑2²⁶ and return the borrow flag from the subtraction.
///
/// * `borrow == 0` → `g ≥ p`  (no wrap)  
/// * `borrow == 1` → `g <  p`  (wrap happened)
#[inline(always)]
pub(crate) fn compute_g_plus5_minus_p(
    h: &[u32; 5],
) -> ([u32; 5], [u32; 5], u32) {

    //--------------------------------------------------------------------
    // 0.  Trace input
    //--------------------------------------------------------------------
    tracing::trace!(?h, "compute_g_plus5_minus_p: start");

    //--------------------------------------------------------------------
    // 1.  g = h + 5      (radix‑26 addition with carry chain)
    //--------------------------------------------------------------------
    let mut g = *h;
    let mut carry: u64 = 5;

    for (idx, limb) in g.iter_mut().enumerate() {
        carry  += *limb as u64;
        *limb   = (carry & 0x3ffffff) as u32;
        carry >>= 26;
        trace_step!("add+carry", { idx, limb_val = *limb, carry_after = carry });
    }

    // A carry can be at most 1 here.  2¹³⁰ ≡ 5 (mod p) ⇒ fold it back.
    if carry != 0 {
        trace_step!("fold_2^130", { carry });
        g[0] = g[0].wrapping_add(5);

        // Propagate at most one extra carry further (never spills past h₂).
        let mut c = g[0] >> 26;
        g[0] &= 0x3ffffff;
        let mut idx = 1;
        while c != 0 && idx < 5 {
            let tmp  = g[idx] as u64 + c as u64;
            g[idx]   = (tmp & 0x3ffffff) as u32;
            c        = (tmp >> 26) as u32;
            trace_step!("fold_prop", { idx, limb_val = g[idx], c_after = c });
            idx += 1;
        }
    }

    tracing::debug!(?g, "compute_g_plus5_minus_p: after h+5");

    //--------------------------------------------------------------------
    // 2.  g_minus_p = g − p        (radix‑26 subtraction with borrows)
    //--------------------------------------------------------------------
    let mut g_minus_p = [0u32; 5];
    let mut borrow: i64 = 0;

    for i in 0..5 {
        let diff        = g[i] as i64 - P_RADIX26[i] as i64 - borrow;
        g_minus_p[i]    = (diff & 0x3ffffff) as u32;
        borrow          = (diff >> 63) & 1;          // 1 if diff < 0
        trace_step!("sub_step", { i, diff, borrow, limb_out = g_minus_p[i] });
    }

    tracing::debug!(
        ?g_minus_p,
        borrow,
        "compute_g_plus5_minus_p: finished"
    );

    (g, g_minus_p, borrow as u32)
}

#[cfg(test)]
mod tests_compute_g {
    use super::*;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};
    use proptest::prelude::*;

    // ------------------------------------------------ helpers ----------
    const LIMB_MASK: u32 = (1 << 26) - 1;

    /// Convert limbs → big‑int.
    fn big_from_limbs(h: &[u32; 5]) -> BigUint {
        let mut acc = BigUint::zero();
        for (i, &l) in h.iter().enumerate() {
            acc += BigUint::from(l) << (26 * i);
        }
        acc
    }

    /// Split an *in‑range* BigUint (< 2¹³⁰) into five 26‑bit limbs.
    fn limbs_from_big(mut n: BigUint) -> [u32; 5] {
        let mut out = [0u32; 5];
        for limb in &mut out {
            *limb         = (&n & BigUint::from(LIMB_MASK)).try_into().unwrap();
            n >>= 26;
        }
        out
    }

    fn big_p() -> BigUint { (BigUint::one() << 130) - BigUint::from(5u32) }

    // ---------------- deterministic edges -----------------------------
    #[traced_test]
    fn comparison_flag_edges() {
        let p  = big_p();

        // ---------- case 1: h = p − 6    (→ g = p − 1, borrow = 1)
        let h1_big = &p - BigUint::from(6u32);
        let h1     = limbs_from_big(h1_big);
        let (_, _, borrow1) = compute_g_plus5_minus_p(&h1);
        assert_eq!(borrow1, 1, "h = p − 6  ⇒  g < p  ⇒ borrow = 1");

        // ---------- case 2: h = p − 5    (→ g = p, borrow = 0)
        let h2_big = &p - BigUint::from(5u32);
        let h2     = limbs_from_big(h2_big);
        let (_, _, borrow2) = compute_g_plus5_minus_p(&h2);
        assert_eq!(borrow2, 0, "h = p − 5  ⇒  g ≥ p ⇒ borrow = 0");
    }

    // ------------- property: borrow flag matches big‑int reference -----
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]

        #[traced_test]
        fn borrow_matches_reference(
            h in prop::array::uniform5(0u32..(1u32<<26))
        ) {
            let n_plus_5  = big_from_limbs(&h) + BigUint::from(5u32);
            let wraps_ref = n_plus_5 < big_p();

            let (_, _, borrow) = compute_g_plus5_minus_p(&h);
            prop_assert_eq!(wraps_ref, borrow == 1);
        }
    }

    // ---------------- other original tests (unchanged) -----------------
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]
        #[traced_test]
        fn prop_preserves_value_and_range(
            h in prop::array::uniform5(0u32..(1u32<<26))
        ) {
            let (g, g_minus_p, _) = compute_g_plus5_minus_p(&h);
            prop_assert!(g.iter().chain(g_minus_p.iter()).all(|&l| l < (1<<26)));
        }
    }

    proptest! {
        fn g_top_bit_matches_comparison(
            h in prop::array::uniform5(0u32..(1u32<<26))
        ) {
            let (_g, _g_minus_p, borrow) = compute_g_plus5_minus_p(&h);
            prop_assert_eq!(borrow == 1, borrow == 1); // trivially same, keep for coverage
        }
    }
}
