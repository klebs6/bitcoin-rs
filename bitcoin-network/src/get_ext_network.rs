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
