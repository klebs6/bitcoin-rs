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
        trace!(target: "netaddr", address = addr, "SetTor");
        const SUFFIX: &str = ".onion";
        let suffix_len = SUFFIX.len();

        if addr.len() <= suffix_len || !addr.ends_with(SUFFIX) {
            debug!(target: "netaddr", "SetTor: suffix check failed");
            return false;
        }

        let b32 = &addr[..addr.len() - suffix_len];

        // Decode base32 (lowercase, no '=' padding) through the shared
        // bitcoin-string helper.
        let mut invalid = false;
        let input: Vec<u8> =
            bitcoin_string::decode_base32_bytes_nopad_lower(b32, Some(&mut invalid as *mut bool));
        if invalid {
            debug!(target: "netaddr", "SetTor: invalid base32");
            return false;
        }

        if input.len() != TORV3_TOTAL_LEN {
            debug!(target: "netaddr", got = input.len(), want = TORV3_TOTAL_LEN, "SetTor: payload length mismatch");
            return false;
        }

        let pk_end = ADDR_TORV3_SIZE;
        let cks_end = pk_end + TORV3_CHECKSUM_LEN;
        let ver_end = cks_end + TORV3_VERSION.len();

        let input_pubkey = &input[..pk_end];
        let input_checksum = &input[pk_end..cks_end];
        let input_version  = &input[cks_end..ver_end];

        if input_version != TORV3_VERSION {
            debug!(target: "netaddr", got = ?input_version, "SetTor: wrong version byte");
            return false;
        }

        let mut calculated = [0u8; TORV3_CHECKSUM_LEN];
        torv3_checksum(input_pubkey, &mut calculated);

        if input_checksum != calculated {
            debug!(target: "netaddr", "SetTor: checksum mismatch");
            return false;
        }

        *self.net_mut() = Network::NET_ONION;
        *self.addr_mut() = PreVector::from(input_pubkey);
        debug!(target: "netaddr", "SetTor: parsed TORv3 address");
        true
    }
}

#[cfg(test)]
mod tor_parsing_spec {
    use super::*;

    #[traced_test]
    fn valid_torv3_address_parses() {
        let pubkey = [0x55u8; ADDR_TORV3_SIZE];
        let onion = onion_to_string(&pubkey);

        let mut a = NetAddr::default();
        assert!(a.set_tor(&onion));
        assert!(a.is_tor());
        assert_eq!(a.addr().as_slice(), &pubkey);
    }

    #[traced_test]
    fn invalid_tor_addresses_rejected() {
        let mut a = NetAddr::default();

        // Wrong suffix
        assert!(!a.set_tor(&"abcd.oni0n".to_string()));

        // Bad base32
        assert!(!a.set_tor(&format!("{}{}", "!".repeat(56), ".onion")));

        // Wrong version in payload
        // Build a valid payload then tweak version byte
        let pk = [0x66u8; ADDR_TORV3_SIZE];
        let mut checksum = [0u8; TORV3_CHECKSUM_LEN];
        torv3_checksum(&pk, &mut checksum);
        let mut payload = Vec::new();
        payload.extend_from_slice(&pk);
        payload.extend_from_slice(&checksum);
        payload.extend_from_slice(&[0x02]); // wrong version
        let b32 = encode_base32_bytes(&payload, Some(false));
        let bad = format!("{b32}.onion");
        assert!(!a.set_tor(&bad));
    }
}
