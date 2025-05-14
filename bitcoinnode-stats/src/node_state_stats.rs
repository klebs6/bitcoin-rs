// ---------------- [ File: bitcoinnode-stats/src/node_state_stats.rs ]
crate::ix!();

pub trait GetNodeStateStats {

    /**
      | Get statistics from node state
      |
      */
    fn get_node_state_stats(&self, 
            nodeid: NodeId,
            stats:  &mut NodeStateStats) -> bool;
}

pub struct NodeStateStats {
    pub n_sync_height:      i32,
    pub n_common_height:    i32,
    pub starting_height:    i32,
    pub ping_wait:          Duration,
    pub height_in_flight:   Vec<i32>,
    pub addr_processed:     u64,
    pub addr_rate_limited:  u64,
    pub addr_relay_enabled: bool,
}

pub const DEFAULT_PING_WAIT: Duration = Duration::microseconds(0); //is this what we want?

impl Default for NodeStateStats {

    fn default() -> Self {
        Self {
            n_sync_height:       -1,
            n_common_height:     -1,
            starting_height:     -1,
            ping_wait:           DEFAULT_PING_WAIT,
            height_in_flight:    vec![],
            addr_processed:      0,
            addr_rate_limited:   0,
            addr_relay_enabled:  false,
        }
    }
}
