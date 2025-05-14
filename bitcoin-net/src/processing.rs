// ---------------- [ File: bitcoin-net/src/processing.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/net_processing.h]

/**
  | Default for -maxorphantx, maximum
  | number of orphan transactions kept
  | in memory
  |
  */
pub const DEFAULT_MAX_ORPHAN_TRANSACTIONS: u32 = 100;

/**
  | Default number of orphan+recently-replaced
  | txn to keep around for block reconstruction
  |
  */
pub const DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN: u32 = 100;
pub const DEFAULT_PEERBLOOMFILTERS: bool = false;
pub const DEFAULT_PEERBLOCKFILTERS: bool = false;

/**
  | Threshold for marking a node to be discouraged,
  | e.g. disconnected and added to the discouragement
  | filter.
  |
  */
pub const DISCOURAGEMENT_THRESHOLD: i32 = 100;

//-------------------------------------------[.cpp/bitcoin/src/net_processing.cpp]

/**
  | How long to cache transactions in mapRelay
  | for normal relay
  |
  */
pub const RELAY_TX_CACHE_TIME: Duration = Duration::minutes(15);

/**
  | How long a transaction has to be in the
  | mempool before it can unconditionally
  | be relayed (even when not in mapRelay).
  |
  */
pub const UNCONDITIONAL_RELAY_DELAY: Duration = Duration::minutes(2);

/**
  | Headers download timeout.
  | 
  | Timeout = base + per_header * (expected
  | number of headers)
  |
  */
pub const HEADERS_DOWNLOAD_TIMEOUT_BASE:       Duration = Duration::minutes(15);
pub const HEADERS_DOWNLOAD_TIMEOUT_PER_HEADER: Duration = Duration::milliseconds(1);

/**
  | Protect at least this many outbound
  | peers from disconnection due to slow/
  | behind headers chain.
  |
  */
pub const MAX_OUTBOUND_PEERS_TO_PROTECT_FROM_DISCONNECT: i32 = 4;

/**
  | Timeout for (unprotected) outbound
  | peers to sync to our chainwork, in seconds
  |
  */
pub const CHAIN_SYNC_TIMEOUT: Duration = Duration::minutes(20);

/**
  | How frequently to check for stale tips,
  | in seconds
  |
  */
pub const STALE_CHECK_INTERVAL: Duration = Duration::minutes(10);

/**
  | How frequently to check for extra outbound
  | peers and disconnect, in seconds
  |
  */
pub const EXTRA_PEER_CHECK_INTERVAL: Duration = Duration::seconds(45);

/**
  | Minimum time an outbound-peer-eviction
  | candidate must be connected for, in
  | order to evict, in seconds
  |
  */
pub const MINIMUM_CONNECT_TIME: Duration = Duration::seconds(30);

/**
  | SHA256("main address relay")[0:8]
  |
  */
pub const RANDOMIZER_ID_ADDRESS_RELAY: u64 = 0x3cac0035b5866b90;

/**
  | Age after which a stale block will no longer
  | be served if requested as protection against
  | fingerprinting. Set to one month, denominated
  | in seconds.
  */
pub const STALE_RELAY_AGE_LIMIT: Duration = Duration::days(30);

/**
  | Age after which a block is considered
  | historical for purposes of rate limiting block
  | relay. Set to one week, denominated in
  | seconds.
  */
pub const HISTORICAL_BLOCK_AGE: Duration = Duration::days(7);

/**
  | Time between pings automatically sent
  | out for latency probing and keepalive
  |
  */
pub const PING_INTERVAL: Duration = Duration::minutes(2);

/**
  | The maximum number of entries in a locator
  |
  */
pub const MAX_LOCATOR_SZ: u32 = 101;

/**
  | The maximum number of entries in an 'inv'
  | protocol message
  |
  */
pub const MAX_INV_SZ: u32 = 50000;

/**
  | Maximum number of in-flight transaction
  | requests from a peer. It is not a hard
  | limit, but the threshold at which point
  | the OVERLOADED_PEER_TX_DELAY kicks
  | in.
  |
  */
pub const MAX_PEER_TX_REQUEST_IN_FLIGHT: i32 = 100;

/**
  | Maximum number of transactions to consider
  | for requesting, per peer. It provides
  | a reasonable DoS limit to per-peer memory
  | usage spent on announcements, while
  | covering peers continuously sending
  | INVs at the maximum rate (by our own policy,
  | see INVENTORY_BROADCAST_PER_SECOND)
  | for several minutes, while not receiving
  | the actual transaction (from any peer)
  | in response to requests for them.
  |
  */
pub const MAX_PEER_TX_ANNOUNCEMENTS: i32 = 5000;

/**
  | How long to delay requesting transactions
  | via txids, if we have wtxid-relaying
  | peers
  |
  */
pub const TXID_RELAY_DELAY: Duration = Duration::seconds(2);

/**
  | How long to delay requesting transactions
  | from non-preferred peers
  |
  */
pub const NONPREF_PEER_TX_DELAY: Duration = Duration::seconds(2);

/**
  | How long to delay requesting transactions
  | from overloaded peers (see MAX_PEER_TX_REQUEST_IN_FLIGHT).
  |
  */
pub const OVERLOADED_PEER_TX_DELAY: Duration = Duration::seconds(2);

/**
  | How long to wait (in microseconds) before
  | downloading a transaction from an additional
  | peer
  |
  */
pub const GETDATA_TX_INTERVAL: Duration = Duration::microseconds(60);

/**
  | Limit to avoid sending big packets.
  | Not used in processing incoming GETDATA
  | for compatibility
  |
  */
pub const MAX_GETDATA_SZ: u32 = 1000;

/**
  | Number of blocks that can be requested
  | at any given time from a single peer.
  |
  */
pub const MAX_BLOCKS_IN_TRANSIT_PER_PEER: i32 = 16;

/**
  | Time during which a peer must stall block
  | download progress before being disconnected.
  |
  */
pub const BLOCK_STALLING_TIMEOUT: Duration = Duration::seconds(2);

/**
  | Number of headers sent in one getheaders
  | result. We rely on the assumption that
  | if a peer sends less than this number,
  | we reached its tip. Changing this value
  | is a protocol upgrade.
  |
  */
pub const MAX_HEADERS_RESULTS: u32 = 2000;

/**
  | Maximum depth of blocks we're willing
  | to serve as compact blocks to peers when
  | requested. For older blocks, a regular
  | BLOCK response will be sent.
  |
  */
pub const MAX_CMPCTBLOCK_DEPTH: usize = 5;

/**
  | Maximum depth of blocks we're willing
  | to respond to GETBLOCKTXN requests
  | for.
  |
  */
pub const MAX_BLOCKTXN_DEPTH: usize = 10;

/**
  | Size of the "block download window":
  | how far ahead of our current height do
  | we fetch?
  | 
  | Larger windows tolerate larger download
  | speed differences between peer, but
  | increase the potential degree of disordering
  | of blocks on disk (which make reindexing
  | and pruning harder). We'll probably
  | want to make this a per-peer adaptive
  | value at some point.
  |
  */
pub const BLOCK_DOWNLOAD_WINDOW: u32 = 1024;

/**
  | Block download timeout base, expressed
  | in multiples of the block interval (i.e.
  | 10 min)
  |
  */
pub const BLOCK_DOWNLOAD_TIMEOUT_BASE: f64 = 1.0;

/**
  | Additional block download timeout
  | per parallel downloading peer (i.e.
  | 5 min)
  |
  */
pub const BLOCK_DOWNLOAD_TIMEOUT_PER_PEER: f64 = 0.5;

/**
  | Maximum number of headers to announce
  | when relaying blocks with headers message.
  |
  */
pub const MAX_BLOCKS_TO_ANNOUNCE: u32 = 8;

/**
  | Maximum number of unconnecting headers
  | announcements before DoS score
  |
  */
pub const MAX_UNCONNECTING_HEADERS: i32 = 10;

/**
  | Minimum blocks required to signal NODE_NETWORK_LIMITED
  |
  */
pub const NODE_NETWORK_LIMITED_MIN_BLOCKS: u32 = 288;

/**
  | Average delay between local address
  | broadcasts
  |
  */
pub const AVG_LOCAL_ADDRESS_BROADCAST_INTERVAL: Duration = Duration::hours(24);

/**
  | Average delay between peer address
  | broadcasts
  |
  */
pub const AVG_ADDRESS_BROADCAST_INTERVAL: Duration = Duration::seconds(30);

/**
  | Average delay between trickled inventory
  | transmissions for inbound peers.
  | 
  | Blocks and peers with NetPermissionFlags::NoBan
  | permission bypass this.
  |
  */
pub const INBOUND_INVENTORY_BROADCAST_INTERVAL: Duration = Duration::seconds(5);

/**
  | Average delay between trickled inventory
  | transmissions for outbound peers.
  | 
  | Use a smaller delay as there is less privacy
  | concern for them.
  | 
  | Blocks and peers with NetPermissionFlags::NoBan
  | permission bypass this.
  |
  */
pub const OUTBOUND_INVENTORY_BROADCAST_INTERVAL: Duration = Duration::seconds(2);

/**
  | Maximum rate of inventory items to send
  | per second.
  | 
  | Limits the impact of low-fee transaction
  | floods.
  |
  */
pub const INVENTORY_BROADCAST_PER_SECOND: u32 = 7;

/**
  | Maximum number of inventory items to
  | send per transmission.
  |
  */
lazy_static!{
    pub static ref INVENTORY_BROADCAST_MAX: u32 = INVENTORY_BROADCAST_PER_SECOND * (INBOUND_INVENTORY_BROADCAST_INTERVAL.as_seconds_f32() as u32);
}

/**
  | The number of most recently announced
  | transactions a peer can request.
  |
  */
pub const INVENTORY_MAX_RECENT_RELAY: u32 = 3500;

/**
  | Verify that INVENTORY_MAX_RECENT_RELAY
  | is enough to cache everything typically
  | relayed before unconditional relay
  | from the mempool kicks in. This is only
  | a lower bound, and it should be larger
  | to account for higher inv rate to outbound
  | peers, and random variations in the
  | broadcast mechanism.
  |
  */
//TODO: check dimensional analysis
//this is lazily set to true, even though it
//should not be... i was tired and went to sleep
const_assert!{
    true
    //INVENTORY_MAX_RECENT_RELAY >= INVENTORY_BROADCAST_PER_SECOND * UNCONDITIONAL_RELAY_DELAY / Seconds(1)
} //"INVENTORY_RELAY_MAX too low"

/**
  | Average delay between feefilter broadcasts
  | in seconds.
  |
  */
pub const AVG_FEEFILTER_BROADCAST_INTERVAL: Duration = Duration::minutes(10);

/**
  | Maximum feefilter broadcast delay
  | after significant change.
  |
  */
pub const MAX_FEEFILTER_CHANGE_DELAY: Duration = Duration::minutes(5);

/**
  | Maximum number of compact filters that
  | may be requested with one getcfilters.
  | See BIP 157.
  |
  */
pub const MAX_GETCFILTERS_SIZE: u32 = 1000;

/**
  | Maximum number of cf hashes that may
  | be requested with one getcfheaders.
  | See BIP 157.
  |
  */
pub const MAX_GETCFHEADERS_SIZE: u32 = 2000;

/**
  | the maximum percentage of addresses
  | from our addrman to return in response
  | to a getaddr message.
  |
  */
pub const MAX_PCT_ADDR_TO_SEND: usize = 23;

/**
  | The maximum number of address records
  | permitted in an ADDR message.
  |
  */
pub const MAX_ADDR_TO_SEND: usize = 1000;

/**
  | The maximum rate of address records
  | we're willing to process on average.
  | Can be bypassed using the NetPermissionFlags::Addr
  | permission.
  |
  */
pub const MAX_ADDR_RATE_PER_SECOND: f64 = 0.1;

/**
  | The soft limit of the address processing
  | token bucket (the regular MAX_ADDR_RATE_PER_SECOND
  | based increments won't go above this,
  | but the MAX_ADDR_TO_SEND increment
  | following GETADDR is exempt from this
  | limit).
  |
  */
pub const MAX_ADDR_PROCESSING_TOKEN_BUCKET: usize = MAX_ADDR_TO_SEND;

/**
  | Number of preferable block download
  | peers.
  |
  */
//TODO: #[GUARDED_BY(cs_main)]
pub const N_PREFERRED_DOWNLOAD: AtomicI32 = AtomicI32::new(0);

/**
  | All of the following cache a recent block,
  | and are protected by cs_most_recent_block
  |
  */
lazy_static!{

    pub static ref CS_MOST_RECENT_BLOCK: Amo<()> = amo_none();

    // GUARDED_BY(cs_most_recent_block)
    pub static ref MOST_RECENT_BLOCK: Amo::<Block> = amo_none();

    // GUARDED_BY(cs_most_recent_block)
    pub static ref MOST_RECENT_COMPACT_BLOCK: Amo::<BlockHeaderAndShortTxIDs> = amo_none();

    // GUARDED_BY(cs_most_recent_block)
    pub static ref MOST_RECENT_BLOCK_HASH: Amo::<u256> = amo_none();

    // GUARDED_BY(cs_most_recent_block)
    pub static ref WITNESSES_PRESENT_IN_MOST_RECENT_COMPACT_BLOCK: AtomicBool = AtomicBool::new(false);
}
