crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/utxo_snapshot.h]

/**
  | Metadata describing a serialized version
  | of a UTXO set from which an assumeutxo
  | 
  | ChainState can be constructed.
  |
  */
#[derive(Default)]
pub struct SnapshotMetadata {

    /**
      | The hash of the block that reflects the
      | tip of the chain for the UTXO set contained
      | in this snapshot.
      |
      */
    base_blockhash: u256,

    /**
      | The number of coins in the UTXO set
      | contained in this snapshot. Used during
      | snapshot load to estimate progress of UTXO
      | set reconstruction.
      */
    coins_count:    u64, // default = 0
}

lazy_static!{
    /*
    SERIALIZE_METHODS(SnapshotMetadata, obj) { 
        READWRITE(obj.m_base_blockhash, obj.m_coins_count); 
    }
    */
}

impl SnapshotMetadata {
    
    pub fn new(
        base_blockhash: &u256,
        coins_count:    u64,
        nchaintx:       u32) -> Self {
    
        todo!();
        /*
        : base_blockhash(base_blockhash),
        : coins_count(coins_count),

        
        */
    }
}

