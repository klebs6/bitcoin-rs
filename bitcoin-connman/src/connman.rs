// ---------------- [ File: bitcoin-connman/src/connman.rs ]
crate::ix!();

pub struct Connman {

    /**
      | Network usage totals
      |
      */
    pub cs_total_bytes_recv:    Amo<ConnmanTotalBytesRecv>,
    pub cs_total_bytes_sent:    Amo<ConnmanTotalBytesSent>,

    /**
      | P2P timeout in seconds
      |
      */
    pub peer_connect_timeout:   Amo<Duration>,

    /**
      | Whitelisted ranges. Any node connecting
      | from these is automatically whitelisted
      | (as well as those connecting to whitelisted
      | binds).
      |
      */
    pub whitelisted_range:      Amo<Vec<NetWhitelistPermissions>>,

    pub n_send_buffer_max_size: AtomicU32, // default = { 0 }
    pub n_receive_flood_size:   AtomicU32, // default = { 0 }
    pub vh_listen_socket:       Amo<Vec<ConnmanListenSocket>>,
    pub network_active:         AtomicBool, // default = { true }
    pub addresses_initialized:  AtomicBool, // default = { false }
    pub addrman:                Amo<AddrMan>,
    pub addr_fetches_mutex:     Amo<ConnmanAddrFetches>,
    pub cs_v_added_nodes:       Amo<ConnmanAddedNodes>,
    pub nodes_disconnected:     Amo<Vec<Amo<Box<dyn NodeInterface>>>>, //this was LinkedList<*mut Node>
    pub cs_v_nodes:             Amo<ConnmanNodes>,
    pub n_last_node_id:         Atomic<NodeId>,                   // default = { 0 }
    pub n_prev_node_count:      AtomicU32,                              // default = { 0 }

    /**
      | Addr responses stored in different
      | caches per (network, local socket)
      | prevent cross-network node identification.
      | If a node for example is multi-homed
      | under Tor and IPv6, a single cache (or
      | no cache at all) would let an attacker
      | to easily detect that it is the same node
      | by comparing responses. Indexing by
      | local socket prevents leakage when
      | a node has multiple listening addresses
      | on the same network.
      | 
      | The used memory equals to 1000 CAddress
      | records (or around 40 bytes) per distinct
      | Network (up to 5) we have/had an inbound
      | peer from, resulting in at most ~196
      | KB. Every separate local socket may
      | add up to ~196 KB extra.
      |
      */
    pub addr_response_caches:     HashMap<u64,ConnmanCachedAddrResponse>,

    /**
      | Services this instance offers.
      | 
      | This data is replicated in each Node
      | instance we create during peer connection
      | (in ConnectNode()) under a member also
      | called nLocalServices.
      | 
      | This data is not marked const, but after
      | being set it should not change. See the
      | note in Node::nLocalServices documentation.
      | \sa Node::nLocalServices
      |
      */
    pub n_local_services:         Amo<ServiceFlags>,

    pub sem_outbound:             Amo<Semaphore>,
    pub sem_addnode:              Amo<Semaphore>,
    pub n_max_connections:        AtomicI32,

    /**
      | How many full-relay (tx, block, addr)
      | outbound peers we want
      |
      */
    pub max_outbound_full_relay:  AtomicI32,

    /**
      | How many block-relay only outbound
      | peers we want
      | 
      | We do not relay tx or addr messages with
      | these peers
      |
      */
    pub max_outbound_block_relay: AtomicI32,

    pub n_max_addnode:            AtomicI32,
    pub n_max_feeler:             AtomicI32,
    pub max_outbound:             AtomicI32,
    pub use_addrman_outgoing:     AtomicBool,
    pub client_interface:         Amo<ClientUIInterface>,
    pub msgproc:                  Amo<Box<dyn NetEventsInterface>>,

    /**
      | Pointer to this node's banman. May be
      | nullptr - check existence before dereferencing.
      |
      */
    pub banman:                   Amo<BanMan>,

    /**
      | Addresses that were saved during the
      | previous clean shutdown. We'll attempt
      | to make block-relay-only connections
      | to them.
      |
      */
    pub anchors:                  Amo<Vec<Address>>,

    /**
      | SipHasher seeds for deterministic
      | randomness
      |
      */
    pub n_seed0:                  u64,

    /**
      | SipHasher seeds for deterministic
      | randomness
      |
      */
    pub n_seed1:                  u64,

    pub cond_msg_proc:            Condvar,
    pub mutex_msg_proc:           Amo<ConnmanMsgProc>,
    pub flag_interrupt_msg_proc:  AtomicBool, // default = { false }

    /**
      | This is signaled when network activity
      | should cease.
      | 
      | A pointer to it is saved in `m_i2p_sam_session`,
      | so make sure that the lifetime of `interruptNet`
      | is not shorter than the lifetime of `m_i2p_sam_session`.
      |
      */
    pub interrupt_net:            Amo<ThreadInterrupt>,

    /**
      | I2P SAM session.
      | 
      | Used to accept incoming and make outgoing
      | I2P connections.
      |
      */
    pub i2p_sam_session:               Amo<Box<SAMSession>>,

    pub thread_dns_address_seed:       Amo<JoinHandle<()>>,
    pub thread_socket_handler:         Amo<JoinHandle<()>>,
    pub thread_open_added_connections: Amo<JoinHandle<()>>,
    pub thread_open_connections:       Amo<JoinHandle<()>>,
    pub thread_message_handler:        Amo<JoinHandle<()>>,
    pub thread_i2p_accept_incoming:    Amo<JoinHandle<()>>,

    /**
      | flag for deciding to connect to an extra
      | outbound peer, in excess of m_max_outbound_full_relay
      | 
      | This takes the place of a feeler connection
      |
      */
    pub try_another_outbound_peer:     AtomicBool,

    /**
      | flag for initiating extra block-relay-only
      | peer connections. this should only
      | be enabled after initial chain sync
      | has occurred, as these connections
      | are intended to be short-lived and low-bandwidth.
      |
      */
    pub start_extra_block_relay_peers: AtomicBool, // default = { false }

    pub next_send_inv_to_incoming:     Atomic<OffsetDateTime>, /* micros */ // default = { 0 }

    /**
      | A vector of -bind=<address>:<port>=onion
      | arguments each of which is an address
      | and port that are designated for incoming
      | Tor connections.
      |
      */
    pub onion_binds:                   Amo<Vec<Service>>,
}

impl Drop for Connman {

    fn drop(&mut self) {
        self.interrupt();

        todo!();//uncomment
        //self.stop();
    }
}

impl Connman {

    pub fn new(
        n_seed_0in:     u64,
        n_seed_1in:     u64,
        addrman_in:     AddrMan,
        network_active: Option<bool>) -> Self {

        let network_active: bool = network_active.unwrap_or(true);
    
        let mut x: Self = unsafe { std::mem::zeroed() };
        x.addrman = addrman_in.into();
        x.n_seed0 = n_seed_0in;
        x.n_seed1 = n_seed_1in;

        x.set_try_new_outbound_peer(false);

        let mut conn_options = ConnmanOptions::default();

        x.init(&mut conn_options);

        x.set_network_active(network_active);

        x
    }
}
