// ---------------- [ File: bitcoin-network/src/netaddr_serde.rs ]
crate::ix!();

impl NetAddr {
    /**
      | Serialize to a stream.
      |
      */
    pub fn serialize<Stream>(&self, s: &mut Stream)
    where
        Stream: bitcoin_bitstream::GetVersion,
        for<'s> &'s mut Stream:
              core::ops::Shl<u8,  Output = &'s mut Stream>
            + core::ops::Shl<u64, Output = &'s mut Stream>,
        for<'s, 'a> &'s mut Stream:
              core::ops::Shl<&'a [u8], Output = &'s mut Stream>,
    {
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
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)
    where
        Stream: bitcoin_bitstream::GetVersion + bitcoin_bitstream::Backend,
        for<'s, 'a> &'s mut Stream:
              core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>
            + core::ops::Shr<&'a mut u8,  Output = &'s mut Stream>
            + core::ops::Shr<&'a mut u64, Output = &'s mut Stream>,
    {
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
                assert_eq!(
                    self.addr().len(),
                    NET_ADDR_V1_SERIALIZATION_SIZE,
                    "IPv6 must be 16 bytes"
                );
                arr.copy_from_slice(self.addr().as_slice());
                debug!(target: "netaddr", "V1 array: wrote bare IPv6");
                return;
            }
            Network::NET_IPV4 => {
                let prefix_size = IPV4_IN_IPV6_PREFIX.len();
                assert_eq!(
                    prefix_size + self.addr().len(),
                    NET_ADDR_V1_SERIALIZATION_SIZE,
                    "IPv4-in-IPv6 size mismatch"
                );
                arr[..prefix_size].copy_from_slice(&IPV4_IN_IPV6_PREFIX);
                arr[prefix_size..].copy_from_slice(self.addr().as_slice());
                debug!(target: "netaddr", "V1 array: wrote IPv4-in-IPv6");
                return;
            }
            Network::NET_INTERNAL => {
                let prefix_size = INTERNAL_IN_IPV6_PREFIX.len();
                assert_eq!(
                    prefix_size + self.addr().len(),
                    NET_ADDR_V1_SERIALIZATION_SIZE,
                    "INTERNAL-in-IPv6 size mismatch"
                );
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
    pub fn serialize_v1stream<Stream>(&self, s: &mut Stream)
    where
        for<'s, 'a> &'s mut Stream: core::ops::Shl<&'a [u8], Output = &'s mut Stream>,
    {
        trace!(target: "netaddr", "SerializeV1Stream");
        let mut serialized = [0u8; NET_ADDR_V1_SERIALIZATION_SIZE];
        self.serialize_v1array(&mut serialized);
        let _s = s << &serialized[..];
        debug!(target: "netaddr", "Wrote V1 array (16 bytes) to stream");
    }

    /**
      | Serialize as ADDRv2 / BIP155.
      |
      */
    pub fn serialize_v2stream<Stream>(&self, s: &mut Stream)
    where
        for<'s> &'s mut Stream:
              core::ops::Shl<u8,  Output = &'s mut Stream>
            + core::ops::Shl<u64, Output = &'s mut Stream>,
        for<'s, 'a> &'s mut Stream:
              core::ops::Shl<&'a [u8], Output = &'s mut Stream>,
    {
        trace!(target: "netaddr", net = ?self.get_net_class(), "SerializeV2Stream");
        if self.is_internal() {
            // Serialize NET_INTERNAL as embedded in IPv6. We need to
            // serialize such addresses from addrman.
            let s = s << (BIP155Network::IPV6 as u8);
            let s = s << (ADDR_IPV6_SIZE as u64); // CompactSize(16)
            self.serialize_v1stream(s);           // writes the v1-encoded IPv6 payload
            debug!(target: "netaddr", "ADDRv2: wrote INTERNAL embedded as IPv6");
            return;
        }

        let s = s << (self.get_bip155network() as u8);
        // In ADDRv2, addresses are serialized as CompactSize length + raw bytes.
        let s = s << (self.addr().len() as u64);
        let _s = s << self.addr().as_slice();
        debug!(
            target: "netaddr",
            addr_len = self.addr().len(),
            "ADDRv2: wrote (network id + varbytes)"
        );
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
    pub fn unserialize_v1stream<Stream>(&mut self, s: &mut Stream)
    where
        for<'s, 'a> &'s mut Stream: core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>,
    {
        trace!(target: "netaddr", "UnserializeV1Stream");
        let mut serialized = [0u8; NET_ADDR_V1_SERIALIZATION_SIZE];
        let _s = s >> &mut serialized[..];
        self.unserialize_v1array(&mut serialized);
        debug!(target: "netaddr", net = ?self.get_net_class(), "V1 stream parsed");
    }

    /**
      | Unserialize from a ADDRv2 / BIP155 format.
      |
      */
    pub fn unserialize_v2stream<Stream>(&mut self, s: &mut Stream)
    where
        Stream: bitcoin_bitstream::Backend,
        for<'s, 'a> &'s mut Stream:
              core::ops::Shr<&'a mut u8,  Output = &'s mut Stream>
            + core::ops::Shr<&'a mut u64, Output = &'s mut Stream>
            + core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>,
    {
        trace!(target: "netaddr", "UnserializeV2Stream");

        // Read network id (BIP155).
        let mut bip155_net: u8 = 0;
        let s = s >> &mut bip155_net;

        // Read CompactSize length into a u64, then convert.
        let mut address_size_u64: u64 = 0;
        let s = s >> &mut address_size_u64;

        let address_size_usize: usize = match usize::try_from(address_size_u64) {
            Ok(v) => v,
            Err(_) => {
                error!(target: "netaddr", got = address_size_u64, "ADDRv2 length does not fit in usize");
                panic!("Address too long to fit in usize: {}", address_size_u64);
            }
        };

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
            let _s = s >> &mut tmp[..];
            *self.addr_mut() = PreVector::from(tmp.as_slice());

            if *self.net() != Network::NET_IPV6 {
                debug!(target: "netaddr", net = ?self.get_net_class(), "Parsed non-IPv6 ADDRv2 address");
                return;
            }

            // Do some special checks on IPv6 addresses.

            // Recognize NET_INTERNAL embedded in IPv6, such addresses are not
            // gossiped but could be coming from addrman, when unserializing from
            // disk.
            if has_prefix(self.addr(), &INTERNAL_IN_IPV6_PREFIX) {
                *self.net_mut() = Network::NET_INTERNAL;

                let prefix = INTERNAL_IN_IPV6_PREFIX.len();
                let mut shrunk = [0u8; ADDR_INTERNAL_SIZE];
                shrunk.copy_from_slice(&self.addr().as_slice()[prefix..prefix + ADDR_INTERNAL_SIZE]);
                *self.addr_mut() = PreVector::from(&shrunk[..]);

                debug!(target: "netaddr", "Detected INTERNAL-in-IPv6 and re-mapped to NET_INTERNAL");
                return;
            }

            if !has_prefix(self.addr(), &IPV4_IN_IPV6_PREFIX)
                && !has_prefix(self.addr(), &TORV2_IN_IPV6_PREFIX)
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
            s.ignore(address_size_usize);
        }

        // Mimic a default-constructed CNetAddr object which is !IsValid() and thus
        // will not be gossiped, but continue reading next addresses from the stream.
        *self.net_mut() = Network::NET_IPV6;
        *self.addr_mut() = PreVector::from(&[0u8; ADDR_IPV6_SIZE][..]);
        debug!(target: "netaddr", "Set NetAddr to !IsValid() placeholder (::)");
    }
}

#[cfg(test)]
mod serde_roundtrip_spec {
    use super::*;
    use std::io::{Read, Write};

    #[derive(Clone)]
    struct MockStream {
        buf: Vec<u8>,
        rd: usize,
        version: i32,
    }

    impl MockStream {
        fn with_version(version: i32) -> Self {
            info!(version, "Initializing MockStream (writer)");
            Self { buf: Vec::new(), rd: 0, version }
        }
        fn from_buf(buf: Vec<u8>, version: i32) -> Self {
            info!(version, len = buf.len(), "Initializing MockStream (reader)");
            Self { buf, rd: 0, version }
        }
    }

    impl bitcoin_bitstream::GetVersion for MockStream {
        fn get_version(&self) -> i32 { self.version }
    }

    impl bitcoin_bitstream::Backend for MockStream {
        fn ignore(&mut self, n: usize) {
            debug!(skip = n, before = self.rd, after = self.rd.saturating_add(n), total = self.buf.len(), "Backend::ignore");
            self.rd = self.rd.saturating_add(n);
        }
        fn size(&self) -> usize {
            self.buf.len()
        }
    }

    impl Read for MockStream {
        fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
            let remaining = self.buf.len().saturating_sub(self.rd);
            let take = remaining.min(out.len());
            if take == 0 {
                return Ok(0);
            }
            out[..take].copy_from_slice(&self.buf[self.rd..self.rd + take]);
            self.rd += take;
            Ok(take)
        }
    }

    impl Write for MockStream {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.buf.extend_from_slice(data);
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }

    impl<'a> core::ops::Shl<u8> for &'a mut MockStream {
        type Output = &'a mut MockStream;
        fn shl(self, rhs: u8) -> Self::Output {
            debug!(byte = rhs, "MockStream Shl<u8>");
            self.buf.push(rhs);
            self
        }
    }

    impl<'a> core::ops::Shl<u64> for &'a mut MockStream {
        type Output = &'a mut MockStream;
        fn shl(self, rhs: u64) -> Self::Output {
            debug!(value = rhs, "MockStream Shl<u64> (little-endian)");
            self.buf.extend_from_slice(&rhs.to_le_bytes());
            self
        }
    }

    // Allow independent lifetimes for &mut MockStream and the slice being written.
    impl<'a, 'b> core::ops::Shl<&'b [u8]> for &'a mut MockStream {
        type Output = &'a mut MockStream;
        fn shl(self, rhs: &'b [u8]) -> Self::Output {
            debug!(len = rhs.len(), "MockStream Shl<&[u8]>");
            self.buf.extend_from_slice(rhs);
            self
        }
    }

    // Allow independent lifetimes for &mut MockStream and the slice being read into.
    impl<'a, 'b> core::ops::Shr<&'b mut [u8]> for &'a mut MockStream {
        type Output = &'a mut MockStream;
        fn shr(self, out: &'b mut [u8]) -> Self::Output {
            let take = out.len();
            out.copy_from_slice(&self.buf[self.rd..self.rd + take]);
            self.rd += take;
            debug!(len = take, pos = self.rd, "MockStream Shr<&mut [u8]>");
            self
        }
    }

    impl<'s, 'a> core::ops::Shr<&'a mut u8> for &'s mut MockStream {
        type Output = &'s mut MockStream;
        fn shr(self, out: &'a mut u8) -> Self::Output {
            *out = self.buf[self.rd];
            self.rd += 1;
            debug!(read = *out, pos = self.rd, "MockStream Shr<&mut u8>");
            self
        }
    }

    impl<'s, 'a> core::ops::Shr<&'a mut u64> for &'s mut MockStream {
        type Output = &'s mut MockStream;
        fn shr(self, out: &'a mut u64) -> Self::Output {
            let mut le = [0u8; 8];
            le.copy_from_slice(&self.buf[self.rd..self.rd + 8]);
            *out = u64::from_le_bytes(le);
            self.rd += 8;
            debug!(read = *out, pos = self.rd, "MockStream Shr<&mut u64>");
            self
        }
    }

    fn make_netaddr_ipv4() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[1u8,2,3,4][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn make_netaddr_ipv6() -> NetAddr {
        let mut b = [0u8; 16];
        b[0..4].copy_from_slice(&[0x20, 0x01, 0x48, 0x60]);
        NetAddrBuilder::default()
            .addr(PreVector::from(&b[..]))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn make_netaddr_onion() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[0x42u8; ADDR_TORV3_SIZE][..]))
            .net(Network::NET_ONION)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn make_netaddr_i2p() -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&[0x22u8; ADDR_I2P_SIZE][..]))
            .net(Network::NET_I2P)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn make_netaddr_cjdns() -> NetAddr {
        let mut b = [0u8; 16];
        b[0] = 0xFC;
        NetAddrBuilder::default()
            .addr(PreVector::from(&b[..]))
            .net(Network::NET_CJDNS)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn v1_serialize_unserialize_roundtrip_ipv4() {
        let a = make_netaddr_ipv4();
        let mut s = MockStream::with_version(0);
        a.serialize(&mut s);

        let mut b = NetAddr::default();
        let mut r = MockStream::from_buf(s.buf.clone(), 0);
        b.unserialize(&mut r);

        assert_eq!(*a.net(), *b.net());
        assert_eq!(a.addr().as_slice(), b.addr().as_slice());
    }

    #[traced_test]
    fn v2_serialize_unserialize_roundtrip_various_networks() {
        let nets = [
            make_netaddr_ipv4(),
            make_netaddr_ipv6(),
            make_netaddr_onion(),
            make_netaddr_i2p(),
            make_netaddr_cjdns(),
        ];

        for a in nets.iter() {
            let mut s = MockStream::with_version(ADDRV2_FORMAT);
            a.serialize(&mut s);

            let mut b = NetAddr::default();
            let mut r = MockStream::from_buf(s.buf.clone(), ADDRV2_FORMAT);
            b.unserialize(&mut r);

            assert_eq!(*a.net(), *b.net());
            assert_eq!(a.addr().as_slice(), b.addr().as_slice());
        }
    }

    #[traced_test]
    fn v2_internal_embedded_as_ipv6_roundtrip() {
        let mut internal = NetAddr::default();
        assert!(internal.set_internal("seed.example"));

        let mut s = MockStream::with_version(ADDRV2_FORMAT);
        internal.serialize(&mut s);

        let mut out = NetAddr::default();
        let mut r = MockStream::from_buf(s.buf.clone(), ADDRV2_FORMAT);
        out.unserialize(&mut r);

        assert!(out.is_internal());
        assert_eq!(out.addr().len(), ADDR_INTERNAL_SIZE);
        assert_eq!(internal.addr().as_slice(), out.addr().as_slice());
    }

    #[traced_test]
    fn v2_unknown_network_id_results_in_invalid_placeholder() {
        // Prepare buffer for: id=250, len=3, payload=0x01,0x02,0x03
        let mut s = MockStream::with_version(ADDRV2_FORMAT);
        &mut s << 250u8;
        &mut s << (3u64);
        &mut s << &[1u8,2,3][..];

        let mut out = NetAddr::default();
        let mut r = MockStream::from_buf(s.buf.clone(), ADDRV2_FORMAT);
        out.unserialize_v2stream(&mut r);

        // Should be reset to invalid IPv6 (::)
        assert_eq!(*out.net(), Network::NET_IPV6);
        assert_eq!(out.addr().as_slice(), &[0u8; ADDR_IPV6_SIZE]);
    }

    #[traced_test]
    fn v2_forbidden_ipv4_or_torv2_embedded_in_ipv6_is_ignored() {
        // id=IPv6(2), len=16, payload begins with IPv4-in-IPv6 prefix
        let mut payload = [0u8; 16];
        payload[..IPV4_IN_IPV6_PREFIX.len()].copy_from_slice(&IPV4_IN_IPV6_PREFIX);

        let mut s = MockStream::with_version(ADDRV2_FORMAT);
        &mut s << (BIP155Network::IPV6 as u8);
        &mut s << (16u64);
        &mut s << &payload[..];

        let mut out = NetAddr::default();
        let mut r = MockStream::from_buf(s.buf.clone(), ADDRV2_FORMAT);
        out.unserialize_v2stream(&mut r);

        assert_eq!(*out.net(), Network::NET_IPV6);
        assert_eq!(out.addr().as_slice(), &[0u8; 16]);
    }
}
