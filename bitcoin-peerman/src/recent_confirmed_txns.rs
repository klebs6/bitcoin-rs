// ---------------- [ File: bitcoin-peerman/src/recent_confirmed_txns.rs ]
crate::ix!();

pub struct PeerManagerRecentConfirmedTransactions {

    /**
      | {48'000, 0.000'001};
      |
      */
    pub recent_confirmed_transactions: RollingBloomFilter,
}
