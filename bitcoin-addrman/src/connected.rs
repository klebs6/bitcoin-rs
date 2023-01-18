crate::ix!();

impl AddrManInner {

    pub fn connected(&mut self, 
        addr:   &Service,
        n_time: i64)  {

        unsafe {

            let info: *mut AddrInfo = self.find(addr, None);

            //  if not found, bail out
            if info == std::ptr::null_mut() {
                return;
            }

            // update info
            let n_update_interval: i64 = 20 * 60;

            let address_ntime: i64 = (*info).address.n_time.try_into().unwrap();

            if n_time - address_ntime > n_update_interval {
                (*info).address.n_time = n_time.try_into().unwrap();
            }
        }
    }
}
