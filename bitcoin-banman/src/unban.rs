crate::ix!();

impl BanMan {

    pub fn unban_netaddr(&mut self, net_addr: &NetAddr) -> bool {

        let sub_net: SubNet = SubNet::from(net_addr);

        self.unban_subnet(&sub_net)
    }
    
    pub fn unban_subnet(&mut self, sub_net: &SubNet) -> bool {
        
        {
            let inner = self.cs_banned.get_mut();

            if inner.banned.remove(sub_net).is_none() {
                return false;
            }

            inner.is_dirty = true;
        }

        if self.client_interface.is_some() {
            self.client_interface.get_mut().banned_list_changed();
        }

        // store banlist to disk immediately
        self.dump_banlist();

        true
    }
}
