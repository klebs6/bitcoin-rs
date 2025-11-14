// ---------------- [ File: bitcoin-network/src/set_internal.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Create an "internal" address that represents
      | a name or FQDN. AddrMan uses these fake
      | addresses to keep track of which DNS
      | seeds were used.
      | 
      | 
      | -----------
      | @return
      | 
      | Whether or not the operation was successful.
      | @see NET_INTERNAL, INTERNAL_IN_IPV6_PREFIX,
      | CNetAddr::IsInternal(), CNetAddr::IsRFC4193()
      |
      */
    pub fn set_internal(&mut self, name: &str) -> bool {
        trace!(target: "netaddr", "SetInternal invoked");
        if name.is_empty() {
            debug!(target: "netaddr", "SetInternal: empty name rejected");
            return false;
        }
        *self.net_mut() = Network::NET_INTERNAL;

        // Derive the 10-byte internal key from a 32-byte digest of the name.
        // We use the same digest carrier type (u256) as elsewhere for consistency.
        let digest_u256: u256 = bitcoin_hash::hash1(name.as_bytes());
        let digest_bytes: &[u8; 32] = digest_u256.as_ref();

        *self.addr_mut() = PreVector::from(&digest_bytes[..ADDR_INTERNAL_SIZE]);
        debug!(target: "netaddr", "SetInternal: derived INTERNAL address from name hash");
        true
    }
}

#[cfg(test)]
mod internal_address_spec {
    use super::*;

    #[traced_test]
    fn set_internal_works_and_is_deterministic() {
        let mut a = NetAddr::default();
        assert!(a.set_internal("dnsseed.example.org"));
        assert!(a.is_internal());
        assert_eq!(a.addr().len(), ADDR_INTERNAL_SIZE);

        let mut b = NetAddr::default();
        assert!(b.set_internal("dnsseed.example.org"));

        assert_eq!(a.addr().as_slice(), b.addr().as_slice());
    }

    #[traced_test]
    fn empty_name_is_rejected() {
        let mut a = NetAddr::default();
        assert!(!a.set_internal(""));
    }
}
