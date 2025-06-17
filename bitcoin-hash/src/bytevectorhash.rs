// ---------------- [ File: bitcoin-hash/src/bytevectorhash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/bytevectorhash.h]
//-------------------------------------------[.cpp/bitcoin/src/util/bytevectorhash.cpp]

/**
  | Implementation of Hash named requirement
  | for types that internally store a byte
  | array. This may be used as the hash function
  | in std::unordered_set or std::unordered_map
  | over such types.
  | 
  | Internally, this uses a random instance
  | of SipHash-2-4.
  |
  */
#[derive(Debug,Clone)]
pub struct ByteVectorHash {
    k0:     u64,
    k1:     u64,
    hasher: SipHasher,
}

impl Default for ByteVectorHash {
    /// Generates process‑wide random SipHash keys.
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

impl BuildHasher for ByteVectorHash {
    type Hasher = Self;

    #[instrument(level = "trace")]
    fn build_hasher(&self) -> Self::Hasher {
        Self {
            k0:     self.k0,
            k1:     self.k1,
            hasher: SipHasher::new_with_keys(self.k0, self.k1),
        }
    }
}

impl Hasher for ByteVectorHash {
    #[inline]
    fn finish(&self) -> u64 {
        self.hasher.clone().finish()
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes);
    }
}

impl ByteVectorHash {

    /// Convenience helper that hashes a complete byte‑vector in one call.
    #[instrument(level = "trace", skip_all)]
    pub fn invoke(&self, input: &[u8]) -> usize {
        let mut h = SipHasher::new_with_keys(self.k0, self.k1);
        h.write(input);
        h.finish() as usize
    }
}

// ---------------- [ File: bitcoin-hash/src/bytevectorhash.rs ] (new test module)
#[cfg(test)]
mod bytevectorhash_spec {
    use super::*;

    #[traced_test]
    fn identical_inputs_yield_identical_hashes() {
        let h = ByteVectorHash::default();
        let lhs = h.invoke(b"hello");
        let rhs = h.invoke(b"hello");
        assert_eq!(lhs, rhs);
    }

    #[traced_test]
    fn distinct_inputs_yield_distinct_hashes() {
        let h = ByteVectorHash::default();
        let lhs = h.invoke(b"hello");
        let rhs = h.invoke(b"good-bye");
        assert_ne!(lhs, rhs);
    }

    #[traced_test]
    fn build_hasher_produces_equivalent_hasher() {
        let builder = ByteVectorHash::default();
        let mut h1 = builder.build_hasher();
        h1.write(b"test");
        let r1 = h1.finish();

        let mut h2 = builder.build_hasher();
        h2.write(b"test");
        let r2 = h2.finish();

        assert_eq!(r1, r2);
    }
}
