// ---------------- [ File: bitcoin-network/src/netaddress.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/netaddress.h]

/**
  | A flag that is ORed into the protocol
  | version to designate that addresses
  | should be serialized in (unserialized
  | from) v2 format (BIP155).
  | 
  | Make sure that this does not collide
  | with any of the values in `version.h`
  | or with `SERIALIZE_TRANSACTION_NO_WITNESS`.
  |
  */
pub const ADDRV2_FORMAT: i32 = 0x20000000;

/**
  | A network type.
  | 
  | -----------
  | @note
  | 
  | An address may belong to more than one
  | network, for example `10.0.0.1` belongs
  | to both `NET_UNROUTABLE` and `NET_IPV4`.
  | 
  | Keep these sequential starting from
  | 0 and `NET_MAX` as the last entry.
  | 
  | We have loops like `for (int i = 0; i < NET_MAX;
  | ++i)` that expect to iterate over all
  | enum values and also `GetExtNetwork()`
  | "extends" this enum by introducing
  | standalone constants starting from
  | `NET_MAX`.
  |
  */
#[repr(u8)]
#[derive(Debug,Serialize,Deserialize,Hash,PartialEq,Eq,Clone)]
pub enum Network {

    /**
      | Addresses from these networks are not
      | publicly routable on the global Internet.
      |
      */
    NET_UNROUTABLE = 0,

    /**
      | IPv4
      |
      */
    NET_IPV4,

    /**
      | IPv6
      |
      */
    NET_IPV6,

    /**
      | TOR (v2 or v3)
      |
      */
    NET_ONION,

    /**
      | I2P
      |
      */
    NET_I2P,

    /**
      | CJDNS
      |
      */
    NET_CJDNS,

    /**
      | A set of addresses that represent the hash
      | of a string or FQDN. We use them in
      | AddrMan to keep track of which DNS seeds
      | were used.
      */
    NET_INTERNAL,

    /**
      | Dummy value to indicate the number of
      | NET_* constants.
      |
      */
    NET_MAX,
}

impl Default for Network {
    fn default() -> Self {
        Self::NET_UNROUTABLE
    }
}

/**
  | Prefix of an IPv6 address when it contains an
  | embedded IPv4 address.
  |
  | Used when (un)serializing addresses in ADDRv1
  | format (pre-BIP155).
  */
pub const IPV4_IN_IPV6_PREFIX: [u8; 12] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF
];

/**
  | Prefix of an IPv6 address when it contains an
  | embedded TORv2 address.
  |
  | Used when (un)serializing addresses in ADDRv1
  | format (pre-BIP155).
  |
  | Such dummy IPv6 addresses are guaranteed to
  | not be publicly routable as they fall under
  | RFC4193's fc00::/7 subnet allocated to
  | unique-local addresses.
  */
pub const TORV2_IN_IPV6_PREFIX: [u8; 6] = [
    0xFD, 0x87, 0xD8, 0x7E, 0xEB, 0x43];

/**
  | Prefix of an IPv6 address when it contains an
  | embedded "internal" address.
  |
  | Used when (un)serializing addresses in ADDRv1
  | format (pre-BIP155).
  |
  | The prefix comes from 0xFD
  | + SHA256("bitcoin")[0:5].
  |
  | Such dummy IPv6 addresses are guaranteed to
  | not be publicly routable as they fall under
  | RFC4193's fc00::/7 subnet allocated to
  | unique-local addresses.
  */
pub const INTERNAL_IN_IPV6_PREFIX: [u8; 6] = [
    0xFD, 0x6B, 0x88, 0xC0, 0x87, 0x24 // 0xFD + sha256("bitcoin")[0:5].
];

/**
  | Size of IPv4 address (in bytes).
  |
  */
pub const ADDR_IPV4_SIZE: usize = 4;

/**
  | Size of IPv6 address (in bytes).
  |
  */
pub const ADDR_IPV6_SIZE: usize = 16;

/**
  | Size of TORv3 address (in bytes). This
  | is the length of just the address as used
  | in BIP155, without the checksum and
  | the version byte.
  |
  */
pub const ADDR_TORV3_SIZE: usize = 32;

/**
  | Size of I2P address (in bytes).
  |
  */
pub const ADDR_I2P_SIZE: usize = 32;

/**
  | Size of CJDNS address (in bytes).
  |
  */
pub const ADDR_CJDNS_SIZE: usize = 16;

/**
  | Size of "internal" (NET_INTERNAL)
  | address (in bytes).
  |
  */
pub const ADDR_INTERNAL_SIZE: usize = 10;

/**
  | SAM 3.1 and earlier do not support specifying
  | ports and force the port to 0.
  |
  */
pub const I2P_SAM31_PORT: u16 = 0;

pub mod torv3 {

    use super::*;

    /**
       https://gitweb.torproject.org/torspec.git/tree/rend-spec-v3.txt#n2135
      */
    pub const CHECKSUM_LEN: usize = 2;
    pub const VERSION:      &[u8] = &[3];
    pub const TOTAL_LEN:    usize = ADDR_TORV3_SIZE + CHECKSUM_LEN + size_of_val(VERSION);

    pub fn checksum(
        addr_pubkey: &[u8],
        checksum:    &mut [u8; CHECKSUM_LEN])  {

        // TORv3 CHECKSUM = H(".onion checksum" | PUBKEY | VERSION)[:2]
        pub const PREFIX: &'static str = ".onion checksum";

        let mut hasher: SHA3_256 = SHA3_256::default();

        hasher.write(PREFIX.as_bytes());

        hasher.write(addr_pubkey);

        hasher.write(VERSION);

        let mut checksum_full = [0_u8; SHA3_256_OUTPUT_SIZE];

        hasher.finalize(checksum_full.as_slice());

        checksum[0..CHECKSUM_LEN].copy_from_slice(&checksum_full[0..CHECKSUM_LEN]);
    }
}

pub fn pv_4to_string(a: &[u8]) -> String {
    
    todo!();
        /*
            return strprintf("%u.%u.%u.%u", a[0], a[1], a[2], a[3]);
        */
}

/**
  | Return an IPv6 address text representation with
  | zero compression as described in RFC 5952 ("A
  | Recommendation for IPv6 Address Text
  | Representation").
  */
pub fn pv_6to_string(
        a:        &[u8],
        scope_id: u32) -> String {
    
    todo!();
        /*
            assert(a.size() == ADDR_IPV6_SIZE);
        const std::array groups{
            ReadBE16(&a[0]),
            ReadBE16(&a[2]),
            ReadBE16(&a[4]),
            ReadBE16(&a[6]),
            ReadBE16(&a[8]),
            ReadBE16(&a[10]),
            ReadBE16(&a[12]),
            ReadBE16(&a[14]),
        };

        // The zero compression implementation is inspired by Rust's std::net::Ipv6Addr, see
        // https://github.com/rust-lang/rust/blob/cc4103089f40a163f6d143f06359cba7043da29b/library/std/src/net/ip.rs#L1635-L1683
        struct ZeroSpan {
            size_t start_index{0};
            size_t len{0};
        };

        // Find longest sequence of consecutive all-zero fields. Use first zero sequence if two or more
        // zero sequences of equal length are found.
        ZeroSpan longest, current;
        for (size_t i{0}; i < groups.size(); ++i) {
            if (groups[i] != 0) {
                current = {i + 1, 0};
                continue;
            }
            current.len += 1;
            if (current.len > longest.len) {
                longest = current;
            }
        }

        std::string r;
        r.reserve(39);
        for (size_t i{0}; i < groups.size(); ++i) {
            // Replace the longest sequence of consecutive all-zero fields with two colons ("::").
            if (longest.len >= 2 && i >= longest.start_index && i < longest.start_index + longest.len) {
                if (i == longest.start_index) {
                    r += "::";
                }
                continue;
            }
            r += strprintf("%s%x", ((!r.empty() && r.back() != ':') ? ":" : ""), groups[i]);
        }

        if (scope_id != 0) {
            r += strprintf("%%%u", scope_id);
        }

        return r;
        */
}

pub fn onion_to_string(addr: &[u8]) -> String {
    
    let mut checksum = [0_u8; torv3::CHECKSUM_LEN];

    torv3::checksum(addr, &mut checksum);

    //  TORv3 onion_address = base32(PUBKEY | CHECKSUM | VERSION) + ".onion"
    let mut address: PreVector::<u8,{torv3::TOTAL_LEN}> = PreVector::from(addr);

    address.extend(checksum);
    address.extend(torv3::VERSION.iter().cloned());

    format!(
        "{}.onion", 
        encode_base32(
            address.as_slice(),
            None
        )
    )
}

/**
  | private extensions to enum Network,
  | only returned by GetExtNetwork, and
  | only used in GetReachabilityFrom
  |
  */
pub const NET_UNKNOWN: i32 = Network::NET_MAX as i32 + 0;
pub const NET_TEREDO:  i32 = Network::NET_MAX as i32 + 1;

pub fn get_ext_network(maybe_addr: Option<&NetAddr>) -> i32 {

    if maybe_addr.is_none() {
        return NET_UNKNOWN;
    }

    let addr = maybe_addr.unwrap();

    if addr.isrfc4380() {
        return NET_TEREDO;
    }

    addr.get_network() as i32
}

/**
  | @return
  | 
  | The number of 1-bits in the prefix of
  | the specified subnet mask. If the specified
  | subnet mask is not a valid one, -1.
  |
  */
#[inline] pub fn netmask_bits(x: u8) -> i32 {
    
    match x {
        0x00  =>  0,
        0x80  =>  1,
        0xc0  =>  2,
        0xe0  =>  3,
        0xf0  =>  4,
        0xf8  =>  5,
        0xfc  =>  6,
        0xfe  =>  7,
        0xff  =>  8,
        _     => -1,
    }
}
