// ---------------- [ File: bitcoin-hash/src/signature_cache_hasher.rs ]
crate::ix!();

/**
  | We're hashing a nonce into the entries
  | themselves, so we don't need extra blinding
  | in the set hash computation.
  | 
  | This may exhibit platform endian dependent
  | behavior but because these are nonced
  | hashes (random) and this state is only
  | ever used locally it is safe.
  | 
  | All that matters is local consistency.
  |
  */
pub struct SignatureCacheHasher { }

impl SignatureCacheHasher {
    #[instrument(level = "debug", skip(self, key))]
    pub fn invoke<const HASH_SELECT: u8>(&self, key: &u256) -> u32 {
        debug_assert!(HASH_SELECT < 8);
        let start = (HASH_SELECT as usize) * 4;
        let mut tmp = [0u8; 4];
        tmp.copy_from_slice(&<u256 as AsRef<[u8]>>::as_ref(key)[start..start + 4]);
        u32::from_le_bytes(tmp)
    }
}

impl bitcoin_cuckoo_cache::EightWayHasher<u256> for SignatureCacheHasher {
    #[inline]
    fn hashes(&self, key: &u256) -> [u32; 8] {
        [
            self.invoke::<0>(key),
            self.invoke::<1>(key),
            self.invoke::<2>(key),
            self.invoke::<3>(key),
            self.invoke::<4>(key),
            self.invoke::<5>(key),
            self.invoke::<6>(key),
            self.invoke::<7>(key),
        ]
    }
}
