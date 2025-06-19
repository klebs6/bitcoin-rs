// ---------------- [ File: bitcoin-muhash/src/limb.rs ]
crate::ix!();

pub type Limb       = num_3072::Limb;
pub type DoubleLimb = num_3072::DoubleLimb;

pub const LIMB_SIZE: usize = num_3072::LIMB_SIZE;

/**
  | 2^3072 - 1103717, the largest 3072-bit
  | safe prime number, is used as the modulus.
  |
  */
pub const MAX_PRIME_DIFF: Limb = 1103717;

/**
  | Extract the lowest limb of [c0,c1,c2]
  | into n, and left shift the number by 1
  | limb.
  |
  */
#[inline]
pub fn extract3(c0: &mut Limb, c1: &mut Limb, c2: &mut Limb, n: &mut Limb) {
    trace!("extract3");
    *n = *c0;
    *c0 = *c1;
    *c1 = *c2;
    *c2 = 0;
}

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

// -----------------------------------------------------------------------------
// File: bitcoin‑muhash/src/limb.rs  (tests)
// -----------------------------------------------------------------------------
#[cfg(test)]
mod limb_arithmetic_validation {
    use super::*;
    use traced_test::traced_test;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use tracing::info;

    const ROUNDS: usize = 10_000;

    /// Verify that `mul` splits a * b correctly into (low, high) limbs.
    #[traced_test]
    fn mul_splits_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = ChaCha20Rng::from_seed([2u8; 32]);
        for _ in 0..ROUNDS {
            let a: Limb = rng.gen();
            let b: Limb = rng.gen();

            let mut c0 = 0 as Limb;
            let mut c1 = 0 as Limb;
            mul(&mut c0, &mut c1, &a, &b);

            let wide: u128 = (a as u128) * (b as u128);
            let exp_low  = wide as Limb;
            let exp_high = (wide >> num_3072::LIMB_SIZE) as Limb;

            assert_eq!(c0, exp_low);
            assert_eq!(c1, exp_high);
        }
        info!("mul_splits_correctly ran {ROUNDS} rounds");
        Ok(())
    }

    /// Exhaustively exercise `extract3`.
    #[traced_test]
    fn extract3_shifts_and_clears() -> Result<(), Box<dyn std::error::Error>> {
        let mut c0: Limb = 0xAAAA_BBBB_AAAA_BBBB_u64 as Limb;
        let mut c1: Limb = 0xCCCC_DDDD_CCCC_DDDD_u64 as Limb;
        let mut c2: Limb = 0xEEEE_FFFF_EEEE_FFFF_u64 as Limb;
        let mut n  : Limb = 0;

        extract3(&mut c0, &mut c1, &mut c2, &mut n);

        assert_eq!(n, 0xAAAA_BBBB_AAAA_BBBB_u64 as Limb);
        assert_eq!(c0, 0xCCCC_DDDD_CCCC_DDDD_u64 as Limb);
        assert_eq!(c1, 0xEEEE_FFFF_EEEE_FFFF_u64 as Limb);
        assert_eq!(c2, 0);
        Ok(())
    }
}
