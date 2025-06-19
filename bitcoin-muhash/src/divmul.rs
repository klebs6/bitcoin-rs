// ---------------- [ File: bitcoin-muhash/src/divmul.rs ]
crate::ix!();

impl core::ops::MulAssign<&MuHash3072> for MuHash3072 {
    /// Multiply (hash of the union of the two sets).
    #[inline]
    fn mul_assign(&mut self, rhs: &MuHash3072) {
        trace!("MuHash3072::mul_assign");
        self.numerator_mut().multiply(rhs.numerator());
        self.denominator_mut().multiply(rhs.denominator());
    }
}

impl core::ops::DivAssign<&MuHash3072> for MuHash3072 {
    /// Divide (hash of the difference of the two sets).
    #[inline]
    fn div_assign(&mut self, rhs: &MuHash3072) {
        trace!("MuHash3072::div_assign");
        self.numerator_mut().multiply(rhs.denominator());
        self.denominator_mut().multiply(rhs.numerator());
    }
}

// -----------------------------------------------------------------------------
// File: bitcoin‑muhash/src/divmul.rs  (tests)
// -----------------------------------------------------------------------------
#[cfg(test)]
mod union_and_difference_validation {
    use super::*;
    use traced_test::traced_test;
    use rand::{RngCore, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use bitcoin_u256::u256;
    use tracing::{info, trace};

    /// Union (`*=`) must equal inserting the same element into a set.
    #[traced_test]
    fn union_matches_insert() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);

        for round in 0..1024 {
            // Independent payloads with variable length.
            let mut buf_a = vec![0u8; (rng.next_u32() % 100 + 1) as usize];
            rng.fill_bytes(&mut buf_a);
            let mut buf_b = vec![0u8; (rng.next_u32() % 100 + 1) as usize];
            rng.fill_bytes(&mut buf_b);

            // Approach 1: start with A, insert B.
            let mut h_insert = MuHash3072::new(&buf_a);
            h_insert.insert(&buf_b);
            let mut out_insert = u256::default();
            h_insert.finalize(&mut out_insert);

            // Approach 2: multiply MuHash(A) by MuHash(B).
            let mut h_mul = MuHash3072::new(&buf_a);
            let h_b     = MuHash3072::new(&buf_b); // rhs
            h_mul *= &h_b;
            let mut out_mul = u256::default();
            h_mul.finalize(&mut out_mul);

            assert_eq!(
                out_insert, out_mul,
                "Union failure at round {round}"
            );
        }
        info!("union_matches_insert passed 1 024 randomized rounds");
        Ok(())
    }

    /// Difference (`/=`) must equal removing an element previously inserted.
    #[traced_test]
    fn difference_matches_remove() -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = ChaCha20Rng::from_seed([1u8; 32]);

        for round in 0..1024 {
            let mut buf_a = vec![0u8; (rng.next_u32() % 100 + 1) as usize];
            rng.fill_bytes(&mut buf_a);
            let mut buf_b = vec![0u8; (rng.next_u32() % 100 + 1) as usize];
            rng.fill_bytes(&mut buf_b);

            // Insert then remove.
            let mut h_rm = MuHash3072::new(&buf_a);
            h_rm.insert(&buf_b);
            h_rm.remove(&buf_b);
            let mut out_rm = u256::default();
            h_rm.finalize(&mut out_rm);

            // Multiply then divide by MuHash(B).
            let mut h_div = MuHash3072::new(&buf_a);
            let h_b       = MuHash3072::new(&buf_b);
            h_div *= &h_b;
            h_div /= &h_b;
            let mut out_div = u256::default();
            h_div.finalize(&mut out_div);

            assert_eq!(
                out_rm, out_div,
                "Difference failure at round {round}"
            );
        }
        info!("difference_matches_remove passed 1 024 randomized rounds");
        Ok(())
    }
}
