crate::ix!();

///--------------------------
pub struct MainSignals<'a> {
    internals: Box<MainSignalsInstance<'a>>,
}

lazy_static!{
    /*
    static CMainSignals g_signals;
    */
}

impl<'a> MainSignals<'a> {
    
    /**
      | Register a CScheduler to give callbacks
      | which should run in the background (may
      | only be called once)
      |
      */
    pub fn register_background_signal_scheduler(&mut self, scheduler: &mut Scheduler)  {
        
        todo!();
        /*
            assert(!m_internals);
        m_internals.reset(new MainSignalsInstance(&scheduler));
        */
    }
    
    /**
      | Unregister a CScheduler to give callbacks
      | which should run in the background -
      | these callbacks will now be dropped!
      |
      */
    pub fn unregister_background_signal_scheduler(&mut self)  {
        
        todo!();
        /*
            m_internals.reset(nullptr);
        */
    }
    
    /**
      | Call any remaining callbacks on the
      | calling thread
      |
      */
    pub fn flush_background_callbacks(&mut self)  {
        
        todo!();
        /*
            if (m_internals) {
            m_internals->m_schedulerClient.EmptyQueue();
        }
        */
    }
    
    pub fn callbacks_pending(&mut self) -> usize {
        
        todo!();
        /*
            if (!m_internals) return 0;
        return m_internals->m_schedulerClient.CallbacksPending();
        */
    }
}

/**
  | The MainSignalsInstance manages a list of
  | shared_ptr<CValidationInterface> callbacks.
  |
  | A std::unordered_map is used to track what
  | callbacks are currently registered, and
  | a std::list is to used to store the callbacks
  | that are currently registered as well as any
  | callbacks that are just unregistered and about
  | to be deleted when they are done executing.
  */
pub struct MainSignalsInstance<'a> {

    mutex:            std::sync::Mutex<main_signals_instance::Inner<'a>>,

    /**
      | We are not allowed to assume the scheduler
      | only runs in one thread, but must ensure
      | all callbacks happen in-order, so we
      | end up creating our own queue here :(
      |
      */
    scheduler_client: SingleThreadedSchedulerClient,
}

pub mod main_signals_instance {
    use super::*;

    pub struct Inner<'a> {
        list:             LinkedList<ListEntry>,
        map:              HashMap<
                                    Rc<RefCell<dyn ValidationInterface>>,
                                    std::collections::linked_list::Iter<'a, ListEntry>
                                >,
    }

    /**
      | List entries consist of a callback pointer
      | and reference count. The count is equal to
      | the number of current executions of that
      | entry, plus 1 if it's registered. It
      | cannot be 0 because that would imply it is
      | unregistered and also not being executed
      | (so shouldn't exist).
      */
    pub struct ListEntry { 
        callbacks: Arc<dyn ValidationInterface>,
        count:     i32, // default = 1
    }
}

impl<'a> MainSignalsInstance<'a> {
    
    pub fn new(pscheduler: *mut Scheduler) -> Self {
    
        todo!();
        /*
        : scheduler_client(pscheduler),

        
        */
    }
    
    pub fn register(&mut self, callbacks: Arc<dyn ValidationInterface>)  {
        
        todo!();
        /*
            LOCK(m_mutex);
            auto inserted = m_map.emplace(callbacks.get(), m_list.end());
            if (inserted.second) inserted.first->second = m_list.emplace(m_list.end());
            inserted.first->second->callbacks = std::move(callbacks);
        */
    }
    
    pub fn unregister(&mut self, callbacks: Rc<RefCell<dyn ValidationInterface>>)  {
        
        todo!();
        /*
            LOCK(m_mutex);
            auto it = m_map.find(callbacks);
            if (it != m_map.end()) {
                if (!--it->second->count) m_list.erase(it->second);
                m_map.erase(it);
            }
        */
    }

    /**
      | Clear unregisters every previously
      | registered callback, erasing every map
      | entry. After this call, the list may still
      | contain callbacks that are currently
      | executing, but it will be cleared when
      | they are done executing.
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            LOCK(m_mutex);
            for (const auto& entry : m_map) {
                if (!--entry.second->count) m_list.erase(entry.second);
            }
            m_map.clear();
        */
    }
    
    
    pub fn iterate<F>(&mut self, f: F)  {
    
        todo!();
        /*
            WAIT_LOCK(m_mutex, lock);
            for (auto it = m_list.begin(); it != m_list.end();) {
                ++it->count;
                {
                    REVERSE_LOCK(lock);
                    f(*it->callbacks);
                }
                it = --it->count ? std::next(it) : m_list.erase(it);
            }
        */
    }
}

pub fn get_main_signals() -> &'static mut MainSignals<'static> {
    
    todo!();
        /*
            return g_signals;
        */
}

impl<'a> UpdatedBlockTip for MainSignals<'a> {

    fn updated_block_tip(&mut self, 
        pindex_new:       Option<Arc<BlockIndex>>,
        pindex_fork:      Option<Arc<BlockIndex>>,
        initial_download: bool)  {
        
        todo!();
        /*
            // Dependencies exist that require UpdatedBlockTip events to be delivered in the order in which
        // the chain actually updates. One way to ensure this is for the caller to invoke this signal
        // in the same critical section where the chain is updated

        auto event = [pindexNew, pindexFork, fInitialDownload, this] {
            m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.UpdatedBlockTip(pindexNew, pindexFork, fInitialDownload); });
        };
        ENQUEUE_AND_LOG_EVENT(event, "%s: new block hash=%s fork block hash=%s (in IBD=%s)", __func__,
                              pindexNew->GetBlockHash().ToString(),
                              pindexFork ? pindexFork->GetBlockHash().ToString() : "null",
                              fInitialDownload);
        */
    }
}

impl<'a> TransactionAddedToMempool for MainSignals<'a> {
    
    fn transaction_added_to_mempool(&mut self, 
        tx:               &TransactionRef,
        mempool_sequence: u64)  {
        
        todo!();
        /*
            auto event = [tx, mempool_sequence, this] {
            m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.TransactionAddedToMempool(tx, mempool_sequence); });
        };
        ENQUEUE_AND_LOG_EVENT(event, "%s: txid=%s wtxid=%s", __func__,
                              tx->GetHash().ToString(),
                              tx->GetWitnessHash().ToString());
        */
    }
}

impl<'a> TransactionRemovedFromMempool for MainSignals<'a> {

    fn transaction_removed_from_mempool(&mut self, 
        tx:               &TransactionRef,
        reason:           MemPoolRemovalReason,
        mempool_sequence: u64)  {
        
        todo!();
        /*
            auto event = [tx, reason, mempool_sequence, this] {
            m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.TransactionRemovedFromMempool(tx, reason, mempool_sequence); });
        };
        ENQUEUE_AND_LOG_EVENT(event, "%s: txid=%s wtxid=%s", __func__,
                              tx->GetHash().ToString(),
                              tx->GetWitnessHash().ToString());
        */
    }
}

impl<'a> BlockConnected for MainSignals<'a> {

    fn block_connected(&mut self, 
        pblock: Arc<Block>,
        pindex: Arc<BlockIndex>)  {
        
        todo!();
        /*
            auto event = [pblock, pindex, this] {
            m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.BlockConnected(pblock, pindex); });
        };
        ENQUEUE_AND_LOG_EVENT(event, "%s: block hash=%s block height=%d", __func__,
                              pblock->GetHash().ToString(),
                              pindex->nHeight);
        */
    }
}

impl<'a> BlockDisconnected for MainSignals<'a> {

    fn block_disconnected(&mut self, 
        pblock: Arc<Block>,
        pindex: Arc<BlockIndex>)  {
        
        todo!();
        /*
            auto event = [pblock, pindex, this] {
            m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.BlockDisconnected(pblock, pindex); });
        };
        ENQUEUE_AND_LOG_EVENT(event, "%s: block hash=%s block height=%d", __func__,
                              pblock->GetHash().ToString(),
                              pindex->nHeight);
        */
    }
}

impl<'a> ChainStateFlushed for MainSignals<'a> {

    fn chain_state_flushed(&mut self, locator: &BlockLocator)  {
        
        todo!();
        /*
            auto event = [locator, this] {
            m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.ChainStateFlushed(locator); });
        };
        ENQUEUE_AND_LOG_EVENT(event, "%s: block hash=%s", __func__,
                              locator.IsNull() ? "null" : locator.vHave.front().ToString());
        */
    }
}
    
impl<'a> BlockChecked for MainSignals<'a> {
    fn block_checked(self: Arc<Self>, 
        block: &Block,
        state: &BlockValidationState)  {
        
        todo!();
        /*
            LOG_EVENT("%s: block hash=%s state=%s", __func__,
                  block.GetHash().ToString(), state.ToString());
        m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.BlockChecked(block, state); });
        */
    }
}

    
impl<'a> NewPoWValidBlock for MainSignals<'a> {

    fn new_pow_valid_block(&mut self, 
        pindex: Arc<BlockIndex>,
        block:  &Arc<Block>)  {
        
        todo!();
        /*
            LOG_EVENT("%s: block hash=%s", __func__, block->GetHash().ToString());
        m_internals->Iterate([&](CValidationInterface& callbacks) { callbacks.NewPoWValidBlock(pindex, block); });
        */
    }
}
