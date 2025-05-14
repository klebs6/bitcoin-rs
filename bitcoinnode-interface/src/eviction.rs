// ---------------- [ File: bitcoinnode-interface/src/eviction.rs ]
crate::ix!();

pub struct NodeEvictionCandidate
{
    pub id:                NodeId,
    pub n_time_connected:  Option<OffsetDateTime>,
    pub min_ping_time:     Option<Duration>, /* micros */
    pub n_last_block_time: Option<OffsetDateTime>,
    pub n_last_tx_time:    Option<OffsetDateTime>,
    pub relevant_services: bool,
    pub relay_txes:        bool,
    pub bloom_filter:      bool,
    pub n_keyed_net_group: u64,
    pub prefer_evict:      bool,
    pub is_local:          bool,
    pub network:           Network,
}

//-----------------------------------------------

/**
  | Sort an array by the specified comparator,
  | then erase the last K elements where
  | predicate is true.
  |
  */
pub fn erase_last_kelements<T, Comparator>(
        elements:   &mut Vec<T>,
        comparator: Comparator,
        k:          usize,
        predicate:  fn(_0: &NodeEvictionCandidate) -> bool)  {

    todo!();
        /*
           let predicate = predicate.unwrap_or([](const NodeEvictionCandidate& n) { return true; });
           std::sort(elements.begin(), elements.end(), comparator);
        size_t eraseSize = std::min(k, elements.size());
        elements.erase(std::remove_if(elements.end() - eraseSize, elements.end(), predicate), elements.end());
        */
}

pub fn compare_node_block_time(
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
    
    todo!();
        /*
            // There is a fall-through here because it is common for a node to have many peers which have not yet relayed a block.
        if (a.nLastBlockTime != b.nLastBlockTime) return a.nLastBlockTime < b.nLastBlockTime;
        if (a.fRelevantServices != b.fRelevantServices) return b.fRelevantServices;
        return a.nTimeConnected > b.nTimeConnected;
        */
}

pub fn compare_node_tx_time(
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
    
    todo!();
        /*
            // There is a fall-through here because it is common for a node to have more than a few peers that have not yet relayed txn.
        if (a.nLastTXTime != b.nLastTXTime) return a.nLastTXTime < b.nLastTXTime;
        if (a.fRelayTxes != b.fRelayTxes) return b.fRelayTxes;
        if (a.fBloomFilter != b.fBloomFilter) return a.fBloomFilter;
        return a.nTimeConnected > b.nTimeConnected;
        */
}

/**
  | Pick out the potential block-relay
  | only peers, and sort them by last block
  | time.
  |
  */
pub fn compare_node_block_relay_only_time(
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
    
    todo!();
        /*
            if (a.fRelayTxes != b.fRelayTxes) return a.fRelayTxes;
        if (a.nLastBlockTime != b.nLastBlockTime) return a.nLastBlockTime < b.nLastBlockTime;
        if (a.fRelevantServices != b.fRelevantServices) return b.fRelevantServices;
        return a.nTimeConnected > b.nTimeConnected;
        */
}

pub fn reverse_compare_node_min_ping_time(
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
    
    todo!();
        /*
            return a.m_min_ping_time > b.m_min_ping_time;
        */
}

pub fn reverse_compare_node_time_connected(
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
    
    todo!();
        /*
            return a.nTimeConnected > b.nTimeConnected;
        */
}

pub fn compare_net_group_keyed(
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
    
    todo!();
        /*
            return a.nKeyedNetGroup < b.nKeyedNetGroup;
        */
}
