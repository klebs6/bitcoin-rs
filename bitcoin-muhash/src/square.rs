// ---------------- [ File: bitcoin-muhash/src/square.rs ]
crate::ix!();

impl Num3072 {

    pub fn square(&mut self) {
        trace!("Num3072::square");
        let mut c0: Limb = 0;
        let mut c1: Limb = 0;
        let mut c2: Limb = 0;

        /*  As in `multiply`, `tmp` must be zero‑initialised.                   */
        let mut tmp = Num3072Builder::default()
            .limbs([0; num_3072::LIMBS])
            .build()
            .unwrap();

        /* Compute limbs 0..N‑2 of this*this into tmp, including one reduction. */
        for j in 0..num_3072::LIMBS - 1 {
            let mut d0: Limb = 0;
            let mut d1: Limb = 0;
            let mut d2: Limb = 0;

            for i in 0..((num_3072::LIMBS - 1 - j) / 2) {
                muldbladd3(
                    &mut d0,
                    &mut d1,
                    &mut d2,
                    &self.limbs()[i + j + 1],
                    &self.limbs()[num_3072::LIMBS - 1 - i],
                );
            }
            if ((j + 1) & 1) != 0 {
                muladd3(
                    &mut d0,
                    &mut d1,
                    &mut d2,
                    &self.limbs()[(num_3072::LIMBS - 1 - j) / 2 + j + 1],
                    &self.limbs()[num_3072::LIMBS - 1 - (num_3072::LIMBS - 1 - j) / 2],
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
                    &self.limbs()[i],
                    &self.limbs()[j - i],
                );
            }
            if ((j + 1) & 1) != 0 {
                muladd3(
                    &mut c0,
                    &mut c1,
                    &mut c2,
                    &self.limbs()[(j + 1) / 2],
                    &self.limbs()[j - (j + 1) / 2],
                );
            }
            extract3(&mut c0, &mut c1, &mut c2, &mut tmp.limbs_mut()[j]);
        }

        debug_assert_eq!(c2, 0);
        for i in 0..(num_3072::LIMBS / 2) {
            muldbladd3(
                &mut c0,
                &mut c1,
                &mut c2,
                &self.limbs()[i],
                &self.limbs()[num_3072::LIMBS - 1 - i],
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

        if self.is_overflow() {
            self.full_reduce();
        }
        if c0 == 1 {
            self.full_reduce();
        }
    }
}
