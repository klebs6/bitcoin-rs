// ---------------- [ File: bitcoin-peerman/src/txorphanage.rs ]
/*!
  | Jesus said to his disciples, "If the
  | world hates you, be aware that it hated
  | me before it hated you. If you belonged
  | to the world, the world would love you
  | as its own. Because you do not belong
  | to the world, but I have chosen you out
  | of the world, therefore the world hates
  | you.
  | 
  | Remember the word that I said to you,
  | 'Servants are not greater than their
  | master.' If they persecuted me, they
  | will persecute you; if they kept my word,
  | they will keep yours also.
  | 
  | John 15:18-21
  |
  */

crate::ix!();

/**
  | Expiration time for orphan transactions
  | in seconds
  |
  */
pub const ORPHAN_TX_EXPIRE_TIME:     Duration = Duration::seconds(20 * 60);

/**
  | Minimum time between orphan transactions
  | expire time checks in seconds
  |
  */
pub const ORPHAN_TX_EXPIRE_INTERVAL: Duration = Duration::seconds(5 * 60);

/**
  | Guards orphan transactions and extra
  | txs for compact blocks
  |
  */
lazy_static!{
    pub static ref G_CS_ORPHANS: ScopedRawMutex = Default::default();
}

pub type OrphanMap         = HashMap<u256,OrphanTx>; //decltype(m_orphans);
pub type OrphanMapIterator = u256; // OrphanMap::Iterator; //decltype(m_orphans);

pub struct OrphanTx {
    pub tx:            TransactionRef,
    pub from_peer:     NodeId,
    pub n_time_expire: OffsetDateTime,
    pub list_pos:      AtomicUsize,
}

impl Clone for OrphanTx {

    fn clone(&self) -> Self {
        Self {
            tx:            self.tx.clone(),
            from_peer:     self.from_peer.clone(),
            n_time_expire: self.n_time_expire.clone(),
            list_pos:      AtomicUsize::new(self.list_pos.load(atomic::Ordering::SeqCst)),
        }
    }
}

impl PartialEq for OrphanTx {

    fn eq(&self, other: &Self) -> bool {
        vec![
            self.tx            == other.tx,
            self.from_peer     == other.from_peer,
            self.n_time_expire == other.n_time_expire,
            self.list_pos.load(atomic::Ordering::SeqCst) == other.list_pos.load(atomic::Ordering::SeqCst),
        ].iter().all(|&x| x == true)
    }
}

impl Eq for OrphanTx {}

pub struct IteratorComparator { }

impl IteratorComparator {

    pub fn invoke<I: PartialOrd>(&self, a: &I, b: &I) -> bool {
    
        &(*a) < &(*b)
    }
}

impl BuildHasher for IteratorComparator {

    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        todo!();
    }
}

impl Hasher for IteratorComparator {

    fn finish(&self) -> u64 {
        todo!();
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!();
    }
}

/**
  | A class to track orphan transactions
  | (failed on TX_MISSING_INPUTS)
  | 
  | Since we cannot distinguish orphans
  | from bad transactions with non-existent
  | inputs, we heavily limit the number
  | of orphans we keep and the duration we
  | keep them for.
  |
  */
//#[GUARDED_BY(G_CS_ORPHANS)]
pub struct TxOrphanage {

    /**
      | Map from txid to orphan transaction
      | record. Limited by
      | 
      | -maxorphantx/DEFAULT_MAX_ORPHAN_TRANSACTIONS
      |
      */
    pub orphans: Arc<Mutex<HashMap<u256,Arc<OrphanTx>>>>,

    /**
      | Index from the parents' OutPoint into
      | the m_orphans. Used to remove orphan
      | transactions from the m_orphans
      | 
      |
      */
    pub outpoint_to_orphan_it: Arc<Mutex<HashMap<OutPoint,HashSet<OrphanMapIterator,IteratorComparator>>>>,

    /**
      | Orphan transactions in vector for quick
      | random eviction
      | 
      |
      */
    pub orphan_list: Arc<Mutex<Vec<OrphanMapIterator>>>,

    /**
      | Index from wtxid into the m_orphans
      | to lookup orphan transactions using
      | their witness ids.
      |
      */
    pub wtxid_to_orphan_it: Arc<Mutex<HashMap<u256,OrphanMapIterator>>>,
}

impl TxOrphanage {

    /**
      | Return how many entries exist in the
      | orphange
      |
      */
    #[LOCKS_EXCLUDED(G_CS_ORPHANS)]
    pub fn size(&self) -> usize {
        
        let mut guard = G_CS_ORPHANS.lock();
        self.orphans.lock().len()
    }
    
    /**
      | Add a new orphan transaction
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    pub fn add_tx(
        self: Arc<Self>, 
        ptx:  &TransactionRef,
        peer: NodeId) -> bool {
        
        assert_lock_held!(G_CS_ORPHANS);

        let tx = ptx.get();

        let hash: &u256 = tx.get_hash();

        if self.orphans.lock().get(hash).is_some() {
            return false;
        }

        // Ignore big transactions, to avoid
        // a send-big-orphans memory exhaustion
        // attack. If a peer has a legitimate
        // large transaction with a missing parent
        // then we assume it will rebroadcast it
        // later, after the parent transaction(s)
        // have been mined or received.
        //
        // 100 orphans, each of which is at most
        // 100,000 bytes big is at most 10
        // megabytes of orphans and somewhat more
        // byprev index (in the worst case):
        let sz: u32 = get_transaction_weight(&tx).try_into().unwrap();

        if sz > MAX_STANDARD_TX_WEIGHT.try_into().unwrap() {

            log_print!(
                LogFlags::MEMPOOL, 
                "ignoring large orphan tx (size: %u, hash: %s)\n", 
                sz, 
                hash.to_string()
            );

            return false;
        }

        let new_orphan = Arc::new(OrphanTx { 
            tx:            ptx.clone(),
            from_peer:     peer,
            n_time_expire: get_datetime() + ORPHAN_TX_EXPIRE_TIME,
            list_pos:      AtomicUsize::new(self.orphan_list.lock().len())
        });

        let ret = self.orphans.lock().insert(
            hash.clone(), 
            new_orphan.clone() 
        );

        // true if a new element was inserted or
        // false if an equivalent key already
        // existed.
        assert!(ret.is_none());

        self.orphan_list.lock().push(hash.clone());

        // Allow for lookups in the orphan pool by
        // wtxid, as well as txid
        self.wtxid_to_orphan_it.lock().insert(
            tx.get_witness_hash().clone(), 
            hash.clone()
        );

        for txin in tx.vin.iter() {

            self.outpoint_to_orphan_it
                .lock()
                .get_mut(&txin.prevout)
                .as_mut()
                .unwrap()
                .insert(hash.clone());
        }

        log_print!(
            LogFlags::MEMPOOL, 
            "stored orphan tx %s (mapsz %u outsz %u)\n", 
            hash.to_string(), 
            self.orphans.lock().len(), 
            self.outpoint_to_orphan_it.lock().len()
        );

        true
    }
    
    /**
      | Erase an orphan by txid
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    pub fn erase_tx(self: Arc<Self>, txid: &u256) -> i32 {
        
        assert_lock_held!(G_CS_ORPHANS);

        let mut orphans     = self.orphans.lock();
        let mut orphan_list = self.orphan_list.lock();

        let (hash,orphan_tx): (u256,Arc<OrphanTx>) = {

            let maybe_pair = orphans.get_key_value(txid);

            if maybe_pair.is_none() {
                return 0;
            }

            let (hash,orphan_tx) = maybe_pair.as_ref().unwrap();

            ((**hash).clone(), (**orphan_tx).clone())
        };

        for txin in orphan_tx.tx.get().vin.iter() {

            let mut guard = self.outpoint_to_orphan_it.lock();

            let mut it_prev = guard.get_mut(&txin.prevout);

            if it_prev.is_none() {
                continue;
            }

            it_prev.as_mut().unwrap().remove(&hash);

            if it_prev.as_ref().unwrap().is_empty() {
                guard.remove(&txin.prevout);
            }
        }

        let old_pos: usize = orphan_tx.list_pos.load(atomic::Ordering::SeqCst);

        let key = &orphan_list[old_pos];

        assert!(orphans.get(&key) == Some(&orphan_tx));

        if old_pos + 1 != orphan_list.len() {

            // Unless we're deleting the last
            // entry in m_orphan_list, move the
            // last entry to the position we're
            // deleting.
            let key = orphan_list.last().clone().unwrap().clone();

            orphan_list[old_pos] = key.clone();

            orphans.get(&key).unwrap().list_pos.store(old_pos, atomic::Ordering::Relaxed);
        }

        orphan_list.pop();

        self.wtxid_to_orphan_it.lock().remove(orphan_tx.tx.get().get_witness_hash());

        orphans.remove(&hash);

        1
    }
    
    /**
      | Erase all orphans announced by a peer
      | (eg, after that peer disconnects)
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    pub fn erase_for_peer(self: Arc<Self>, peer: NodeId)  {
        
        assert_lock_held!(G_CS_ORPHANS);

        let mut n_erased: i32 = 0;

        let orphans = self.orphans.lock();

        let mut iter = orphans.iter().peekable();

        while iter.peek().is_some() {

            // increment to avoid iterator
            // becoming invalid
            let maybe_erase = iter.peek().unwrap().1.clone();

            iter.next();

            if maybe_erase.from_peer == peer {
                n_erased += self.clone().erase_tx(maybe_erase.tx.get().get_hash());
            }
        }

        if n_erased > 0 {
            log_print!(
                LogFlags::MEMPOOL, 
                "Erased %d orphan tx from peer=%d\n", 
                n_erased, 
                peer
            );
        }
    }
    
    /**
      | Limit the orphanage to the given maximum
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    pub fn limit_orphans(self: Arc<Self>, max_orphans: u32) -> u32 {
        
        assert_lock_held!(G_CS_ORPHANS);

        let mut n_evicted: u32 = 0;

        lazy_static!{
            static ref N_NEXT_SWEEP: Amo<OffsetDateTime> = amo_none();
        }

        let n_now: OffsetDateTime = get_datetime();

        if N_NEXT_SWEEP.is_none() || *N_NEXT_SWEEP.get() <= n_now {

            // Sweep out expired orphan pool
            // entries:
            let mut n_erased: i32 = 0;

            let mut n_min_exp_time: OffsetDateTime = n_now + ORPHAN_TX_EXPIRE_TIME - ORPHAN_TX_EXPIRE_INTERVAL;

            let orphans = self.orphans.lock();

            let mut iter = orphans.iter().peekable();

            while iter.peek() != None {

                let maybe_erase = iter.peek().unwrap().1.clone();

                iter.next();

                if maybe_erase.n_time_expire <= n_now {
                    n_erased += self.clone().erase_tx(maybe_erase.tx.get().get_hash());
                } else {
                    n_min_exp_time = min(maybe_erase.n_time_expire,n_min_exp_time);
                }
            }

            // Sweep again 5 minutes after the
            // next entry that expires in order to
            // batch the linear scan.
            *N_NEXT_SWEEP.get_mut() = n_min_exp_time + ORPHAN_TX_EXPIRE_INTERVAL;

            if n_erased > 0 {
                log_print!(LogFlags::MEMPOOL, "Erased %d orphan tx due to expiration\n", n_erased);
            }
        }

        let mut rng = FastRandomContext::default();

        let orphans     = self.orphans.lock();
        let orphan_list = self.orphan_list.lock();

        while orphans.len() > max_orphans.try_into().unwrap() {

            // Evict a random orphan:
            let randompos: usize = rng.randrange(orphan_list.len().try_into().unwrap()).try_into().unwrap();

            let key = &orphan_list[randompos];

            self.clone().erase_tx(&key);

            n_evicted += 1;
        }

        n_evicted
    }
    
    /**
      | Add any orphans that list a particular
      | tx as a parent into a peer's work set (ie
      | orphans that may have found their final
      | missing parent, and so should be reconsidered
      | for the mempool)
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    pub fn add_children_to_work_set(&self, 
        tx:              &Transaction,
        orphan_work_set: &mut HashSet<u256>)  {
        
        assert_lock_held!(G_CS_ORPHANS);

        for i in 0..tx.vout.len() {

            let guard = self.outpoint_to_orphan_it.lock();

            let it_by_prev = guard.get(
                &OutPoint::new(tx.get_hash(),i.try_into().unwrap())
            );

            if it_by_prev.is_some() {

                for elem in it_by_prev.as_ref().unwrap().iter() {
                    orphan_work_set.insert(elem.clone());
                }
            }
        }
    }
    
    /**
      | Check if we already have an orphan transaction
      | (by txid or wtxid)
      |
      */
    #[LOCKS_EXCLUDED(::G_CS_ORPHANS)]
    pub fn have_tx(&self, gtxid: &GenTxId) -> bool {
        
        let mut guard = G_CS_ORPHANS.lock();

        if gtxid.is_wtxid() {
            self.wtxid_to_orphan_it.lock().get(gtxid.get_hash()).is_some()
        } else {

            let orphans = self.orphans.lock();

            orphans.get(gtxid.get_hash()).is_some()
        }
    }
    
    /**
      | Get an orphan transaction and its originating
      | peer (Transaction ref will be nullptr
      | if not found)
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    pub fn get_tx(&self, txid: &u256) -> (TransactionRef,NodeId) {
        
        assert_lock_held!(G_CS_ORPHANS);

        let orphans = self.orphans.lock();

        let it = orphans.get(txid);

        if it == None {
            return (TransactionRef::none(),-1);
        }

        (
            it.as_ref().unwrap().tx.clone(),
            it.as_ref().unwrap().from_peer
        )
    }
    
    /**
      | Erase all orphans included in or invalidated
      | by a new block
      |
      */
    #[LOCKS_EXCLUDED(::G_CS_ORPHANS)]
    pub fn erase_for_block(self: Arc<Self>, block: &Block)  {
        
        let mut guard = G_CS_ORPHANS.lock();

        let mut orphan_erase: Vec<u256> = vec![];

        let orphans = self.orphans.lock();

        for ptx in block.vtx.iter() {

            let tx = ptx.get();

            // Which orphan pool entries must we
            // evict?
            for txin in tx.vin.iter() {

                let guard = self.outpoint_to_orphan_it.lock();

                let it_by_prev = guard.get(&txin.prevout);

                if it_by_prev.is_none() {
                    continue;
                }

                for set in it_by_prev.iter() {

                    for key in set.iter() {

                        if let Some(mi) = orphans.get(key) {

                            let orphan_tx = mi.tx.get();

                            let orphan_hash: &u256 = orphan_tx.get_hash();

                            orphan_erase.push(orphan_hash.clone());
                        }
                    }
                }
            }
        }

        // Erase orphan transactions included or
        // precluded by this block
        if orphan_erase.len() != 0 {

            let mut n_erased: i32 = 0;

            for orphan_hash in orphan_erase.iter() {
                n_erased += self.clone().erase_tx(orphan_hash);
            }

            log_print!(
                LogFlags::MEMPOOL, 
                "Erased %d orphan tx included or conflicted by block\n", 
                n_erased
            );
        }
    }
}
