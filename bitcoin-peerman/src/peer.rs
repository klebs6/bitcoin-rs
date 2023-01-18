crate::ix!();

/**
  | Data structure for an individual peer.
  | This struct is not protected by CS_MAIN
  | since it does not contain validation-critical
  | data.
  | 
  | Memory is owned by shared pointers and
  | this object is destructed when the refcount
  | drops to zero.
  | 
  | Mutexes inside this struct must not
  | be held when locking m_peer_mutex.
  | 
  | TODO: move most members from NodeState
  | to this structure.
  | 
  | TODO: move remaining application-layer
  | data members from Node to this structure.
  |
  */
pub struct Peer {

    /**
      | Same id as the Node object for this peer
      |
      */
    pub id:                NodeId, // default = { 0 }

    /**
      | Protects misbehavior data members
      |
      */
    pub misbehavior: Arc<Mutex<PeerMisbehavior>>,

    /**
      | Protects block inventory data members
      |
      */
    pub block_inv_mutex:   Arc<Mutex<PeerBlockInv>>,

    /**
      | This peer's reported block height when
      | we connected
      |
      */
    pub starting_height: Atomic<i32>, // default = { -1 }

    /**
      | The pong reply we're expecting, or 0
      | if no pong expected.
      |
      */
    pub ping_nonce_sent: Atomic<u64>, // default = { 0 }

    /**
      | When the last ping was sent, or 0 if no
      | ping was ever sent
      |
      */
    pub ping_start:      Atomic<OffsetDateTime /* micros */>, // default = { 0 }

    /**
      | Whether a ping has been requested by
      | the user
      |
      */
    pub ping_queued:     AtomicBool, // default = { false }

    /**
      | A vector of addresses to send to the peer,
      | limited to MAX_ADDR_TO_SEND.
      |
      */
    pub addrs_to_send:         Arc<Mutex<Vec<Address>>>,

    /**
      | Probabilistic filter to track recent
      | addr messages relayed with this peer.
      | Used to avoid relaying redundant addresses
      | to this peer.
      | 
      | We initialize this filter for outbound
      | peers (other than block-relay-only
      | connections) or when an inbound peer
      | sends us an address related message
      | (ADDR, ADDRV2, GETADDR).
      | 
      | Presence of this filter must correlate
      | with m_addr_relay_enabled.
      |
      */
    pub addr_known:            Option<RollingBloomFilter>,

    /**
      | Whether we are participating in address
      | relay with this connection.
      | 
      | We set this bool to true for outbound
      | peers (other than block-relay-only
      | connections), or when an inbound peer
      | sends us an address related message
      | (ADDR, ADDRV2, GETADDR).
      | 
      | We use this bool to decide whether a peer
      | is eligible for gossiping addr messages.
      | This avoids relaying to peers that are
      | unlikely to forward them, effectively
      | blackholing self announcements. Reasons
      | peers might support addr relay on the
      | link include that they connected to
      | us as a block-relay-only peer or they
      | are a light client.
      | 
      | This field must correlate with whether
      | m_addr_known has been initialized.
      |
      */
    pub addr_relay_enabled:    AtomicBool, // default = { false }

    /**
      | Whether a getaddr request to this peer
      | is outstanding.
      |
      */
    pub getaddr_sent:          bool, // default = { false }

    /**
      | Guards address sending timers.
      |
      */
    pub addr_send_times_mutex: Arc<Mutex<PeerAddrSendTimes>>,

    /**
      | Whether the peer has signaled support
      | for receiving ADDRv2 (BIP155) messages,
      | indicating a preference to receive
      | ADDRv2 instead of ADDR ones.
      |
      */
    pub wants_addrv2:           AtomicBool, // default = { false }

    /**
      | Whether this peer has already sent us
      | a getaddr message.
      |
      */
    pub getaddr_recvd:          bool, // default = { false }

    /**
      | Number of addresses that can be processed
      | from this peer. Start at 1 to permit self-announcement.
      |
      */
    pub addr_token_bucket:      f64, // default = { 1.0 }

    /**
      | When m_addr_token_bucket was last
      | updated
      |
      */
    pub addr_token_timestamp:   OffsetDateTime /* micros */,

    /**
      | Total number of addresses that were
      | dropped due to rate limiting.
      |
      */
    pub addr_rate_limited:      Atomic<u64>, // default = { 0 }

    /**
      | Total number of addresses that were
      | processed (excludes rate-limited
      | ones).
      |
      */
    pub addr_processed:         Atomic<u64>, // default = { 0 }

    //#[GUARDED_BY(g_cs_orphans)]
    pub orphan_work_set:        PeerOrphans,

    /**
      | Work queue of items requested by this
      | peer
      |
      */
    pub getdata_requests: Arc<Mutex<VecDeque<Inv>>>,
}

impl Peer {
    
    pub fn new(id: NodeId) -> Self {
    
        todo!();
        /*
        : id(id),

        
        */
    }
}
