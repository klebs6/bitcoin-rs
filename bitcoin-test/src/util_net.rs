crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/net.h]

pub struct ConnmanTestMsg {
    base: Connman,
}

impl ConnmanTestMsg {
    
    pub fn set_peer_connect_timeout(&mut self, timeout: i64)  {
        
        todo!();
        /*
            m_peer_connect_timeout = timeout;
        */
    }
    
    pub fn add_test_node(&mut self, node: &mut Node)  {
        
        todo!();
        /*
            LOCK(cs_vNodes);
            vNodes.push_back(&node);
        */
    }
    
    pub fn clear_test_nodes(&mut self)  {
        
        todo!();
        /*
            LOCK(cs_vNodes);
            for (Node* node : vNodes) {
                delete node;
            }
            vNodes.clear();
        */
    }
    
    pub fn process_messages_once(&mut self, node: &mut AmoWriteGuard<Node>)  {
        
        todo!();
        /*
            m_msgproc->ProcessMessages(&node, flagInterruptMsgProc);
        */
    }
    
    pub fn node_receive_msg_bytes(&self, 
        node:      &mut Node,
        msg_bytes: &[u8],
        complete:  &mut bool)  {
        
        todo!();
        /*
            assert(node.ReceiveMsgBytes(msg_bytes, complete));
        if (complete) {
            size_t nSizeAdded = 0;
            auto it(node.vRecvMsg.begin());
            for (; it != node.vRecvMsg.end(); ++it) {
                // vRecvMsg contains only completed CNetMessage
                // the single possible partially deserialized message are held by TransportDeserializer
                nSizeAdded += it->m_raw_message_size;
            }
            {
                LOCK(node.cs_vProcessMsg);
                node.vProcessMsg.splice(node.vProcessMsg.end(), node.vRecvMsg, node.vRecvMsg.begin(), it);
                node.nProcessQueueSize += nSizeAdded;
                node.fPauseRecv = node.nProcessQueueSize > nReceiveFloodSize;
            }
        }
        */
    }
    
    pub fn receive_msg_from(&self, 
        node:    &mut Node,
        ser_msg: &mut SerializedNetMsg) -> bool {
        
        todo!();
        /*
            std::vector<uint8_t> ser_msg_header;
        node.m_serializer->prepareForTransport(ser_msg, ser_msg_header);

        bool complete;
        NodeReceiveMsgBytes(node, ser_msg_header, complete);
        NodeReceiveMsgBytes(node, ser_msg.data, complete);
        return complete;
        */
    }
}

pub const ALL_SERVICE_FLAGS: &[ServiceFlags] = &[
    ServiceFlags::NODE_NONE,
    ServiceFlags::NODE_NETWORK,
    ServiceFlags::NODE_BLOOM,
    ServiceFlags::NODE_WITNESS,
    ServiceFlags::NODE_COMPACT_FILTERS,
    ServiceFlags::NODE_NETWORK_LIMITED,
];

pub const ALL_NET_PERMISSION_FLAGS: &[NetPermissionFlags] = &[
    NetPermissionFlags::None,
    NetPermissionFlags::BloomFilter,
    NetPermissionFlags::Relay,
    NetPermissionFlags::ForceRelay,
    NetPermissionFlags::NoBan,
    NetPermissionFlags::Mempool,
    NetPermissionFlags::Addr,
    NetPermissionFlags::Download,
    NetPermissionFlags::Implicit,
    NetPermissionFlags::All,
];

pub const ALL_CONNECTION_TYPES: &[ConnectionType] = &[
    ConnectionType::INBOUND,
    ConnectionType::OUTBOUND_FULL_RELAY,
    ConnectionType::MANUAL,
    ConnectionType::FEELER,
    ConnectionType::BLOCK_RELAY,
    ConnectionType::ADDR_FETCH,
];

pub const ALL_NETWORKS: &[Network] = &[
    Network::NET_UNROUTABLE,
    Network::NET_IPV4,
    Network::NET_IPV6,
    Network::NET_ONION,
    Network::NET_I2P,
    Network::NET_CJDNS,
    Network::NET_INTERNAL,
];

/**
  | A mocked Sock alternative that returns
  | a statically contained data upon read
  | and succeeds and ignores all writes.
  | The data to be returned is given to the
  | constructor and when it is exhausted
  | an EOF is returned by further reads.
  |
  */
pub struct StaticContentsSock {
    base:     Sock,
    contents: String,
    consumed: RefCell<usize>,
}

impl Drop for StaticContentsSock {
    fn drop(&mut self) {
        todo!();
        /*
            Reset();
        */
    }
}

impl StaticContentsSock {

    pub fn new(contents: &String) -> Self {
    
        todo!();
        /*


            : m_contents{contents}, m_consumed{0}
            // Just a dummy number that is not INVALID_SOCKET.
            m_socket = INVALID_SOCKET - 1;
        */
    }
    
    pub fn assign_from(&mut self, other: Sock) -> &mut StaticContentsSock {
        
        todo!();
        /*
            assert(false && "Move of Sock into MockSock not allowed.");
            return *this;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            m_socket = INVALID_SOCKET;
        */
    }
    
    pub fn send(&self, 
        _0:  *const c_void,
        len: usize,
        _2:  i32) -> isize {
        
        todo!();
        /*
            return len;
        */
    }
    
    pub fn recv(&self, 
        buf:   *mut c_void,
        len:   usize,
        flags: i32) -> isize {
        
        todo!();
        /*
            const size_t consume_bytes{std::min(len, m_contents.size() - m_consumed)};
            std::memcpy(buf, m_contents.data() + m_consumed, consume_bytes);
            if ((flags & MSG_PEEK) == 0) {
                m_consumed += consume_bytes;
            }
            return consume_bytes;
        */
    }
    
    pub fn connect(&self, 
        _0: *const SocketAddr,
        _1: libc::socklen_t) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    pub fn get_sock_opt(&self, 
        level:    i32,
        opt_name: i32,
        opt_val:  *mut c_void,
        opt_len:  *mut libc::socklen_t) -> i32 {
        
        todo!();
        /*
            std::memset(opt_val, 0x0, *opt_len);
            return 0;
        */
    }
    
    pub fn wait(&self, 
        timeout:   Duration /* milliseconds */,
        requested: libevent_sys::event,
        occurred:  Option<*mut libevent_sys::event>) -> bool {

        todo!();
        /*
            if (occurred != nullptr) {
                *occurred = requested;
            }
            return true;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/test/util/net.cpp]

pub fn get_random_node_eviction_candidates(
        n_candidates:   i32,
        random_context: &mut FastRandomContext) -> Vec<NodeEvictionCandidate> {
    
    todo!();
        /*
            std::vector<NodeEvictionCandidate> candidates;
        for (int id = 0; id < n_candidates; ++id) {
            candidates.push_back({
                /* id */ id,
                /* nTimeConnected */ static_cast<int64_t>(random_context.randrange(100)),
                /* m_min_ping_time */ std::chrono::microseconds{random_context.randrange(100)},
                /* nLastBlockTime */ static_cast<int64_t>(random_context.randrange(100)),
                /* nLastTXTime */ static_cast<int64_t>(random_context.randrange(100)),
                /* fRelevantServices */ random_context.randbool(),
                /* fRelayTxes */ random_context.randbool(),
                /* fBloomFilter */ random_context.randbool(),
                /* nKeyedNetGroup */ random_context.randrange(100),
                /* prefer_evict */ random_context.randbool(),
                /* m_is_local */ random_context.randbool(),
                /* m_network */ ALL_NETWORKS[random_context.randrange(ALL_NETWORKS.size())],
            });
        }
        return candidates;
        */
}
