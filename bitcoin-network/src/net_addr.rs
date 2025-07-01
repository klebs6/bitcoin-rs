// ---------------- [ File: bitcoin-network/src/net_addr.rs ]
crate::ix!();

/**
  | Network address.
  |
  */
#[derive(Builder,Setters,Getters,MutGetters,Debug,Serialize,Deserialize,Clone,Hash)]
#[getset(get="pub",set="pub",get_mut="pub")]
#[builder(setter(into))]
pub struct NetAddr {

    /**
      | Raw representation of the network address.
      | 
      | In network byte order (big endian) for
      | IPv4 and IPv6.
      |
      */
    addr:     PreVector<u8,ADDR_IPV6_SIZE>,

    /**
      | Network to which this address belongs.
      |
      */
    net:      Network,

    /**
      | Scope id if scoped/link-local IPV6
      | address.
      | 
      | See https://tools.ietf.org/html/rfc4007
      |
      */
    scope_id: u32,
}

impl Default for NetAddr {

    /**
      | Construct an unspecified IPv6 network
      | address (::/128).
      | 
      | -----------
      | @note
      | 
      | This address is considered invalid
      | by
      | 
      | CNetAddr::IsValid()
      |
      */
    fn default() -> Self {
        Self {
            addr:     PreVector::with_capacity(ADDR_IPV6_SIZE),
            net:      Network::NET_IPV6,
            scope_id: 0,
        }
    }
}

impl PartialEq<NetAddr> for NetAddr {
    
    #[inline] fn eq(&self, other: &NetAddr) -> bool {
        todo!();
        /*
            return a.m_net == b.m_net && a.m_addr == b.m_addr;
        */
    }
}

impl Eq for NetAddr {}

impl Ord for NetAddr {
    
    #[inline] fn cmp(&self, other: &NetAddr) -> Ordering {
        todo!();
        /*
            return std::tie(a.m_net, a.m_addr) < std::tie(b.m_net, b.m_addr);
        */
    }
}

impl PartialOrd<NetAddr> for NetAddr {
    #[inline] fn partial_cmp(&self, other: &NetAddr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&InAddr> for NetAddr {
    fn from(ipv_4addr: &InAddr) -> Self {
    
        todo!();
        /*


            m_net = NET_IPV4;
        const uint8_t* ptr = reinterpret_cast<const uint8_t*>(&ipv4Addr);
        m_addr.assign(ptr, ptr + ADDR_IPV4_SIZE);
        */
    }
}

impl NetAddr {
    
    pub fn new(
        ipv_6addr: &In6Addr,
        scope:     Option<u32>) -> Self {

        let scope: u32 = scope.unwrap_or(0);
    
        todo!();
        /*


            SetLegacyIPv6(Span<const uint8_t>(reinterpret_cast<const uint8_t*>(&ipv6Addr), sizeof(ipv6Addr)));
        m_scope_id = scope;
        */
    }
    
    pub fn get_addr_bytes(&self) -> Vec<u8> {
        
        todo!();
        /*
            if (IsAddrV1Compatible()) {
            uint8_t serialized[NET_ADDR_V1_SERIALIZATION_SIZE];
            SerializeV1Array(serialized);
            return {std::begin(serialized), std::end(serialized)};
        }
        return std::vector<unsigned char>(m_addr.begin(), m_addr.end());
        */
    }
    
    pub fn get_hash(&self) -> u64 {
        
        todo!();
        /*
            uint256 hash = Hash(m_addr);
        uint64_t nRet;
        memcpy(&nRet, &hash, sizeof(nRet));
        return nRet;
        */
    }
}
