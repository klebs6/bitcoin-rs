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
        
        todo!();
        /*
            static const char* suffix{".onion"};
        static constexpr size_t suffix_len{6};

        if (addr.size() <= suffix_len || addr.substr(addr.size() - suffix_len) != suffix) {
            return false;
        }

        bool invalid;
        const auto& input = DecodeBase32(addr.substr(0, addr.size() - suffix_len).c_str(), &invalid);

        if (invalid) {
            return false;
        }

        if (input.size() == torv3::TOTAL_LEN) {
            Span<const uint8_t> input_pubkey{input.data(), ADDR_TORV3_SIZE};
            Span<const uint8_t> input_checksum{input.data() + ADDR_TORV3_SIZE, torv3::CHECKSUM_LEN};
            Span<const uint8_t> input_version{input.data() + ADDR_TORV3_SIZE + torv3::CHECKSUM_LEN, sizeof(torv3::VERSION)};

            if (input_version != torv3::VERSION) {
                return false;
            }

            uint8_t calculated_checksum[torv3::CHECKSUM_LEN];
            torv3::Checksum(input_pubkey, calculated_checksum);

            if (input_checksum != calculated_checksum) {
                return false;
            }

            m_net = NET_ONION;
            m_addr.assign(input_pubkey.begin(), input_pubkey.end());
            return true;
        }

        return false;
        */
    }
}
