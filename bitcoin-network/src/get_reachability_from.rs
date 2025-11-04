// ---------------- [ File: bitcoin-network/src/get_reachability_from.rs ]
crate::ix!();

impl NetAddr {

    /**
      | Calculates a metric for how reachable
      | (*this) is from a given partner
      |
      */
    pub fn get_reachability_from(&self, paddr_partner: *const NetAddr) -> i32 {

        trace!(
            target: "netaddr", 
            ours_net = ?self.get_net_class(), 
            "Computing reachability score from peer"
        );

        // Reachability categories (kept as integers to mirror the original control flow)
        const REACH_UNREACHABLE: i32 = 0;
        const REACH_DEFAULT:     i32 = 1;
        const REACH_TEREDO:      i32 = 2;
        const REACH_IPV6_WEAK:   i32 = 3;
        const REACH_IPV4:        i32 = 4;
        const REACH_IPV6_STRONG: i32 = 5;
        const REACH_PRIVATE:     i32 = 6;

        if !self.is_routable() || self.is_internal() {
            debug!(target: "netaddr", "Our address is not routable or is internal → unreachable");
            return REACH_UNREACHABLE;
        }

        let our_net  = get_ext_network(Some(self));

        let their_net = unsafe {
            if paddr_partner.is_null() {
                get_ext_network(None)
            } else {
                get_ext_network(Some(&*paddr_partner))
            }
        };

        let f_tunnel = self.isrfc3964() || self.isrfc6052() || self.isrfc6145();

        debug!(
            target: "netaddr", 
            our_net, 
            their_net, 
            f_tunnel, 
            "Extended networks and tunnel status"
        );

        match their_net {
            x if x == Network::NET_IPV4 as i32 => {
                match our_net {
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    _ => REACH_DEFAULT,
                }
            }
            x if x == Network::NET_IPV6 as i32 => {
                match our_net {
                    y if y == NET_TEREDO => REACH_TEREDO,
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    y if y == Network::NET_IPV6 as i32 => {
                        if f_tunnel { REACH_IPV6_WEAK } else { REACH_IPV6_STRONG }
                    }
                    _ => REACH_DEFAULT,
                }
            }
            x if x == Network::NET_ONION as i32 => {
                match our_net {
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4, // Tor users can connect to IPv4 as well
                    y if y == Network::NET_ONION as i32 => REACH_PRIVATE,
                    _ => REACH_DEFAULT,
                }
            }
            x if x == Network::NET_I2P as i32 => {
                match our_net {
                    y if y == Network::NET_I2P as i32 => REACH_PRIVATE,
                    _ => REACH_DEFAULT,
                }
            }
            x if x == NET_TEREDO => {
                match our_net {
                    y if y == NET_TEREDO => REACH_TEREDO,
                    y if y == Network::NET_IPV6 as i32 => REACH_IPV6_WEAK,
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    _ => REACH_DEFAULT,
                }
            }
            _ => {
                // NET_UNKNOWN or NET_UNROUTABLE or any other
                match our_net {
                    y if y == NET_TEREDO => REACH_TEREDO,
                    y if y == Network::NET_IPV6 as i32 => REACH_IPV6_WEAK,
                    y if y == Network::NET_IPV4 as i32 => REACH_IPV4,
                    y if y == Network::NET_ONION as i32 => REACH_PRIVATE, // either from Tor, or don't care about our address
                    _ => REACH_DEFAULT,
                }
            }
        }
    }
}
