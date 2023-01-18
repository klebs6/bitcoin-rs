crate::ix!();

/**
  | Dumb serialization/storage-helper
  | for BlockHeaderAndShortTxIDs and
  | PartiallyDownloadedBlock
  |
  */
pub struct PrefilledTransaction {

    /**
      | Used as an offset since last prefilled tx
      | in CBlockHeaderAndShortTxIDs, as a proper
      | transaction-in-block-index in
      | PartiallyDownloadedBlock
      */
    pub index: u16,

    pub tx:    TransactionRef,

}

lazy_static!{
    /*
    SERIALIZE_METHODS(PrefilledTransaction, obj)
    { 
        READWRITE(COMPACTSIZE(obj.index),
        Using<TransactionCompression>(obj.tx)); 
    }
    */
}
