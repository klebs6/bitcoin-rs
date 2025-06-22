// ---------------- [ File: bitcoin-poly1305/src/propagate_26bit_carries_once.rs ]
crate::ix!();

/// (1 << 26) − 1
const MASK: u32 = 0x03ff_ffff;

/// Propagate carries once.
///
/// * `h` – mutable reference to the 5‑limb vector.
///
/// This routine is **constant–time** with respect to the limb values
/// (no data‑dependent branching).
#[inline]
pub fn propagate_26bit_carries_once(h: &mut [u32; 5]) {
    // ------------------------------------------------------------------
    // Step 0 : initial state
    // ------------------------------------------------------------------
    trace_step!("prop‑start", { h = ?h });

    // ------------------------------------------------------------------
    // Limb‑by‑limb carries: 0 → 1 → 2 → 3 → 4
    // ------------------------------------------------------------------
    // —— carry out of limb 0 ——
    let c = h[0] >> 26;
    let h0_before = h[0];
    h[0] &= MASK;
    h[1] = h[1].wrapping_add(c);
    trace_step!("prop‑c0", { c, h0_before, h0_after = h[0] });

    // —— carry out of limb 1 ——
    let c = h[1] >> 26;
    let h1_before = h[1];
    h[1] &= MASK;
    h[2] = h[2].wrapping_add(c);
    trace_step!("prop‑c1", { c, h1_before });

    // —— carry out of limb 2 ——
    let c = h[2] >> 26;
    let h2_before = h[2];
    h[2] &= MASK;
    h[3] = h[3].wrapping_add(c);
    trace_step!("prop‑c2", { c, h2_before });

    // —— carry out of limb 3 ——
    let c = h[3] >> 26;
    let h3_before = h[3];
    h[3] &= MASK;
    h[4] = h[4].wrapping_add(c);
    trace_step!("prop‑c3", { c, h3_before });

    // ------------------------------------------------------------------
    // Top‑limb wrap‑around (2^130 ≡ 5 mod p)
    // ------------------------------------------------------------------
    let c = h[4] >> 26;                 // overflow from limb 4
    let h4_before = h[4];
    h[4] &= MASK;
    h[0] = h[0].wrapping_add(c.wrapping_mul(5));
    trace_step!("prop‑c4", { c, h4_before });

    // ------------------------------------------------------------------
    // One extra fix‑up carry for h[0]  →  h[1]
    // (after adding 5·c, h0 may now be ≥ 2^26).
    // A single extra pass is sufficient: 5 ≤ 2^26, so the carry is ≤ 4.
    // ------------------------------------------------------------------
    let c = h[0] >> 26;
    let h0_before_fix = h[0];
    h[0] &= MASK;
    h[1] = h[1].wrapping_add(c);
    trace_step!("prop‑c5", { c, h0_before_fix });

    // ------------------------------------------------------------------
    // Done
    // ------------------------------------------------------------------
    trace_step!("prop‑done", { h = ?h });
}

#[cfg(test)]
mod tests_propagate_26bit_carries {
    use super::*;
    use proptest::prelude::*;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    /// Convert 5‑limb (26‑bit) representation into an arbitrary‑precision integer.
    fn big_from_limbs(h: &[u32; 5]) -> BigUint {
        let mut acc = BigUint::zero();
        for (i, &limb) in h.iter().enumerate() {
            let shift = 26 * i;
            let part  = BigUint::from(limb) << shift;
            acc += part;
        }
        acc
    }

    // --- deterministic edge cases ---------------------------------------

    #[traced_test]
    fn all_zero_is_stable() {
        let mut h = [0u32; 5];
        propagate_26bit_carries_once(&mut h);
        assert_eq!(h, [0u32; 5]);
    }

    #[traced_test]
    fn single_overflow_in_h4_folds_back() {
        // h₄ = (1<<26) + 3 should roll c=1 into h₀.
        let mut h = [7, 0, 0, 0, (1<<26) + 3];
        propagate_26bit_carries_once(&mut h);
        assert_eq!(h[4], 3);
        assert_eq!(h[0], 7 + 5); // +5 from carry·5
    }

    // --- property‑based verification ------------------------------------

    // --- helper: convert limbs → BigUint mod p --------------------------
    fn limbs_to_biguint_mod_p(h: &[u32; 5]) -> BigUint {
        const P: &str = "1361129467683753853853498429727072845819"; // 2^130 − 5
        let p = BigUint::parse_bytes(P.as_bytes(), 10).unwrap();

        let mut acc = BigUint::zero();
        let mut pow = BigUint::from(1u32);

        for &limb in h.iter() {
            acc += &pow * BigUint::from(limb);
            pow <<= 26;                        // multiply by 2²⁶
        }
        acc % p
    }

    /// After one carry‑propagation pass:
    /// * the numerical value must stay **identical mod p**
    /// * every limb must be in range `[0, 2²⁶)`.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(600))]

        #[traced_test]
        fn prop_preserves_value_and_range(mut h in any::<[u32; 5]>()) {
            // reduce each random limb modulo 2²⁶ so we start in the
            // “allowed” super‑range the algorithm expects.
            for limb in &mut h {
                *limb &= (1 << 28) - 1;        // 28 bits so we get carry cases too
            }

            let before = limbs_to_biguint_mod_p(&h);
            propagate_26bit_carries_once(&mut h);
            let after  = limbs_to_biguint_mod_p(&h);

            // --- 1. Same value mod p
            prop_assert_eq!(before, after);

            // --- 2. each limb < 2²⁶
            prop_assert!(h.iter().all(|&x| x < (1 << 26)),
                         "limbs out of range: {:?}", h);
        }
    }
}
