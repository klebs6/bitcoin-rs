crate::ix!();

impl AddrManInner {

    /**
      | Find an entry.
      |
      */
    pub fn find(&mut self, 
        addr:  &Service,
        mut pn_id: Option<*mut i32>) -> *mut AddrInfo {

        if let Some(it) = self.map_addr.get_mut(addr) {

            if let Some(ref mut pn_id) = pn_id {
                *pn_id = it;
            }

            if let Some(it2) = self.map_info.get_mut(it) {
                return &mut *it2;
            }
        }

        null_mut()
    }
}
