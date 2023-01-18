crate::ix!();

/**
  | Default for -whitelistrelay.
  |
  */
pub const DEFAULT_WHITELISTRELAY: bool = true;

/**
  | Default for -whitelistforcerelay.
  |
  */
pub const DEFAULT_WHITELISTFORCERELAY: bool = false;

/**
  | Time after which to disconnect, after
  | waiting for a ping response (or inactivity).
  |
  */
pub const TIMEOUT_INTERVAL: Duration = Duration::seconds(20 * 60);

/**
  | Run the feeler connection loop once
  | every 2 minutes. *
  |
  */
pub const FEELER_INTERVAL: Duration = Duration::minutes(2);

/**
  | Run the extra block-relay-only connection
  | loop once every 5 minutes. *
  |
  */
pub const EXTRA_BLOCK_RELAY_ONLY_PEER_INTERVAL: Duration = Duration::minutes(5);

/**
  | Maximum length of incoming protocol
  | messages (no message over 4 MB is currently
  | acceptable).
  |
  */
pub const MAX_PROTOCOL_MESSAGE_LENGTH: u32 = 4 * 1000 * 1000;

/**
  | Maximum length of the user agent string
  | in `version` message
  |
  */
pub const MAX_SUBVERSION_LENGTH: usize = 256;

/**
  | Maximum number of automatic outgoing
  | nodes over which we'll relay everything
  | (blocks, tx, addrs, etc)
  |
  */
pub const MAX_OUTBOUND_FULL_RELAY_CONNECTIONS: usize = 8;

/**
  | Maximum number of addnode outgoing
  | nodes
  |
  */
pub const MAX_ADDNODE_CONNECTIONS: usize = 8;

/**
  | Maximum number of block-relay-only
  | outgoing connections
  |
  */
pub const MAX_BLOCK_RELAY_ONLY_CONNECTIONS: usize = 2;

/**
  | Maximum number of feeler connections
  |
  */
pub const MAX_FEELER_CONNECTIONS: usize = 1;

/**
  | -listen default
  |
  */
pub const DEFAULT_LISTEN: bool = true;

/**
  | The maximum number of peer connections
  | to maintain.
  |
  */
pub const DEFAULT_MAX_PEER_CONNECTIONS: usize = 125;

/**
  | The default for -maxuploadtarget.
  | 0 = Unlimited
  |
  */
pub const DEFAULT_MAX_UPLOAD_TARGET: u64 = 0;

/**
  | Default for blocks only
  |
  */
pub const DEFAULT_BLOCKSONLY: bool = false;

/**
  | -peertimeout default
  |
  */
pub const DEFAULT_PEER_CONNECT_TIMEOUT: Duration = Duration::seconds(60);

/**
  | Number of file descriptors required
  | for message capture *
  |
  */
pub const NUM_FDS_MESSAGE_CAPTURE: i32 = 1;

pub const DEFAULT_FORCEDNSSEED:     bool = false;
pub const DEFAULT_DNSSEED:          bool = true;
pub const DEFAULT_FIXEDSEEDS:       bool = true;
pub const DEFAULT_MAXRECEIVEBUFFER: usize = 5 * 1000;
pub const DEFAULT_MAXSENDBUFFER:    usize = 1 * 1000;



//-------------------------------------------[.cpp/bitcoin/src/net.cpp]

/**
  | Maximum number of block-relay-only
  | anchor connections
  |
  */
pub const MAX_BLOCK_RELAY_ONLY_ANCHORS: usize = 2;

const_assert!{
    MAX_BLOCK_RELAY_ONLY_ANCHORS <= MAX_BLOCK_RELAY_ONLY_CONNECTIONS as usize
} //"MAX_BLOCK_RELAY_ONLY_ANCHORS must not exceed MAX_BLOCK_RELAY_ONLY_CONNECTIONS."

/**
  | Anchor IP address database file name
  |
  */
pub const ANCHORS_DATABASE_FILENAME: &'static str = "anchors.dat";

/**
  | How often to dump addresses to peers.dat
  |
  */
pub const DUMP_PEERS_INTERVAL: Duration = Duration::minutes(15);

/**
  | Number of DNS seeds to query when the
  | number of connections is low.
  |
  */
pub const DNSSEEDS_TO_QUERY_AT_ONCE: i32 = 3;

/**
  | How long to delay before querying DNS
  | seeds
  | 
  | If we have more than THRESHOLD entries
  | in addrman, then it's likely that we
  | got those addresses from having previously
  | connected to the P2P network, and that
  | we'll be able to successfully reconnect
  | to the P2P network via contacting one
  | of them. So if that's the case, spend
  | a little longer trying to connect to
  | known peers before querying the
  | 
  | DNS seeds.
  |
  */
pub const DNSSEEDS_DELAY_FEW_PEERS:      Duration = Duration::seconds(11);
pub const DNSSEEDS_DELAY_MANY_PEERS:     Duration = Duration::minutes(5);

pub const DNSSEEDS_DELAY_PEER_THRESHOLD: i32 = 1000; // "many" vs "few" peers

/**
  | The default timeframe for -maxuploadtarget.
  | 1 day.
  |
  */
pub const MAX_UPLOAD_TIMEFRAME: Duration = Duration::seconds(60 * 60 * 24);

/**
  | We add a random period time (0 to 1 seconds)
  | to feeler connections to prevent synchronization.
  |
  */
pub const FEELER_SLEEP_WINDOW: usize = 1;

/**
  | Used to pass flags to the Bind() function
  |
  */
bitflags!{
    pub struct BindFlags: u32 {
        const BF_NONE         = 0;
        const BF_EXPLICIT     = 1 << 0;
        const BF_REPORT_ERROR = 1 << 1;

        /**
         | Do not call AddLocal() for our special
         | addresses, e.g., for incoming Tor connections,
         | to prevent gossiping them over the network.
         |
         */
        const BF_DONT_ADVERTISE = 1 << 2;
    }
}

/**
  | The set of sockets cannot be modified while
  | waiting
  |
  | The sleep time needs to be small to avoid new
  | sockets stalling
  */
pub const SELECT_TIMEOUT_MILLISECONDS: i64 = 50;

pub const NET_MESSAGE_COMMAND_OTHER: &'static str = "*other*";

pub const RANDOMIZER_ID_NETGROUP:       u64 = 0x6c0edd8036ef4036; // SHA256("netgroup")[0:8]
pub const RANDOMIZER_ID_LOCALHOSTNONCE: u64 = 0xd93e69e2bbfa5735; // SHA256("localhostnonce")[0:8]
pub const RANDOMIZER_ID_ADDRCACHE:      u64 = 0x1cf2e4ddd306dda9; // SHA256("addrcache")[0:8]

/**
   Global state variables
  */

lazy_static!{
    pub static ref DISCOVER: bool = true;
    pub static ref LISTEN:   bool = true;
    pub static ref STR_SUBVERSION: String = String::default();
}

lazy_static!{
    /*
    static RecursiveMutex cs_mapLocalHost;
    static std::map<CNetAddr, LocalServiceInfo> mapLocalHost GUARDED_BY(cs_mapLocalHost);
    static bool vfLimited[NET_MAX] GUARDED_BY(cs_mapLocalHost) = {};
    */
}
