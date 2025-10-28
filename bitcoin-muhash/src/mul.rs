// ---------------- [ File: bitcoin-muhash/src/mul.rs ]
crate::ix!();

/**
  | [c0,c1] = a * b
  |
  */
#[inline]
pub fn mul(c0: &mut Limb, c1: &mut Limb, a: &Limb, b: &Limb) {
    trace!("mul");
    let t: DoubleLimb = (*a as DoubleLimb) * (*b as DoubleLimb);
    *c1 = (t >> LIMB_SIZE) as Limb;
    *c0 = t as Limb;
}

/**
  | [c0,c1,c2] += n * [d0,d1,d2]. c2 is 0
  | initially
  |
  */
#[inline]
pub fn mulnadd3(
    c0: &mut Limb,
    c1: &mut Limb,
    c2: &mut Limb,
    d0: &mut Limb,
    d1: &mut Limb,
    d2: &mut Limb,
    n: &Limb,
) {
    trace!("mulnadd3");
    let mut t: DoubleLimb = (*d0 as DoubleLimb) * (*n as DoubleLimb) + *c0 as DoubleLimb;
    *c0 = t as Limb;
    t >>= LIMB_SIZE;
    t += (*d1 as DoubleLimb) * (*n as DoubleLimb) + *c1 as DoubleLimb;
    *c1 = t as Limb;
    t >>= LIMB_SIZE;
    *c2 = (t + (*d2 as DoubleLimb) * (*n as DoubleLimb)) as Limb;
}

/**
  | [c0,c1] *= n
  |
  */
#[inline]
pub fn muln2(c0: &mut Limb, c1: &mut Limb, n: &Limb) {
    trace!("muln2");
    let mut t: DoubleLimb = (*c0 as DoubleLimb) * (*n as DoubleLimb);
    *c0 = t as Limb;
    t >>= LIMB_SIZE;
    t += (*c1 as DoubleLimb) * (*n as DoubleLimb);
    *c1 = t as Limb;
}

/**
  | [c0,c1,c2] += a * b
  |
  */
#[inline]
pub fn muladd3(
    c0: &mut Limb,
    c1: &mut Limb,
    c2: &mut Limb,
    a: &Limb,
    b: &Limb,
) {
    trace!("muladd3");
    let t: DoubleLimb = (*a as DoubleLimb) * (*b as DoubleLimb);
    let th: Limb = (t >> LIMB_SIZE) as Limb;
    let tl: Limb = t as Limb;

    let (new_c0, carry0) = c0.overflowing_add(tl);
    *c0 = new_c0;
    let mut th = th + if carry0 { 1 } else { 0 };

    let (new_c1, carry1) = c1.overflowing_add(th);
    *c1 = new_c1;
    *c2 += if carry1 { 1 } else { 0 };
}

/**
  | [c0,c1,c2] += 2 * a * b
  |
  */
#[inline]
pub fn muldbladd3(
    c0: &mut Limb,
    c1: &mut Limb,
    c2: &mut Limb,
    a: &Limb,
    b: &Limb,
) {
    trace!("muldbladd3");
    // First add
    muladd3(c0, c1, c2, a, b);
    // Second add
    muladd3(c0, c1, c2, a, b);
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    fn limb_bits() -> u32 { (mem::size_of::<Limb>() * 8) as u32 }
    fn mask() -> u128 { (1u128 << limb_bits()) - 1 }

    #[test]
    fn mul_basic_and_high_split() {
        // 3 * 5 = 15
        let mut c0 = 0 as Limb;
        let mut c1 = 0 as Limb;
        mul(&mut c0, &mut c1, &3, &5);
        assert_eq!(c0 as u128, 15);
        assert_eq!(c1, 0);

        // (MAX)*(MAX) -> low=1, high=2^n-2
        let max = Limb::MAX;
        let mut c0 = 0 as Limb;
        let mut c1 = 0 as Limb;
        mul(&mut c0, &mut c1, &max, &max);
        let hi_expected = ((1u128 << limb_bits()) - 2) as Limb;
        assert_eq!(c0, 1);
        assert_eq!(c1, hi_expected);
    }

    #[test]
    fn mulnadd3_matches_reference() {
        let m = mask();
        let nbits = limb_bits();

        let mut seed: u64 = 0xCAFEBABE_1234_5678;
        for _ in 0..64 {
            let mut next = || {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                (seed as u128 & m) as Limb
            };

            // Inputs for the call
            let (mut c0, mut c1, mut c2) = (next(), next(), 0); // c2 MUST start at 0 by contract
            let (mut d0, mut d1, mut d2) = (next(), next(), next());
            let n = next();

            // Snapshot the inputs for the reference path
            let c0_in = c0;
            let c1_in = c1;
            let d0_in = d0;
            let d1_in = d1;
            let d2_in = d2;
            let n_in  = n;

            // DUT
            mulnadd3(&mut c0, &mut c1, &mut c2, &mut d0, &mut d1, &mut d2, &n);

            // Reference
            let mut t = (d0_in as u128) * (n_in as u128) + (c0_in as u128);
            let c0_ref = (t & m) as Limb;
            t >>= nbits;
            t += (d1_in as u128) * (n_in as u128) + (c1_in as u128);
            let c1_ref = (t & m) as Limb;
            t >>= nbits;
            let c2_ref = ((t + (d2_in as u128) * (n_in as u128)) & m) as Limb;

            assert_eq!((c0, c1, c2), (c0_ref, c1_ref, c2_ref));
        }
    }


    #[test]
    fn muln2_matches_reference() {
        let m = mask();
        let nbits = limb_bits();

        let mut seed: u64 = 0xDEADBEEF_F00D_BA5E;
        for _ in 0..64 {
            let mut c0 = (seed as u128 & m) as Limb;
            seed = seed.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
            let mut c1 = (seed as u128 & m) as Limb;
            seed = seed.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
            let n = (seed as u128 & m) as Limb;
            seed = seed.wrapping_mul(2862933555777941757).wrapping_add(3037000493);

            let (c0_orig, c1_orig) = (c0, c1);

            muln2(&mut c0, &mut c1, &n);

            let mut t = (c0_orig as u128) * (n as u128);
            let c0_ref = (t & m) as Limb;
            t >>= nbits;
            t += (c1_orig as u128) * (n as u128);
            let c1_ref = (t & m) as Limb;

            assert_eq!((c0, c1), (c0_ref, c1_ref));
        }
    }

    #[test]
    fn muladd3_carry_chains_and_reference() {
        let m = mask();
        let nbits = limb_bits();

        // Designed to force carry from c0 -> c1 and c1 -> c2
        let mut c0 = Limb::MAX;
        let mut c1 = Limb::MAX;
        let mut c2 = 0 as Limb;
        let a = Limb::MAX;
        let b = Limb::MAX;

        muladd3(&mut c0, &mut c1, &mut c2, &a, &b);

        // Reference
        let t = (a as u128) * (b as u128);
        let lo = (t & m) as Limb;
        let hi = (t >> nbits) as Limb;

        let (sum0, carry0) = (Limb::MAX).overflowing_add(lo);
        let th = hi.wrapping_add(if carry0 { 1 } else { 0 });
        let (sum1, carry1) = (Limb::MAX).overflowing_add(th);
        let c2_ref = if carry1 { 1 } else { 0 } as Limb;

        assert_eq!(c0, sum0);
        assert_eq!(c1, sum1);
        assert_eq!(c2, c2_ref);

        // Random equivalence
        let mut seed: u64 = 0xC001D00D_D15EA5ED;
        for _ in 0..64 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut c0 = (seed as u128 & m) as Limb;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut c1 = (seed as u128 & m) as Limb;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut c2 = (seed as u128 & m) as Limb;

            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = (seed as u128 & m) as Limb;
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let b = (seed as u128 & m) as Limb;

            let (c0_before, c1_before, c2_before) = (c0, c1, c2);

            muladd3(&mut c0, &mut c1, &mut c2, &a, &b);

            // Reference
            let t = (a as u128) * (b as u128);
            let tl = (t & m) as Limb;
            let th = (t >> nbits) as Limb;

            let (s0, c0_carry) = c0_before.overflowing_add(tl);
            let th_plus = th.wrapping_add(if c0_carry { 1 } else { 0 });
            let (s1, c1_carry) = c1_before.overflowing_add(th_plus);
            let s2 = c2_before.wrapping_add(if c1_carry { 1 } else { 0 });

            assert_eq!((c0, c1, c2), (s0, s1, s2));
        }
    }

    #[test]
    fn muldbladd3_equals_two_muladd3() {
        let m = mask();
        let mut seed: u64 = 0xBADC0FFE_0DD_F00D;

        for _ in 0..32 {
            let mut next = || {
                seed = seed.wrapping_mul(11400714819323198485).wrapping_add(0x9E3779B97F4A7C15);
                (seed as u128 & m) as Limb
            };

            let (mut c0, mut c1, mut c2) = (next(), next(), next());
            let (mut x0, mut x1, mut x2) = (c0, c1, c2);
            let a = next();
            let b = next();

            muldbladd3(&mut c0, &mut c1, &mut c2, &a, &b);

            muladd3(&mut x0, &mut x1, &mut x2, &a, &b);
            muladd3(&mut x0, &mut x1, &mut x2, &a, &b);

            assert_eq!((c0, c1, c2), (x0, x1, x2));
        }
    }
}
