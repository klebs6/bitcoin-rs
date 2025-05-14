// ---------------- [ File: bitcoin-txmempool/src/info.rs ]
crate::ix!();

/**
  | Information about a mempool transaction.
  |
  */
pub struct TxMemPoolInfo
{
    /**
      | The transaction itself
      |
      */
    pub tx:          TransactionRef,

    /**
      | Time the transaction entered the mempool.
      |
      */
    pub time:        OffsetDateTime, /* seconds */

    /**
      | Fee of the transaction.
      |
      */
    pub fee:         Amount,

    /**
      | Virtual size of the transaction.
      |
      */
    pub vsize:       usize,

    /**
      | The fee delta.
      |
      */
    pub n_fee_delta: i64,
}
