// ---------------- [ File: bitcoin-addrman/src/create.rs ]
crate::ix!();

impl AddrManInner {

    /**
      | Create a new entry and add it to the
      | internal data structures mapInfo, mapAddr
      | and vRandom.
      */
    pub fn create(&mut self, 
        addr:        &Address,
        addr_source: &NetAddr,
        pn_id:       Option<*mut i32>) -> *mut AddrInfo {

        let n_id: i32 = {
            let old = self.n_id_count;
            self.n_id_count += 1;
            old
        };

        if let Some(x) = self.map_info.get_mut(&n_id) {
            *x = AddrInfo::new((*addr).clone(),(*addr_source).clone());
        }

        if let Some(x) = self.map_addr.get_mut(&addr.service) {
            *x = n_id;
        }

        *(self.map_info[&n_id].n_random_pos.borrow_mut()) = self.random.borrow().len() as i32;

        self.random.borrow_mut().push(n_id);

        if let Some(ref item) = pn_id {
            unsafe {
                **item = n_id;
            }
        }

        self.map_info.get_mut(&n_id).unwrap()
    }
}
