// ---------------- [ File: bitcoin-portmap/src/natpmp_client.rs ]
crate::ix!();

/// Real wrapper around the `natpmp` crate,
/// exposing the exact same function names as the C API.
#[cfg(feature = "natpmp")]
pub struct NatpmpClient {
    inner: Natpmp,
}

#[cfg(feature = "natpmp")]
impl NatpmpClient {
    pub fn new() -> Result<Self, String> {
        Natpmp::new()
            .map(|inner| Self { inner })
            .map_err(|e| e.to_string())
    }
}

#[cfg(feature = "natpmp")]
impl NatpmpApi for NatpmpClient {
    fn natpmp_init(&mut self) -> bool {
        true // already initialised in `new`
    }

    fn natpmp_discover(&mut self, external_ipv4_addr: &mut Ipv4Addr) -> bool {
        // `natpmp 0.5` returns the IP directly.
        match self.inner.send_public_address_request() {
            Ok(addr) => {
                *external_ipv4_addr = addr;
                true
            }
            Err(e) => {
                warn!(?e, "natpmp: discovery failed");
                false
            }
        }
    }

    fn natpmp_mapping(
        &mut self,
        _external_ipv4_addr: Ipv4Addr,
        private_port: u16,
        _external_ip_discovered: bool,
    ) -> bool {
        self.inner
            .send_port_mapping_request(Protocol::TCP, private_port, private_port, 3_600)
            .is_ok()
    }

    fn sendnewportmappingrequest(
        &mut self,
        protocol: Protocol,
        private_port: u16,
        public_port: u16,
        lifetime: u32,
    ) -> i32 {
        match self
            .inner
            .send_port_mapping_request(protocol, private_port, public_port, lifetime)
        {
            Ok(_) => 12, // keep the C constant for “OK”
            Err(e) => {
                warn!(?e, "natpmp: sendnewportmappingrequest failed");
                -1
            }
        }
    }

    fn closenatpmp(&mut self) {
        // nothing to do; drop handles implicitly
    }
}
