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

        if addr.len() != B32_LEN + SUFFIX_LEN
            || addr[B32_LEN..].to_ascii_lowercase() != SUFFIX
        {
            debug!(target: "netaddr", "SetI2P: length/suffix check failed");
            return false;
        }

        // Remove the ".b32.i2p" suffix and pad to a multiple of 8 chars, so DecodeBase32()
        // can decode it.
        let mut b32_padded = String::with_capacity(B32_LEN + 4);
        b32_padded.push_str(&addr[..B32_LEN]);
        b32_padded.push_str("====");

        let address_bytes: Vec<u8> = match decode_base32(&b32_padded) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        if address_bytes.len() != ADDR_I2P_SIZE {
            return false;
        }

        *self.net_mut() = Network::NET_I2P;
        *self.addr_mut() = PreVector::from(address_bytes.as_slice());
        debug!(target: "netaddr", "SetI2P: parsed destination successfully");
        true
    }
}
