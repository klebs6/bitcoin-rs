// ---------------- [ File: bitcoin-node/src/node_impl.rs ]
crate::ix!();

pub struct NodeImpl {
    base:    Node,
    context: *mut dyn NodeContextInterface, // default = { nullptr }
}

impl NodeImpl {

    pub fn chainman(&mut self) -> &mut ChainstateManager {
        
        todo!();
        /*
            return *Assert(m_context->chainman);
        */
    }
    
    pub fn new(context: &mut dyn NodeContextInterface) -> Self {
    
        todo!();
        /*


            setContext(&context);
        */
    }
    
    pub fn init_logging(&mut self)  {
        
        todo!();
        /*
            InitLogging(*Assert(m_context->args));
        */
    }
    
    pub fn init_parameter_interaction(&mut self)  {
        
        todo!();
        /*
            InitParameterInteraction(*Assert(m_context->args));
        */
    }
    
    pub fn get_warnings(&mut self) -> BilingualStr {
        
        todo!();
        /*
            return GetWarnings(true);
        */
    }
    
    pub fn get_log_categories(&mut self) -> u32 {
        
        todo!();
        /*
            return LogInstance().GetCategoryMask();
        */
    }
    
    pub fn base_initialize(&mut self) -> bool {
        
        todo!();
        /*
            return AppInitBasicSetup(gArgs) && AppInitParameterInteraction(gArgs) && AppInitSanityChecks() &&
                   AppInitLockDataDirectory() && AppInitInterfaces(*m_context);
        */
    }
    
    pub fn app_init_main(&mut self, tip_info: *mut BlockAndHeaderTipInfo) -> bool {
        
        todo!();
        /*
            return AppInitMain(*m_context, tip_info);
        */
    }
    
    pub fn app_shutdown(&mut self)  {
        
        todo!();
        /*
            Interrupt(*m_context);
            Shutdown(*m_context);
        */
    }
    
    pub fn start_shutdown(&mut self)  {
        
        todo!();
        /*
            StartShutdown();
            // Stop RPC for clean shutdown if any of waitfor* commands is executed.
            if (gArgs.GetBoolArg("-server", false)) {
                InterruptRPC();
                StopRPC();
            }
        */
    }
    
    pub fn shutdown_requested(&mut self) -> bool {
        
        todo!();
        /*
            return ShutdownRequested();
        */
    }
    
    pub fn map_port(&mut self, 
        use_upnp:   bool,
        use_natpmp: bool)  {
        
        todo!();
        /*
            StartMapPort(use_upnp, use_natpmp);
        */
    }
    
    pub fn get_proxy(&mut self, 
        net:        Network,
        proxy_info: &mut ProxyType) -> bool {
        
        todo!();
        /*
            return GetProxy(net, proxy_info);
        */
    }
    
    pub fn get_node_count(&mut self, flags: ConnectionDirection) -> usize {
        
        todo!();
        /*
            return m_context->connman ? m_context->connman->GetNodeCount(flags) : 0;
        */
    }
    
    pub fn get_nodes_stats(&mut self, stats: &mut NodesStats) -> bool {
        
        todo!();
        /*
            stats.clear();

            if (m_context->connman) {
                std::vector<NodeStats> stats_temp;
                m_context->connman->GetNodeStats(stats_temp);

                stats.reserve(stats_temp.size());
                for (auto& node_stats_temp : stats_temp) {
                    stats.emplace_back(std::move(node_stats_temp), false, NodeStateStats());
                }

                // Try to retrieve the NodeStateStats for each node.
                if (m_context->peerman) {
                    TRY_LOCK(::cs_main, lockMain);
                    if (lockMain) {
                        for (auto& node_stats : stats) {
                            std::get<1>(node_stats) =
                                m_context->peerman->GetNodeStateStats(std::get<0>(node_stats).nodeid, std::get<2>(node_stats));
                        }
                    }
                }
                return true;
            }
            return false;
        */
    }
    
    pub fn get_banned(&mut self, banmap: &mut BanMap) -> bool {
        
        todo!();
        /*
            if (m_context->banman) {
                m_context->banman->GetBanned(banmap);
                return true;
            }
            return false;
        */
    }
    
    pub fn ban(&mut self, 
        net_addr:        &NetAddr,
        ban_time_offset: i64) -> bool {
        
        todo!();
        /*
            if (m_context->banman) {
                m_context->banman->Ban(net_addr, ban_time_offset);
                return true;
            }
            return false;
        */
    }
    
    pub fn unban(&mut self, ip: &SubNet) -> bool {
        
        todo!();
        /*
            if (m_context->banman) {
                m_context->banman->Unban(ip);
                return true;
            }
            return false;
        */
    }
    
    pub fn disconnect_by_address(&mut self, net_addr: &NetAddr) -> bool {
        
        todo!();
        /*
            if (m_context->connman) {
                return m_context->connman->DisconnectNode(net_addr);
            }
            return false;
        */
    }
    
    pub fn disconnect_by_id(&mut self, id: NodeId) -> bool {
        
        todo!();
        /*
            if (m_context->connman) {
                return m_context->connman->DisconnectNode(id);
            }
            return false;
        */
    }
    
    pub fn external_signers(&mut self) -> Vec<ExternalSigner> {
        
        todo!();
        /*
            #ifdef ENABLE_EXTERNAL_SIGNER
            std::vector<ExternalSigner> signers = {};
            const std::string command = gArgs.GetArg("-signer", "");
            if (command == "") return signers;
            ExternalSigner::Enumerate(command, signers, Params().NetworkIDString());
            return signers;
    #else
            // This result is indistinguishable from a successful call that returns
            // no signers. For the current GUI this doesn't matter, because the wallet
            // creation dialog disables the external signer checkbox in both
            // cases. The return type could be changed to std::optional<std::vector>
            // (or something that also includes error messages) if this distinction
            // becomes important.
            return {};
    #endif // ENABLE_EXTERNAL_SIGNER
        */
    }
    
    pub fn get_total_bytes_recv(&mut self) -> i64 {
        
        todo!();
        /*
            return m_context->connman ? m_context->connman->GetTotalBytesRecv() : 0;
        */
    }
    
    pub fn get_total_bytes_sent(&mut self) -> i64 {
        
        todo!();
        /*
            return m_context->connman ? m_context->connman->GetTotalBytesSent() : 0;
        */
    }
    
    pub fn get_mempool_size(&mut self) -> usize {
        
        todo!();
        /*
            return m_context->mempool ? m_context->mempool->size() : 0;
        */
    }
    
    pub fn get_mempool_dynamic_usage(&mut self) -> usize {
        
        todo!();
        /*
            return m_context->mempool ? m_context->mempool->DynamicMemoryUsage() : 0;
        */
    }
    
    pub fn get_header_tip(&mut self, 
        height:     &mut i32,
        block_time: &mut i64) -> bool {
        
        todo!();
        /*
            LOCK(::cs_main);
            if (::pindexBestHeader) {
                height = ::pindexBestHeader->nHeight;
                block_time = ::pindexBestHeader->GetBlockTime();
                return true;
            }
            return false;
        */
    }
    
    pub fn get_num_blocks(&mut self) -> i32 {
        
        todo!();
        /*
            LOCK(::cs_main);
            return chainman().ActiveChain().Height();
        */
    }
    
    pub fn get_best_block_hash(&mut self) -> u256 {
        
        todo!();
        /*
            const CBlockIndex* tip = 
        [&]() { LOCK(::cs_main);  return chainman().ActiveChain().Tip() }()
        ;
            return tip ? tip->GetBlockHash() : Params().GenesisBlock().GetHash();
        */
    }
    
    pub fn get_last_block_time(&mut self) -> i64 {
        
        todo!();
        /*
            LOCK(::cs_main);
            if (chainman().ActiveChain().Tip()) {
                return chainman().ActiveChain().Tip()->GetBlockTime();
            }
            return Params().GenesisBlock().GetBlockTime(); // Genesis block's time of current network
        */
    }
    
    pub fn get_verification_progress(&mut self) -> f64 {
        
        todo!();
        /*
            const CBlockIndex* tip;
            {
                LOCK(::cs_main);
                tip = chainman().ActiveChain().Tip();
            }
            return GuessVerificationProgress(Params().TxData(), tip);
        */
    }
    
    pub fn is_initial_block_download(&mut self) -> bool {
        
        todo!();
        /*
            return chainman().ActiveChainstate().IsInitialBlockDownload();
        */
    }
    
    pub fn get_reindex(&mut self) -> bool {
        
        todo!();
        /*
            return ::fReindex;
        */
    }
    
    pub fn get_importing(&mut self) -> bool {
        
        todo!();
        /*
            return ::fImporting;
        */
    }
    
    pub fn set_network_active(&mut self, active: bool)  {
        
        todo!();
        /*
            if (m_context->connman) {
                m_context->connman->SetNetworkActive(active);
            }
        */
    }
    
    pub fn get_network_active(&mut self) -> bool {
        
        todo!();
        /*
            return m_context->connman && m_context->connman->GetNetworkActive();
        */
    }
    
    pub fn get_dust_relay_fee(&mut self) -> FeeRate {
        
        todo!();
        /*
            return ::dustRelayFee;
        */
    }
    
    pub fn execute_rpc(&mut self, 
        command: &String,
        params:  &UniValue,
        uri:     &String) -> UniValue {
        
        todo!();
        /*
            JSONRPCRequest req;
            req.context = m_context;
            req.params = params;
            req.strMethod = command;
            req.URI = uri;
            return ::tableRPC.execute(req);
        */
    }
    
    pub fn list_rpc_commands(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return ::tableRPC.listCommands();
        */
    }
    
    pub fn rpc_set_timer_interface_if_unset<'a>(&mut self, iface: &'a mut dyn RPCTimerInterface)  {
        
        todo!();
        /*
            RPCSetTimerInterfaceIfUnset(iface);
        */
    }
    
    pub fn rpc_unset_timer_interface<'a>(&mut self, iface: &'a mut dyn RPCTimerInterface)  {
        
        todo!();
        /*
            RPCUnsetTimerInterface(iface);
        */
    }
    
    pub fn get_unspent_output(&mut self, 
        output: &OutPoint,
        coin:   &mut Coin) -> bool {
        
        todo!();
        /*
            LOCK(::cs_main);
            return chainman().ActiveChainstate().CoinsTip().GetCoin(output, coin);
        */
    }
    
    pub fn wallet_client(&mut self) -> Rc<RefCell<dyn WalletClient>> {
        
        todo!();
        /*
            return *Assert(m_context->wallet_client);
        */
    }
    
    pub fn handle_init_message(&mut self, fn_: NodeInitMessageFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.InitMessage_connect(fn));
        */
    }
    
    pub fn handle_message_box(&mut self, fn_: NodeMessageBoxFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.ThreadSafeMessageBox_connect(fn));
        */
    }
    
    pub fn handle_question(&mut self, fn_: NodeQuestionFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.ThreadSafeQuestion_connect(fn));
        */
    }
    
    pub fn handle_show_progress(&mut self, fn_: NodeShowProgressFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.ShowProgress_connect(fn));
        */
    }
    
    pub fn handle_notify_num_connections_changed(&mut self, fn_: NodeNotifyNumConnectionsChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.NotifyNumConnectionsChanged_connect(fn));
        */
    }
    
    pub fn handle_notify_network_active_changed(&mut self, fn_: NodeNotifyNetworkActiveChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.NotifyNetworkActiveChanged_connect(fn));
        */
    }
    
    pub fn handle_notify_alert_changed(&mut self, fn_: NodeNotifyAlertChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.NotifyAlertChanged_connect(fn));
        */
    }
    
    pub fn handle_banned_list_changed(&mut self, fn_: NodeBannedListChangedFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.BannedListChanged_connect(fn));
        */
    }
    
    pub fn handle_notify_block_tip(&mut self, fn_: NodeNotifyBlockTipFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(::uiInterface.NotifyBlockTip_connect([fn](SynchronizationState sync_state, const CBlockIndex* block) {
                fn(sync_state, BlockTip{block->nHeight, block->GetBlockTime(), block->GetBlockHash()},
                    GuessVerificationProgress(Params().TxData(), block));
            }));
        */
    }
    
    pub fn handle_notify_header_tip(&mut self, fn_: NodeNotifyHeaderTipFn) -> Box<dyn Handler> {
        
        todo!();
        /*
            return MakeHandler(
                ::uiInterface.NotifyHeaderTip_connect([fn](SynchronizationState sync_state, const CBlockIndex* block) {
                    fn(sync_state, BlockTip{block->nHeight, block->GetBlockTime(), block->GetBlockHash()},
                        /* verification progress is unused when a header was received */ 0);
                }));
        */
    }
    
    pub fn context(&mut self) -> *mut dyn NodeContextInterface {
        
        todo!();
        /*
            return m_context;
        */
    }
    
    pub fn set_context(&mut self, context: *mut dyn NodeContextInterface)  {
        
        todo!();
        /*
            m_context = context;
        */
    }
}
