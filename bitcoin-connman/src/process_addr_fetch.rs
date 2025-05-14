// ---------------- [ File: bitcoin-connman/src/process_addr_fetch.rs ]
crate::ix!();

impl Connman {

    pub fn process_addr_fetch(&self)  {
        
        let mut str_dest = String::default();

        {
            let mut guard = self.addr_fetches_mutex.get_mut();

            if guard.addr_fetches.is_empty() {
                return;
            }

            str_dest = guard.addr_fetches.front().as_ref().unwrap().to_string();

            guard.addr_fetches.pop_front();
        }

        let mut addr = Address::default();

        let mut grant: SemaphoreGrant = SemaphoreGrant::new(
            self.sem_outbound.clone(), 
            Some(true)
        );

        if (&grant).into() {

            self.open_network_connection(
                &addr, 
                false, 
                Some(&mut grant), 
                str_dest.as_ptr() as *const u8, 
                ConnectionType::ADDR_FETCH
            );
        }
    }
}
