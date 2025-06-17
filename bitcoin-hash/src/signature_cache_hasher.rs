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
        tmp.copy_from_slice(&key.as_ref()[start..start + 4]);
        u32::from_le_bytes(tmp)
    }
}
