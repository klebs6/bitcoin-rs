// ---------------- [ File: bitcoin-addrman/src/delete.rs ]
crate::ix!();

impl AddrManInner {

    /**
      | Delete an entry. It must not be in tried,
      | and have refcount 0.
      |
      */
    pub fn delete(&mut self, n_id: i32)  {
        
        unsafe {

            assert!(self.map_info.contains_key(&n_id));

            let info: *mut AddrInfo 
            = self.map_info.get_mut(&n_id).unwrap() as *mut AddrInfo;

            assert!(!(*info).in_tried);
            assert!((*info).n_ref_count == 0);

            let n_random_pos: usize 
            = (*(*info).n_random_pos.borrow())
                .try_into()
                .unwrap();

            self.swap_random(
                n_random_pos, 
                self.random.borrow().len() - 1
            );

            self.random.borrow_mut().pop();

            self.map_addr.remove(&(*info).address.service);
            self.map_info.remove(&n_id);

            self.n_new -= 1;
        }
    }
}
