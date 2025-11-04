// ---------------- [ File: bitcoin-network/src/set_tor.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Parse a Tor address and set this object
      | to it.
      | 
      | -----------
      | @param[in] addr
      | 
      | Address to parse, must be a valid C string,
      | for example pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion.
      | 
      | -----------
      | @return
      | 
      | Whether the operation was successful.
      | @see CNetAddr::IsTor()
      |
      */
    pub fn set_tor(&mut self, addr: &String) -> bool {
        const SUFFIX: &str = ".onion";
        let suffix_len = SUFFIX.len();

        if addr.len() <= suffix_len || !addr.ends_with(SUFFIX) {
            debug!(target: "netaddr", "SetTor: suffix check failed");
            return false;
        }

        let b32 = &addr[..addr.len() - suffix_len];

        // Decode base32; reject invalid encodings.
        let input: Vec<u8> = match decode_base32(b32) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        if input.len() == TORV3_TOTAL_LEN {
            let pk_end = ADDR_TORV3_SIZE;
            let cks_end = pk_end + TORV3_CHECKSUM_LEN;
            let ver_end = cks_end + TORV3_VERSION.len();

            if ver_end != input.len() {
                return false;
            }

            let input_pubkey = &input[..pk_end];
            let input_checksum = &input[pk_end..cks_end];
            let input_version  = &input[cks_end..ver_end];

            if input_version != TORV3_VERSION {
                return false;
            }

            let mut calculated = [0u8; TORV3_CHECKSUM_LEN];
            torv3_checksum(input_pubkey, &mut calculated);

            if input_checksum != calculated {
                return false;
            }

            *self.net_mut() = Network::NET_ONION;
            *self.addr_mut() = PreVector::from(input_pubkey);
            debug!(target: "netaddr", "SetTor: parsed TORv3 address");
            return true;
        }

        false
    }
}
