// ---------------- [ File: bitcoin-connman/src/options.rs ]
crate::ix!();

pub struct ConnmanOptions
{
    pub n_local_services:         ServiceFlags, 
    pub n_max_connections:        i32, 
    pub max_outbound_full_relay:  i32,
    pub max_outbound_block_relay: i32, 
    pub n_max_addnode:            i32, 
    pub n_max_feeler:             i32, 
    pub ui_interface:             Option<ClientUIInterface>,
    pub msgproc:                  Option<Box<dyn NetEventsInterface>>,
    pub banman:                   Option<BanMan>,
    pub n_send_buffer_max_size:   u32, 
    pub n_receive_flood_size:     u32, 
    pub n_max_outbound_limit:     u64, 
    pub peer_connect_timeout:     Duration, 
    pub seed_nodes:               Vec<String>,
    pub whitelisted_range:        Option<Vec<NetWhitelistPermissions>>,
    pub white_binds:              Vec<NetWhitebindPermissions>,
    pub binds:                    Vec<Service>,
    pub onion_binds:              Vec<Service>,

    /**
      | True if the user did not specify -bind=
      | or -whitebind= and thus we should bind
      | on `0.0.0.0` (IPv4) and `::` (IPv6).
      |
      */
    pub bind_on_any:              bool,

    pub use_addrman_outgoing:     bool, 
    pub specified_outgoing:       Vec<String>,
    pub added_nodes:              Vec<String>,
    pub i2p_accept_incoming:      bool,
}

impl Default for ConnmanOptions {

    fn default() -> Self {
        Self {
            n_local_services:         ServiceFlags::NODE_NONE,
            n_max_connections:        0,
            max_outbound_full_relay:  0,
            max_outbound_block_relay: 0,
            n_max_addnode:            0,
            n_max_feeler:             0,
            ui_interface:             None,
            msgproc:                  None,
            banman:                   None,
            n_send_buffer_max_size:   0,
            n_receive_flood_size:     0,
            n_max_outbound_limit:     0,
            peer_connect_timeout:     DEFAULT_PEER_CONNECT_TIMEOUT,
            seed_nodes:               vec![],
            whitelisted_range:        None,
            white_binds:              vec![],
            binds:                    vec![],
            onion_binds:              vec![],
            bind_on_any:              false,
            use_addrman_outgoing:     true,
            specified_outgoing:       vec![],
            added_nodes:              vec![],
            i2p_accept_incoming:      false,
        }
    }
}
