crate::ix!();

/**
  | A "source" is a source address from which
  | we have received a bunch of other addresses.
  |
  */
pub const NUM_SOURCES:              usize = 64;
pub const NUM_ADDRESSES_PER_SOURCE: usize = 256;

lazy_static!{
    pub static ref SOURCES:   Arc<Mutex<Vec<Address>>>      = Arc::new(Mutex::new(vec![]));
    pub static ref ADDRESSES: Arc<Mutex<Vec<Vec<Address>>>> = Arc::new(Mutex::new(vec![]));
}

pub fn create_addresses()  {

    if ADDRESSES.lock().len() > 0 {
        return;
    }

    /*
    let mut rng: FastRandomContext = FastRandomContext::from(&u256::from(&vec![123;32]));

    let mut rand_addr = || {

        unsafe {
            let mut addr: In6Addr = unsafe { std::mem::zeroed() };

            memcpy(
                &mut addr as *mut _ as *mut c_void, 
                rng.randbytes(size_of_val(&addr)).as_ptr() as *const c_void, 
                size_of_val(&addr)
            );

            let mut port = u16::default();

            memcpy(
                &mut port as *mut _ as *mut c_void, 
                rng.randbytes(size_of_val(&port)).as_ptr() as *const c_void, 
                size_of_val(&port)
            );

            if port == 0 {
                port = 1;
            }

            let service = Service::new_from_ip6(&addr,port);

            let mut ret: Address = Address::new(service, ServiceFlags::NODE_NETWORK);

            ret.n_time = get_adjusted_time().try_into().unwrap();

            ret
        }
    };

    for source_i in 0..NUM_SOURCES {
        G_SOURCES.lock().push(rand_addr());

        ADDRESSES.lock().push(Default::default());

        for addr_i in 0..NUM_ADDRESSES_PER_SOURCE {
            ADDRESSES.lock()[source_i].push(rand_addr());
        }
    }
    */
}

/**
  | A Service with information about it
  | as peer
  |
  */
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Address {

    pub service:    Service,

    /**
      | Always included in serialization.
      |
      */
    pub n_time:     u32,

    /**
      | Serialized as uint64_t in V1, and as
      | 
      | CompactSize in V2.
      |
      */
    pub n_services: ServiceFlags,
}

pub trait GetAddrRef {

    fn addr(&self) -> &Address;
}

pub trait GetAddrMut {
    fn addr_mut(&mut self) -> &mut Address;
}

impl Address {

    delegate!{

        to self.service {

            pub fn set_sock_addr(&mut self, paddr: *const SocketAddr) -> bool;

            pub fn to_string(&self) -> String;

            pub fn get_port(&self) -> u16;

            pub fn get_sock_addr(&self, 
                paddr:   *mut SocketAddr,
                addrlen: *mut libc::socklen_t) -> bool;

            pub fn get_key(&self) -> Vec<u8>;
        }

        to self.service.base {

            pub fn is_reachable(&self) -> bool;

            pub fn is_relayable(&self) -> bool;

            pub fn serialize<Stream>(&self, s: &mut Stream);

            pub fn unserialize<Stream>(&mut self, s: &mut Stream);

            pub fn serialize_v1array(&self, arr: &mut [u8; net_addr::V1_SERIALIZATION_SIZE]);

            pub fn serialize_v1stream<Stream>(&self, s: &mut Stream);

            pub fn serialize_v2stream<Stream>(&self, s: &mut Stream);

            pub fn unserialize_v1array(&mut self, arr: &mut [u8; net_addr::V1_SERIALIZATION_SIZE]);

            pub fn unserialize_v1stream<Stream>(&mut self, s: &mut Stream);

            pub fn unserialize_v2stream<Stream>(&mut self, s: &mut Stream);

            pub fn get_bip155network(&self) -> net_addr::BIP155Network;

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

impl Default for Address {
    
    fn default() -> Self {
        Self {
            service:    Service::default(),
            n_time:     address::TIME_INIT,
            n_services: ServiceFlags::NODE_NONE,
        }
    }
}

pub mod address {

    use super::*;

    pub const TIME_INIT: u32 = 100000000;

    /** 
     | Historically, CAddress disk serialization
     | stored the CLIENT_VERSION, optionally
     | OR'ed with the ADDRV2_FORMAT flag to
     | indicate V2 serialization. The first field
     | has since been disentangled from client
     | versioning, and now instead:
     |
     |  - The low bits (masked by
     |    DISK_VERSION_IGNORE_MASK) store the
     |    fixed value DISK_VERSION_INIT, (in case
     |    any code exists that treats it as
     |    a client version) but are ignored on
     |    deserialization.
     |
     |  - The high bits (masked by
     |    ~DISK_VERSION_IGNORE_MASK) store actual
     |    serialization information.  Only 0 or
     |    DISK_VERSION_ADDRV2 (equal to the
     |    historical ADDRV2_FORMAT) are valid now,
     |    and any other value triggers
     |    a deserialization failure. Other values
     |    can be added later if needed.
     |
     |  For disk deserialization, ADDRV2_FORMAT in
     |  the stream version signals that ADDRV2
     |  deserialization is permitted, but the
     |  actual format is determined by the high
     |  bits in the stored version field. For
     |  network serialization, the stream version
     |  having ADDRV2_FORMAT or not determines the
     |  actual format used (as it has no embedded
     |  version number).
     */
    pub const DISK_VERSION_INIT:        u32 = 220000;
    pub const DISK_VERSION_IGNORE_MASK: u32 = 0b00000000_00000111_11111111_11111111;

    /**
      | The version number written in disk serialized
      | addresses to indicate V2 serializations.
      | 
      | It must be exactly 1<<29, as that is the
      | value that historical versions used
      | for this (they used their internal ADDRV2_FORMAT
      | flag here).
      |
      */
    pub const DISK_VERSION_ADDRV2: u32 = 1 << 29;

    /**
      | DISK_VERSION_INIT must be covered
      | by
      | DISK_VERSION_IGNORE_MASK
      |
      */
    const_assert!{
        (DISK_VERSION_INIT & !DISK_VERSION_IGNORE_MASK) == 0
     }

    /**
      | DISK_VERSION_ADDRV2 must not be covered
      | by DISK_VERSION_IGNORE_MASK
      |
      */
    const_assert!{
        (DISK_VERSION_ADDRV2 & DISK_VERSION_IGNORE_MASK) == 0
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CAddress, obj)
        {
            // CAddress has a distinct network serialization and a disk serialization, but it should never
            // be hashed (except through CHashWriter in addrdb.cpp, which sets SER_DISK), and it's
            // ambiguous what that would mean. Make sure no code relying on that is introduced:
            assert(!(s.GetType() & SER_GETHASH));
            bool use_v2;
            if (s.GetType() & SER_DISK) {
                // In the disk serialization format, the encoding (v1 or v2) is determined by a flag version
                // that's part of the serialization itself. ADDRV2_FORMAT in the stream version only determines
                // whether V2 is chosen/permitted at all.
                uint32_t stored_format_version = DISK_VERSION_INIT;
                if (s.GetVersion() & ADDRV2_FORMAT) stored_format_version |= DISK_VERSION_ADDRV2;
                READWRITE(stored_format_version);
                stored_format_version &= ~DISK_VERSION_IGNORE_MASK; // ignore low bits
                if (stored_format_version == 0) {
                    use_v2 = false;
                } else if (stored_format_version == DISK_VERSION_ADDRV2 && (s.GetVersion() & ADDRV2_FORMAT)) {
                    // Only support v2 deserialization if ADDRV2_FORMAT is set.
                    use_v2 = true;
                } else {
                    throw std::ios_base::failure("Unsupported CAddress disk format version");
                }
            } else {
                // In the network serialization format, the encoding (v1 or v2) is determined directly by
                // the value of ADDRV2_FORMAT in the stream version, as no explicitly encoded version
                // exists in the stream.
                assert(s.GetType() & SER_NETWORK);
                use_v2 = s.GetVersion() & ADDRV2_FORMAT;
            }

            SER_READ(obj, obj.nTime = TIME_INIT);
            READWRITE(obj.nTime);
            // nServices is serialized as CompactSize in V2; as uint64_t in V1.
            if (use_v2) {
                uint64_t services_tmp;
                SER_WRITE(obj, services_tmp = obj.nServices);
                READWRITE(Using<CompactSizeFormatter<false>>(services_tmp));
                SER_READ(obj, obj.nServices = static_cast<ServiceFlags>(services_tmp));
            } else {
                READWRITE(Using<CustomUintFormatter<8>>(obj.nServices));
            }
            // Invoke V1/V2 serializer for CService parent object.
            OverrideStream<Stream> os(&s, s.GetType(), use_v2 ? ADDRV2_FORMAT : 0);
            SerReadWriteMany(os, ser_action, ReadWriteAsHelper<CService>(obj));
        }
    */
}

impl PartialEq<Address> for Address {
    
    #[inline] fn eq(&self, other: &Address) -> bool {

        self.n_time == other.n_time 
        && self.n_services == other.n_services 
        && self.service  == other.service
    }
}

impl Eq for Address {}

impl Address {

    pub fn new(
        ip_in:         Service,
        n_services_in: ServiceFlags) -> Self {
    
        todo!();
        /*


            : CService{ipIn}, nServices{nServicesIn}
        */
    }
    
    pub fn new_with_time_in(
        ip_in:         Service,
        n_services_in: ServiceFlags,
        n_time_in:     u32) -> Self {
    
        todo!();
        /*


            : CService{ipIn}, nTime{nTimeIn}, nServices{nServicesIn} 
    }{
        */
    }
}
