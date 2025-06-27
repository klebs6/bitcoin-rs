// ---------------- [ File: bitcoin-portmap/src/natpmp_api.rs ]
crate::ix!();

/// The literal C‑function surface we want to preserve.
/// Production code uses the real implementation; tests can inject a mock.
#[cfg(feature = "natpmp")]
pub trait NatpmpApi {
    fn natpmp_init(&mut self) -> bool;
    fn natpmp_discover(&mut self, external_ipv4_addr: &mut Ipv4Addr) -> bool;
    fn natpmp_mapping(
        &mut self,
        external_ipv4_addr: Ipv4Addr,
        private_port: u16,
        external_ip_discovered: bool,
    ) -> bool;
    fn sendnewportmappingrequest(
        &mut self,
        protocol: Protocol,
        private_port: u16,
        public_port: u16,
        lifetime: u32,
    ) -> i32;
    fn closenatpmp(&mut self);
}
