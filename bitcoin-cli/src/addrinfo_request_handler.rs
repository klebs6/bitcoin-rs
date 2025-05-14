// ---------------- [ File: bitcoin-cli/src/addrinfo_request_handler.rs ]
crate::ix!();

pub const ADDRINFO_REQUEST_HANDLER_NETWORKS: &[&'static str] = &[
    "ipv4", 
    "ipv6", 
    "onion", 
    "i2p"
];

impl AddrinfoRequestHandler {
    
    pub fn network_string_to_id(&self, str_: &str) -> i8 {
        
        for i in 0..ADDRINFO_REQUEST_HANDLER_NETWORKS.len() {
            if str_ == ADDRINFO_REQUEST_HANDLER_NETWORKS[i] {
                return i.try_into().unwrap();
            }
        }

        UNKNOWN_NETWORK
    }
}
