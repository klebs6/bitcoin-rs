// ---------------- [ File: bitcoin-network/src/ipv6_to_string.rs ]
crate::ix!();

/// Return an IPv6 address text representation with zero compression as described in RFC 5952 ("A Recommendation for IPv6 Address Text Representation").
#[inline]
pub fn ipv6_to_string(a: &[u8], scope_id: u32) -> String {
    debug!(target: "netaddr", bytes=?a, scope_id, "Formatting IPv6 address");
    assert!(
        a.len() == ADDR_IPV6_SIZE,
        "ipv6_to_string expects 16‑byte slice"
    );

    let addr = std::net::Ipv6Addr::from(<[u8; 16]>::try_from(a).unwrap());
    let mut s = addr.to_string(); // Rust's implementation follows RFC 5952

    if scope_id != 0 {
        s.push('%');
        s.push_str(&scope_id.to_string());
    }
    s
}

#[cfg(test)]
mod ipv6_fmt_tests {
    use super::*;

    #[traced_test]
    fn loopback_formatting() {
        let ip = [0u8; 15].into_iter().chain(std::iter::once(1)).collect::<Vec<_>>();
        assert_eq!(ipv6_to_string(&ip, 0), "::1");
    }

    #[traced_test]
    fn scoped_link_local() {
        // FE80::1234 with scope id 4 → "fe80::1234%4"
        let mut ip = [0u8; 16];
        ip[0] = 0xFE;
        ip[1] = 0x80;
        ip[14] = 0x12;
        ip[15] = 0x34;
        assert_eq!(ipv6_to_string(&ip, 4), "fe80::1234%4");
    }
}
