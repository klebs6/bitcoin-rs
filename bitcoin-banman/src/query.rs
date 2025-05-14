// ---------------- [ File: bitcoin-banman/src/query.rs ]
crate::ix!();

impl BanMan {
    
    /**
      | Return whether net_addr is banned
      |
      */
    pub fn is_netaddr_banned(&self, net_addr: &NetAddr) -> bool {
        
        let current_time = OffsetDateTime::now_utc();

        let inner = self.cs_banned.lock();

        for it in inner.banned.iter() {

            let sub_net:     &SubNet = it.0;
            let ban_entry: &BanEntry = it.1;

            if current_time < ban_entry.n_ban_until && sub_net.match_(net_addr) {
                return true;
            }
        }

        false
    }
    
    /**
      | Return whether sub_net is exactly banned
      |
      */
    pub fn is_subnet_banned(&self, sub_net: &SubNet) -> bool {
        
        let current_time = OffsetDateTime::now_utc();

        let inner = self.cs_banned.lock();

        if let Some(ban_entry) = inner.banned.get(sub_net) {

            if current_time < ban_entry.n_ban_until {
                return true;
            }
        }

        false
    }
}
