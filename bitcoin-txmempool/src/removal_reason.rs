// ---------------- [ File: bitcoin-txmempool/src/removal_reason.rs ]
crate::ix!();

/**
  | Reason why a transaction was removed
  | from the mempool, this is passed to the
  | notification signal.
  |
  */
pub enum MemPoolRemovalReason {

    /**
      | Expired from mempool
      |
      */
    EXPIRY,      

    /**
      | Removed in size limiting
      |
      */
    SIZELIMIT,   

    /**
      | Removed for reorganization
      |
      */
    REORG,       

    /**
      | Removed for block
      |
      */
    BLOCK,       

    /**
      | Removed for conflict with in-block
      | transaction
      |
      */
    CONFLICT,    

    /**
      | Removed for replacement
      |
      */
    REPLACED,    
}
