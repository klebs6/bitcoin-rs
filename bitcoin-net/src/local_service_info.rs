crate::ix!();

pub const LOCAL_NONE:   usize = 0; // unknown
pub const LOCAL_IF:     usize = 1; // address a local interface listens on
pub const LOCAL_BIND:   usize = 2; // address explicit bound to
pub const LOCAL_MAPPED: usize = 3; // address reported by UPnP or NAT-PMP
pub const LOCAL_MANUAL: usize = 4; // address explicitly specified (-externalip = )
pub const LOCAL_MAX:    usize = 5;

/**
  | Subversion as sent to the P2P network
  | in `version` messages
  |
  */
lazy_static!{
    /*
    extern std::string strSubVersion;
    */
}

pub struct LocalServiceInfo {
    n_score: i32,
    n_port:  u16,
}

lazy_static!{
    /*
    extern RecursiveMutex cs_mapLocalHost;
    extern std::map<CNetAddr, LocalServiceInfo> mapLocalHost GUARDED_BY(cs_mapLocalHost);

    extern const std::string NET_MESSAGE_COMMAND_OTHER;
    */
}

lazy_static!{
    /*
    extern bool fDiscover;
    extern bool fListen;
    */
}
