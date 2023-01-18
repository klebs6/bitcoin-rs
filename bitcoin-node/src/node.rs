crate::ix!();

/**
  | Return implementation of Node interface.
  |
  */
pub fn make_node(context: &mut dyn NodeContextInterface) -> Box<Node> {
    
    todo!();
        /*
            return std::make_unique<NodeImpl>(context);
        */
}

/**
  | Information about a peer
  |
  */
pub struct Node {
    
    pub deserializer:                 Option<Box<dyn TransportDeserializer>>,
    pub serializer:                   Option<Box<dyn TransportSerializer>>,
    pub permission_flags:             NetPermissionFlags,
    pub n_services:                   Atomic<ServiceFlags>,

    pub cs_v_send:                    Arc<Mutex<NodeVSend>>,
    pub cs_h_socket:                  Arc<Mutex<NodeHSocket>>,
    pub cs_v_recv:                    Arc<Mutex<NodeVRecv>>,
    pub cs_v_process_msg:             Arc<Mutex<NodeVProcessMsg>>,

    pub n_process_queue_size:         AtomicUsize,
    pub cs_send_processing:           RawMutex,

    pub n_last_send:                  Atomic<Option<OffsetDateTime>>,
    pub n_last_recv:                  Atomic<Option<OffsetDateTime>>,

    /**
      | Unix epoch time at peer connection,
      | in seconds.
      |
      */
    pub n_time_connected:             Atomic<Option<OffsetDateTime>>,
    pub n_time_offset:                Atomic<Option<Duration>>,

    /**
      | Address of this peer
      |
      */
    pub addr:                         Address,

    /**
      | Bind address of our side of the connection
      |
      */
    pub addr_bind:                    Address,
    pub addr_name:                    String,

    /**
      | Whether this peer is an inbound onion,
      | i.e. connected via our Tor onion service.
      |
      */
    pub inbound_onion:                bool,
    pub n_version:                    AtomicI32,

    /**
      | cleanSubVer is a sanitized string of
      | the user agent byte array we read from
      | the wire. This cleaned string can safely
      | be logged or displayed.
      |
      */
    pub clean_sub_ver:                Arc<Mutex<String>>,

    /**
      | This peer is preferred for eviction.
      |
      */
    pub prefer_evict:                 bool,

    /**
      | set by version message
      |
      */
    pub client:                       bool,

    /**
      | after BIP159, set by version message
      |
      */
    pub limited_node:                 bool,

    /**
      | fSuccessfullyConnected is set to true
      | on receiving VERACK from the peer.
      |
      */
    pub successfully_connected:       AtomicBool,

    /**
      | Setting fDisconnect to true will cause
      | the node to be disconnected the next
      | time
      | 
      | DisconnectNodes() runs
      |
      */
    pub disconnect:                   AtomicBool,

    pub grant_outbound:               SemaphoreGrant,
    pub n_ref_count:                  AtomicI32,
    pub n_keyed_net_group:            u64,
    pub pause_recv:                   AtomicBool,
    pub pause_send:                   AtomicBool,

    /**
      | We selected peer as (compact blocks)
      | high-bandwidth peer (BIP152)
      |
      */
    pub bip152_highbandwidth_to:      AtomicBool,

    /**
      | Peer selected us as (compact blocks)
      | high-bandwidth peer (BIP152)
      |
      */
    pub bip152_highbandwidth_from:    AtomicBool,

    /**
      | m_tx_relay == nullptr if we're not relaying
      | transactions with this peer
      |
      */
    pub tx_relay:                     Amo<NodeTxRelay>,

    /**
      | UNIX epoch time of the last block received
      | from this peer that we had not yet seen
      | (e.g. not already received from another
      | peer), that passed preliminary validity
      | checks and was saved to disk, even if
      | we don't connect the block or it eventually
      | fails connection. Used as an inbound
      | peer eviction criterium in Connman::AttemptToEvictConnection.
      |
      */
    pub n_last_block_time:            Atomic<Option<OffsetDateTime>>,

    /**
      | UNIX epoch time of the last transaction
      | received from this peer that we had not
      | yet seen (e.g. not already received
      | from another peer) and that was accepted
      | into our mempool. Used as an inbound
      | peer eviction criterium in Connman::AttemptToEvictConnection.
      |
      */
    pub n_last_tx_time:               Atomic<Option<OffsetDateTime>>,

    /**
      | Last measured round-trip time. Used
      | only for RPC/GUI stats/debugging.
      |
      */
    pub last_ping_time:               Atomic<Option<Instant>>, /* micros */

    /**
      | Lowest measured round-trip time. Used
      | as an inbound peer eviction criterium
      | in Connman::AttemptToEvictConnection.
      |
      */
    pub min_ping_time:                Atomic<Option<Duration>>, /* micros */

    pub id:                           NodeId,
    pub n_local_host_nonce:           u64,
    pub conn_type:                    Option<ConnectionType>,
    pub greatest_common_version:      AtomicI32,

    /**
      | Services offered to this peer.
      |
      | This is supplied by the parent Connman
      | during peer connection
      | (Connman::ConnectNode()) from its
      | attribute of the same name.
      |
      | This is const because there is no protocol
      | defined for renegotiating services
      | initially offered to a peer. The set of
      | local services we offer should not change
      | after initialization.
      |
      | An interesting example of this is
      | NODE_NETWORK and initial block download:
      | a node which starts up from scratch
      | doesn't have any blocks to serve, but
      | still advertises NODE_NETWORK because it
      | will eventually fulfill this role after
      | IBD completes. P2P code is written in such
      | a way that it can gracefully handle peers
      | who don't make good on their service
      | advertisements.
      */
    pub n_local_services:             ServiceFlags,

    /**
      | Used only by SocketHandler thread
      |
      */
    pub recv_msg:                     Arc<Mutex<Vec<NetMessage>>>,
    pub cs_addr_local:                Arc<Mutex<NodeAddrLocal>>,
}

impl PartialEq for Node {

    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Default for Node {

    fn default() -> Self {

        Self {
            deserializer:                 None,
            serializer:                   None,
            permission_flags:             NetPermissionFlags::None,
            n_services:                   Atomic::new(ServiceFlags::NODE_NONE),
            cs_v_send:                    Arc::new(Mutex::<NodeVSend>::default()),
            cs_h_socket:                  Arc::new(Mutex::<NodeHSocket>::default()),
            cs_v_recv:                    Arc::new(Mutex::<NodeVRecv>::default()),
            cs_v_process_msg:             Arc::new(Mutex::new(NodeVProcessMsg::default())),
            n_process_queue_size:         AtomicUsize::new(0),
            cs_send_processing:           RawMutex::INIT,
            n_last_send:                  Atomic::new(None),
            n_last_recv:                  Atomic::new(None),
            n_time_connected:             Atomic::new(None),
            n_time_offset:                Atomic::new(None),
            addr:                         Address::default(),
            addr_bind:                    Address::default(),
            addr_name:                    String::default(),
            inbound_onion:                false,
            n_version:                    AtomicI32::new(0),
            clean_sub_ver:                Default::default(),
            prefer_evict:                 false,
            client:                       false,
            limited_node:                 false,
            successfully_connected:       AtomicBool::new(false),
            disconnect:                   AtomicBool::new(false),
            grant_outbound:               SemaphoreGrant::default(),
            n_ref_count:                  AtomicI32::new(0),
            n_keyed_net_group:            0,
            pause_recv:                   AtomicBool::new(false),
            pause_send:                   AtomicBool::new(false),
            bip152_highbandwidth_to:      AtomicBool::new(false),
            bip152_highbandwidth_from:    AtomicBool::new(false),
            tx_relay:                     Amo::<NodeTxRelay>::none(),
            n_last_block_time:            Atomic::new(None),
            n_last_tx_time:               Atomic::new(None),
            last_ping_time:               Atomic::<Option::<Instant>>::new(None),
            min_ping_time:                Atomic::<Option::<Duration>>::new(None),
            id:                           NodeId::default(),
            n_local_host_nonce:           0,
            conn_type:                    None,
            greatest_common_version:      AtomicI32::new(INIT_PROTO_VERSION),
            n_local_services:             ServiceFlags::default(),
            recv_msg:                     Arc::new(Mutex::new(Vec::<NetMessage>::default())),
            cs_addr_local:                Arc::new(Mutex::<NodeAddrLocal>::default()),
        }
    }
}

unsafe impl Send for Node {}
unsafe impl Sync for Node {}

impl Drop for Node {

    fn drop(&mut self) {
        todo!();
        /*
            CloseSocket(hSocket);
        */
    }
}

impl GetNodeId for Node {

    fn get_id(&self) -> NodeId {
        
        todo!();
        /*
            return id;
        */
    }
}

impl IsBlockOnlyConn for Node {

    fn is_block_only_conn(&self) -> bool {
        
        todo!();
        /*
            return m_conn_type == ConnectionType::BLOCK_RELAY;
        */
    }
    
}

impl IsInboundConn for Node {

    fn is_inbound_conn(&self) -> bool {
        
        todo!();
        /*
            return m_conn_type == ConnectionType::INBOUND;
        */
    }
}

impl GetNTimeConnected for Node {

    #[inline] fn get_n_time_connected(&self) -> OffsetDateTime {
        self.n_time_connected
            .load(atomic::Ordering::Relaxed).unwrap()
    }
}

impl IsOutboundOrBlockRelayConn for Node {

    fn is_outbound_or_block_relay_conn(&self) -> bool {
        
        todo!();
        /*
            switch (m_conn_type) {
                case ConnectionType::OUTBOUND_FULL_RELAY:
                case ConnectionType::BLOCK_RELAY:
                    return true;
                case ConnectionType::INBOUND:
                case ConnectionType::MANUAL:
                case ConnectionType::ADDR_FETCH:
                case ConnectionType::FEELER:
                    return false;
            } // no default case, so the compiler can warn about missing cases

            assert(false);
        */
    }
}
    
impl IsFullOutboundConn for Node {

    fn is_full_outbound_conn(&self) -> bool {
        
        todo!();
        /*
            return m_conn_type == ConnectionType::OUTBOUND_FULL_RELAY;
        */
    }
}
    
impl IsManualConn for Node {

    fn is_manual_conn(&self) -> bool {
        
        todo!();
        /*
            return m_conn_type == ConnectionType::MANUAL;
        */
    }
}
    
impl IsFeelerConn for Node {

    fn is_feeler_conn(&self) -> bool {
        
        todo!();
        /*
            return m_conn_type == ConnectionType::FEELER;
        */
    }
}
    
impl IsAddrFetchConn for Node {

    fn is_addr_fetch_conn(&self) -> bool {
        
        todo!();
        /*
            return m_conn_type == ConnectionType::ADDR_FETCH;
        */
    }
}

impl MarkForDisconnect for Node {

    fn mark_for_disconnect(&self) {
        self.disconnect.store(true, atomic::Ordering::Relaxed);
    }
}

impl GetServiceRef for Node {

    fn service(&self) -> &Service {
        &self.addr.service
    }
}

impl GetServiceMut for Node {

    fn service_mut(&mut self) -> &mut Service {
        &mut self.addr.service
    }
}

impl GetAddrRef for Node {

    fn addr(&self) -> &Address {
        &self.addr
    }
}

impl GetAddrMut for Node {

    fn addr_mut(&mut self) -> &mut Address {
        &mut self.addr
    }
}

impl GetLocalServices for Node {

    fn get_local_services(&self) -> ServiceFlags {
        
        todo!();
        /*
            return nLocalServices;
        */
    }
}

impl GetLocalNonce for Node {

    fn get_local_nonce(&self) -> u64 {
        
        todo!();
        /*
            return nLocalHostNonce;
        */
    }
}

impl GetCommonVersion for Node {

    fn get_common_version(&self) -> i32 {
        
        todo!();
        /*
            return m_greatest_common_version;
        */
    }
}

impl MarkedForDisconnect for Node {

    fn marked_for_disconnect(&self) -> bool {
        self.disconnect.load(atomic::Ordering::Relaxed)
    }
}

impl HasPermission for Node {

    fn has_permission(&self, permission: NetPermissionFlags) -> bool {
        
        todo!();
        /*
            return NetPermissions::HasFlag(m_permissionFlags, permission);
        */
    }
}

impl GetTxRelay for Node {

    fn get_tx_relay(&self) -> AmoReadGuard<NodeTxRelay> {

        self.tx_relay.get()
    }
}

impl GetTxRelayMut for Node {

    fn get_tx_relay_mut(&self) -> AmoWriteGuard<NodeTxRelay> {

        self.tx_relay.get_mut()
    }
}

impl HasTxRelay for Node {

    fn has_tx_relay(&self) -> bool {

        self.tx_relay.is_some()
    }
}

impl DisconnectOnStall for Node {

    fn disconnect_on_stall(&self) {

        // Stalling only triggers when the block
        // download window cannot move. During
        // normal steady state, the download
        // window should be much larger than the
        // to-be-downloaded set of blocks, so
        // disconnection should only happen during
        // initial block download.
        log_printf!(
            "Peer=%d is stalling block download, disconnecting\n",
            self.get_id()
        );

        self.disconnect.store(true, atomic::Ordering::Relaxed);
    }
}

impl NVersion for Node {

    fn n_version(&self) -> i32 {

        self.n_version.load(atomic::Ordering::Relaxed)
    }
}

impl SetSuccessfullyConnected for Node {

    fn set_successfully_connected(&self, val: bool) {

        self.successfully_connected.store(val, atomic::Ordering::Relaxed);
    }
}

impl IsSuccessfullyConnected for Node {

    fn is_successfully_connected(&self) -> bool {

        self.successfully_connected.load(atomic::Ordering::Relaxed)
    }
}

impl ExpectServicesFromConn for Node {

    fn expect_services_from_conn(&self) -> bool {
        
        todo!();
        /*
            switch (m_conn_type) {
                case ConnectionType::INBOUND:
                case ConnectionType::MANUAL:
                case ConnectionType::FEELER:
                    return false;
                case ConnectionType::OUTBOUND_FULL_RELAY:
                case ConnectionType::BLOCK_RELAY:
                case ConnectionType::ADDR_FETCH:
                    return true;
            } // no default case, so the compiler can warn about missing cases

            assert(false);
        */
    }
}

impl AddKnownTx for Node {

    fn add_known_tx(&mut self, hash: &u256)  {
        
        todo!();
        /*
            if (m_tx_relay != nullptr) {
                LOCK(m_tx_relay->cs_tx_inventory);
                m_tx_relay->filterInventoryKnown.insert(hash);
            }
        */
    }
}

impl PongReceived for Node {

    /**
      | A ping-pong round trip has completed
      | successfully. Update latest and minimum
      | ping times.
      |
      */
    fn pong_received(&mut self, ping_time: Duration /* micros */)  {
        
        todo!();
        /*
            m_last_ping_time = ping_time;
            m_min_ping_time = std::min(m_min_ping_time.load(), ping_time);
        */
    }
}

impl SetCommonVersion for Node {

    fn set_common_version(&mut self, greatest_common_version: i32)  {
        
        todo!();
        /*
            Assume(m_greatest_common_version == INIT_PROTO_VERSION);
            m_greatest_common_version = greatest_common_version;
        */
    }
}
    
impl SetAddrLocal for Node {
    
    /**
      | May not be called more than once
      |
      */
    fn set_addr_local(&mut self, addr_local_in: &Service)  {
        
        todo!();
        /*
            LOCK(cs_addrLocal);
        if (addrLocal.IsValid()) {
            error("Addr local already set for node: %i. Refusing to change from %s to %s", id, addrLocal.ToString(), addrLocalIn.ToString());
        } else {
            addrLocal = addrLocalIn;
        }
        */
    }
}

impl SetNLastBlockTime for Node {

    fn set_n_last_block_time(&mut self, x: Option<OffsetDateTime>) {
        self.n_last_block_time.store(x, atomic::Ordering::Relaxed);
    }
}

impl SendPaused for Node {

    fn send_paused(&self) -> bool {
        self.pause_send.load(atomic::Ordering::Relaxed)
    }
}

impl IsClient for Node {

    fn is_client(&self) -> bool {
        self.client
    }
}

impl SetIsClient for Node {

    fn set_is_client(&mut self, x: bool) {
        self.client = x;
    }
}

impl IsLimitedNode for Node {

    fn is_limited_node(&self) -> bool {
        self.limited_node
    }
}

impl SetLimitedNode for Node {

    fn set_limited_node(&mut self, x: bool) {
        self.limited_node = x;
    }
}

impl SetNTimeOffset for Node {

    fn set_n_time_offset(&mut self, x: Option<Duration>) {
        self.n_time_offset.store(x, atomic::Ordering::Relaxed);
    }
}

impl SetNLastTxTime for Node {

    fn set_n_last_tx_time(&mut self, x: Option<OffsetDateTime>) {
        self.n_last_tx_time.store(x, atomic::Ordering::Relaxed);
    }
}

impl SetBip152HighBandwidthFrom for Node {

    fn set_bip152_highbandwidth_from(&mut self, x: bool) {
        self.bip152_highbandwidth_from.store(x, atomic::Ordering::Relaxed);
    }
}

impl SetBip152HighBandwidthTo for Node {

    fn set_bip152_highbandwidth_to(&mut self, x: bool) {
        self.bip152_highbandwidth_to.store(x, atomic::Ordering::Relaxed);
    }
}

impl DecrementNProcessQueueSize for Node {

    fn decrement_n_process_queue_size(&self, val: usize) {
        self.n_process_queue_size.fetch_sub(val, atomic::Ordering::Relaxed);
    }
}

impl GetNProcessQueueSize for Node {

    fn get_n_process_queue_size(&self) -> usize {
        self.n_process_queue_size.load(atomic::Ordering::Relaxed)
    }
}

impl SetPauseRecv for Node {

    fn set_pause_recv(&self, x: bool) {
        self.pause_recv.store(x, atomic::Ordering::Relaxed)
    }
}

impl LockVProcessMsg for Node {

    fn lock_v_process_msg(&self) -> MutexGuard<NodeVProcessMsg> {
        self.cs_v_process_msg.lock()
    }
}

impl SetNServices for Node {

    fn set_n_services(&mut self, x: ServiceFlags) {
        self.n_services.store(x, atomic::Ordering::Relaxed);
    }
}

impl SetNVersion for Node {

    fn set_n_version(&mut self, x: i32) {
        self.n_version.store(x, atomic::Ordering::Relaxed);
    }
}

impl SetCleanSubVer for Node {

    fn set_clean_sub_ver(&mut self, x: &str) {
        *self.clean_sub_ver.lock() = x.to_string();
    }
}

impl Node {

    pub fn new(
        id_in:                 NodeId,
        n_local_services_in:   ServiceFlags,
        h_socket_in:           CSocket,
        addr_in:               &Address,
        n_keyed_net_group_in:  u64,
        n_local_host_nonce_in: u64,
        addr_bind_in:          &Address,
        addr_name_in:          &String,
        conn_type_in:          ConnectionType,
        inbound_onion:         bool) -> Self {
    
        todo!();
        /*


            : nTimeConnected(GetTimeSeconds()),
          addr(addrIn),
          addrBind(addrBindIn),
          m_addr_name{addrNameIn.empty() ? addr.ToStringIPPort() : addrNameIn},
          m_inbound_onion(inbound_onion),
          nKeyedNetGroup(nKeyedNetGroupIn),
          id(idIn),
          nLocalHostNonce(nLocalHostNonceIn),
          m_conn_type(conn_type_in),
          nLocalServices(nLocalServicesIn)

        if (inbound_onion) assert(conn_type_in == ConnectionType::INBOUND);
        hSocket = hSocketIn;
        if (conn_type_in != ConnectionType::BLOCK_RELAY) {
            m_tx_relay = std::make_unique<TxRelay>();
        }

        for (const std::string &msg : getAllNetMessageTypes())
            mapRecvBytesPerMsgCmd[msg] = 0;
        mapRecvBytesPerMsgCmd[NET_MESSAGE_COMMAND_OTHER] = 0;

        if (fLogIPs) {
            LogPrint(BCLog::NET, "Added connection to %s peer=%d\n", m_addr_name, id);
        } else {
            LogPrint(BCLog::NET, "Added connection peer=%d\n", id);
        }

        m_deserializer = std::make_unique<V1TransportDeserializer>(V1TransportDeserializer(Params(), GetId(), SER_NETWORK, INIT_PROTO_VERSION));
        m_serializer = std::make_unique<V1TransportSerializer>(V1TransportSerializer());
        */
    }

    pub fn get_addr_local(&self) -> Service {
        
        todo!();
        /*
            LOCK(cs_addrLocal);
        return addrLocal;
        */
    }
    

    pub fn connection_type_as_string(&self) -> String {
        
        todo!();
        /*
            return ::ConnectionTypeAsString(m_conn_type);
        */
    }
}

impl ConnType for Node {

    fn conn_type(&self) -> Option<ConnectionType> {
        self.conn_type
    }
}

impl NTimeConnected for Node {

    fn n_time_connected(&self) -> Option<OffsetDateTime> {
        self.n_time_connected.load(atomic::Ordering::Relaxed)
    }
}

impl MinPingTime for Node {

    fn min_ping_time(&self) -> Option<Duration> {
        self.min_ping_time.load(atomic::Ordering::Relaxed)
    }
}

impl NLastBlockTime for Node { 

    fn n_last_block_time(&self) -> Option<OffsetDateTime> {
        self.n_last_block_time.load(atomic::Ordering::Relaxed)
    }
}

impl NLastTxTime for Node {

    fn n_last_tx_time(&self) -> Option<OffsetDateTime> {
        self.n_last_tx_time.load(atomic::Ordering::Relaxed)
    }
}

impl NServices for Node { 

    fn n_services(&self) -> ServiceFlags {
        self.n_services.load(atomic::Ordering::Relaxed)
    }
}

impl NKeyedNetGroup for Node { 

    fn n_keyed_net_group(&self) -> u64 {
        self.n_keyed_net_group
    }
}

impl PreferEvict for Node { 

    fn prefer_evict(&self) -> bool {
        self.prefer_evict
    }
}

impl ConnectedThroughNetwork for Node { 

    /**
      | Get network the peer connected through.
      | 
      | Returns Network::NET_ONION for *inbound*
      | onion connections, and CNetAddr::GetNetClass()
      | otherwise. The latter cannot be used
      | directly because it doesn't detect
      | the former, and it's not the responsibility
      | of the CNetAddr class to know the actual
      | network a peer is connected through.
      | 
      | 
      | -----------
      | @return
      | 
      | network the peer connected through.
      |
      */
    fn connected_through_network(&self) -> Network {
        
        todo!();
        /*
            return m_inbound_onion ? NET_ONION : addr.GetNetClass();
        */
    }
}

impl SuccessfullyConnected for Node { 

    fn successfully_connected(&self) -> bool {
        self.successfully_connected.load(atomic::Ordering::Relaxed)
    }
}

impl NodeInterfaceAddRef for Node { 

    fn add_ref(&mut self) -> *mut dyn NodeInterface {
        
        todo!();
        /*
            nRefCount++;
            return this;
        */
    }
}

impl SetPermissionFlags for Node { 

    fn set_permission_flags(&mut self, x: NetPermissionFlags) {
        self.permission_flags = x;
    }
}

impl SetPreferEvict for Node { 

    fn set_prefer_evict(&mut self, x: bool) {
        self.prefer_evict = x;
    }
}

impl ReleaseGrantOutbound for Node { 

    fn release_grant_outbound(&mut self) {
        self.grant_outbound.release();
    }
}

impl CloseSocketDisconnect for Node { 

    fn close_socket_disconnect(&mut self)  {
        
        todo!();
        /*
            fDisconnect = true;
        LOCK(cs_hSocket);
        if (hSocket != INVALID_SOCKET)
        {
            LogPrint(BCLog::NET, "disconnecting peer=%d\n", id);
            CloseSocket(hSocket);
        }
        */
    }
}

impl Release for Node { 

    fn release(&mut self)  {
        
        todo!();
        /*
            nRefCount--;
        */
    }
}

impl GetRefCount for Node { 

    fn get_ref_count(&self) -> i32 {
        
        todo!();
        /*
            assert(nRefCount >= 0);
            return nRefCount;
        */
    }
}

impl AddrName for Node { 

    fn addr_name(&self) -> &str {
        self.addr_name.as_str()
    }
}

impl PauseRecv for Node { 

    fn pause_recv(&self) -> bool {
        self.pause_recv.load(atomic::Ordering::Relaxed)
    }
}

impl LockVSend for Node { 

    fn lock_v_send(&self) -> MutexGuard<NodeVSend> {
        self.cs_v_send.lock()
    }
}

impl LockHSocket for Node { 

    fn lock_h_socket(&self) -> MutexGuard<NodeHSocket> {
        self.cs_h_socket.lock()
    }
}

impl CopyStats for Node { 

    fn copy_stats(&mut self, stats: &mut NodeStats)  {

        macro_rules! x {
            ($name:ident) => {
                /*
                        stats.name = name
                */
            }
        }
        
        todo!();
        /*
            stats.nodeid = this->GetId();
        X(nServices);
        X(addr);
        X(addrBind);
        stats.m_network = ConnectedThroughNetwork();
        if (m_tx_relay != nullptr) {
            LOCK(m_tx_relay->cs_filter);
            stats.fRelayTxes = m_tx_relay->fRelayTxes;
        } else {
            stats.fRelayTxes = false;
        }
        X(nLastSend);
        X(nLastRecv);
        X(nLastTXTime);
        X(nLastBlockTime);
        X(nTimeConnected);
        X(nTimeOffset);
        X(m_addr_name);
        X(nVersion);
        {
            LOCK(cs_SubVer);
            X(cleanSubVer);
        }
        stats.fInbound = IsInboundConn();
        X(m_bip152_highbandwidth_to);
        X(m_bip152_highbandwidth_from);
        {
            LOCK(cs_vSend);
            X(mapSendBytesPerMsgCmd);
            X(nSendBytes);
        }
        {
            LOCK(cs_vRecv);
            X(mapRecvBytesPerMsgCmd);
            X(nRecvBytes);
        }
        X(m_permissionFlags);
        if (m_tx_relay != nullptr) {
            stats.minFeeFilter = m_tx_relay->minFeeFilter;
        } else {
            stats.minFeeFilter = 0;
        }

        X(m_last_ping_time);
        X(m_min_ping_time);

        // Leave string empty if addrLocal invalid (not filled in yet)
        CService addrLocalUnlocked = GetAddrLocal();
        stats.addrLocal = addrLocalUnlocked.IsValid() ? addrLocalUnlocked.ToString() : "";

        X(m_conn_type);
        */
    }
}

impl AddrBind for Node { 

    fn addr_bind(&self) -> &Address {
        &self.addr_bind
    }
}

impl NLastRecv for Node { 

    fn n_last_recv(&self) -> Option<OffsetDateTime> {
        self.n_last_recv.load(atomic::Ordering::Relaxed)
    }
}

impl NLastSend for Node { 

    fn n_last_send(&self) -> Option<OffsetDateTime> {

        self.n_last_send.load(atomic::Ordering::Relaxed)
    }
}

impl GrantOutbound for Node { 

    fn grant_outbound(&self) -> SemaphoreGrant {
        self.grant_outbound.clone()
    }
}

impl GetTransportSerializer for Node { 

    fn get_transport_serializer(&self) -> &Box<dyn TransportSerializer> {
        self.serializer.as_ref().unwrap()
    }
}

impl GetTransportSerializerMut for Node { 

    fn get_transport_serializer_mut(&mut self) -> &mut Box<dyn TransportSerializer> {
        self.serializer.as_mut().unwrap()
    }
}

impl SetPauseSend for Node { 

    fn set_pause_send(&self, x: bool) {
        self.pause_send.store(x, atomic::Ordering::Relaxed);
    }
}

impl ReceiveMsgBytes for Node { 

    /**
      | Receive bytes from the buffer and deserialize
      | them into messages.
      | 
      | -----------
      | @param[in] msg_bytes
      | 
      | The raw data
      | ----------
      | @param[out] complete
      | 
      | Set True if at least one message has been
      | deserialized and is ready to be processed
      | 
      | -----------
      | @return
      | 
      | True if the peer should stay connected,
      | 
      | False if the peer should be disconnected
      | from.
      |
      */
    fn receive_msg_bytes(&mut self, 
        msg_bytes: &[u8],
        complete:  &mut bool) -> bool {
        
        todo!();
        /*
            complete = false;
        const auto time = GetTime<microseconds>();
        LOCK(cs_vRecv);
        nLastRecv = duration_cast<seconds>(time).count();
        nRecvBytes += msg_bytes.size();
        while (msg_bytes.size() > 0) {
            // absorb network data
            int handled = m_deserializer->Read(msg_bytes);
            if (handled < 0) {
                // Serious header problem, disconnect from the peer.
                return false;
            }

            if (m_deserializer->Complete()) {
                // decompose a transport agnostic CNetMessage from the deserializer
                uint32_t out_err_raw_size{0};
                std::optional<CNetMessage> result{m_deserializer->GetMessage(time, out_err_raw_size)};
                if (!result) {
                    // Message deserialization failed.  Drop the message but don't disconnect the peer.
                    // store the size of the corrupt message
                    mapRecvBytesPerMsgCmd.find(NET_MESSAGE_COMMAND_OTHER)->second += out_err_raw_size;
                    continue;
                }

                //store received bytes per message command
                //to prevent a memory DOS, only allow valid commands
                mapMsgCmdSize::iterator i = mapRecvBytesPerMsgCmd.find(result->m_command);
                if (i == mapRecvBytesPerMsgCmd.end())
                    i = mapRecvBytesPerMsgCmd.find(NET_MESSAGE_COMMAND_OTHER);
                assert(i != mapRecvBytesPerMsgCmd.end());
                i->second += result->m_raw_message_size;

                // push the message to the process queue,
                vRecvMsg.push_back(std::move(*result));

                complete = true;
            }
        }

        return true;
        */
    }
}

impl LockRecvMsg for Node { 

    fn lock_recv_msg(&self) -> MutexGuard<Vec<NetMessage>> {
        self.recv_msg.lock()
    }
}

impl IncrementNProcessQueueSize for Node { 

    fn increment_n_process_queue_size(&self, x: usize) {
        self.n_process_queue_size.fetch_add(x, atomic::Ordering::Relaxed);
    }
}

impl NProcessQueueSize for Node { 

    fn n_process_queue_size(&self) -> usize {
        self.n_process_queue_size.load(atomic::Ordering::Relaxed)
    }
}

impl LockSendProcessing for Node { 

    unsafe fn lock_send_processing(&self) {
        self.cs_send_processing.lock();
    }
}

impl UnlockSendProcessing for Node { 

    unsafe fn unlock_send_processing(&self) {
        self.cs_send_processing.unlock();
    }
}

impl PushTxInventory for Node {

    fn push_tx_inventory(&mut self, hash: &u256)  {
        
        todo!();
        /*
            if (m_tx_relay == nullptr) return;
            LOCK(m_tx_relay->cs_tx_inventory);
            if (!m_tx_relay->filterInventoryKnown.contains(hash)) {
                m_tx_relay->setInventoryTxToSend.insert(hash);
            }
        */
    }
}
