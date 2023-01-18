crate::ix!();

pub struct NodeTxRelay {

    pub cs_filter:       Arc<Mutex<NodeTxRelayFilter>>,
    pub cs_tx_inventory: Arc<Mutex<NodeTxRelayTxInventory>>,

    /**
      | Set of transaction ids we still have
      | to announce.
      | 
      | They are sorted by the mempool before
      | relay, so the order is not important.
      |
      */
    pub set_inventory_tx_to_send: Arc<Mutex<HashSet<u256>>>,

    /**
      | Last time a "MEMPOOL" request was serviced.
      |
      */
    pub last_mempool_req:         Atomic<Option<OffsetDateTime>>, /* micros */

    pub n_next_inv_send:          Arc<Mutex<Option<OffsetDateTime>>>, /* micros */

    /**
      | Minimum fee rate with which to filter
      | inv's to this node
      |
      */
    pub min_fee_filter:           Atomic<Amount>,

    pub last_sent_fee_filter:     Amount,
    pub next_send_feefilter:      Option<OffsetDateTime>, /* micros */
}

impl Default for NodeTxRelay {

    fn default() -> Self {
        Self {
            cs_filter:                Arc::new(Mutex::new(NodeTxRelayFilter::default())),
            cs_tx_inventory:          Arc::new(Mutex::new(NodeTxRelayTxInventory::default())),
            set_inventory_tx_to_send: Default::default(),
            last_mempool_req:         Atomic::new(None),
            n_next_inv_send:          Arc::new(Mutex::new(None)),
            min_fee_filter:           Atomic::new(0),
            last_sent_fee_filter:     0,
            next_send_feefilter:      None,
        }
    }
}

//-----------------------------------------------
pub struct NodeTxRelayFilter {

    /**
      | We use fRelayTxes for two purposes -
      |
      | a) it allows us to not relay tx invs
      | before receiving the peer's version
      | message
      |
      | b) the peer may tell us in its version
      | message that we should not relay tx
      | invs unless it loads a bloom filter.
      |
      */
    pub relay_txes:      bool,
    pub pfilter:         Option<BloomFilter>,
}

impl Default for NodeTxRelayFilter {
    fn default() -> Self {
        Self {
            relay_txes: false,
            pfilter:    None,
        }
    }
}

//-----------------------------------------------
pub struct NodeTxRelayTxInventory {

    pub filter_inventory_known:   RollingBloomFilter,

    /**
      | Used for BIP35 mempool sending
      |
      */
    pub send_mempool:             bool,
}

impl Default for NodeTxRelayTxInventory {

    fn default() -> Self {
        Self {
            filter_inventory_known:   RollingBloomFilter::new(50000, 0.000001),
            send_mempool:             false,
        }
    }
}
