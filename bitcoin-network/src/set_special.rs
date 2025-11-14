// ---------------- [ File: bitcoin-network/src/set_special.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Parse a Tor or I2P address and set this
      | object to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, for example pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion
      | or ukeu3k5oycgaauneqgtnvselmt4yemvoilkln7jpvamvfx7dnkdq.b32.i2p.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsTor(), CNetAddr::IsI2P()
      |
      */
    pub fn set_special(&mut self, addr: &String) -> bool {
        trace!(target: "netaddr", address = addr, "SetSpecial");
        if !valid_as_cstring(addr) {
            debug!(target: "netaddr", "SetSpecial: not a valid C string");
            return false;
        }

        if self.set_tor(addr) {
            return true;
        }

        if self.seti2p(addr) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod special_address_parser_spec {
    use super::*;

    #[traced_test]
    fn set_special_accepts_tor_and_i2p() {
        // TOR
        let pk = [0x33u8; ADDR_TORV3_SIZE];
        let tor = onion_to_string(&pk);
        let mut a = NetAddr::default();
        assert!(a.set_special(&tor));
        assert!(a.is_tor());
        assert_eq!(a.addr().as_slice(), &pk);

        // I2P
        let raw = [0x44u8; ADDR_I2P_SIZE];
        let i2p = format!("{}.b32.i2p", encode_base32(&raw, Some(false)));
        let mut b = NetAddr::default();
        assert!(b.set_special(&i2p));
        assert!(b.isi2p());
        assert_eq!(b.addr().as_slice(), &raw);
    }

    #[traced_test]
    fn set_special_rejects_non_cstring() {
        // contains '\0' -> invalid C string
        let s = "abc\0def.onion".to_string();
        let mut a = NetAddr::default();
        assert!(!a.set_special(&s));
    }
}
