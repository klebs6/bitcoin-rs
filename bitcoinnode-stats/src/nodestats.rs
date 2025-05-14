// ---------------- [ File: bitcoinnode-stats/src/nodestats.rs ]
crate::ix!();

#[derive(Default)]
pub struct NodeStats {
    pub nodeid:                     NodeId,
    pub n_services:                 ServiceFlags,
    pub relay_txes:                 bool,
    pub n_last_send:                Option<OffsetDateTime>,
    pub n_last_recv:                Option<OffsetDateTime>,
    pub n_last_tx_time:             Option<OffsetDateTime>,
    pub n_last_block_time:          Option<OffsetDateTime>,
    pub n_time_connected:           Option<OffsetDateTime>,
    pub n_time_offset:              Option<Duration>,
    pub addr_name:                  String,
    pub n_version:                  i32,
    pub clean_sub_ver:              Arc<Mutex<String>>,
    pub inbound:                    bool,
    pub bip152_highbandwidth_to:    bool,
    pub bip152_highbandwidth_from:  bool,
    pub starting_height:            i32,
    pub n_send_bytes:               u64,
    pub map_send_bytes_per_msg_cmd: MapMsgCmdSize,
    pub n_recv_bytes:               u64,
    pub map_recv_bytes_per_msg_cmd: MapMsgCmdSize,
    pub permission_flags:           NetPermissionFlags,
    pub last_ping_time:             Option<OffsetDateTime> /* micros */,
    pub min_ping_time:              Option<OffsetDateTime> /* micros */,
    pub min_fee_filter:             Amount,

    /**
      | Our address, as reported by the peer
      |
      */
    pub addr_local:                 String,

    /**
      | Address of this peer
      |
      */
    pub addr:                       Address,

    /**
      | Bind address of our side of the connection
      |
      */
    pub addr_bind:                  Address,

    /**
      | Network the peer connected through
      |
      */
    pub network:                    Network,

    pub mapped_as:                  u32,
    pub conn_type:                  ConnectionType,
}

pub type NodesStats = Vec<(NodeStats,bool,NodeStateStats)>;
