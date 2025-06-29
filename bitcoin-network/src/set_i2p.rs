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
        
        todo!();
        /*
            // I2P addresses that we support consist of 52 base32 characters + ".b32.i2p".
        static constexpr size_t b32_len{52};
        static const char* suffix{".b32.i2p"};
        static constexpr size_t suffix_len{8};

        if (addr.size() != b32_len + suffix_len || ToLower(addr.substr(b32_len)) != suffix) {
            return false;
        }

        // Remove the ".b32.i2p" suffix and pad to a multiple of 8 chars, so DecodeBase32()
        // can decode it.
        const std::string b32_padded = addr.substr(0, b32_len) + "====";

        bool invalid;
        const auto& address_bytes = DecodeBase32(b32_padded.c_str(), &invalid);

        if (invalid || address_bytes.size() != ADDR_I2P_SIZE) {
            return false;
        }

        m_net = NET_I2P;
        m_addr.assign(address_bytes.begin(), address_bytes.end());

        return true;
        */
    }
}
