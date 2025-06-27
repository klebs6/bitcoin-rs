// ---------------- [ File: bitcoin-siphash/src/finalize.rs ]
crate::ix!();

impl BitcoinSipHasher {

    /// Return the 64‑bit SipHash‑2‑4 of the data written so far *without*
    /// consuming or mutating `self`.
    ///
    #[inline]
    pub fn finalize(&self) -> u64 {
        let mut v0 = self.v()[0];
        let mut v1 = self.v()[1];
        let mut v2 = self.v()[2];
        let mut v3 = self.v()[3];

        let t = self.tmp() | ((self.count() as u64) << 56);

        v3 ^= t;
        sipround!(v0, v1, v2, v3);
        sipround!(v0, v1, v2, v3);
        v0 ^= t;

        v2 ^= 0xFF;
        sipround!(v0, v1, v2, v3);
        sipround!(v0, v1, v2, v3);
        sipround!(v0, v1, v2, v3);
        sipround!(v0, v1, v2, v3);

        v0 ^ v1 ^ v2 ^ v3
    }

}

#[cfg(test)]
mod finalize_tests {
    use super::*;

    /// Verify that calling `finalize` twice yields identical results
    /// and leaves the object unchanged.
    #[traced_test]
    fn finalize_is_pure() {
        let k0 = 0x0f0e_0d0c_0b0a_0908;
        let k1 = 0x0706_0504_0302_0100;
        let mut h = BitcoinSipHasher::new(k0, k1);
        h.write(&[1, 2, 3, 4, 5]);

        let a = h.finalize();
        let b = h.finalize();
        assert_eq!(a, b, "finalize must be pure");
    }
}
