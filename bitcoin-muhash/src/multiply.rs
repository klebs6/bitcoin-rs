// ---------------- [ File: bitcoin-muhash/src/multiply.rs ]
crate::ix!();

impl Num3072 {

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
                &self.limbs()[1 + j],
                &a.limbs()[num_3072::LIMBS - 1],
            );
            for i in (2 + j)..num_3072::LIMBS {
                muladd3(
                    &mut d0,
                    &mut d1,
                    &mut d2,
                    &self.limbs()[i],
                    &a.limbs()[num_3072::LIMBS + j - i],
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
                    &self.limbs()[i],
                    &a.limbs()[j - i],
                );
            }
            extract3(&mut c0, &mut c1, &mut c2, &mut tmp.limbs_mut()[j]);
        }

        /* Compute limb N-1 of a*b into tmp. */
        debug_assert_eq!(c2, 0);
        for i in 0..num_3072::LIMBS {
            muladd3(
                &mut c0,
                &mut c1,
                &mut c2,
                &self.limbs()[i],
                &a.limbs()[num_3072::LIMBS - 1 - i],
            );
        }
        extract3(
            &mut c0,
            &mut c1,
            &mut c2,
            &mut tmp.limbs_mut()[num_3072::LIMBS - 1],
        );

        // second reduction
        muln2(&mut c0, &mut c1, &MAX_PRIME_DIFF);
        for j in 0..num_3072::LIMBS {
            addnextract2(&mut c0, &mut c1, &tmp.limbs()[j], &mut self.limbs_mut()[j]);
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
}
