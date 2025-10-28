// ---------------- [ File: bitcoin-muhash/src/finalize.rs ]
crate::ix!();

impl MuHash3072 {

    /// Combine numerator and denominator and return a 32‑byte MuHash digest.
    pub fn finalize(&mut self, out: &mut u256) {
        trace!("MuHash3072::finalize");

        let orig_denom = self.denominator().clone();
        /* --------------------------------------------------------------------
         * (1)  Combine numerator / denominator
         * ------------------------------------------------------------------ */
        self.numerator_mut().divide(&orig_denom);
        self.denominator_mut().set_to_one();           // keep object valid

        /*  NEW:  canonicalise representation so that logically‑equal
         *        states serialise identically (fixes equality‑test
         *        mismatches after different update sequences)          */
        self.numerator_mut().full_reduce();

        /* --------------------------------------------------------------------
         * (2)  Serialize Num3072 → little‑endian bytes
         * ------------------------------------------------------------------ */
        let mut data = [0u8; num_3072::BYTE_SIZE];
        self.numerator().to_bytes(&mut data);

        /* --------------------------------------------------------------------
         * (3)  Single SHA‑256
         * ------------------------------------------------------------------ */
        let mut sha = Sha256::default();
        sha.write(&data);
        let mut digest = [0u8; 32];
        sha.finalize(&mut digest);

        *out = u256::from_le_bytes(digest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::muhash::MuHash3072;

    #[test]
    fn finalize_same_multiset_same_digest() {
        let mut h1 = MuHash3072::default();
        h1.insert(b"a").insert(b"b").insert(b"c");

        let mut h2 = MuHash3072::default();
        h2.insert(b"c").insert(b"a").insert(b"b");

        let mut out1 = u256::from_le_bytes([0u8; 32]);
        let mut out2 = u256::from_le_bytes([0u8; 32]);
        h1.finalize(&mut out1);
        h2.finalize(&mut out2);
        assert_eq!(out1, out2);
    }

    #[test]
    fn finalize_set_difference_via_remove() {
        let mut h = MuHash3072::default();
        h.insert(b"x").insert(b"y").insert(b"z").remove(b"y");

        let mut h_baseline = MuHash3072::default();
        h_baseline.insert(b"x").insert(b"z");

        let mut x = u256::from_le_bytes([0u8; 32]);
        let mut y = u256::from_le_bytes([0u8; 32]);
        h.finalize(&mut x);
        h_baseline.finalize(&mut y);
        assert_eq!(x, y);
    }
}
