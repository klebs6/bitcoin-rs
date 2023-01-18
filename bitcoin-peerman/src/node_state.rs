crate::ix!();

/**
  | Maintain validation-specific state
  | about nodes, protected by CS_MAIN,
  | instead by Node's own locks. This simplifies
  | asynchronous operation, where processing
  | of incoming data is done after the ProcessMessage
  | call returns, and we're no longer holding
  | the node's locks.
  |
  */
pub struct NodeState {

    /**
      | The best known block we know this peer
      | has announced.
      |
      */
    pub pindex_best_known_block:  Option<Arc<BlockIndex>>, // default = { nullptr }

    /**
      | The hash of the last unknown block this
      | peer has announced.
      |
      */
    pub hash_last_unknown_block:  u256,

    /**
      | The last full block we both have.
      |
      */
    pub pindex_last_common_block: Option<Arc<BlockIndex>>, // default = { nullptr }

    /**
      | The best header we have sent our peer.
      |
      */
    pub pindex_best_header_sent:  Option<Arc<BlockIndex>>, // default = { nullptr }

    /**
      | Length of current-streak of unconnecting
      | headers announcements
      |
      */
    pub n_unconnecting_headers:   AtomicI32, // default = { 0 }

    /**
      | Whether we've started headers synchronization
      | with this peer.
      |
      */
    pub sync_started:             AtomicBool, // default = { false }

    /**
      | When to potentially disconnect peer
      | for stalling headers download
      |
      */
    pub headers_sync_timeout:  Option<OffsetDateTime> /* micros */, // default = { 0 }

    /**
      | Since when we're stalling block download
      | progress (in microseconds), or 0.
      |
      */
    pub stalling_since:        Option<OffsetDateTime> /* micros */, // default = { 0 }

    pub blocks_in_flight:      Vec<QueuedBlock>,

    /**
      | When the first entry in vBlocksInFlight
      | started downloading. Don't care when
      | vBlocksInFlight is empty.
      |
      */
    pub downloading_since:     OffsetDateTime /* micros */, // default = { 0 }

    pub n_blocks_in_flight:    AtomicI32, // default = { 0 }

    /**
      | Whether we consider this a preferred
      | download peer.
      |
      */
    pub preferred_download:    AtomicBool, // default = { false }

    /**
      | Whether this peer wants invs or headers
      | (when possible) for block announcements.
      |
      */
    pub prefer_headers:        AtomicBool, // default = { false }

    /**
      | Whether this peer wants invs or cmpctblocks
      | (when possible) for block announcements.
      |
      */
    pub prefer_header_and_ids: AtomicBool, // default = { false }

    /**
      | Whether this peer will send us cmpctblocks
      | if we request them.
      | 
      | This is not used to gate request logic,
      | as we really only care about fSupportsDesiredCmpctVersion,
      | but is used as a flag to "lock in" the version
      | of compact blocks (fWantsCmpctWitness)
      | we send.
      |
      */
    pub provides_header_and_ids:        AtomicBool, // default = { false }

    /**
      | Whether this peer can give us witnesses
      |
      */
    pub have_witness:                   AtomicBool, // default = { false }

    /**
      | Whether this peer wants witnesses in
      | cmpctblocks/blocktxns
      |
      */
    pub wants_cmpct_witness:            AtomicBool, // default = { false }

    /**
      | If we've announced NODE_WITNESS to
      | this peer: whether the peer sends witnesses
      | in cmpctblocks/blocktxns, otherwise:
      | whether this peer sends non-witnesses
      | in cmpctblocks/blocktxns.
      |
      */
    pub supports_desired_cmpct_version: AtomicBool, // default = { false }

    pub chain_sync:              NodeStateChainSyncTimeoutState,

    /**
      | Time of last new block announcement
      |
      */
    pub last_block_announcement: Option<OffsetDateTime>, // default = { 0 }

    /**
      | Whether this peer is an inbound connection
      |
      */
    pub is_inbound:              AtomicBool,

    /**
      | A rolling bloom filter of all announced
      | tx CInvs to this peer. 
      | = CRollingBloomFilter{INVENTORY_MAX_RECENT_RELAY, 0.000001};
      |
      */
    pub recently_announced_invs: RollingBloomFilter,

    /**
      | Whether this peer relays txs via wtxid
      |
      */
    pub wtxid_relay:             AtomicBool, // default = { false }
}

impl NodeState {
    
    pub fn blocks_in_flight_iter(&mut self) -> QueuedBlockIter {
        todo!();
    }

    pub fn new(is_inbound: bool) -> Self {
    
        todo!();
        /*
        : is_inbound(is_inbound),

        
        */
    }

    // Detect whether we're stalling
    pub fn detect_stalling(&self, current_time: OffsetDateTime) -> bool {

        self.stalling_since.is_some() 
        && self.stalling_since.as_ref().unwrap() < &(current_time - BLOCK_STALLING_TIMEOUT) 
    }
}

/**
  | Map maintaining per-node state.
  |
  */
lazy_static!{

    //TODO: #[GUARDED_BY(CS_MAIN)]
    pub static ref MAP_NODE_STATE: Arc<Mutex<HashMap<NodeId, Amo<NodeState>>>> = Default::default();
}

#[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
pub fn create_state(pnode: NodeId) -> Amo<NodeState> {
    
    let guard = MAP_NODE_STATE.lock();

    let it = guard.get(&pnode);

    if it.is_none() {
        return Amo::<NodeState>::none();
    }

    it.unwrap().clone()
}
