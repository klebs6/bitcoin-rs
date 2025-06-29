// ---------------- [ File: bitcoin-network/src/get_reachability_from.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Calculates a metric for how reachable
      | (*this) is from a given partner
      |
      */
    pub fn get_reachability_from(&self, paddr_partner: *const NetAddr) -> i32 {
        
        todo!();
        /*
            enum Reachability {
            REACH_UNREACHABLE,
            REACH_DEFAULT,
            REACH_TEREDO,
            REACH_IPV6_WEAK,
            REACH_IPV4,
            REACH_IPV6_STRONG,
            REACH_PRIVATE
        };

        if (!IsRoutable() || IsInternal())
            return REACH_UNREACHABLE;

        int ourNet = GetExtNetwork(this);
        int theirNet = GetExtNetwork(paddrPartner);
        bool fTunnel = IsRFC3964() || IsRFC6052() || IsRFC6145();

        switch(theirNet) {
        case NET_IPV4:
            switch(ourNet) {
            default:       return REACH_DEFAULT;
            case NET_IPV4: return REACH_IPV4;
            }
        case NET_IPV6:
            switch(ourNet) {
            default:         return REACH_DEFAULT;
            case NET_TEREDO: return REACH_TEREDO;
            case NET_IPV4:   return REACH_IPV4;
            case NET_IPV6:   return fTunnel ? REACH_IPV6_WEAK : REACH_IPV6_STRONG; // only prefer giving our IPv6 address if it's not tunnelled
            }
        case NET_ONION:
            switch(ourNet) {
            default:         return REACH_DEFAULT;
            case NET_IPV4:   return REACH_IPV4; // Tor users can connect to IPv4 as well
            case NET_ONION:    return REACH_PRIVATE;
            }
        case NET_I2P:
            switch (ourNet) {
            case NET_I2P: return REACH_PRIVATE;
            default: return REACH_DEFAULT;
            }
        case NET_TEREDO:
            switch(ourNet) {
            default:          return REACH_DEFAULT;
            case NET_TEREDO:  return REACH_TEREDO;
            case NET_IPV6:    return REACH_IPV6_WEAK;
            case NET_IPV4:    return REACH_IPV4;
            }
        case NET_UNKNOWN:
        case NET_UNROUTABLE:
        default:
            switch(ourNet) {
            default:          return REACH_DEFAULT;
            case NET_TEREDO:  return REACH_TEREDO;
            case NET_IPV6:    return REACH_IPV6_WEAK;
            case NET_IPV4:    return REACH_IPV4;
            case NET_ONION:     return REACH_PRIVATE; // either from Tor, or don't care about our address
            }
        }
        */
    }
}
