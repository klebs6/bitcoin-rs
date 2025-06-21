// ---------------- [ File: bitcoin-poly1305/src/final_carry_and_sub_p.rs ]
crate::ix!();

// bitcoin-poly1305/src/final_carry_and_sub_p.rs
#[inline(always)]
pub(crate) fn final_carry_and_sub_p(h: &mut [u32; 5]) {
    // 1. finish the single carry‑propagation pass
    propagate_26bit_carries_once(h);

    // 2. compute g = h + 5  and  g_minus_p = g − p
    let (_g, g_minus_p, borrow) = compute_g_plus5_minus_p(h);

    // 3. constant‑time selection
    //    borrow == 0  →  g wrapped past p  →  take g_minus_p
    //    borrow == 1  →  no wrap           →  keep the original h
    let mask = borrow.wrapping_sub(1);      // 0xFFFF_FFFF if borrow==0 else 0
    ct_select_limbs(h, &g_minus_p, mask);   // one *single* selection, no g

    tracing::debug!(h_reduced = ?*h, "final_carry_and_sub_p: finished");
}

#[cfg(test)]
mod tests_final_and_tag {
    use super::*;
    use hex_literal::hex;
    use proptest::prelude::*;

    #[traced_test]
    fn rfc7539_vector_1_tag_matches() {
        let key = hex!("85d6be7857556d337f4452fe42d506a80103808afb0db2fd4abff6af4149f51b");
        let msg = b"Cryptographic Forum Research Group";
        let expected = hex!("a8061dc1305136c6c22b8baf0c0127a9");

        let mut tag = [0u8; POLY1305_TAGLEN];
        poly1305_auth(&mut tag, msg, &key);
        assert_eq!(tag, expected);
    }

    #[traced_test]
    fn output_limbs_all_below_2pow26() {
        let mut h = [0x3ffffff + 123, 0x3ffffff + 987, 0x3ffffff + 1, 0x3ffffff, 0x3ffffff];
        final_carry_and_sub_p(&mut h);
        for &limb in &h {
            assert!(limb < (1 << 26), "limb should be reduced below 2^26");
        }
    }

    /// Property‑based verification: any overflowing limb‑array is reduced.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]

        #[traced_test]
        fn reduces_all_limbs(
            mut h in proptest::array::uniform5(0u32..(1u32 << 30)), // deliberately allow overflow
        ) {
            final_carry_and_sub_p(&mut h);
            for &limb in &h {
                prop_assert!(limb < (1 << 26));
            }
        }
    }

    #[traced_test]
    fn simple_range_check() {
        let mut h = [0x3ffffff + 17, 0x3ffffff + 3, 0, 1, 2];
        final_carry_and_sub_p(&mut h);
        for &l in &h { assert!(l < (1 << 26)); }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]

        #[traced_test]
        fn always_reduces(
            mut h in prop::array::uniform5(0u32..(1u32<<30))  // allow deliberate overflow
        ) {
            final_carry_and_sub_p(&mut h);
            prop_assert!(h.iter().all(|&l| l < (1<<26)));
        }
    }
}
