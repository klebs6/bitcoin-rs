crate::ix!();

pub struct BlockHasher { }

impl BlockHasher {

    /**
      | this used to call `GetCheapHash()` in
      | uint256, which was later moved; the cheap
      | hash function simply calls ReadLE64()
      | however, so the end result is identical
      */
    #[instrument(level = "debug", skip(self, hash))]
    pub fn invoke(&self, hash: &u256) -> usize {
        read_le64(hash.as_ref()) as usize
    }
}
