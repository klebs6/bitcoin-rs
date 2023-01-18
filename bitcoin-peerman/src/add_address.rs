crate::ix!();

/**
  | Interval between compact filter checkpoints.
  | See BIP 157.
  |
  */
pub const CFCHECKPT_INTERVAL: i32 = 1000;

impl Peer {

    pub fn add_address_known(
        &mut self,
        addr: &Address) {
        
        assert!(self.addr_known.is_some());

        self.addr_known.as_mut().unwrap().insert_key(&addr.get_key());
    }

    /**
      | Whether the peer supports the address.
      | For example, a peer that does not implement
      | BIP155 cannot receive Tor v3 addresses
      | because it requires
      | 
      | ADDRv2 (BIP155) encoding.
      |
      */
    pub fn is_addr_compatible(
        &self,
        addr: &Address) -> bool {

        self.wants_addrv2.load(atomic::Ordering::Relaxed) || addr.is_addr_v1compatible()
    }

    pub fn push_address(
        &self,
        addr:          &Address,
        insecure_rand: &mut FastRandomContext)  {

        //  Known checking here is only to save space
        //  from duplicates.
        //
        //  Before sending, we'll filter it again for
        //  known addresses that were added after
        //  addresses were pushed.
        assert!(self.addr_known.is_some());

        let mut guard = self.addrs_to_send.lock();

        if addr.is_valid() 
        && !self.addr_known.as_ref().unwrap().contains_key(&addr.get_key()) 
        && self.is_addr_compatible(addr) 
        {
            if guard.len() >= MAX_ADDR_TO_SEND {

                let len: u64 = guard.len().try_into().unwrap();

                let idx: usize = insecure_rand.randrange(len).try_into().unwrap();

                guard[idx] = addr.clone();

            } else {

                guard.push(addr.clone());
            }
        }
    }
}


