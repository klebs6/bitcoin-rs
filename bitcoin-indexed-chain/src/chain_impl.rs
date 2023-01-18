crate::ix!();

pub struct ChainImpl {
    base: Chain,
    node: Rc<RefCell<NodeContext>>,
}

impl ChainImpl {
    
    pub fn chainman(&mut self) -> &mut ChainstateManager {
        
        todo!();
        /*
            return *Assert(m_node.chainman);
        */
    }
    
    pub fn new(node: &mut NodeContext) -> Self {
    
        todo!();
        /*
        : node(node),

        
        */
    }
    
    pub fn get_height(&mut self) -> Option<i32> {
        
        todo!();
        /*
            LOCK(::cs_main);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            int height = active.Height();
            if (height >= 0) {
                return height;
            }
            return std::nullopt;
        */
    }
    
    pub fn get_block_hash(&mut self, height: i32) -> u256 {
        
        todo!();
        /*
            LOCK(::cs_main);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            CBlockIndex* block = active[height];
            assert(block);
            return block->GetBlockHash();
        */
    }
    
    pub fn have_block_on_disk(&mut self, height: i32) -> bool {
        
        todo!();
        /*
            LOCK(cs_main);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            CBlockIndex* block = active[height];
            return block && ((block->nStatus & BLOCK_HAVE_DATA) != 0) && block->nTx > 0;
        */
    }
    
    pub fn get_tip_locator(&mut self) -> BlockLocator {
        
        todo!();
        /*
            LOCK(cs_main);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            return active.GetLocator();
        */
    }
    
    pub fn check_final_tx(&mut self, tx: &Transaction) -> bool {
        
        todo!();
        /*
            LOCK(cs_main);
            return CheckFinalTx(chainman().ActiveChain().Tip(), tx);
        */
    }
    
    pub fn find_locator_fork(&mut self, locator: &BlockLocator) -> Option<i32> {
        
        todo!();
        /*
            LOCK(cs_main);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            if (CBlockIndex* fork = m_node.chainman->m_blockman.FindForkInGlobalIndex(active, locator)) {
                return fork->nHeight;
            }
            return std::nullopt;
        */
    }
    
    pub fn find_block(&mut self, 
        hash:  &u256,
        block: &FoundBlock) -> bool {
        
        todo!();
        /*
            WAIT_LOCK(cs_main, lock);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            return FillBlock(m_node.chainman->m_blockman.LookupBlockIndex(hash), block, lock, active);
        */
    }
    
    pub fn find_first_block_with_time_and_height(&mut self, 
        min_time:   i64,
        min_height: i32,
        block:      &FoundBlock) -> bool {
        
        todo!();
        /*
            WAIT_LOCK(cs_main, lock);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            return FillBlock(active.FindEarliestAtLeast(min_time, min_height), block, lock, active);
        */
    }
    
    pub fn find_ancestor_by_height(&mut self, 
        block_hash:      &u256,
        ancestor_height: i32,
        ancestor_out:    &FoundBlock) -> bool {
        
        todo!();
        /*
            WAIT_LOCK(cs_main, lock);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            if (const CBlockIndex* block = m_node.chainman->m_blockman.LookupBlockIndex(block_hash)) {
                if (const CBlockIndex* ancestor = block->GetAncestor(ancestor_height)) {
                    return FillBlock(ancestor, ancestor_out, lock, active);
                }
            }
            return FillBlock(nullptr, ancestor_out, lock, active);
        */
    }
    
    pub fn find_ancestor_by_hash(&mut self, 
        block_hash:    &u256,
        ancestor_hash: &u256,
        ancestor_out:  &FoundBlock) -> bool {
        
        todo!();
        /*
            WAIT_LOCK(cs_main, lock);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            const CBlockIndex* block = m_node.chainman->m_blockman.LookupBlockIndex(block_hash);
            const CBlockIndex* ancestor = m_node.chainman->m_blockman.LookupBlockIndex(ancestor_hash);
            if (block && ancestor && block->GetAncestor(ancestor->nHeight) != ancestor) ancestor = nullptr;
            return FillBlock(ancestor, ancestor_out, lock, active);
        */
    }
    
    pub fn find_common_ancestor(&mut self, 
        block_hash1:  &u256,
        block_hash2:  &u256,
        ancestor_out: &FoundBlock,
        block1_out:   &FoundBlock,
        block2_out:   &FoundBlock) -> bool {
        
        todo!();
        /*
            WAIT_LOCK(cs_main, lock);
            const CChain& active = Assert(m_node.chainman)->ActiveChain();
            const CBlockIndex* block1 = m_node.chainman->m_blockman.LookupBlockIndex(block_hash1);
            const CBlockIndex* block2 = m_node.chainman->m_blockman.LookupBlockIndex(block_hash2);
            const CBlockIndex* ancestor = block1 && block2 ? LastCommonAncestor(block1, block2) : nullptr;
            // Using & instead of && below to avoid short circuiting and leaving
            // output uninitialized.
            return FillBlock(ancestor, ancestor_out, lock, active) & FillBlock(block1, block1_out, lock, active) & FillBlock(block2, block2_out, lock, active);
        */
    }
    
    pub fn find_coins(&mut self, coins: &mut HashMap<OutPoint,Coin>)  {
        
        todo!();
        /*
            return FindCoins(m_node, coins);
        */
    }
    
    pub fn guess_verification_progress(&mut self, block_hash: &u256) -> f64 {
        
        todo!();
        /*
            LOCK(cs_main);
            return GuessVerificationProgress(Params().TxData(), chainman().m_blockman.LookupBlockIndex(block_hash));
        */
    }
    
    pub fn has_blocks(&mut self, 
        block_hash: &u256,
        min_height: i32,
        max_height: Option<i32>) -> bool {
        
        todo!();
        /*
            // hasBlocks returns true if all ancestors of block_hash in specified
            // range have block data (are not pruned), false if any ancestors in
            // specified range are missing data.
            //
            // For simplicity and robustness, min_height and max_height are only
            // used to limit the range, and passing min_height that's too low or
            // max_height that's too high will not crash or change the result.
            LOCK(::cs_main);
            if (CBlockIndex* block = chainman().m_blockman.LookupBlockIndex(block_hash)) {
                if (max_height && block->nHeight >= *max_height) block = block->GetAncestor(*max_height);
                for (; block->nStatus & BLOCK_HAVE_DATA; block = block->pprev) {
                    // Check pprev to not segfault if min_height is too low
                    if (block->nHeight <= min_height || !block->pprev) return true;
                }
            }
            return false;
        */
    }
    
    pub fn is_rbf_opt_in(&mut self, tx: &Transaction) -> RBFTransactionState {
        
        todo!();
        /*
            if (!m_node.mempool) return IsRBFOptInEmptyMempool(tx);
            LOCK(m_node.mempool->cs);
            return IsRBFOptIn(tx, *m_node.mempool);
        */
    }
    
    pub fn is_in_mempool(&mut self, txid: &u256) -> bool {
        
        todo!();
        /*
            if (!m_node.mempool) return false;
            LOCK(m_node.mempool->cs);
            return m_node.mempool->exists(GenTxId::Txid(txid));
        */
    }
    
    pub fn has_descendants_in_mempool(&mut self, txid: &u256) -> bool {
        
        todo!();
        /*
            if (!m_node.mempool) return false;
            LOCK(m_node.mempool->cs);
            auto it = m_node.mempool->GetIter(txid);
            return it && (*it)->GetCountWithDescendants() > 1;
        */
    }
    
    pub fn broadcast_transaction(&mut self, 
        tx:         &TransactionRef,
        max_tx_fee: &Amount,
        relay:      bool,
        err_string: &mut String) -> bool {
        
        todo!();
        /*
            const TransactionError err = BroadcastTransaction(m_node, tx, err_string, max_tx_fee, relay, /*wait_callback*/ false);
            // Chain clients only care about failures to accept the tx to the mempool. Disregard non-mempool related failures.
            // Note: this will need to be updated if BroadcastTransactions() is updated to return other non-mempool failures
            // that Chain clients do not need to know about.
            return TransactionError::OK == err;
        */
    }
    
    pub fn get_transaction_ancestry(&mut self, 
        txid:         &u256,
        ancestors:    &mut usize,
        descendants:  &mut usize,
        ancestorsize: *mut usize,
        ancestorfees: *mut Amount)  {
        
        todo!();
        /*
            ancestors = descendants = 0;
            if (!m_node.mempool) return;
            m_node.mempool->GetTransactionAncestry(txid, ancestors, descendants, ancestorsize, ancestorfees);
        */
    }
    
    pub fn get_package_limits(&mut self, 
        limit_ancestor_count:   &mut u32,
        limit_descendant_count: &mut u32)  {
        
        todo!();
        /*
            limit_ancestor_count = gArgs.GetIntArg("-limitancestorcount", DEFAULT_ANCESTOR_LIMIT);
            limit_descendant_count = gArgs.GetIntArg("-limitdescendantcount", DEFAULT_DESCENDANT_LIMIT);
        */
    }
    
    pub fn check_chain_limits(&mut self, tx: &TransactionRef) -> bool {
        
        todo!();
        /*
            if (!m_node.mempool) return true;
            LockPoints lp;
            CTxMemPoolEntry entry(tx, 0, 0, 0, false, 0, lp);
            CTxMemPool::setEntries ancestors;
            auto limit_ancestor_count = gArgs.GetIntArg("-limitancestorcount", DEFAULT_ANCESTOR_LIMIT);
            auto limit_ancestor_size = gArgs.GetIntArg("-limitancestorsize", DEFAULT_ANCESTOR_SIZE_LIMIT) * 1000;
            auto limit_descendant_count = gArgs.GetIntArg("-limitdescendantcount", DEFAULT_DESCENDANT_LIMIT);
            auto limit_descendant_size = gArgs.GetIntArg("-limitdescendantsize", DEFAULT_DESCENDANT_SIZE_LIMIT) * 1000;
            std::string unused_error_string;
            LOCK(m_node.mempool->cs);
            return m_node.mempool->CalculateMemPoolAncestors(
                entry, ancestors, limit_ancestor_count, limit_ancestor_size,
                limit_descendant_count, limit_descendant_size, unused_error_string);
        */
    }
    
    pub fn estimate_smart_fee(&mut self, 
        num_blocks:   i32,
        conservative: bool,
        calc:         *mut FeeCalculation) -> FeeRate {
        
        todo!();
        /*
            if (!m_node.fee_estimator) return {};
            return m_node.fee_estimator->estimateSmartFee(num_blocks, calc, conservative);
        */
    }
    
    pub fn estimate_max_blocks(&mut self) -> u32 {
        
        todo!();
        /*
            if (!m_node.fee_estimator) return 0;
            return m_node.fee_estimator->HighestTargetTracked(FeeEstimateHorizon::LONG_HALFLIFE);
        */
    }
    
    pub fn mempool_min_fee(&mut self) -> FeeRate {
        
        todo!();
        /*
            if (!m_node.mempool) return {};
            return m_node.mempool->GetMinFee(gArgs.GetIntArg("-maxmempool", DEFAULT_MAX_MEMPOOL_SIZE) * 1000000);
        */
    }
    
    pub fn relay_min_fee(&mut self) -> FeeRate {
        
        todo!();
        /*
            return ::minRelayTxFee;
        */
    }
    
    pub fn relay_incremental_fee(&mut self) -> FeeRate {
        
        todo!();
        /*
            return ::incrementalRelayFee;
        */
    }
    
    pub fn relay_dust_fee(&mut self) -> FeeRate {
        
        todo!();
        /*
            return ::dustRelayFee;
        */
    }
    
    pub fn have_pruned(&mut self) -> bool {
        
        todo!();
        /*
            LOCK(cs_main);
            return ::fHavePruned;
        */
    }
    
    pub fn is_ready_to_broadcast(&mut self) -> bool {
        
        todo!();
        /*
            return !::fImporting && !::fReindex && !isInitialBlockDownload();
        */
    }
    
    pub fn is_initial_block_download(&mut self) -> bool {
        
        todo!();
        /*
            return chainman().ActiveChainstate().IsInitialBlockDownload();
        */
    }
    
    pub fn shutdown_requested(&mut self) -> bool {
        
        todo!();
        /*
            return ShutdownRequested();
        */
    }
    
    pub fn get_adjusted_time(&mut self) -> i64 {
        
        todo!();
        /*
            return GetAdjustedTime();
        */
    }
    
    pub fn init_message(&mut self, message: &String)  {
        
        todo!();
        /*
            ::uiInterface.InitMessage(message);
        */
    }
    
    pub fn init_warning(&mut self, message: &BilingualStr)  {
        
        todo!();
        /*
            InitWarning(message);
        */
    }
    
    pub fn init_error(&mut self, message: &BilingualStr)  {
        
        todo!();
        /*
            InitError(message);
        */
    }
    
    pub fn show_progress(&mut self, 
        title:           &String,
        progress:        i32,
        resume_possible: bool)  {
        
        todo!();
        /*
            ::uiInterface.ShowProgress(title, progress, resume_possible);
        */
    }
    
    pub fn handle_notifications(&mut self, notifications: Arc<dyn ChainNotifications>) -> Box<dyn Handler> {
        
        todo!();
        /*
            return std::make_unique<NotificationsHandlerImpl>(std::move(notifications));
        */
    }
    
    pub fn wait_for_notifications_if_tip_changed(&mut self, old_tip: &u256)  {
        
        todo!();
        /*
            if (!old_tip.IsNull()) {
                LOCK(::cs_main);
                const CChain& active = Assert(m_node.chainman)->ActiveChain();
                if (old_tip == active.Tip()->GetBlockHash()) return;
            }
            SyncWithValidationInterfaceQueue();
        */
    }
    
    pub fn handle_rpc(&mut self, command: &RPCCommand) -> Box<dyn Handler> {
        
        todo!();
        /*
            return std::make_unique<RpcHandlerImpl>(command);
        */
    }
    
    pub fn rpc_enable_deprecated(&mut self, method: &String) -> bool {
        
        todo!();
        /*
            return IsDeprecatedRPCEnabled(method);
        */
    }
    
    pub fn rpc_run_later(&mut self, 
        name:    &String,
        fn_:     fn() -> (),
        seconds: i64)  {
        
        todo!();
        /*
            RPCRunLater(name, std::move(fn), seconds);
        */
    }
    
    pub fn rpc_serialization_flags(&mut self) -> i32 {
        
        todo!();
        /*
            return RPCSerializationFlags();
        */
    }
    
    pub fn get_setting(&mut self, name: &String) -> SettingsValue {
        
        todo!();
        /*
            return gArgs.GetSetting(name);
        */
    }
    
    pub fn get_settings_list(&mut self, name: &String) -> Vec<SettingsValue> {
        
        todo!();
        /*
            return gArgs.GetSettingsList(name);
        */
    }
    
    pub fn get_rw_setting(&mut self, name: &String) -> SettingsValue {
        
        todo!();
        /*
            SettingsValue result;
            gArgs.LockSettings([&](const Settings& settings) {
                if (const SettingsValue* value = FindKey(settings.rw_settings, name)) {
                    result = *value;
                }
            });
            return result;
        */
    }
    
    pub fn update_rw_setting(&mut self, 
        name:  &String,
        value: &SettingsValue,
        write: bool) -> bool {
        
        todo!();
        /*
            gArgs.LockSettings([&](Settings& settings) {
                if (value.isNull()) {
                    settings.rw_settings.erase(name);
                } else {
                    settings.rw_settings[name] = value;
                }
            });
            return !write || gArgs.WriteSettingsFile();
        */
    }
    
    pub fn request_mempool_transactions(&mut self, notifications: Rc<RefCell<dyn ChainNotifications>>)  {
        
        todo!();
        /*
            if (!m_node.mempool) return;
            LOCK2(::cs_main, m_node.mempool->cs);
            for (const CTxMemPoolEntry& entry : m_node.mempool->mapTx) {
                notifications.transactionAddedToMempool(entry.GetSharedTx(), 0 /* mempool_sequence */);
            }
        */
    }
    
    pub fn is_taproot_active(&mut self) -> bool {
        
        todo!();
        /*
            LOCK(::cs_main);
            const CBlockIndex* tip = Assert(m_node.chainman)->ActiveChain().Tip();
            return DeploymentActiveAfter(tip, Params().GetConsensus(), consensus::DEPLOYMENT_TAPROOT);
        */
    }
}
