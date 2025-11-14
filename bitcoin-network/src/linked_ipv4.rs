// ---------------- [ File: bitcoin-network/src/linked_ipv4.rs ]
crate::ix!();

/// Read big‑endian u32 from the first 4 bytes (panics if slice too short)
#[inline(always)]
fn read_be32(b: &[u8]) -> u32 {
    ((b[0] as u32) << 24)
        | ((b[1] as u32) << 16)
        | ((b[2] as u32) << 8)
        | (b[3] as u32)
}

impl NetAddr {
    
    /**
      | Whether this address has a linked IPv4
      | address (see `get_linked_ipv4`).
      |
      */
    #[inline]
    pub fn has_linked_ipv4(&self) -> bool {
        self.is_routable()
            && (self.is_ipv4()
                || self.isrfc6145()
                || self.isrfc6052()
                || self.isrfc3964()
                || self.isrfc4380())
    }

    /// Extract the relevant IPv4 for mapped / tunnelled forms. Always returns
    /// the address in network‑byte‑order.
    /// 
    /// **Panics** if called when `has_linked_ipv4()` is false (same contract as
    /// the C++ original).
    ///
    /// For IPv4, mapped IPv4, SIIT translated
    /// 
    /// IPv4, Teredo, 6to4 tunneled addresses, return the relevant IPv4 address
    /// as a uint32.
    ///
    pub fn get_linked_ipv4(&self) -> u32 {
        trace!(target: "netaddr", net = ?self.net(), "Deriving linked IPv4");
        if self.is_ipv4() {
            return read_be32(self.addr());
        }

        if self.isrfc6052() || self.isrfc6145() {
            // mapped IPv4, SIIT translated IPv4: the IPv4 address is the last 4 bytes of the address
            return read_be32(&self.addr()[12..]);
        }

        if self.isrfc3964() {
            // 6to4 tunneled IPv4: the IPv4 address is in bytes 2-6
            return read_be32(&self.addr()[2..6]);
        }

        if self.isrfc4380() {
            // Teredo tunneled IPv4: the IPv4 address is in the last 4 bytes of the address, but bitflipped
            return !read_be32(&self.addr()[12..]);
        }

        panic!("get_linked_ipv4() called on address without an embedded IPv4");
    }
}

#[cfg(test)]
mod linked_ipv4_tests {
    use super::*;

    fn v4(o: [u8; 4]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(o.to_vec().as_slice()))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    fn v6(bytes: [u8; 16]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(bytes.to_vec().as_slice()))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn plain_ipv4_roundtrip() {
        let ip = v4([8, 8, 4, 4]);
        assert!(ip.has_linked_ipv4());
        assert_eq!(ip.get_linked_ipv4(), 0x08080404);
    }

    #[traced_test]
    fn rfc6052_mapping() {
        // 64:ff9b::0808:0808 -> mapped 8.8.8.8
        let mut bytes = [0u8; 16];
        bytes[..12].copy_from_slice(&[0x00, 0x64, 0xFF, 0x9B, 0, 0, 0, 0, 0, 0, 0, 0]);
        bytes[12..].copy_from_slice(&[8, 8, 8, 8]);
        let ip = v6(bytes);
        assert!(ip.has_linked_ipv4());
        assert_eq!(ip.get_linked_ipv4(), 0x08080808);
    }
}

#[cfg(test)]
mod linked_ipv4_additional_spec {
    use super::*;

    fn v6(bytes: [u8; 16]) -> NetAddr {
        NetAddrBuilder::default()
            .addr(PreVector::from(&bytes[..]))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn rfc6145_translated_ipv4_is_last_four_bytes() {
        // ::ffff:0:0/96 (RFC6145) + 203.0.113.6
        let mut b = [0u8; 16];
        b[..12].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0xFF, 0xFF, 0x00, 0x00]);
        b[12..].copy_from_slice(&[203, 0, 113, 6]);
        let a = v6(b);
        assert!(a.has_linked_ipv4());
        let got = a.get_linked_ipv4();
        info!(got_hex = format_args!("{:#010x}", got), "RFC6145 mapped IPv4");
        assert_eq!(got, 0xCB007106);
    }

    #[traced_test]
    fn rfc3964_6to4_extracts_middle_bytes() {
        // 6to4: 2002:0102:0304:: → embedded IPv4 1.2.3.4
        let mut b = [0u8; 16];
        b[0] = 0x20;
        b[1] = 0x02;
        b[2..6].copy_from_slice(&[1, 2, 3, 4]);
        let a = v6(b);
        assert!(a.has_linked_ipv4());
        let got = a.get_linked_ipv4();
        info!(got_hex = format_args!("{:#010x}", got), "RFC3964 6to4 embedded IPv4");
        assert_eq!(got, 0x01020304);
    }

    #[traced_test]
    fn rfc4380_teredo_last_four_bytes_bitflipped() {
        // Teredo: 2001::/32, last 4 bytes are NOT of the IPv4 address
        // Target IPv4 = 9.8.7.6 → embed as bitwise NOT: [246, 247, 248, 249]
        let mut b = [0u8; 16];
        b[..4].copy_from_slice(&[0x20, 0x01, 0x00, 0x00]);
        b[12..].copy_from_slice(&[!9u8, !8u8, !7u8, !6u8]);
        let a = v6(b);
        assert!(a.has_linked_ipv4());
        let got = a.get_linked_ipv4();
        info!(got_hex = format_args!("{:#010x}", got), "RFC4380 Teredo IPv4 (bit‑flipped)");
        assert_eq!(got, 0x09080706);
    }

    #[traced_test]
    fn rfc3964_6to4_extracts_embedded_ipv4() {
        // 6to4: 2002:0102:0304:: -> embeds 1.2.3.4 in bytes 2..6
        let mut b = [0u8; 16];
        b[0..2].copy_from_slice(&[0x20, 0x02]);
        b[2..6].copy_from_slice(&[1, 2, 3, 4]);
        let ip = v6(b);
        assert!(ip.has_linked_ipv4());
        let linked = ip.get_linked_ipv4();
        info!(linked_hex = format_args!("{:#x}", linked), "Linked IPv4 from 6to4");
        assert_eq!(linked, 0x01020304);
    }

    #[traced_test]
    fn teredo_extracts_inverted_tail_ipv4() {
        // Teredo: 2001::/32 with last 4 bytes bit‑flipped
        let mut b = [0u8; 16];
        b[0..4].copy_from_slice(&[0x20, 0x01, 0x00, 0x00]);
        // tail is NOT(1.2.3.4) = FE FD FC FB
        b[12..16].copy_from_slice(&[0xFE, 0xFD, 0xFC, 0xFB]);
        let ip = v6(b);
        assert!(ip.has_linked_ipv4());
        let linked = ip.get_linked_ipv4();
        debug!(linked_hex = format_args!("{:#x}", linked), "Linked IPv4 from Teredo");
        assert_eq!(linked, 0x01020304);
    }
}
