crate::ix!();

impl NetAddr {

    pub fn to_stringip(&self) -> String {
        
        todo!();
        /*
            switch (m_net) {
        case NET_IPV4:
            return IPv4ToString(m_addr);
        case NET_IPV6:
            return IPv6ToString(m_addr, m_scope_id);
        case NET_ONION:
            return OnionToString(m_addr);
        case NET_I2P:
            return EncodeBase32(m_addr, false /* don't pad with = */) + ".b32.i2p";
        case NET_CJDNS:
            return IPv6ToString(m_addr, 0);
        case NET_INTERNAL:
            return EncodeBase32(m_addr) + ".internal";
        case NET_UNROUTABLE: // m_net is never and should not be set to NET_UNROUTABLE
        case NET_MAX:        // m_net is never and should not be set to NET_MAX
            assert(false);
        } // no default case, so the compiler can warn about missing cases

        assert(false);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return ToStringIP();
        */
    }
}
