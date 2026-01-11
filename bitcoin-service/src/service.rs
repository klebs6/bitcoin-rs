// ---------------- [ File: bitcoin-service/src/service.rs ]
crate::ix!();

/**
  | A combination of a network address (CNetAddr)
  | and a (TCP) port
  |
  */
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Service {

    pub base: NetAddr,

    /**
      | host order
      |
      */
    pub port: u16,
}

pub trait GetServiceRef {
    fn service(&self) -> &Service;
}

pub trait GetServiceMut {
    fn service_mut(&mut self) -> &mut Service;
}

impl Service {

    delegate!{

        to self.base {

            pub fn is_relayable(&self) -> bool;

            pub fn serialize<Stream>(&self, s: &mut Stream)
            where
                Stream: GetVersion,
                for<'s> &'s mut Stream: core::ops::Shl<u8,  Output = &'s mut Stream>
                    + core::ops::Shl<u64, Output = &'s mut Stream>,
                for<'s, 'a> &'s mut Stream: core::ops::Shl<&'a [u8], Output = &'s mut Stream>;

            pub fn unserialize<Stream>(&mut self, s: &mut Stream)
            where
                Stream: GetVersion + Backend,
                for<'s, 'a> &'s mut Stream: core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>
                    + core::ops::Shr<&'a mut u8,  Output = &'s mut Stream>
                    + core::ops::Shr<&'a mut u64, Output = &'s mut Stream>;

            pub fn serialize_v1array(&self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE]);

            pub fn serialize_v1stream<Stream>(&self, s: &mut Stream)
            where
                for<'s, 'a> &'s mut Stream: core::ops::Shl<&'a [u8], Output = &'s mut Stream>;

            pub fn serialize_v2stream<Stream>(&self, s: &mut Stream)
            where
                for<'s> &'s mut Stream: core::ops::Shl<u8,  Output = &'s mut Stream>
                    + core::ops::Shl<u64, Output = &'s mut Stream>,
                for<'s, 'a> &'s mut Stream: core::ops::Shl<&'a [u8], Output = &'s mut Stream>;

            pub fn unserialize_v1array(&mut self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE]);

            pub fn unserialize_v1stream<Stream>(&mut self, s: &mut Stream)
            where
                for<'s, 'a> &'s mut Stream: core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>;

            pub fn unserialize_v2stream<Stream>(&mut self, s: &mut Stream)
            where
                Stream: Backend,
                for<'s, 'a> &'s mut Stream: core::ops::Shr<&'a mut u8,  Output = &'s mut Stream>
                    + core::ops::Shr<&'a mut u64, Output = &'s mut Stream>
                    + core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>;

            pub fn get_bip155network(&self) -> BIP155Network;

            pub fn set_net_from_bip155network(&mut self, 
                possible_bip155_net: u8,
                address_size:        usize) -> bool;

            pub fn setip(&mut self, ip_in: &NetAddr);

            pub fn set_legacy_ipv6(&mut self, ipv6: &[u8]);

            pub fn set_internal(&mut self, name: &str) -> bool;

            pub fn set_special(&mut self, addr: &String) -> bool;

            pub fn set_tor(&mut self, addr: &String) -> bool;

            pub fn seti2p(&mut self, addr: &String) -> bool;

            pub fn is_bind_any(&self) -> bool;

            pub fn is_ipv4(&self) -> bool;

            pub fn is_ipv6(&self) -> bool;

            pub fn isrfc1918(&self) -> bool;

            pub fn isrfc2544(&self) -> bool;

            pub fn isrfc3927(&self) -> bool;

            pub fn isrfc6598(&self) -> bool;

            pub fn isrfc5737(&self) -> bool;

            pub fn isrfc3849(&self) -> bool;

            pub fn isrfc3964(&self) -> bool;

            pub fn isrfc6052(&self) -> bool;

            pub fn isrfc4380(&self) -> bool;

            pub fn isrfc4862(&self) -> bool;

            pub fn isrfc4193(&self) -> bool;

            pub fn isrfc6145(&self) -> bool;

            pub fn isrfc4843(&self) -> bool;

            pub fn isrfc7343(&self) -> bool;

            pub fn is_he_net(&self) -> bool;

            pub fn is_tor(&self) -> bool;

            pub fn isi2p(&self) -> bool;

            pub fn iscjdns(&self) -> bool;

            pub fn is_local(&self) -> bool;

            pub fn is_valid(&self) -> bool;

            pub fn is_routable(&self) -> bool;

            pub fn is_internal(&self) -> bool;

            pub fn is_addr_v1compatible(&self) -> bool;

            pub fn get_network(&self) -> Network;

            pub fn get_in_addr(&self, pipv_4addr: *mut InAddr) -> bool;

            pub fn get_in_6addr(&self, pipv_6addr: *mut In6Addr) -> bool;

            pub fn has_linked_ipv4(&self) -> bool;

            pub fn get_linked_ipv4(&self) -> u32;

            pub fn get_net_class(&self) -> Network;

            pub fn get_mappedas(&self, asmap: &Vec<bool>) -> u32;

            pub fn get_group(&self, asmap: &Vec<bool>) -> Vec<u8>;

            pub fn get_addr_bytes(&self) -> Vec<u8>;

            pub fn get_hash(&self) -> u64;

            pub fn get_reachability_from(&self, paddr_partner: *const NetAddr) -> i32;
        }
    }
}

impl Hash for Service {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.net().hash(state);
        self.port.hash(state);
        self.base.addr().hash(state);
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CService, obj)
        {
            READWRITEAS(CNetAddr, obj);
            READWRITE(Using<BigEndianFormatter<2>>(obj.port));
        }
    */
}

impl PartialEq<Service> for Service {
    
    #[inline] fn eq(&self, other: &Service) -> bool {
        todo!();
        /*
            return static_cast<CNetAddr>(a) == static_cast<CNetAddr>(b) && a.port == b.port;
        */
    }
}

impl Eq for Service {}

impl Ord for Service {
    
    #[inline] fn cmp(&self, other: &Service) -> Ordering {
        todo!();
        /*
            return static_cast<CNetAddr>(a) < static_cast<CNetAddr>(b) || (static_cast<CNetAddr>(a) == static_cast<CNetAddr>(b) && a.port < b.port);
        */
    }
}

impl PartialOrd<Service> for Service {
    #[inline] fn partial_cmp(&self, other: &Service) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&libc::sockaddr_in> for Service {
    fn from(addr: &libc::sockaddr_in) -> Self {
    
        todo!();
        /*
        : net_addr(addr.sin_addr),
        : port(ntohs(addr.sin_port)),

            assert(addr.sin_family == AF_INET);
        */
    }
}

impl From<&libc::sockaddr_in6> for Service {
    
    fn from(addr: &libc::sockaddr_in6) -> Self {
    
        todo!();
        /*
        : net_addr(addr.sin6_addr, addr.sin6_scope_id),
        : port(ntohs(addr.sin6_port)),

            assert(addr.sin6_family == AF_INET6);
        */
    }
}

impl Default for Service {

    fn default() -> Self {
    
        todo!();
        /*
        : port(0),
        */
    }
}

impl Service {
    
    pub fn new_from_net_addr(
        cip:     &NetAddr,
        port_in: u16) -> Self {
    
        todo!();
        /*
        : net_addr(cip),
        : port(portIn),

        
        */
    }
    
    pub fn new_from_ip4(
        ipv4_addr: &InAddr,
        port_in:   u16) -> Self {
    
        todo!();
        /*
        : net_addr(ipv4Addr),
        : port(portIn),

        
        */
    }
    
    pub fn new_from_ip6(
        ipv6_addr: &In6Addr,
        port_in:   u16) -> Self {
    
        todo!();
        /*
        : net_addr(ipv6Addr),
        : port(portIn),

        
        */
    }
    
    pub fn set_sock_addr(&mut self, paddr: *const SocketAddr) -> bool {
        
        todo!();
        /*
            switch (paddr->sa_family) {
        case AF_INET:
            *this = CService(*(const struct sockaddr_in*)paddr);
            return true;
        case AF_INET6:
            *this = CService(*(const struct sockaddr_in6*)paddr);
            return true;
        default:
            return false;
        }
        */
    }
    
    pub fn get_port(&self) -> u16 {
        
        todo!();
        /*
            return port;
        */
    }

    /**
      | Obtain the IPv4/6 socket address this
      | represents.
      | 
      | -----------
      | @param[out] paddr
      | 
      | The obtained socket address.
      | ----------
      | @param[in,out] addrlen
      | 
      | The size, in bytes, of the address structure
      | pointed to by paddr. The value that's
      | pointed to by this parameter might change
      | after calling this function if the size
      | of the corresponding address structure
      | changed.
      | 
      | -----------
      | @return
      | 
      | Whether or not the operation was successful.
      |
      */
    pub fn get_sock_addr(&self, 
        paddr:   *mut SocketAddr,
        addrlen: *mut libc::socklen_t) -> bool {
        
        todo!();
        /*
            if (IsIPv4()) {
            if (*addrlen < (socklen_t)sizeof(struct sockaddr_in))
                return false;
            *addrlen = sizeof(struct sockaddr_in);
            struct sockaddr_in *paddrin = (struct sockaddr_in*)paddr;
            memset(paddrin, 0, *addrlen);
            if (!GetInAddr(&paddrin->sin_addr))
                return false;
            paddrin->sin_family = AF_INET;
            paddrin->sin_port = htons(port);
            return true;
        }
        if (IsIPv6()) {
            if (*addrlen < (socklen_t)sizeof(struct sockaddr_in6))
                return false;
            *addrlen = sizeof(struct sockaddr_in6);
            struct sockaddr_in6 *paddrin6 = (struct sockaddr_in6*)paddr;
            memset(paddrin6, 0, *addrlen);
            if (!GetIn6Addr(&paddrin6->sin6_addr))
                return false;
            paddrin6->sin6_scope_id = m_scope_id;
            paddrin6->sin6_family = AF_INET6;
            paddrin6->sin6_port = htons(port);
            return true;
        }
        return false;
        */
    }

    /**
      | @return
      | 
      | An identifier unique to this service's
      | address and port number.
      |
      */
    pub fn get_key(&self) -> Vec<u8> {
        
        todo!();
        /*
            auto key = GetAddrBytes();
        key.push_back(port / 0x100); // most significant byte of our port
        key.push_back(port & 0x0FF); // least significant byte of our port
        return key;
        */
    }
    
    pub fn to_string_port(&self) -> String {
        
        todo!();
        /*
            return strprintf("%u", port);
        */
    }
    
    pub fn to_string_ip_port(&self) -> String {
        
        todo!();
        /*
            if (IsIPv4() || IsTor() || IsI2P() || IsInternal()) {
            return ToStringIP() + ":" + ToStringPort();
        } else {
            return "[" + ToStringIP() + "]:" + ToStringPort();
        }
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return ToStringIPPort();
        */
    }

}

///-------------------
pub struct ServiceHash {
    salt_k0: u64, 
    salt_k1: u64, 
}

impl Default for ServiceHash {
    fn default() -> Self {
        Self {
            salt_k0: Self::get_rand_salt(),
            salt_k1: Self::get_rand_salt(),
        }
    }
}

impl BuildHasher for ServiceHash {
    type Hasher = SipHasher;
    fn build_hasher(&self) -> Self::Hasher {
        SipHasher::new_with_keys(self.salt_k0, self.salt_k1)
    }
}

impl ServiceHash {

    pub fn get_rand_salt() -> u64 {
        get_rand(u64::MAX)
    }
}
