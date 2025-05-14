// ---------------- [ File: bitcoinnode-interface/src/traits.rs ]
crate::ix!();

pub trait InitLogging {

    /**
      | Init logging.
      |
      */
    fn init_logging(&mut self);
}

pub trait InitParameterInteraction {

    /**
      | Init parameter interaction.
      |
      */
    fn init_parameter_interaction(&mut self);
}

pub trait GetWarnings {

    /**
      | Get warnings.
      |
      */
    fn get_warnings(&mut self) -> BilingualStr;
}

pub trait GetLogCategories {

    /**
      | Get log flags.
      |
      */
    fn get_log_categories(&mut self) -> u32;
}

pub trait BaseInitialize {

    /**
      | Initialize app dependencies.
      |
      */
    fn base_initialize(&mut self) -> bool;
}

pub trait AppInitMain {

    /**
      | Start node.
      |
      */
    fn app_init_main(&mut self, tip_info: *mut BlockAndHeaderTipInfo) -> bool;
}

pub trait AppShutdown {

    /**
      | Stop node.
      |
      */
    fn app_shutdown(&mut self);
}

pub trait StartShutdown {

    /**
      | Start shutdown.
      |
      */
    fn start_shutdown(&mut self);
}

pub trait ShutdownRequested {

    /**
      | Return whether shutdown was requested.
      |
      */
    fn shutdown_requested(&mut self) -> bool;
}

pub trait MapPort {

    fn map_port(&mut self, 
            use_upnp:   bool,
            use_natpmp: bool);
}

pub trait GetProxy {

    fn get_proxy(&mut self, 
        net:        Network,
        proxy_info: &mut ProxyType) -> bool;
}

pub trait GetNodeCount {

    /**
      | Get number of connections.
      |
      */
    fn get_node_count(&mut self, flags: ConnectionDirection) -> usize;
}

pub trait GetNodesStats {

    /**
      | Get stats for connected nodes.
      |
      */
    fn get_nodes_stats(&mut self, stats: &mut NodesStats) -> bool;
}

pub trait GetBanned {

    /**
      | Get ban map entries.
      |
      */
    fn get_banned(&mut self, banmap: &mut BanMap) -> bool;
}

pub trait Ban {

    /**
      | Ban node.
      |
      */
    fn ban(&mut self, 
            net_addr:        &NetAddr,
            ban_time_offset: i64) -> bool;
}

pub trait UnBan {

    /**
      | Unban node.
      |
      */
    fn unban(&mut self, ip: &SubNet) -> bool;
}

pub trait DisconnectByAddress {

    /**
      | Disconnect node by address.
      |
      */
    fn disconnect_by_address(&mut self, net_addr: &NetAddr) -> bool;
}

pub trait DisconnectById {

    /**
      | Disconnect node by id.
      |
      */
    fn disconnect_by_id(&mut self, id: NodeId) -> bool;
}

pub trait DisconnectOnStall {

    fn disconnect_on_stall(&self);
}

pub trait ExternalSigners {

    /**
      | List external signers
      |
      */
    fn external_signers(&mut self) -> Vec<ExternalSigner>;
}

pub trait GetTotalBytesRecv {

    /**
      | Get total bytes recv.
      |
      */
    fn get_total_bytes_recv(&mut self) -> i64;
}

pub trait GetTotalBytesSent {

    /**
      | Get total bytes sent.
      |
      */
    fn get_total_bytes_sent(&mut self) -> i64;
}

pub trait GetMempoolSize {

    fn get_mempool_size(&mut self) -> usize;
}

pub trait GetMempoolDynamicUsage {

    fn get_mempool_dynamic_usage(&mut self) -> usize;
}

pub trait GetHeaderTip {

    /**
      | Get header tip height and time.
      |
      */
    fn get_header_tip(&mut self, 
            height:     &mut i32,
            block_time: &mut i64) -> bool;
}

pub trait GetNumBlocks {

    fn get_num_blocks(&mut self) -> i32;
}

pub trait GetBestBlockHash {

    fn get_best_block_hash(&mut self) -> u256;
}

pub trait GetLastBlockTime {

    fn get_last_block_time(&mut self) -> i64;
}

pub trait GetVerificationProgress {

    fn get_verification_progress(&mut self) -> f64;
}

pub trait GetReindex {

    fn get_reindex(&mut self) -> bool;
}

pub trait GetImporting {

    fn get_importing(&mut self) -> bool;
}

pub trait SetNetworkActive {

    fn set_network_active(&mut self, active: bool);
}

pub trait GetNetworkActive {

    fn get_network_active(&mut self) -> bool;
}

pub trait GetDustRelayFee {
    fn get_dust_relay_fee(&mut self) -> FeeRate;
}

pub trait ExecuteRpc {

    fn execute_rpc(&mut self, 
            command: &String,
            params:  &UniValue,
            uri:     &String) -> UniValue;
}

pub trait ListRpcCommands {

    fn list_rpc_commands(&mut self) -> Vec<String>;
}

pub trait RpcSetTimerInterfaceIfUnset {

    fn rpc_set_timer_interface_if_unset<'a>(&mut self, iface: &'a mut dyn RPCTimerInterface);
}

pub trait RpcunsetTimerInterface {

    fn rpc_unset_timer_interface<'a>(&mut self, iface: &'a mut dyn RPCTimerInterface);
}

pub trait GetUnspentOutput {

    /**
      | Get unspent outputs associated with
      | a transaction.
      |
      */
    fn get_unspent_output(&mut self, 
            output: &OutPoint,
            coin:   &mut Coin) -> bool;
}

pub trait HandleInitMessage {

    /**
      | Register handler for init messages.
      |
      */
    fn handle_init_message(&mut self, fn_: NodeInitMessageFn) -> Box<dyn Handler>;
}

pub trait HandleMessageBox {

    /**
      | Register handler for message box messages.
      |
      */
    fn handle_message_box(&mut self, fn_: NodeMessageBoxFn) -> Box<dyn Handler>;
}

pub trait NodeHandleQuestion {

    /**
      | Register handler for question messages.
      |
      */
    fn handle_question(&mut self, fn_: NodeQuestionFn) -> Box<dyn Handler>;
}

pub trait HandleNotifyNumConnectionsChanged {

    /**
      | Register handler for number of connections
      | changed messages.
      |
      */
    fn handle_notify_num_connections_changed(&mut self, fn_: NodeNotifyNumConnectionsChangedFn) -> Box<dyn Handler>;
}

pub trait HandleNotifyNetworkActiveChanged {

    /**
      | Register handler for network active
      | messages.
      |
      */
    fn handle_notify_network_active_changed(&mut self, fn_: NodeNotifyNetworkActiveChangedFn) -> Box<dyn Handler>;
}

pub trait HandleNotifyAlertChanged {

    /**
      | Register handler for notify alert messages.
      |
      */
    fn handle_notify_alert_changed(&mut self, fn_: NodeNotifyAlertChangedFn) -> Box<dyn Handler>;
}

pub trait HandleBannedListChanged {

    /**
      | Register handler for ban list messages.
      |
      */
    fn handle_banned_list_changed(&mut self, fn_: NodeBannedListChangedFn) -> Box<dyn Handler>;
}

pub trait HandleNotifyBlockTip {

    /**
      | Register handler for block tip messages.
      |
      */
    fn handle_notify_block_tip(&mut self, fn_: NodeNotifyBlockTipFn) -> Box<dyn Handler>;
}

pub trait HandleNotifyHeaderTip {

    /**
      | Register handler for header tip messages.
      |
      */
    fn handle_notify_header_tip(&mut self, fn_: NodeNotifyHeaderTipFn) -> Box<dyn Handler>;
}

pub trait GetNodeContext {

    /**
      | Get and set internal node context. Useful
      | for testing, but not accessible across
      | processes.
      |
      */
    fn context(&mut self) -> *mut dyn NodeContextInterface {
        
        todo!();
        /*
            return nullptr;
        */
    }
}

pub trait SetNodeContext {

    fn set_context(&mut self, context: *mut dyn NodeContextInterface)  {
        
        todo!();
        /*
        
        */
    }
}

pub trait GetNodeId {

    fn get_id(&self) -> NodeId;
}

pub trait IsBlockOnlyConn {

    fn is_block_only_conn(&self) -> bool;
}

pub trait IsInboundConn {

    fn is_inbound_conn(&self) -> bool;
}

pub trait GetNTimeConnected {

    fn get_n_time_connected(&self) -> OffsetDateTime;
}

pub trait IsOutboundOrBlockRelayConn {

    fn is_outbound_or_block_relay_conn(&self) -> bool;
}

pub trait IsFullOutboundConn {

    fn is_full_outbound_conn(&self) -> bool;
}

pub trait IsManualConn {

    fn is_manual_conn(&self) -> bool;
}

pub trait IsFeelerConn {

    fn is_feeler_conn(&self) -> bool;
}

pub trait IsAddrFetchConn {

    fn is_addr_fetch_conn(&self) -> bool;
}

pub trait MarkForDisconnect {

    /**
      | Disconnect the handler.
      |
      */
    fn mark_for_disconnect(&self);
}

pub trait GetLocalServices {

    fn get_local_services(&self) -> ServiceFlags;
}

pub trait GetLocalNonce {

    fn get_local_nonce(&self) -> u64;
}

pub trait GetCommonVersion {

    fn get_common_version(&self) -> i32;
}

pub trait MarkedForDisconnect {

    fn marked_for_disconnect(&self) -> bool;
}

pub trait HasPermission {

    fn has_permission(&self, permission: NetPermissionFlags) -> bool;
}

pub trait ExpectServicesFromConn {

    fn expect_services_from_conn(&self) -> bool;
}

pub trait SetSuccessfullyConnected {

    fn set_successfully_connected(&self, val: bool);
}

pub trait IsSuccessfullyConnected {

    fn is_successfully_connected(&self) -> bool;
}

pub trait NVersion {

    fn n_version(&self) -> i32;
}

pub trait GetTxRelay {

    fn get_tx_relay(&self) -> AmoReadGuard<NodeTxRelay>;
}

pub trait GetTxRelayMut {

    fn get_tx_relay_mut(&self) -> AmoWriteGuard<NodeTxRelay>;
}

pub trait HasTxRelay {

    fn has_tx_relay(&self) -> bool;
}

pub trait AddKnownTx {

    fn add_known_tx(&mut self, hash: &u256);
}

pub trait PongReceived {

    fn pong_received(&mut self, ping_time: Duration /* micros */);
}

pub trait SetCommonVersion {

    fn set_common_version(&mut self, greatest_common_version: i32);
}

pub trait SetAddrLocal {

    fn set_addr_local(&mut self, addr_local_in: &Service);
}

pub trait SetNLastBlockTime {

    fn set_n_last_block_time(&mut self, x: Option<OffsetDateTime>);
}

pub trait SendPaused {

    fn send_paused(&self) -> bool;
}

pub trait IsClient {

    fn is_client(&self) -> bool;
}

pub trait SetIsClient {

    fn set_is_client(&mut self, x: bool);
}

pub trait IsLimitedNode {

    fn is_limited_node(&self) -> bool;
}

pub trait SetLimitedNode {

    fn set_limited_node(&mut self, x: bool);
}

pub trait SetNTimeOffset {

    fn set_n_time_offset(&mut self, x: Option<Duration>);
}

pub trait SetNServices {

    fn set_n_services(&mut self, x: ServiceFlags);
}

pub trait SetCleanSubVer {

    fn set_clean_sub_ver(&mut self, x: &str);
}

pub trait SetNVersion {

    fn set_n_version(&mut self, x: i32);
}

pub trait SetNLastTxTime {

    fn set_n_last_tx_time(&mut self, x: Option<OffsetDateTime>);
}

pub trait SetBip152HighBandwidthFrom {

    fn set_bip152_highbandwidth_from(&mut self, x: bool);
}

pub trait SetBip152HighBandwidthTo {

    fn set_bip152_highbandwidth_to(&mut self, x: bool);
}

pub trait DecrementNProcessQueueSize {

    fn decrement_n_process_queue_size(&self, val: usize);
}

pub trait GetNProcessQueueSize {

    fn get_n_process_queue_size(&self) -> usize;
}

pub trait SetPauseRecv {

    fn set_pause_recv(&self, x: bool);
}

pub trait LockVProcessMsg {

    fn lock_v_process_msg(&self) -> MutexGuard<NodeVProcessMsg>;
}

//-------------------------------------
pub trait ConnType {

    fn conn_type(&self) -> Option<ConnectionType>;
}

pub trait NTimeConnected {

    fn n_time_connected(&self) -> Option<OffsetDateTime>;
}

pub trait MinPingTime {

    fn min_ping_time(&self) -> Option<Duration>;
}

pub trait NLastBlockTime { 

    fn n_last_block_time(&self) -> Option<OffsetDateTime>; 
}

pub trait NLastTxTime {

    fn n_last_tx_time(&self) -> Option<OffsetDateTime>; 
}

pub trait NServices { 

    fn n_services(&self) -> ServiceFlags; 
}

pub trait NKeyedNetGroup { 

    fn n_keyed_net_group(&self) -> u64; 
}

pub trait PreferEvict { 

    fn prefer_evict(&self) -> bool; 
}

pub trait ConnectedThroughNetwork { 

    fn connected_through_network(&self) -> Network;
}

pub trait SuccessfullyConnected { 

    fn successfully_connected(&self) -> bool; 
}

pub trait NodeInterfaceAddRef { 

    fn add_ref(&mut self) -> *mut dyn NodeInterface;
}

pub trait SetPermissionFlags { 

    fn set_permission_flags(&mut self, x: NetPermissionFlags); 
}

pub trait SetPreferEvict { 

    fn set_prefer_evict(&mut self, x: bool); 
}

pub trait ReleaseGrantOutbound { 

    fn release_grant_outbound(&mut self); 
}

pub trait CloseSocketDisconnect { 

    fn close_socket_disconnect(&mut self);
}

pub trait Release { 

    fn release(&mut self); 
}

pub trait GetRefCount { 

    fn get_ref_count(&self) -> i32;
}

pub trait AddrName { 

    fn addr_name(&self) -> &str; 
}

pub trait PauseRecv { 

    fn pause_recv(&self) -> bool; 
}

pub trait LockVSend { 

    fn lock_v_send(&self) -> MutexGuard<NodeVSend>; 
}

pub trait LockHSocket { 

    fn lock_h_socket(&self) -> MutexGuard<NodeHSocket>; 
}

pub trait CopyStats { 

    fn copy_stats(&mut self, stats: &mut NodeStats);
}

pub trait AddrBind { 

    fn addr_bind(&self) -> &Address; 
}

pub trait NLastRecv { 

    fn n_last_recv(&self) -> Option<OffsetDateTime>; 
}

pub trait NLastSend { 

    fn n_last_send(&self) -> Option<OffsetDateTime>; 
}

pub trait GrantOutbound { 

    fn grant_outbound(&self) -> SemaphoreGrant; 
}

pub trait GetTransportSerializer { 

    fn get_transport_serializer(&self) -> &Box<dyn TransportSerializer>; 
}

pub trait GetTransportSerializerMut { 

    fn get_transport_serializer_mut(&mut self) -> &mut Box<dyn TransportSerializer>; 
}

pub trait SetPauseSend { 

    fn set_pause_send(&self, x: bool); 
}

pub trait ReceiveMsgBytes { 

    fn receive_msg_bytes(&mut self, 
        msg_bytes: &[u8],
        complete:  &mut bool) -> bool;
}

pub trait LockRecvMsg { 

    fn lock_recv_msg(&self) -> MutexGuard<Vec<NetMessage>>; 
}

pub trait IncrementNProcessQueueSize { 

    fn increment_n_process_queue_size(&self, x: usize); 
}

pub trait NProcessQueueSize { 

    fn n_process_queue_size(&self) -> usize; 
}

pub trait LockSendProcessing { 

    unsafe fn lock_send_processing(&self); 
}

pub trait UnlockSendProcessing { 

    unsafe fn unlock_send_processing(&self); 
}

pub trait PushTxInventory {

    fn push_tx_inventory(&mut self, hash: &u256);
}
