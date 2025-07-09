// ---------------- [ File: bitcoin-network/src/ipv4_to_string.rs ]
crate::ix!();

/// Render an IPv4 address (`a[0]…a[3]`) as `d.d.d.d`.
#[inline]
pub fn ipv4_to_string(a: &[u8]) -> String {
    debug!(target: "netaddr", octets=?a, "Formatting IPv4 address");
    assert!(a.len() == ADDR_IPV4_SIZE, "ipv4_to_string expects 4 bytes");
    format!("{}.{}.{}.{}", a[0], a[1], a[2], a[3])
}

#[cfg(test)]
mod ipv4_fmt_tests {
    use super::*;

    #[traced_test]
    fn basic_formatting() {
        let ip = [192u8, 168, 0, 1];
        assert_eq!(ipv4_to_string(&ip), "192.168.0.1");
    }
}
