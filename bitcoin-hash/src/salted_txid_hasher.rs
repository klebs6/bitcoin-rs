crate::ix!();

pub struct SaltedTxidHasher {

    /**
      | Salt
      |
      */
    k0: u64,
    k1: u64,
}

impl Default for SaltedTxidHasher {
    #[instrument(level = "trace")]
    fn default() -> Self {
        let mut rng = rand::rngs::OsRng::default();
        Self {
            k0: rng.next_u64(),
            k1: rng.next_u64(),
        }
    }
}

impl SaltedTxidHasher {
    /// Hash a 256‑bit transaction id with SipHash‑2‑4.
    #[instrument(level = "debug", skip(self, txid))]
    pub fn invoke(&self, txid: &u256) -> usize {
        let mut h = SipHasher::new_with_keys(self.k0, self.k1);
        h.write(txid.as_ref());
        h.finish() as usize
    }
}
