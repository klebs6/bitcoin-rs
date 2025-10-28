// ---------------- [ File: bitcoin-muhash/src/get_inverse.rs ]
crate::ix!();

impl Num3072 {

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::muhash::MuHash3072;

    fn num(b: &[u8]) -> Num3072 { MuHash3072::to_num3072(b) }

    #[test]
    fn inverse_of_one_is_one() {
        let one = Num3072::default();
        let inv = one.get_inverse();
        assert!(inv.is_one());
    }

    #[test]
    fn inverse_correctness_x_times_inv_is_one() {
        let candidates = [b"alice".as_ref(), b"bob".as_ref(), b"carol".as_ref()];
        for m in candidates {
            let x = num(m);
            // Avoid the (extremely unlikely) zero element—if it happened, pick another.
            if x.limbs().iter().any(|&v| v != 0) {
                let mut prod = x;
                let inv = prod.get_inverse();
                prod.multiply(&inv);

                if prod.is_overflow() {
                    prod.full_reduce();
                }

                assert!(prod.is_one(), "x * inv(x) must be 1");
            }
        }
    }

    #[test]
    fn inverse_is_two_sided() {
        let x = num(b"dave");

        let inv = x.get_inverse();

        let mut left = inv;
        left.multiply(&x);

        let mut right = x;
        right.multiply(&inv);

        if left.is_overflow() {
            left.full_reduce();
        }
        if right.is_overflow() {
            right.full_reduce();
        }
        assert!(left.is_one() && right.is_one());
    }
}
