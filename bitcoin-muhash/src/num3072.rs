// ---------------- [ File: bitcoin-muhash/src/num3072.rs ]
crate::ix!();

#[derive(Debug, Copy, Clone)]
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

    /// Return the modular inverse using sliding‑window exponentiation.
    ///
    /// For fast exponentiation a sliding window exponentiation with repunit precomputation is
    /// utilized. 
    ///
    /// See "Fast Point Decompression for Standard Elliptic Curves" (Brumley, Järvinen, 2008).
    ///
    pub fn get_inverse(&self) -> Num3072 {
        trace!("Num3072::get_inverse");

        // p[i] = a^(2^(2^i) − 1)
        let mut p = [Num3072::default(); 12];
        p[0] = *self;

        for i in 0..11 {
            p[i + 1] = p[i];
            for _ in 0..(1 << i) {
                p[i + 1].square();
            }
            let base = p[i];           // copy, avoids overlapping borrows
            p[i + 1].multiply(&base);
        }

        let mut out = p[11];
        square_n_mul(&mut out, 512, &p[9]);
        square_n_mul(&mut out, 256, &p[8]);
        square_n_mul(&mut out, 128, &p[7]);
        square_n_mul(&mut out,  64, &p[6]);
        square_n_mul(&mut out,  32, &p[5]);
        square_n_mul(&mut out,   8, &p[3]);
        square_n_mul(&mut out,   2, &p[1]);
        square_n_mul(&mut out,   1, &p[0]);
        square_n_mul(&mut out,   5, &p[2]);
        square_n_mul(&mut out,   3, &p[0]);
        square_n_mul(&mut out,   2, &p[0]);
        square_n_mul(&mut out,   4, &p[0]);
        square_n_mul(&mut out,   4, &p[1]);
        square_n_mul(&mut out,   3, &p[0]);
        out
    }
    
    pub fn multiply(&mut self, a: &Num3072) {
        trace!("Num3072::multiply");
        let mut c0: Limb = 0;
        let mut c1: Limb = 0;
        let mut c2: Limb = 0;
        let mut tmp = Num3072::default();

        // limbs 0 .. N‑2 with one reduction
        for j in 0..num_3072::LIMBS - 1 {
            let mut d0: Limb = 0;
            let mut d1: Limb = 0;
            let mut d2: Limb = 0;

            mul(
                &mut d0,
                &mut d1,
                &self.limbs[1 + j],
                &a.limbs[num_3072::LIMBS - 1],
            );
            for i in (2 + j)..num_3072::LIMBS {
                muladd3(
                    &mut d0,
                    &mut d1,
                    &mut d2,
                    &self.limbs[i],
                    &a.limbs[num_3072::LIMBS + j - i],
                );
            }
            mulnadd3(
                &mut c0,
                &mut c1,
                &mut c2,
                &mut d0,
                &mut d1,
                &mut d2,
                &MAX_PRIME_DIFF,
            );
            for i in 0..=j {
                muladd3(
                    &mut c0,
                    &mut c1,
                    &mut c2,
                    &self.limbs[i],
                    &a.limbs[j - i],
                );
            }
            extract3(&mut c0, &mut c1, &mut c2, &mut tmp.limbs[j]);
        }

        /* Compute limb N-1 of a*b into tmp. */
        debug_assert_eq!(c2, 0);
        for i in 0..num_3072::LIMBS {
            muladd3(
                &mut c0,
                &mut c1,
                &mut c2,
                &self.limbs[i],
                &a.limbs[num_3072::LIMBS - 1 - i],
            );
        }
        extract3(
            &mut c0,
            &mut c1,
            &mut c2,
            &mut tmp.limbs[num_3072::LIMBS - 1],
        );

        // second reduction
        muln2(&mut c0, &mut c1, &MAX_PRIME_DIFF);
        for j in 0..num_3072::LIMBS {
            addnextract2(&mut c0, &mut c1, &tmp.limbs[j], &mut self.limbs[j]);
        }
        debug_assert!(c1 == 0 && (c0 == 0 || c0 == 1));

        /* Perform up to two more reductions if the internal state has already overflown the MAX of Num3072 or if it is larger than the modulus or if both are the case. */

        if self.is_overflow() {
            self.full_reduce();
        }
        if c0 == 1 {
            self.full_reduce();
        }
    }
    
    pub fn square(&mut self) {

        trace!("Num3072::square");
        let mut c0: Limb = 0;
        let mut c1: Limb = 0;
        let mut c2: Limb = 0;
        let mut tmp = Num3072::default();

        /* Compute limbs 0..N-2 of this*this into tmp, including one reduction. */
        for j in 0..num_3072::LIMBS - 1 {
            let mut d0: Limb = 0;
            let mut d1: Limb = 0;
            let mut d2: Limb = 0;

            for i in 0..((num_3072::LIMBS - 1 - j) / 2) {
                muldbladd3(
                    &mut d0,
                    &mut d1,
                    &mut d2,
                    &self.limbs[i + j + 1],
                    &self.limbs[num_3072::LIMBS - 1 - i],
                );
            }
            if ((j + 1) & 1) != 0 {
                muladd3(
                    &mut d0,
                    &mut d1,
                    &mut d2,
                    &self.limbs[(num_3072::LIMBS - 1 - j) / 2 + j + 1],
                    &self.limbs[num_3072::LIMBS
                    - 1
                    - (num_3072::LIMBS - 1 - j) / 2],
                );
            }
            mulnadd3(
                &mut c0,
                &mut c1,
                &mut c2,
                &mut d0,
                &mut d1,
                &mut d2,
                &MAX_PRIME_DIFF,
            );
            for i in 0..((j + 1) / 2) {
                muldbladd3(
                    &mut c0,
                    &mut c1,
                    &mut c2,
                    &self.limbs[i],
                    &self.limbs[j - i],
                );
            }
            if ((j + 1) & 1) != 0 {
                muladd3(
                    &mut c0,
                    &mut c1,
                    &mut c2,
                    &self.limbs[(j + 1) / 2],
                    &self.limbs[j - (j + 1) / 2],
                );
            }
            extract3(&mut c0, &mut c1, &mut c2, &mut tmp.limbs[j]);
        }

        debug_assert_eq!(c2, 0);
        for i in 0..(num_3072::LIMBS / 2) {
            muldbladd3(
                &mut c0,
                &mut c1,
                &mut c2,
                &self.limbs[i],
                &self.limbs[num_3072::LIMBS - 1 - i],
            );
        }
        extract3(
            &mut c0,
            &mut c1,
            &mut c2,
            &mut tmp.limbs[num_3072::LIMBS - 1],
        );

        // second reduction
        muln2(&mut c0, &mut c1, &MAX_PRIME_DIFF);
        for j in 0..num_3072::LIMBS {
            addnextract2(&mut c0, &mut c1, &tmp.limbs[j], &mut self.limbs[j]);
        }
        debug_assert!(c1 == 0 && (c0 == 0 || c0 == 1));

        /* Perform up to two more reductions if the internal state has already overflown the MAX of Num3072 or if it is larger than the modulus or if both are the case. */
        if self.is_overflow() {
            self.full_reduce();
        }
        if c0 == 1 {
            self.full_reduce();
        }
    }
   
    pub fn set_to_one(&mut self) {
        trace!("Num3072::set_to_one");
        self.limbs[0] = 1;
        for limb in &mut self.limbs[1..] {
            *limb = 0;
        }
    }

    pub fn divide(&mut self, a: &Num3072) {
        trace!("Num3072::divide");
        if self.is_overflow() {
            self.full_reduce();
        }

        let inv = if a.is_overflow() {
            let mut b = *a;
            b.full_reduce();
            b.get_inverse()
        } else {
            a.get_inverse()
        };

        self.multiply(&inv);
        if self.is_overflow() {
            self.full_reduce();
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
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    /// set_to_one followed by multiply then divide by the same element
    /// must return to the original value.
    #[traced_test]
    fn multiply_then_divide_is_identity() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = ChaCha20Rng::from_seed([3u8; 32]);

        for round in 0..512 {
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
        info!("multiply_then_divide_is_identity completed 512 rounds");
        Ok(())
    }

    /// Verify that FullReduce actually produces a non‑overflowing value.
    #[traced_test]
    fn full_reduce_removes_overflow() -> Result<(), Box<dyn std::error::Error>> {
        let mut n = Num3072 { limbs: [Limb::MAX; num_3072::LIMBS] };
        assert!(n.is_overflow());
        n.full_reduce();
        assert!(!n.is_overflow());
        Ok(())
    }
}
