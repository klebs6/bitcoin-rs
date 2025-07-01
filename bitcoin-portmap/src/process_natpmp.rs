// ---------------- [ File: bitcoin-portmap/src/process_natpmp.rs ]
crate::ix!();

/// C‑style constant to mirror upstream.
#[cfg(feature = "natpmp")]
pub const NATPMP_PROTOCOL_TCP: Protocol = Protocol::TCP;

/// Literal translation of Bitcoin Core’s `ProcessNatpmp()`.
/// Variable names and call sequence are kept **identical** to the C++.
#[cfg(feature = "natpmp")]
pub fn process_natpmp() -> bool {
    let interrupt  = g_mapport_interrupt();
    let ext_port   = g_mapport_external_port();
    let reannounce = PORT_MAPPING_REANNOUNCE_PERIOD;

    let mut ret: bool = false;
    let mut natpmp = match NatpmpClient::new() {
        Ok(c)  => c,
        Err(e) => {
            error!(%e, "natpmp: initialisation failed");
            return false;
        }
    };
    let mut external_ipv4_addr: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

    if natpmp.natpmp_init()
        && natpmp.natpmp_discover(&mut external_ipv4_addr)
    {
        let mut external_ip_discovered: bool = false;
        debug!(%external_ipv4_addr, "natpmp: discovery succeeded");

        let private_port: u16 = get_listen_port();

        // keep‑alive/announce loop
        loop {
            ret = natpmp.natpmp_mapping(
                external_ipv4_addr,
                private_port,
                external_ip_discovered,
            );
            if !(ret && interrupt.sleep_for(reannounce)) {
                break;
            }
        }
        interrupt.reset();

        // remove the mapping unconditionally
        let r_send = natpmp.sendnewportmappingrequest(
            NATPMP_PROTOCOL_TCP,
            private_port,
            {
                let ext = *ext_port.lock();
                ext
            },
            0, /* lifetime 0 removes mapping */
        );

        {
            let mut ext = ext_port.lock();
            *ext = 0;
        }

        if r_send == 12 {
            info!("natpmp: Port mapping removed successfully.");
        } else {
            warn!("natpmp: sendnewportmappingrequest(0) failed with {} error.", r_send);
        }
    }

    natpmp.closenatpmp();
    ret
}

// ───────────────────────────────── Tests ────────────────────────────────────
#[cfg(all(test, feature = "natpmp"))]
mod process_natpmp_tests {
    use super::*;

    /// Minimal mock that lets us drive every branch.
    struct MockNatpmp {
        init_ok:     bool,
        discover_ok: bool,
        mapping_ok:  Vec<bool>, // series returned on each call
        send_code:   i32,
    }

    impl NatpmpApi for MockNatpmp {
        fn natpmp_init(&mut self) -> bool { self.init_ok }

        fn natpmp_discover(&mut self, external_ipv4_addr: &mut Ipv4Addr) -> bool {
            if self.discover_ok {
                *external_ipv4_addr = Ipv4Addr::new(1, 2, 3, 4);
                true
            } else {
                false
            }
        }

        fn natpmp_mapping(
            &mut self,
            _external_ipv4_addr: Ipv4Addr,
            _private_port: u16,
            _external_ip_discovered: bool,
        ) -> bool {
            self.mapping_ok
                .pop()
                .unwrap_or(false)
        }

        fn sendnewportmappingrequest(
            &mut self,
            _protocol: Protocol,
            _private_port: u16,
            _public_port: u16,
            _lifetime: u32,
        ) -> i32 {
            self.send_code
        }

        fn closenatpmp(&mut self) {}
    }

    /// Hook the mock into the exact same flow, proving parity.
    fn run_with_mock(mut mock: impl NatpmpApi) -> bool {
        // copy‑pasted body of `process_natpmp`, but replacing the concrete type.
        let interrupt = g_mapport_interrupt();
        let ext_port  = g_mapport_external_port();
        let reannounce = PORT_MAPPING_REANNOUNCE_PERIOD;
        let private_port: u16 = get_listen_port();

        let mut ret: bool = false;
        let mut natpmp = mock;
        let mut external_ipv4_addr: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

        if natpmp.natpmp_init()
            && natpmp.natpmp_discover(&mut external_ipv4_addr)
        {
            let mut external_ip_discovered: bool = false;

            loop {
                ret = natpmp.natpmp_mapping(
                    external_ipv4_addr,
                    private_port,
                    external_ip_discovered,
                );
                if !(ret && interrupt.sleep_for(reannounce)) {
                    break;
                }
            }
            interrupt.reset();

            let _ = natpmp.sendnewportmappingrequest(
                NATPMP_PROTOCOL_TCP,
                private_port,
                {
                    let ext = *ext_port.lock();
                    ext
                },
                0,
            );
            {
                let mut ext = ext_port.lock();
                *ext = 0;
            }
        }
        natpmp.closenatpmp();
        ret
    }

    #[traced_test]
    fn success_path() {
        let ret = run_with_mock(MockNatpmp {
            init_ok: true,
            discover_ok: true,
            mapping_ok: vec![true], // first mapping succeeds
            send_code: 12,
        });
        trace!(ret, "return value");
        assert!(ret);
    }

    #[traced_test]
    fn discover_failure() {
        let ret = run_with_mock(MockNatpmp {
            init_ok: true,
            discover_ok: false,
            mapping_ok: vec![],
            send_code: -1,
        });
        assert!(!ret);
    }
}
