crate::ix!();

pub struct SaltedSipHasher {

    /**
      | Salt
      |
      */
    k0: u64,
    k1: u64,
}

impl Default for SaltedSipHasher {
    #[instrument(level = "trace")]
    fn default() -> Self {
        let mut rng = rand::rngs::OsRng::default();
        Self {
            k0: rng.next_u64(),
            k1: rng.next_u64(),
        }
    }
}

impl SaltedSipHasher {
    #[instrument(level = "debug", skip(self, script))]
    pub fn invoke(&self, script: &[u8]) -> usize {
        let mut h = SipHasher::new_with_keys(self.k0, self.k1);
        h.write(script);
        h.finish() as usize
    }
}
