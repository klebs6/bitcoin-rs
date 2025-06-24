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
