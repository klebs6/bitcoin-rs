// ---------------- [ File: bitcoin-banman/src/discourage.rs ]
crate::ix!();

impl BanMan {

    /**
      | Return whether net_addr is discouraged.
      |
      */
    pub fn is_discouraged(&self, net_addr: &NetAddr) -> bool {
        
        self.cs_banned.lock()
            .discouraged
            .contains_key(&net_addr.get_addr_bytes())
    }

    pub fn discourage(&mut self, net_addr: &NetAddr)  {
        
        self.cs_banned
            .get_mut()
            .discouraged
            .insert_key(&net_addr.get_addr_bytes());
    }
}
