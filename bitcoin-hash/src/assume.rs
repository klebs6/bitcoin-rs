// ---------------- [ File: bitcoin-hash/src/assume.rs ]
crate::ix!();

pub struct AssumeUtxoHash {
    base: BaseHash<u256>,
}

impl AssumeUtxoHash {

    pub fn new(hash: &u256) -> Self {
    
        todo!();
        /*
        : base_hash(hash),

        
        */
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
