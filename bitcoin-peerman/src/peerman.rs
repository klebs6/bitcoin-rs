crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/txorphanage.h]
//-------------------------------------------[.cpp/bitcoin/src/txorphanage.cpp]
//-------------------------------------------[.cpp/bitcoin/src/txrequest.h]
//-------------------------------------------[.cpp/bitcoin/src/txrequest.cpp]
///----------------------
pub type PeerManagerMapRelay         = HashMap<u256,TransactionRef>;
pub type PeerManagerMapRelayIterator = (Arc<u256>,TransactionRef);

/**
  | Map of all Peer objects, keyed by peer
  | id. This map is protected by the m_peer_mutex.
  | Once a shared pointer reference is taken,
  | the lock may be released. Individual
  | fields are protected by their own locks.
  |
  */
pub type PeerMap = HashMap<NodeId,Amo<Peer>>;

lazy_static!{

    pub static ref IMPORTING:         AtomicBool = AtomicBool::new(false);
    pub static ref REINDEX:           AtomicBool = AtomicBool::new(false);

    /* ---- Pruning-related variables and constants  ---- */

    /**
      | True if any block files have ever been
      | pruned.
      |
      */
    pub static ref HAVE_PRUNED:       bool = false;

    /**
      | True if we're running in -prune mode.
      |
      */
    pub static ref PRUNE_MODE:        bool = false;

    /**
      | Number of MiB of block files that we're
      | trying to stay below.
      |
      */
    pub static ref N_PRUNE_TARGET:    u64 = 0;

    pub static ref CS_LAST_BLOCK_FILE: Amo<()> = Amo::<()>::none();

    pub static ref VINFO_BLOCK_FILE:   Vec<BlockFileInfo> = vec![];

    pub static ref N_LAST_BLOCK_FILE: i32 = 0;

    /**
      | Global flag to indicate we should check
      | to see if there are block/undo files
      | that should be deleted. Set on startup
      | or if we allocate more file space when
      | we're in prune mode
      |
      */
    pub static ref CHECK_FOR_PRUNING: bool = false;

    /**
      | Dirty block index entries.
      |
      */
    pub static ref SET_DIRTY_BLOCK_INDEX: HashSet::<Option<Arc<BlockIndex>>> = Default::default();

    /**
      | Dirty block file entries.
      |
      */
    pub static ref SET_DIRTY_FILE_INFO:   HashSet::<i32> = Default::default();
}

pub trait PeerManagerInterface:
    StartScheduledTasks
    + GetNodeStateStats
    + IgnoresIncomingTxs 
    + RelayTransaction 
    + SendPings 
    + SetBestHeight 
    + Misbehaving 
{}

pub struct PeerManager {

    pub chainparams:           Arc<ChainParams>,
    pub connman:               Amo<Connman>,
    pub addrman:               Amo<AddrMan>,

    /**
      | Pointer to this node's banman. May be
      | nullptr - check existence before dereferencing.
      |
      */
    pub banman:                Amo<BanMan>,

    pub chainman:              Amo<ChainstateManager>,
    pub mempool:               Amo<TxMemPool>,

    /**
      | The height of the best chain
      |
      */
    pub best_height:           Atomic<i32>, // default = { -1 }

    /**
      | Next time to check for stale tip
      |
      */
    pub stale_tip_check_time:  Atomic<OffsetDateTime>, // default = { 0 }

    /**
      | Whether this node is running in blocks
      | only mode
      |
      */
    pub ignore_incoming_txs:   bool,

    /**
      | Whether we've completed initial sync
      | yet, for determining when to turn on
      | extra block-relay-only peers.
      |
      */
    pub initial_sync_finished: AtomicBool, // default = { false }

    /**
      | Protects m_peer_map. This mutex must
      | not be locked while holding a lock on
      | any of the mutexes inside a Peer object.
      |
      */
    pub peer_map:            Amo<PeerMap>,

    /**
      | Filter for transactions that have been
      | recently confirmed.
      | 
      | We use this to avoid requesting transactions
      | that have already been confirnmed.
      | 
      | Blocks don't typically have more than
      | 4000 transactions, so this should be
      | at least six blocks (~1 hr) worth of transactions
      | that we can store, inserting both a txid
      | and wtxid for every observed transaction.
      | 
      | If the number of transactions appearing
      | in a block goes up, or if we are seeing
      | getdata requests more than an hour after
      | initial announcement, we can increase
      | this number.
      | 
      | The false positive rate of 1/1M should
      | come out to less than 1 transaction per
      | day that would be inadvertently ignored
      | (which is the same probability that
      | we have in the reject filter).
      |
      */
    pub recent_confirmed_transactions_mutex: Amo<PeerManagerRecentConfirmedTransactions>,

    /**
      | When our tip was last updated.
      |
      */
    pub last_tip_update:      Atomic<Option<OffsetDateTime>>, // default = { 0 }

    /**
      | Storage for orphan information
      |
      */
    pub orphanage:                         Arc<TxOrphanage>,

    //#[GUARDED_BY(G_CS_ORPHANS)]
    pub orphan_data:                       PeerManagerOrphans,

    //TODO: #[GUARDED_BY(::CS_MAIN)]
    pub inner:                             Arc<Mutex<PeerManagerInner>>,
}

impl PeerManager {

    pub fn make(&mut self, 
        chainparams:         &ChainParams,
        connman:             &mut Connman,
        addrman:             &mut AddrMan,
        banman:              *mut BanMan,
        chainman:            &mut ChainstateManager,
        pool:                &mut TxMemPool,
        ignore_incoming_txs: bool) -> Box<PeerManager> {
        
        Box::new(
            PeerManager::new(
                chainparams,
                connman,
                addrman,
                banman,
                chainman,
                pool,
                ignore_incoming_txs
            )
        )
    }
    
    pub fn new(
        chainparams:         &ChainParams,
        connman:             &mut Connman,
        addrman:             &mut AddrMan,
        banman:              *mut BanMan,
        chainman:            &mut ChainstateManager,
        pool:                &mut TxMemPool,
        ignore_incoming_txs: bool) -> Self {
    
        todo!();
        /*
        : chainparams(chainparams),
        : connman(connman),
        : addrman(addrman),
        : banman(banman),
        : chainman(chainman),
        : mempool(pool),
        : ignore_incoming_txs(ignore_incoming_txs),

        
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    pub fn relay_transaction_impl(&self, 
        txid:  u256,
        wtxid: u256)  {

        let mut relay_txn = move |pnode: Amo<Box<dyn NodeInterface>>| {

            // EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)
            assert_lock_held!(CS_MAIN);

            let state: Amo<NodeState> = create_state(pnode.get().get_id());

            if state.is_none() {
                return;
            }

            if state.get().wtxid_relay.load(atomic::Ordering::Relaxed) {
                pnode.get_mut().push_tx_inventory(&wtxid);
            } else {
                pnode.get_mut().push_tx_inventory(&txid);
            }
        };

        self.connman
            .get_mut()
            .for_each_node_mut(&mut relay_txn);
    }
}
