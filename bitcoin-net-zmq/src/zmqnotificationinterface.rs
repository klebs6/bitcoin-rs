// ---------------- [ File: bitcoin-net-zmq/src/zmqnotificationinterface.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqnotificationinterface.h]
pub struct ZMQNotificationInterface {
    pcontext:  *mut c_void,
    notifiers: LinkedList<Box<ZMQAbstractNotifier>>,
}

impl ValidationInterface for ZMQNotificationInterface { }

impl BlockChecked      for ZMQNotificationInterface {}
impl NewPoWValidBlock  for ZMQNotificationInterface {}
impl ChainStateFlushed for ZMQNotificationInterface {}

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqnotificationinterface.cpp]
lazy_static!{
    /*
    CZMQNotificationInterface* g_zmq_notification_interface = nullptr;
    */
}

impl Drop for ZMQNotificationInterface {
    fn drop(&mut self) {
        todo!();
        /*
            Shutdown();
        */
    }
}

impl Default for ZMQNotificationInterface {

    /* ------------- ValidationInterface  ------------- */
    
    fn default() -> Self {
    
        todo!();
        /*
        : pcontext(nullptr),

        
        */
    }
}

impl ZMQNotificationInterface {

    pub fn get_active_notifiers(&self) -> LinkedList<*const ZMQAbstractNotifier> {
        
        todo!();
        /*
            std::list<const CZMQAbstractNotifier*> result;
        for (const auto& n : notifiers) {
            result.push_back(n.get());
        }
        return result;
        */
    }
    
    pub fn create(&mut self) -> *mut ZMQNotificationInterface {
        
        todo!();
        /*
            std::map<std::string, CZMQNotifierFactory> factories;
        factories["pubhashblock"] = CZMQAbstractNotifier::Create<CZMQPublishHashBlockNotifier>;
        factories["pubhashtx"] = CZMQAbstractNotifier::Create<CZMQPublishHashTransactionNotifier>;
        factories["pubrawblock"] = CZMQAbstractNotifier::Create<CZMQPublishRawBlockNotifier>;
        factories["pubrawtx"] = CZMQAbstractNotifier::Create<CZMQPublishRawTransactionNotifier>;
        factories["pubsequence"] = CZMQAbstractNotifier::Create<CZMQPublishSequenceNotifier>;

        std::list<std::unique_ptr<CZMQAbstractNotifier>> notifiers;
        for (const auto& entry : factories)
        {
            std::string arg("-zmq" + entry.first);
            const auto& factory = entry.second;
            for (const std::string& address : gArgs.GetArgs(arg)) {
                std::unique_ptr<CZMQAbstractNotifier> notifier = factory();
                notifier->SetType(entry.first);
                notifier->SetAddress(address);
                notifier->SetOutboundMessageHighWaterMark(static_cast<int>(gArgs.GetIntArg(arg + "hwm", CZMQAbstractNotifier::DEFAULT_ZMQ_SNDHWM)));
                notifiers.push_back(std::move(notifier));
            }
        }

        if (!notifiers.empty())
        {
            std::unique_ptr<CZMQNotificationInterface> notificationInterface(new CZMQNotificationInterface());
            notificationInterface->notifiers = std::move(notifiers);

            if (notificationInterface->Initialize()) {
                return notificationInterface.release();
            }
        }

        return nullptr;
        */
    }
}

impl Initialize for ZMQNotificationInterface {

    /**
      | Called at startup to conditionally
      | set up ZMQ socket(s)
      |
      */
    fn initialize(&mut self, pcontext: *mut c_void) -> bool {
        
        todo!();
        /*
            int major = 0, minor = 0, patch = 0;
        zmq_version(&major, &minor, &patch);
        LogPrint(LogFlags::ZMQ, "zmq: version %d.%d.%d\n", major, minor, patch);

        LogPrint(LogFlags::ZMQ, "zmq: Initialize notification interface\n");
        assert(!pcontext);

        pcontext = zmq_ctx_new();

        if (!pcontext)
        {
            zmqError("Unable to initialize context");
            return false;
        }

        for (auto& notifier : notifiers) {
            if (notifier->Initialize(pcontext)) {
                LogPrint(LogFlags::ZMQ, "zmq: Notifier %s ready (address = %s)\n", notifier->GetType(), notifier->GetAddress());
            } else {
                LogPrint(LogFlags::ZMQ, "zmq: Notifier %s failed (address = %s)\n", notifier->GetType(), notifier->GetAddress());
                return false;
            }
        }

        return true;
        */
    }
}

impl crate::traits::Shutdown for ZMQNotificationInterface {

    /**
      | Called during shutdown sequence
      |
      */
    fn shutdown(&mut self)  {
        
        todo!();
        /*
            LogPrint(LogFlags::ZMQ, "zmq: Shutdown notification interface\n");
        if (pcontext)
        {
            for (auto& notifier : notifiers) {
                LogPrint(LogFlags::ZMQ, "zmq: Shutdown notifier %s at %s\n", notifier->GetType(), notifier->GetAddress());
                notifier->Shutdown();
            }
            zmq_ctx_term(pcontext);

            pcontext = nullptr;
        }
        */
    }
}
    
impl UpdatedBlockTip for ZMQNotificationInterface {

    fn updated_block_tip(&mut self, 
        pindex_new:       Option<Arc<BlockIndex>>,
        pindex_fork:      Option<Arc<BlockIndex>>,
        initial_download: bool)  {
        
        todo!();
        /*
            if (fInitialDownload || pindexNew == pindexFork) // In IBD or blocks were disconnected without any new ones
            return;

        TryForEachAndRemoveFailed(notifiers, [pindexNew](CZMQAbstractNotifier* notifier) {
            return notifier->NotifyBlock(pindexNew);
        });
        */
    }
}
    
impl TransactionAddedToMempool for ZMQNotificationInterface {

    fn transaction_added_to_mempool(&mut self, 
        ptx:              &TransactionRef,
        mempool_sequence: u64)  {
        
        todo!();
        /*
            const CTransaction& tx = *ptx;

        TryForEachAndRemoveFailed(notifiers, [&tx, mempool_sequence](CZMQAbstractNotifier* notifier) {
            return notifier->NotifyTransaction(tx) && notifier->NotifyTransactionAcceptance(tx, mempool_sequence);
        });
        */
    }
}

impl TransactionRemovedFromMempool for ZMQNotificationInterface {

    fn transaction_removed_from_mempool(&mut self, 
        ptx:              &TransactionRef,
        reason:           MemPoolRemovalReason,
        mempool_sequence: u64)  {
        
        todo!();
        /*
            // Called for all non-block inclusion reasons
        const CTransaction& tx = *ptx;

        TryForEachAndRemoveFailed(notifiers, [&tx, mempool_sequence](CZMQAbstractNotifier* notifier) {
            return notifier->NotifyTransactionRemoval(tx, mempool_sequence);
        });
        */
    }
}
    
impl BlockConnected for ZMQNotificationInterface {

    fn block_connected(&mut self, 
        pblock:           Arc<Block>,
        pindex_connected: Arc<BlockIndex>)  {
        
        todo!();
        /*
            for (const CTransactionRef& ptx : pblock->vtx) {
            const CTransaction& tx = *ptx;
            TryForEachAndRemoveFailed(notifiers, [&tx](CZMQAbstractNotifier* notifier) {
                return notifier->NotifyTransaction(tx);
            });
        }

        // Next we notify BlockConnect listeners for *all* blocks
        TryForEachAndRemoveFailed(notifiers, [pindexConnected](CZMQAbstractNotifier* notifier) {
            return notifier->NotifyBlockConnect(pindexConnected);
        });
        */
    }
}
    
impl BlockDisconnected for ZMQNotificationInterface {

    fn block_disconnected(&mut self, 
        pblock:              Arc<Block>,
        pindex_disconnected: Arc<BlockIndex>)  {
        
        todo!();
        /*
            for (const CTransactionRef& ptx : pblock->vtx) {
            const CTransaction& tx = *ptx;
            TryForEachAndRemoveFailed(notifiers, [&tx](CZMQAbstractNotifier* notifier) {
                return notifier->NotifyTransaction(tx);
            });
        }

        // Next we notify BlockDisconnect listeners for *all* blocks
        TryForEachAndRemoveFailed(notifiers, [pindexDisconnected](CZMQAbstractNotifier* notifier) {
            return notifier->NotifyBlockDisconnect(pindexDisconnected);
        });
        */
    }
}

pub fn try_for_each_and_remove_failed<Function>(
        notifiers: &mut LinkedList<Box<ZMQAbstractNotifier>>,
        func:      &Function)  {

    todo!();
        /*
            for (auto i = notifiers.begin(); i != notifiers.end(); ) {
            CZMQAbstractNotifier* notifier = i->get();
            if (func(notifier)) {
                ++i;
            } else {
                notifier->Shutdown();
                i = notifiers.erase(i);
            }
        }
        */
}
