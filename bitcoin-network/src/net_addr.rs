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
    
    #[inline]
    fn eq(&self, other: &NetAddr) -> bool {
        trace!(
            target: "netaddr",
            ours  = ?self.net(),
            theirs = ?other.net(),
            "Comparing NetAddr equality"
        );
        *self.net() == *other.net() && self.addr() == other.addr()
    }
}

impl Eq for NetAddr {}

impl Ord for NetAddr {
    
    #[inline]
    fn cmp(&self, other: &NetAddr) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        // First compare the network type; if those are equal,
        // fall back to a lexicographic comparison of the raw bytes.
        let by_net = (*self.net() as u8).cmp(&(*other.net() as u8));
        if by_net == Ordering::Equal {
            self.addr().cmp(other.addr())
        } else {
            by_net
        }
    }
}

impl PartialOrd<NetAddr> for NetAddr {
    #[inline] fn partial_cmp(&self, other: &NetAddr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&InAddr> for NetAddr {
    fn from(ipv4_addr: &InAddr) -> Self {
        trace!(target: "netaddr", "Constructing NetAddr from InAddr (IPv4)");
        // Safety: an `InAddr` is exactly four bytes in network byte‑order.
        let bytes = unsafe {
            std::slice::from_raw_parts(ipv4_addr as *const _ as *const u8, ADDR_IPV4_SIZE)
        };
        NetAddrBuilder::default()
            .addr(PreVector::from(bytes))
            .net(Network::NET_IPV4)
            .scope_id(0_u32)
            .build()
            .expect("building IPv4 NetAddr never fails")
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
