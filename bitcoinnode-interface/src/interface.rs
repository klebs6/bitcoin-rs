crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/interfaces/node.h]

pub trait NodeContextInterface {}

/**
  | Top-level interface for a bitcoin node
  | (bitcoind process).
  |
  */
pub trait NodeInterface:
InitLogging
+ Send
+ Sync
+ InitParameterInteraction
+ GetWarnings
+ GetLogCategories
+ BaseInitialize
+ AppInitMain
+ AppShutdown
+ StartShutdown
+ ShutdownRequested
+ MapPort
+ GetProxy
+ GetNodeCount
+ GetNodesStats
+ UnBan
+ DisconnectByAddress
+ DisconnectById
+ DisconnectOnStall
+ ExternalSigners
+ GetTotalBytesRecv
+ GetTotalBytesSent
+ GetMempoolSize
+ GetMempoolDynamicUsage
+ GetHeaderTip
+ GetNumBlocks
+ GetBestBlockHash
+ GetLastBlockTime
+ GetVerificationProgress
+ IsInitialBlockDownload
+ GetReindex
+ GetImporting
+ SetNetworkActive
+ GetNetworkActive
+ GetDustRelayFee
+ ExecuteRpc
+ ListRpcCommands
+ RpcSetTimerInterfaceIfUnset
+ RpcunsetTimerInterface
+ GetUnspentOutput
+ HandleInitMessage
+ HandleMessageBox
+ NodeHandleQuestion
+ HandleNotifyNumConnectionsChanged
+ HandleNotifyNetworkActiveChanged
+ HandleNotifyAlertChanged
+ HandleBannedListChanged
+ HandleNotifyBlockTip
+ HandleNotifyHeaderTip
+ GetNodeContext
+ SetNodeContext 
+ GetNodeId
+ IsBlockOnlyConn
+ IsInboundConn
+ GetNTimeConnected
+ IsOutboundOrBlockRelayConn
+ IsFullOutboundConn
+ IsManualConn
+ IsFeelerConn
+ IsAddrFetchConn
+ MarkForDisconnect
+ GetServiceRef
+ GetAddrRef
+ GetLocalServices
+ GetLocalNonce
+ GetCommonVersion
+ MarkedForDisconnect
+ HasPermission
+ ExpectServicesFromConn
+ SetSuccessfullyConnected
+ IsSuccessfullyConnected
+ GetTxRelay
+ GetTxRelayMut
+ HasTxRelay
+ NVersion
+ AddKnownTx
+ PongReceived
+ SetCommonVersion
+ SetAddrLocal
+ SetNLastBlockTime
+ SendPaused
+ IsClient
+ SetIsClient
+ IsLimitedNode
+ SetLimitedNode
+ SetNTimeOffset
+ SetNServices
+ SetCleanSubVer
+ SetNVersion
+ SetNLastTxTime
+ SetBip152HighBandwidthFrom
+ SetBip152HighBandwidthTo
+ DecrementNProcessQueueSize
+ GetNProcessQueueSize
+ SetPauseRecv
+ LockVProcessMsg
+ ConnType
+ NTimeConnected
+ MinPingTime
+ NLastBlockTime
+ NLastTxTime
+ NServices
+ NKeyedNetGroup
+ PreferEvict
+ ConnectedThroughNetwork
+ SuccessfullyConnected
+ NodeInterfaceAddRef
+ SetPermissionFlags
+ SetPreferEvict
+ ReleaseGrantOutbound
+ CloseSocketDisconnect
+ Release
+ GetRefCount
+ AddrName
+ PauseRecv
+ LockVSend
+ LockHSocket
+ CopyStats
+ AddrBind
+ NLastRecv
+ NLastSend
+ GrantOutbound
+ GetTransportSerializer
+ GetTransportSerializerMut
+ SetPauseSend
+ ReceiveMsgBytes
+ LockRecvMsg
+ IncrementNProcessQueueSize
+ NProcessQueueSize
+ LockSendProcessing
+ UnlockSendProcessing
+ PushTxInventory
+ HandleShowProgress<Callback = NodeShowProgressFn>
+ GetWalletClient
{ }
