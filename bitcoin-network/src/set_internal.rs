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
        
        if name.is_empty() {
            return false;
        }
        *self.net_mut() = Network::NET_INTERNAL;

        let mut digest = [0u8; 32];
        let mut sha = Sha256::new();
        sha.write(name.as_bytes());
        sha.finalize(&mut digest);

        *self.addr_mut() = PreVector::from(&digest[..ADDR_INTERNAL_SIZE]);
        debug!(target: "netaddr", "SetInternal: derived INTERNAL address from name hash");
        true
    }
}
