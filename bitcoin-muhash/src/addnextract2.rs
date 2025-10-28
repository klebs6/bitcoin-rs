// ---------------- [ File: bitcoin-muhash/src/addnextract2.rs ]
crate::ix!();

/**
  | Add limb a to [c0,c1]: [c0,c1] += a. Then
  | extract the lowest limb of [c0,c1] into
  | n, and left shift the number by 1 limb.
  |
  */
#[inline]
pub fn addnextract2(c0: &mut Limb, c1: &mut Limb, a: &Limb, n: &mut Limb) {

    trace!("addnextract2");

    let mut c2: Limb = 0;

    // add
    let (new_c0, carry0) = c0.overflowing_add(*a);

    *c0 = new_c0;

    if carry0 {

        let (new_c1, carry1) = c1.overflowing_add(1);

        *c1 = new_c1;

        // Handle case when c1 has overflown
        if carry1 {
            c2 = 1;
        }
    }

    // extract
    *n = *c0;
    *c0 = *c1;
    *c1 = c2;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    fn max() -> Limb { Limb::MAX }
    fn one() -> Limb { 1 as Limb }
    fn zero() -> Limb { 0 as Limb }

    #[test]
    fn addnextract2_no_carry() {
        let mut c0: Limb = 5;
        let mut c1: Limb = 7;
        let a: Limb = 3;
        let mut n: Limb = 999;

        addnextract2(&mut c0, &mut c1, &a, &mut n);

        assert_eq!(n, 8, "n should get original c0+a");
        assert_eq!(c0, 7, "c0 should shift in previous c1");
        assert_eq!(c1, 0, "c1 should become 0 when no double-carry");
    }

    #[test]
    fn addnextract2_single_carry_into_c1() {
        let mut c0 = max();
        let mut c1 = 10 as Limb;
        let a = one();
        let mut n = zero();

        addnextract2(&mut c0, &mut c1, &a, &mut n);

        assert_eq!(n, 0, "c0+1 wraps to 0");
        assert_eq!(c0, 11, "c1 incremented");
        assert_eq!(c1, 0, "no overflow into c2");
    }

    #[test]
    fn addnextract2_double_carry_into_c2() {
        let mut c0 = max();
        let mut c1 = max();
        let a = one();
        let mut n = 0;

        addnextract2(&mut c0, &mut c1, &a, &mut n);

        assert_eq!(n, 0, "low limb wraps");
        assert_eq!(c0, 0, "c1 wrapped to 0 and shifted into c0");
        assert_eq!(c1, 1, "c2 becomes 1 due to double carry");
    }

    #[test]
    fn addnextract2_randomized_equivalence() {
        // Reference model using generic arithmetic/overflow rules.
        let limb_bits = (mem::size_of::<Limb>() * 8) as u32;
        let mask = if limb_bits == 64 { u128::MAX } else { (1u128 << limb_bits) - 1 };

        let mut seed: u64 = 0x1234_5678_9ABC_DEF0;
        for _ in 0..64 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut c0 = (seed as u128 & mask) as Limb;

            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut c1 = (seed as u128 & mask) as Limb;

            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = (seed as u128 & mask) as Limb;

            let mut n_hw = 0 as Limb;
            let mut c0_hw = c0;
            let mut c1_hw = c1;

            // Run DUT
            addnextract2(&mut c0, &mut c1, &a, &mut n_hw);

            // Reference logic
            let (sum0, carry0) = c0_hw.overflowing_add(a);
            c0_hw = sum0;
            let mut c2_hw: Limb = 0;
            if carry0 {
                let (sum1, carry1) = c1_hw.overflowing_add(1);
                c1_hw = sum1;
                if carry1 { c2_hw = 1; }
            }
            let n_ref = c0_hw;
            let c0_ref = c1_hw;
            let c1_ref = c2_hw;

            assert_eq!((n_hw, c0, c1), (n_ref, c0_ref, c1_ref));
        }
    }
}
