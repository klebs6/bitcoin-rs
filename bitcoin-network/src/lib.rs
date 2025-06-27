// ---------------- [ File: bitcoin-network/src/lib.rs ]
#![feature(const_size_of_val)]

#[macro_use] mod imports; use imports::*;

//-------------------------------------------[.cpp/bitcoin/src/netaddress.h]
//-------------------------------------------[.cpp/bitcoin/src/netaddress.cpp]

x!{address_format}
x!{bip155}
x!{check_is_reachable}
x!{get_ext_network}
x!{get_group}
x!{get_in_addr}
x!{get_mapped_as}
x!{get_net_class}
x!{get_network}
x!{get_reachability_from}
x!{ipv4_to_string}
x!{ipv6_to_string}
x!{linked_ipv4}
x!{net_addr}
x!{netaddr_checks}
x!{netaddr_serde}
x!{netmask_bits}
x!{network_enum}
x!{onion_to_string}
x!{prefixes}
x!{sam3_1}
x!{set_i2p}
x!{set_internal}
x!{set_ip}
x!{set_legacy_ipv6}
x!{set_special}
x!{set_tor}
x!{sizes}
x!{to_string}
x!{torv3}
