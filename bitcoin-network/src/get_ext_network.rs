// ---------------- [ File: bitcoin-network/src/get_ext_network.rs ]
crate::ix!();

/**
  | private extensions to enum Network,
  | only returned by GetExtNetwork, and
  | only used in GetReachabilityFrom
  |
  */
pub const NET_UNKNOWN: i32 = Network::NET_MAX as i32 + 0;
pub const NET_TEREDO:  i32 = Network::NET_MAX as i32 + 1;

pub fn get_ext_network(maybe_addr: Option<&NetAddr>) -> i32 {

    if maybe_addr.is_none() {
        return NET_UNKNOWN;
    }

    let addr = maybe_addr.unwrap();

    if addr.isrfc4380() {
        return NET_TEREDO;
    }

    addr.get_network() as i32
}

#[cfg(test)]
mod ext_network_classification_spec {
    use super::*;

    fn make_ipv6_prefix(prefix: [u8; 4]) -> NetAddr {
        let mut bytes = [0u8; ADDR_IPV6_SIZE];
        bytes[..4].copy_from_slice(&prefix);
        NetAddrBuilder::default()
            .addr(PreVector::from(bytes.as_slice()))
            .net(Network::NET_IPV6)
            .scope_id(0u32)
            .build()
            .unwrap()
    }

    #[traced_test]
    fn none_argument_maps_to_unknown() {
        assert_eq!(get_ext_network(None), NET_UNKNOWN);
    }

    #[traced_test]
    fn teredo_detection_is_correct() {
        // RFC 4380 Teredo: 2001::/32
        let v6 = make_ipv6_prefix([0x20, 0x01, 0x00, 0x00]);
        assert_eq!(get_ext_network(Some(&v6)), NET_TEREDO);
    }

    #[traced_test]
    fn passthrough_known_networks() {
        // IPv4
        let v4 = NetAddrBuilder::default()
            .addr(PreVector::from(&[8,8,8,8][..]))
            .net(Network::NET_IPV4)
            .scope_id(0u32)
            .build()
            .unwrap();
        assert_eq!(get_ext_network(Some(&v4)), Network::NET_IPV4 as i32);

        // Plain IPv6 (not Teredo)
        let v6 = make_ipv6_prefix([0x20, 0x01, 0x04, 0x70]); // HE.NET prefix but still NET_IPV6
        assert_eq!(get_ext_network(Some(&v6)), Network::NET_IPV6 as i32);
    }
}
