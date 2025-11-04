// ---------------- [ File: bitcoin-network/src/netaddr_serde.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Serialize to a stream.
      |
      */
    pub fn serialize<Stream>(&self, s: &mut Stream) {
        trace!(target: "netaddr", "NetAddr::serialize entry");
        // Preserve Core's feature-bit gate for ADDRv2 (BIP155).
        let ver = s.get_version();
        if (ver & ADDRV2_FORMAT) != 0 {
            debug!(target: "netaddr", version = ver, "Using ADDRv2 (BIP155) serialization");
            self.serialize_v2stream(s);
        } else {
            debug!(target: "netaddr", version = ver, "Using legacy ADDRv1 serialization");
            self.serialize_v1stream(s);
        }
    }

    /**
      | Unserialize from a stream.
      |
      */
    pub fn unserialize<Stream>(&mut self, s: &mut Stream) {
        trace!(target: "netaddr", "NetAddr::unserialize entry");
        let ver = s.get_version();
        if (ver & ADDRV2_FORMAT) != 0 {
            debug!(target: "netaddr", version = ver, "Using ADDRv2 (BIP155) unserialization");
            self.unserialize_v2stream(s);
        } else {
            debug!(target: "netaddr", version = ver, "Using legacy ADDRv1 unserialization");
            self.unserialize_v1stream(s);
        }
    }

    /**
      | Serialize in pre-ADDRv2/BIP155 format
      | to an array.
      |
      */
    pub fn serialize_v1array(&self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE]) {
        trace!(target: "netaddr", net = ?self.get_net_class(), "SerializeV1Array");
        match *self.net() {
            Network::NET_IPV6 => {
                assert_eq!(self.addr().len(), NET_ADDR_V1_SERIALIZATION_SIZE, "IPv6 must be 16 bytes");
                arr.copy_from_slice(self.addr().as_slice());
                debug!(target: "netaddr", "V1 array: wrote bare IPv6");
                return;
            }
            Network::NET_IPV4 => {
                let prefix_size = IPV4_IN_IPV6_PREFIX.len();
                assert_eq!(prefix_size + self.addr().len(), NET_ADDR_V1_SERIALIZATION_SIZE, "IPv4-in-IPv6 size mismatch");
                arr[..prefix_size].copy_from_slice(&IPV4_IN_IPV6_PREFIX);
                arr[prefix_size..].copy_from_slice(self.addr().as_slice());
                debug!(target: "netaddr", "V1 array: wrote IPv4-in-IPv6");
                return;
            }
            Network::NET_INTERNAL => {
                let prefix_size = INTERNAL_IN_IPV6_PREFIX.len();
                assert_eq!(prefix_size + self.addr().len(), NET_ADDR_V1_SERIALIZATION_SIZE, "INTERNAL-in-IPv6 size mismatch");
                arr[..prefix_size].copy_from_slice(&INTERNAL_IN_IPV6_PREFIX);
                arr[prefix_size..].copy_from_slice(self.addr().as_slice());
                debug!(target: "netaddr", "V1 array: wrote INTERNAL-in-IPv6");
                return;
            }
            Network::NET_ONION | Network::NET_I2P | Network::NET_CJDNS => {
                // fall-through to zero-fill below
            }
            Network::NET_UNROUTABLE | Network::NET_MAX => {
                panic!("m_net is never and should not be set to NET_UNROUTABLE/NET_MAX");
            }
        }

        // Serialize ONION, I2P and CJDNS as all-zeros.
        arr.fill(0u8);
        debug!(target: "netaddr", "V1 array: wrote all-zeros for ONION/I2P/CJDNS");
    }

    /**
      | Serialize in pre-ADDRv2/BIP155 format
      | to a stream.
      |
      */
    pub fn serialize_v1stream<Stream>(&self, s: &mut Stream) {
        trace!(target: "netaddr", "SerializeV1Stream");
        let mut serialized = [0u8; NET_ADDR_V1_SERIALIZATION_SIZE];
        self.serialize_v1array(&mut serialized);
        *s << &serialized[..];
        debug!(target: "netaddr", "Wrote V1 array (16 bytes) to stream");
    }

    /**
      | Serialize as ADDRv2 / BIP155.
      |
      */
    pub fn serialize_v2stream<Stream>(&self, s: &mut Stream) {
        trace!(target: "netaddr", net = ?self.get_net_class(), "SerializeV2Stream");
        if self.is_internal() {
            // Serialize NET_INTERNAL as embedded in IPv6. We need to
            // serialize such addresses from addrman.
            *s << (BIP155Network::IPV6 as u8);
            *s << (ADDR_IPV6_SIZE as usize); // COMPACTSIZE(ADDR_IPV6_SIZE)
            self.serialize_v1stream(s);
            debug!(target: "netaddr", "ADDRv2: wrote INTERNAL embedded as IPv6");
            return;
        }

        *s << (self.get_bip155network() as u8);
        *s << self.addr(); // PreVector serializes as CompactSize length + bytes
        debug!(target: "netaddr", addr_len = self.addr().len(), "ADDRv2: wrote (network id + varbytes)");
    }

    /**
      | Unserialize from a pre-ADDRv2/BIP155
      | format from an array.
      |
      */
    pub fn unserialize_v1array(&mut self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE]) {
        trace!(target: "netaddr", "UnserializeV1Array");
        // Use SetLegacyIPv6() so that m_net is set correctly. For example
        // ::FFFF:0102:0304 should be set as m_net=NET_IPV4 (1.2.3.4).
        self.set_legacy_ipv6(arr);
        debug!(target: "netaddr", net = ?self.get_net_class(), "V1 array parsed via legacy IPv6 rules");
    }

    /**
      | Unserialize from a pre-ADDRv2/BIP155
      | format from a stream.
      |
      */
    pub fn unserialize_v1stream<Stream>(&mut self, s: &mut Stream) {
        trace!(target: "netaddr", "UnserializeV1Stream");
        let mut serialized = [0u8; NET_ADDR_V1_SERIALIZATION_SIZE];
        *s >> &mut serialized[..];
        self.unserialize_v1array(&mut serialized);
        debug!(target: "netaddr", net = ?self.get_net_class(), "V1 stream parsed");
    }

    /**
      | Unserialize from a ADDRv2 / BIP155 format.
      |
      */
    pub fn unserialize_v2stream<Stream>(&mut self, s: &mut Stream)  {
    
        trace!(target: "netaddr", "UnserializeV2Stream");
        // Read network id (BIP155).
        let mut bip155_net: u8 = 0;
        *s >> &mut bip155_net;

        // Read CompactSize length into a native usize.
        let mut address_size_usize: usize = 0;
        *s >> &mut address_size_usize;

        if address_size_usize > BIP155_MAX_ADDRV2_SIZE {
            error!(
                target: "netaddr",
                got = address_size_usize,
                max = BIP155_MAX_ADDRV2_SIZE,
                "ADDRv2 address too long"
            );
            panic!(
                "Address too long: {} > {}",
                address_size_usize, BIP155_MAX_ADDRV2_SIZE
            );
        }

        *self.scope_id_mut() = 0;

        if self.set_net_from_bip155network(bip155_net, address_size_usize) {
            // Read exactly address_size bytes into m_addr.
            let mut tmp = vec![0u8; address_size_usize];
            *s >> &mut tmp[..];
            *self.addr_mut() = PreVector::from(tmp.as_slice());

            if *self.net() != Network::NET_IPV6 {
                debug!(target: "netaddr", net = ?self.get_net_class(), "Parsed non-IPv6 ADDRv2 address");
                return;
            }

            // Do some special checks on IPv6 addresses.

            // Recognize NET_INTERNAL embedded in IPv6, such addresses are not
            // gossiped but could be coming from addrman, when unserializing from
            // disk.
            if has_prefix(self.addr().as_slice(), &INTERNAL_IN_IPV6_PREFIX) {
                *self.net_mut() = Network::NET_INTERNAL;

                let prefix = INTERNAL_IN_IPV6_PREFIX.len();
                let mut shrunk = [0u8; ADDR_INTERNAL_SIZE];
                shrunk.copy_from_slice(&self.addr().as_slice()[prefix..prefix + ADDR_INTERNAL_SIZE]);
                *self.addr_mut() = PreVector::from(&shrunk[..]);

                debug!(target: "netaddr", "Detected INTERNAL-in-IPv6 and re-mapped to NET_INTERNAL");
                return;
            }

            if !has_prefix(self.addr().as_slice(), &IPV4_IN_IPV6_PREFIX)
                && !has_prefix(self.addr().as_slice(), &TORV2_IN_IPV6_PREFIX)
            {
                // Normal IPv6; keep as-is.
                debug!(target: "netaddr", "Parsed ordinary IPv6 ADDRv2 address");
                return;
            }

            // IPv4 and TORv2 are not supposed to be embedded in IPv6 (like in V1
            // encoding). Unserialize as !IsValid(), thus ignoring them.
            warn!(target: "netaddr", "Found forbidden IPv4/TORv2 embedding in ADDRv2 IPv6 payload; marking invalid");
        } else {
            // If we receive an unknown BIP155 network id (from the future?) then
            // ignore the address - unserialize as !IsValid().
            debug!(target: "netaddr", id = bip155_net, size = address_size_usize, "Unknown BIP155 network id; skipping payload");
            s.ignore(address_size_usize).expect("stream ignore failed");
        }

        // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
        // will not be gossiped, but continue reading next addresses from the stream.
        *self.net_mut() = Network::NET_IPV6;
        *self.addr_mut() = PreVector::from(&[0u8; ADDR_IPV6_SIZE][..]);
        debug!(target: "netaddr", "Set NetAddr to !IsValid() placeholder (::)");
    }
}
