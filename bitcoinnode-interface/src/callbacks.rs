// ---------------- [ File: bitcoinnode-interface/src/callbacks.rs ]
crate::ix!();

pub type NodeInitMessageFn = fn(message: &String) -> ();

pub type NodeMessageBoxFn = fn(
    message: &BilingualStr,
    caption: &String,
    style:   u32
) -> bool;

pub type NodeQuestionFn = fn(
    message:                 &BilingualStr,
    non_interactive_message: &String,
    caption:                 &String,
    style:                   u32
) -> bool;

pub type NodeShowProgressFn = fn(
    title:           &String,
    progress:        i32,
    resume_possible: bool
) -> ();

pub type NodeNotifyNumConnectionsChangedFn = fn(new_num_connections: i32) -> ();

pub type NodeNotifyNetworkActiveChangedFn = fn(network_active: bool) -> ();

pub type NodeNotifyAlertChangedFn = fn() -> ();

pub type NodeBannedListChangedFn = fn() -> ();

pub type NodeNotifyBlockTipFn = fn(
    _0:                    SynchronizationState,
    tip:                   BlockTip,
    verification_progress: f64
) -> ();

pub type NodeNotifyHeaderTipFn = fn(
    _0:                    SynchronizationState,
    tip:                   BlockTip,
    verification_progress: f64
) -> ();
