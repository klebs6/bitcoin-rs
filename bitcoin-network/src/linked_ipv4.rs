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
