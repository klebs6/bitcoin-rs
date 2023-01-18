crate::ix!();

/**
  | Implement this to subscribe to events
  | generated in validation
  | 
  | Each CValidationInterface() subscriber
  | will receive event callbacks in the
  | order in which the events were generated
  | by validation.
  | 
  | Furthermore, each ValidationInterface()
  | subscriber may assume that callbacks
  | effectively run in a single thread with
  | single-threaded memory consistency.
  | That is, for a given ValidationInterface()
  | instantiation, each callback will
  | complete before the next one is invoked.
  | This means, for example when a block
  | is connected that the
  | 
  | UpdatedBlockTip() callback may depend
  | on an operation performed in the BlockConnected()
  | callback without worrying about explicit
  | synchronization. No ordering should
  | be assumed across
  | 
  | ValidationInterface() subscribers.
  |
  */
pub trait ValidationInterface:
UpdatedBlockTip
+ TransactionAddedToMempool
+ TransactionRemovedFromMempool
+ BlockConnected
+ BlockDisconnected
+ ChainStateFlushed
+ BlockChecked
+ NewPoWValidBlock { }
