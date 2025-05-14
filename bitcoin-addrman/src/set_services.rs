// ---------------- [ File: bitcoin-addrman/src/set_services.rs ]
crate::ix!();

impl AddrManInner {

    pub fn set_services(&mut self, 
        addr:       &Service,
        n_services: ServiceFlags)  {
        
        unsafe {
            let info: *mut AddrInfo = self.find(addr, None);

            //  if not found, bail out
            if info == std::ptr::null_mut() {
                return;
            }

            //  update info
            (*info).address.n_services = n_services;
        }
    }
}
