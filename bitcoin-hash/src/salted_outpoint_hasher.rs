crate::ix!();

#[derive(Clone)]
pub struct SaltedOutpointHasher {
    /// salt
    k0:     u64,
    k1:     u64,
    hasher: SipHasher,
}

impl Default for SaltedOutpointHasher {
    #[instrument(level = "trace")]
    fn default() -> Self {
        let mut rng = rand::rngs::OsRng::default();
        let k0 = rng.next_u64();
        let k1 = rng.next_u64();
        Self {
            k0,
            k1,
            hasher: SipHasher::new_with_keys(k0, k1),
        }
    }
}

impl BuildHasher for SaltedOutpointHasher {
    type Hasher = Self;

    #[instrument(level = "trace", skip(self))]
    fn build_hasher(&self) -> Self::Hasher {
        Self {
            k0:     self.k0,
            k1:     self.k1,
            hasher: SipHasher::new_with_keys(self.k0, self.k1),
        }
    }
}

impl Hasher for SaltedOutpointHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hasher.clone().finish()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes);
    }
}

impl SaltedOutpointHasher {

    /// Having the hash allows libstdc++'s unordered_map to recalculate the hash during rehash, so
    /// it does not have to cache the value. 
    ///
    /// This reduces node's memory by sizeof(size_t). The required recalculation has a slight
    /// performance penalty (around 1.6%), but this is compensated by memory savings of about 9%
    /// which allow for a larger dbcache setting.
    /// 
    /// -----------
    /// @note
    /// 
    /// see https://gcc.gnu.org/onlinedocs/gcc-9.2.0/libstdc++/manual/manual/unordered_associative.html
    ///
    /// Hash an `OutPoint` (txid + vout index) with an extra 32‑bit word, mirroring Bitcoin Core’s
    /// `SipHashUint256Extra`.
    #[instrument(level = "debug", skip(self, id))]
    pub fn invoke(&self, id: &OutPoint) -> usize {
        let mut h = SipHasher::new_with_keys(self.k0, self.k1);
        h.write(id.hash().as_ref());
        h.write(&id.n().to_le_bytes());
        h.finish() as usize
    }
}
