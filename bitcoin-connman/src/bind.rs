// ---------------- [ File: bitcoin-connman/src/bind.rs ]
crate::ix!();

pub const IN6ADDR_ANY_INIT: In6Addr = In6Addr {
    s6_addr: [0; 16],
};

impl Connman {

    pub fn bind(&self, 
        addr:        &Service,
        flags:       u32,
        permissions: NetPermissionFlags) -> bool {

        if (flags & BindFlags::BF_EXPLICIT.bits()) == 0 && !addr.base.is_reachable() {
            return false;
        }

        let mut str_error = BilingualStr::default();

        if !self.bind_listen_port(addr,&mut str_error,permissions) {

            if (flags & BindFlags::BF_REPORT_ERROR.bits()) != 0 
            && self.client_interface.is_some() {

                self.client_interface
                    .get_mut()
                    .thread_safe_message_box(
                        &str_error, 
                        &"".to_string(), 
                        MessageBoxFlags::MSG_ERROR.bits()
                    );
            }

            return false;
        }

        if addr.base.is_routable() 
        && discover() 
        && (flags & BindFlags::BF_DONT_ADVERTISE.bits()) == 0 
        && !NetPermissions::has_flag(&permissions,NetPermissionFlags::NoBan) 
        {
            add_local(addr, Some(i32::try_from(LOCAL_BIND).unwrap()));
        }

        true
    }
    
    pub fn init_binds(&self, options: &ConnmanOptions) -> bool {

        let mut bound: bool = false;

        for addr_bind in options.binds.iter() {
            bound |= self.bind(
                addr_bind,
                (BindFlags::BF_EXPLICIT | BindFlags::BF_REPORT_ERROR).bits(),
                NetPermissionFlags::None
            );
        }

        for addr_bind in options.white_binds.iter() {
            bound |= self.bind(
                &addr_bind.service,
                (BindFlags::BF_EXPLICIT | BindFlags::BF_REPORT_ERROR).bits(),
                addr_bind.base.flags
            );
        }

        for addr_bind in options.onion_binds.iter() {
            bound |= self.bind(
                addr_bind, 
                (BindFlags::BF_EXPLICIT | BindFlags::BF_DONT_ADVERTISE).bits(),
                NetPermissionFlags::None
            );
        }

        if options.bind_on_any {

            let mut inaddr_any: InAddr = unsafe { std::mem::zeroed() };

            inaddr_any.s_addr = u32::from_be_bytes(u32::to_be_bytes(libc::INADDR_ANY));

            let inaddr6_any: In6Addr = IN6ADDR_ANY_INIT;

            bound |= self.bind(
                &Service::new_from_ip6(&inaddr6_any,get_listen_port()),
                BindFlags::BF_NONE.bits(),
                NetPermissionFlags::None
            );

            bound |= self.bind(
                &Service::new_from_ip4(&inaddr_any,get_listen_port()),
                match !bound {
                    true   => BindFlags::BF_REPORT_ERROR,
                    false  => BindFlags::BF_NONE
                }.bits(),
                NetPermissionFlags::None
            );
        }

        bound
    }
}
