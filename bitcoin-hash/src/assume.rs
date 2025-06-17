// ---------------- [ File: bitcoin-hash/src/assume.rs ]
crate::ix!();

pub struct AssumeUtxoHash {
    base: BaseHash<u256>,
}

impl AssumeUtxoHash {

    /// Construct a new [`AssumeUtxoHash`] from the supplied 256‑bit hash.
    #[instrument(level = "debug", skip(hash))]
    pub fn new(hash: &u256) -> Self {
        Self {
            base: BaseHash::new(hash),
        }
    }
}

/**
  | Holds configuration for use during
  | UTXO snapshot load and validation.
  | The contents here are security critical,
  | since they dictate which UTXO snapshots
  | are recognized as valid.
  |
  */
pub struct AssumeUtxoData {

    /**
      | The expected hash of the deserialized
      | UTXO set.
      |
      */
    hash_serialized: AssumeUtxoHash,

    /**
      | Used to populate the nChainTx value, which
      | is used during
      | BlockManager::LoadBlockIndex().
      |
      | We need to hardcode the value here because
      | this is computed cumulatively using block
      | data, which we do not necessarily have at
      | the time of snapshot load.
      */
    n_chain_tx: u32,
}

pub type MapAssumeUtxo = HashMap<i32,AssumeUtxoData>;

// ---------------- [ File: bitcoin-hash/src/assume.rs ] (new test module)
#[cfg(test)]
mod assume_utxo_hash_spec {
    use super::*;

    /// Verify that constructing an [`AssumeUtxoHash`] succeeds
    /// for a non‑zero and the zero hash alike.
    #[traced_test]
    fn construction_is_lossless() {
        // GIVEN a zero 256‑bit hash
        let zero_hash = u256::default();
        // WHEN we construct an `AssumeUtxoHash`
        let snapshot = AssumeUtxoHash::new(&zero_hash);

        // THEN converting the inner `BaseHash` back to bytes must
        // yield exactly 32 zeroed bytes.
        let bytes: Vec<u8> = snapshot.base.clone().into();
        assert_eq!(bytes, vec![0u8; 32]);

        // …and the same must hold for a non‑zero hash.
        let non_zero = u256::from(42);
        let snapshot = AssumeUtxoHash::new(&non_zero);
        let bytes: Vec<u8> = snapshot.base.into();
        assert_ne!(bytes, vec![0u8; 32]);
    }
}
