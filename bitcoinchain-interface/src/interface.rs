crate::ix!();

/**
  | Interface giving clients (wallet processes,
  | maybe other analysis tools in the future)
  | ability to access to the chain state, receive
  | notifications, estimate fees, and submit
  | transactions.
  |
  | TODO: Current chain methods are too low level,
  | exposing too much of the internal workings of
  | the bitcoin node, and not being very
  | convenient to use.  Chain methods should be
  | cleaned up and simplified over time. Examples:
  |
  | * The initMessages() and showProgress()
  |   methods which the wallet uses to send
  |   notifications to the GUI should go away when
  |   GUI and wallet can directly communicate with
  |   each other without going through the node
  |   (https://github.com/bitcoin/bitcoin/pull/15288#discussion_r253321096).
  |
  | * The handleRpc, registerRpcs,
  |   rpcEnableDeprecated methods and other RPC
  |   methods can go away if wallets listen for
  |   HTTP requests on their own ports instead of
  |   registering to handle requests on the node
  |   HTTP port.
  |
  | * Move fee estimation queries to an
  |   asynchronous interface and let the wallet
  |   cache it, fee estimation being driven by
  |   node mempool, wallet should be the consumer.
  |
  | * `guessVerificationProgress` and similar
  |   methods can go away if rescan logic moves
  |   out of the wallet, and the wallet just
  |   requests scans from the node
  |   (https://github.com/bitcoin/bitcoin/issues/11756)
  */
pub trait ChainInterface:
ChainHeight
+ GetBlockHash
+ HaveBlockOnDisk
+ Tip
+ Contains<Arc<BlockIndex>>
+ GetLocator<Arc<BlockIndex>, LocatorType = BlockLocator>
+ GetTipLocator
+ FindLocatorFork
+ CheckFinalTx
+ FindBlock
+ FindFirstBlockWithTimeAndHeight
+ FindAncestorByHeight
+ FindAncestorByHash
+ FindCommonAncestor
+ FindCoins
+ GuessVerificationProgress
+ HasBlocks
+ IsRBFOptIn
+ IsInMempool
+ HasDescendantsInMempool
+ BroadcastTransaction
+ GetTransactionAncestry
+ GetPackageLimits
+ CheckChainLimits
+ EstimateSmartFee
+ EstimateMaxBlocks
+ MemPoolMinFee
+ RelayMinFee
+ RelayIncrementalFee
+ RelayDustFee
+ HavePruned
+ IsReadyToBroadcast
+ IsInitialBlockDownload
+ ShutdownRequested
+ GetAdjustedTime
+ InitMessage
+ InitWarning
+ InitError
+ ShowProgress
+ HandleNotifications
+ WaitForNotificationsIfTipChanged
+ HandleRpc
+ RpcEnableDeprecated
+ RpcRunLater
+ RpcSerializationFlags
+ GetSetting
+ GetSettingsList
+ GetRwSetting
+ UpdateRwSetting
+ RequestMempoolTransactions
+ IsTaprootActive
+ ChainNext
+ Index<i32, Output = Option<Arc<BlockIndex>>>
{}
