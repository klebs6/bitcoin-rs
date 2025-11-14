// ---------------- [ File: bitcoin-network/src/set_i2p.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Parse an I2P address and set this object
      | to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, must be a valid C string,
      | for example ukeu3k5oycgaauneqgtnvselmt4yemvoilkln7jpvamvfx7dnkdq.b32.i2p.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsI2P()
      |
      */
    pub fn seti2p(&mut self, addr: &String) -> bool {
        trace!(target: "netaddr", address = addr, "SetI2P");
        // I2P addresses that we support consist of 52 base32 characters + ".b32.i2p".
        const B32_LEN: usize = 52;
        const SUFFIX: &str = ".b32.i2p";
        const SUFFIX_LEN: usize = 8;

        if addr.len() != B32_LEN + SUFFIX_LEN || addr[B32_LEN..].to_ascii_lowercase() != SUFFIX {
            debug!(target: "netaddr", "SetI2P: length/suffix check failed");
            return false;
        }

        let b32 = &addr[..B32_LEN];

        let mut invalid = false;
        let address_bytes: Vec<u8> =
            bitcoin_string::decode_base32_bytes_nopad_lower(b32, Some(&mut invalid as *mut bool));

        if invalid || address_bytes.len() != ADDR_I2P_SIZE {
            debug!(target: "netaddr", invalid, got = address_bytes.len(), "SetI2P: base32 decode failed or wrong length");
            return false;
        }

        *self.net_mut() = Network::NET_I2P;
        *self.addr_mut() = PreVector::from(address_bytes.as_slice());
        debug!(target: "netaddr", "SetI2P: parsed destination successfully");
        true
    }
}

#[cfg(test)]
mod i2p_parsing_spec {
    use super::*;

    fn make_i2p_address_bytes() -> [u8; ADDR_I2P_SIZE] {
        let mut bytes = [0u8; ADDR_I2P_SIZE];
        for (i, b) in bytes.iter_mut().enumerate() { *b = i as u8 ^ 0xA5; }
        bytes
    }

    #[traced_test]
    fn valid_i2p_address_parses() {
        let raw = make_i2p_address_bytes();
        let b32 = encode_base32_bytes(&raw, Some(false)); // no padding
        assert_eq!(b32.len(), 52, "52 base32 characters expected for 32 bytes");

        let s = format!("{b32}.b32.i2p");
        let mut a = NetAddr::default();
        assert!(a.seti2p(&s));

        assert!(a.isi2p());
        assert_eq!(a.addr().as_slice(), &raw);
    }

    #[traced_test]
    fn invalid_i2p_addresses_rejected() {
        // Wrong suffix
        let raw = make_i2p_address_bytes();
        let s1 = format!("{}.b33.i2p", encode_base32_bytes(&raw, Some(false)));
        let mut a1 = NetAddr::default();
        assert!(!a1.seti2p(&s1));

        // Wrong length (trim one char)
        let mut b32 = encode_base32_bytes(&raw, Some(false));
        b32.pop();
        let s2 = format!("{b32}.b32.i2p");
        let mut a2 = NetAddr::default();
        assert!(!a2.seti2p(&s2));

        // Invalid base32 characters
        let s3 = format!("{}{}.b32.i2p", "!" .repeat(52), "");
        let mut a3 = NetAddr::default();
        assert!(!a3.seti2p(&s3));
    }
}
