crate::ix!();

pub struct PeerManagerRecentConfirmedTransactions {

    /**
      | {48'000, 0.000'001};
      |
      */
    pub recent_confirmed_transactions: RollingBloomFilter,
}
