// ---------------- [ File: bitcoin-addrman/src/attempt.rs ]
crate::ix!();

impl AddrManInner {

    pub fn attempt(&mut self, 
        addr:          &Service,
        count_failure: bool,
        n_time:        i64)  {
        
        unsafe {

            let info: *mut AddrInfo = self.find(addr, None);

            // if not found, bail out
            if info == std::ptr::null_mut() {
                return;
            }

            //  update info
            (*info).n_last_try = n_time;

            if count_failure && (*info).n_last_count_attempt < self.n_last_good {
                (*info).n_last_count_attempt = n_time;
                (*info).n_attempts += 1;
            }
        }
    }
}
