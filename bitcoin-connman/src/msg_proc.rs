// ---------------- [ File: bitcoin-connman/src/msg_proc.rs ]
crate::ix!();

pub struct ConnmanMsgProc {

    /**
      | flag for waking the message processor.
      | 
      |
      */
    pub msg_proc_wake: AtomicBool,
}
