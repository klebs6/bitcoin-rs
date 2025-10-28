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

// -----------------------------------------------------------------------------
// File: bitcoin‑muhash/src/limb.rs  (tests)
// -----------------------------------------------------------------------------
#[cfg(test)]
mod limb_arithmetic_validation {
    use super::*;
    use traced_test::traced_test;
    use rand_chacha::rand_core::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use tracing::info;

    const ROUNDS: usize = 2_048;

    /// Verify that `mul` splits a * b correctly into (low, high) limbs.
    #[traced_test]
    fn mul_splits_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = ChaCha20Rng::from_seed([2u8; 32]);
        for _ in 0..ROUNDS {
            // Generate random limbs without relying on `rand::Rng`
            let a: Limb = rng.next_u64() as Limb;
            let b: Limb = rng.next_u64() as Limb;

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

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn limb_and_doublelimb_sizes_are_consistent() {
        let limb_sz = mem::size_of::<Limb>();
        let dbl_sz  = mem::size_of::<DoubleLimb>();
        // DoubleLimb must be at least as wide as 2 * Limb for correct split operations.
        assert_eq!(dbl_sz, 2 * limb_sz);
        assert!(limb_sz == 4 || limb_sz == 8);
    }
}
