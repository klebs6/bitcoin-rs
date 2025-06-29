// ---------------- [ File: bitcoin-network/src/set_ip.rs ]
crate::ix!();

impl NetAddr {

    pub fn setip(&mut self, ip_in: &NetAddr)  {
        
        todo!();
        /*
            // Size check.
        switch (ipIn.m_net) {
        case NET_IPV4:
            assert(ipIn.m_addr.size() == ADDR_IPV4_SIZE);
            break;
        case NET_IPV6:
            assert(ipIn.m_addr.size() == ADDR_IPV6_SIZE);
            break;
        case NET_ONION:
            assert(ipIn.m_addr.size() == ADDR_TORV3_SIZE);
            break;
        case NET_I2P:
            assert(ipIn.m_addr.size() == ADDR_I2P_SIZE);
            break;
        case NET_CJDNS:
            assert(ipIn.m_addr.size() == ADDR_CJDNS_SIZE);
            break;
        case NET_INTERNAL:
            assert(ipIn.m_addr.size() == ADDR_INTERNAL_SIZE);
            break;
        case NET_UNROUTABLE:
        case NET_MAX:
            assert(false);
        } // no default case, so the compiler can warn about missing cases

        m_net = ipIn.m_net;
        m_addr = ipIn.m_addr;
        */
    }
}
