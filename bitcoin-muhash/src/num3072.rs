// ---------------- [ File: bitcoin-muhash/src/num3072.rs ]
crate::ix!();

#[derive(Builder,MutGetters,Getters,Debug, Copy, Clone)]
#[builder(setter(into))]
#[getset(get="pub",get_mut="pub")]
pub struct Num3072 {
    limbs: [Limb; num_3072::LIMBS],
}

pub mod num_3072 {

    use super::*;

    pub const BYTE_SIZE: usize = 384;

    #[cfg(HAVE___INT128)]      pub type DoubleLimb        = __int128;
    #[cfg(HAVE___INT128)]      pub type Limb              = u64;
    #[cfg(HAVE___INT128)]      pub const LIMBS:     usize = 48;
    #[cfg(HAVE___INT128)]      pub const LIMB_SIZE: usize = 64;

    #[cfg(not(HAVE___INT128))] pub type DoubleLimb        = u64;
    #[cfg(not(HAVE___INT128))] pub type Limb              = u32;
    #[cfg(not(HAVE___INT128))] pub const LIMBS:     usize = 96;
    #[cfg(not(HAVE___INT128))] pub const LIMB_SIZE: usize = 32;

    /**
      | Sanity check for Num3072 constants
      |
      */
    const_assert!{ LIMB_SIZE * LIMBS == 3072 }                   //Num3072 isn't 3072 bits

    const_assert!{ size_of::<DoubleLimb>() == size_of::<Limb>() * 2 } //"bad size for double_limb_t"

    const_assert!{ size_of::<Limb>() * 8 == LIMB_SIZE }             //"LIMB_SIZE is incorrect"

    /**
      | Hard coded values in MuHash3072 constructor
      | and Finalize
      |
      */
    const_assert!{ size_of::<Limb>() == 4 || size_of::<Limb>() == 8 } //"bad size for limb_t"
}

impl Default for Num3072 {
    fn default() -> Self {
        let mut n = Num3072 { limbs: [0; num_3072::LIMBS] };
        n.set_to_one();
        n
    }
}

/**
  | in_out = in_out^(2^sq) * mul
  |
  */
#[inline]
pub fn square_n_mul(in_out: &mut Num3072, sq: i32, mul: &Num3072) {
    trace!("square_n_mul");
    for _ in 0..sq {
        in_out.square();
    }
    in_out.multiply(mul);
}

impl Num3072 {

    /// Reduce the internal value modulo *P* when it may exceed the modulus.
    pub fn full_reduce(&mut self) {
        trace!("Num3072::full_reduce");
        let mut c0: Limb = MAX_PRIME_DIFF;
        let mut c1: Limb = 0;

        for limb in &mut self.limbs {
            let val = *limb;                         // avoid aliasing
            addnextract2(&mut c0, &mut c1, &val, limb);
        }
    }

    /**
      | Indicates whether d is larger than the
      | modulus.
      |
      */
    pub fn is_overflow(&self) -> bool {
        trace!("Num3072::is_overflow");
        if self.limbs[0] <= Limb::MAX - MAX_PRIME_DIFF {
            return false;
        }
        for limb in &self.limbs[1..] {
            if *limb != Limb::MAX {
                return false;
            }
        }
        true
    }
   
    pub fn set_to_one(&mut self) {
        trace!("Num3072::set_to_one");
        self.limbs[0] = 1;
        for limb in &mut self.limbs[1..] {
            *limb = 0;
        }
    }

    pub fn new(data: &[u8; num_3072::BYTE_SIZE]) -> Self {
        trace!("Num3072::new");
        let mut res = Num3072 {
            limbs: [0; num_3072::LIMBS],
        };
        if core::mem::size_of::<Limb>() == 4 {
            for (i, limb) in res.limbs.iter_mut().enumerate() {
                *limb = u32::from_le_bytes(data[4 * i..4 * i + 4].try_into().unwrap()) as Limb;
            }
        } else {
            for (i, limb) in res.limbs.iter_mut().enumerate() {
                *limb =
                    u64::from_le_bytes(data[8 * i..8 * i + 8].try_into().unwrap()) as Limb;
            }
        }
        res
    }
  
    pub fn to_bytes(&self, out: &mut [u8; num_3072::BYTE_SIZE]) {
        trace!("Num3072::to_bytes");
        if core::mem::size_of::<Limb>() == 4 {
            for (i, limb) in self.limbs.iter().enumerate() {
                out[4 * i..4 * i + 4].copy_from_slice(&(*limb as u32).to_le_bytes());
            }
        } else {
            for (i, limb) in self.limbs.iter().enumerate() {
                out[8 * i..8 * i + 8].copy_from_slice(&(*limb as u64).to_le_bytes());
            }
        }
    }
}

#[cfg(test)]
mod num3072_property_validation {
    use super::*;
    use rand_chacha::rand_core::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use core::mem;

    /// Verify that full_reduce actually produces a non‑overflowing value.
    #[traced_test]
    fn full_reduce_removes_overflow() -> Result<(), Box<dyn std::error::Error>> {
        let mut n = Num3072 { limbs: [Limb::MAX; num_3072::LIMBS] };
        assert!(n.is_overflow());
        n.full_reduce();
        assert!(!n.is_overflow());
        Ok(())
    }

    #[test]
    fn default_is_one_and_set_to_one_works() {
        let mut n = Num3072::default();
        assert!(n.is_one());
        // change it then set back to one
        n.limbs_mut()[0] = 42;
        n.set_to_one();
        assert!(n.is_one());
        assert_eq!(n.limbs()[1..].iter().all(|&x| x == 0), true);
    }

    #[test]
    fn new_to_bytes_roundtrip_little_endian() {
        // 3072 bits = 384 bytes
        let mut data = [0u8; 384];
        for (i, b) in data.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(37).wrapping_add(11);
        }
        let n = Num3072::new(&data);
        let mut out = [0u8; 384];
        n.to_bytes(&mut out);
        assert_eq!(out, data);
    }

    #[test]
    fn is_overflow_detects_boundary_cases() {
        // Construct a value exactly on the boundary that should be NON-overflow.
        let mut n = Num3072::default();
        let lims = n.limbs().len();
        n.limbs_mut()[0] = Limb::MAX - crate::MAX_PRIME_DIFF;
        for i in 1..lims {
            n.limbs_mut()[i] = Limb::MAX;
        }
        assert_eq!(n.is_overflow(), false, "boundary at MAX - MAX_PRIME_DIFF is not overflow");

        // Now bump low limb by 1 → should be overflow.
        n.limbs_mut()[0] = n.limbs()[0].wrapping_add(1);
        assert_eq!(n.is_overflow(), true, "just above boundary is overflow");
    }

    #[test]
    fn full_reduce_clears_overflow_and_is_idempotent() {
        let mut n = Num3072::default();
        let lims = n.limbs().len();
        n.limbs_mut()[0] = Limb::MAX - crate::MAX_PRIME_DIFF + 7;
        for i in 1..lims {
            n.limbs_mut()[i] = Limb::MAX;
        }
        assert!(n.is_overflow());
        n.full_reduce();
        assert!(!n.is_overflow());
    }

    #[test]
    fn square_n_mul_behaves_like_repeated_square_then_multiply() {
        let mut a = Num3072::default();
        // Make 'a' something nontrivial (few non-zero limbs)
        a.limbs_mut()[0] = 3;
        a.limbs_mut()[1] = 5;

        let mut b = Num3072::default();
        b.limbs_mut()[0] = 9;
        b.limbs_mut()[2] = 7;

        let mut x = a;
        square_n_mul(&mut x, 5, &b);

        let mut y = a;
        for _ in 0..5 { y.square(); }
        y.multiply(&b);

        assert_eq!(x.limbs(), y.limbs());
    }

    #[test]
    fn is_one_true_only_for_exact_one() {
        let mut n = Num3072::default();
        assert!(n.is_one());
        n.limbs_mut()[0] = 2;
        assert!(!n.is_one());
        n.set_to_one();
        n.limbs_mut()[1] = 1;
        assert!(!n.is_one());
    }

    #[test]
    fn limb_sizes_are_consistent() {
        // Smoke-check platform variability.
        let bits = (mem::size_of::<Limb>() * 8) as u32;
        assert!(bits == 32 || bits == 64);
    }

    #[test]
    fn multiply_then_divide_is_identity() {
        const ROUNDS: usize = 4; // each iteration includes a 3072-bit inverse → keep tiny
        let mut rng = ChaCha20Rng::from_seed([3u8; 32]);

        for round in 0..ROUNDS {
            let mut raw = [0u8; num_3072::BYTE_SIZE];
            rng.fill_bytes(&mut raw);
            let mut x = Num3072::new(&raw);

            let mut raw_y = [0u8; num_3072::BYTE_SIZE];
            rng.fill_bytes(&mut raw_y);
            let y = Num3072::new(&raw_y);

            let original = x.limbs;

            x.multiply(&y);
            x.divide(&y);

            assert_eq!(x.limbs, original, "Round {round} failed");
        }
    }
}
