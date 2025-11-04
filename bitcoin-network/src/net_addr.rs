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
    
    pub fn new(ipv_6addr: &In6Addr, scope: Option<u32>) -> Self {
        trace!(target: "netaddr", scope = scope.unwrap_or(0), "Constructing NetAddr from legacy IPv6");
        // Safety: In6Addr is exactly 16 bytes.
        let bytes = unsafe {
            std::slice::from_raw_parts(ipv_6addr as *const _ as *const u8, ADDR_IPV6_SIZE)
        };

        let mut out = NetAddr::default();
        out.set_legacy_ipv6(bytes);
        *out.scope_id_mut() = scope.unwrap_or(0);
        debug!(target: "netaddr", net = ?out.get_net_class(), "Constructed NetAddr from IPv6 input");
        out
    }

    pub fn get_addr_bytes(&self) -> Vec<u8> {
        trace!(target: "netaddr", net = ?self.get_net_class(), "Computing NetAddr::get_addr_bytes");
        if self.is_addr_v1compatible() {
            let mut serialized = [0u8; NET_ADDR_V1_SERIALIZATION_SIZE];
            self.serialize_v1array(&mut serialized);
            debug!(target: "netaddr", "Serialized in legacy (v1) ADDR format");
            serialized.to_vec()
        } else {
            debug!(target: "netaddr", len = self.addr().len(), "Returning raw address bytes");
            self.addr().as_slice().to_vec()
        }
    }

    pub fn get_hash(&self) -> u64 {

        trace!(target: "netaddr", "Computing NetAddr::get_hash (Core-compatible double-SHA256)");

        // Compute the 256-bit double-SHA256 of the raw address bytes.
        let hash: u256 = bitcoin_hash::hash1(self.addr().as_slice());

        // Reinterpret the first 8 bytes of the hash as a little-endian u64.
        let bytes = hash.as_ref();
        assert!(bytes.len() >= 8, "u256 must contain at least eight bytes");

        let mut tmp = [0u8; 8];
        tmp.copy_from_slice(&bytes[..8]);
        let nret = u64::from_le_bytes(tmp);

        debug!(target: "netaddr", hash = ?hash, hash64 = nret, "Derived 64-bit NetAddr hash");
        nret
    }
}
