crate::ix!();

pub trait Initialize {
    fn initialize(&mut self, pcontext: *mut c_void) -> bool;
}

pub trait Shutdown {
    fn shutdown(&mut self);
}

pub trait NotifyBlock {

    /**
      | Notifies of ConnectTip result, i.e.,
      | new active tip only
      |
      */
    fn notify_block(&mut self, pindex: *const BlockIndex) -> bool;
}

pub trait NotifyBlockConnect {

    /**
      | Notifies of every block connection
      |
      */
    fn notify_block_connect(&mut self, pindex: *const BlockIndex) -> bool;
}

pub trait NotifyBlockDisconnect {

    /**
      | Notifies of every block disconnection
      |
      */
    fn notify_block_disconnect(&mut self, pindex: *const BlockIndex) -> bool;
}

pub trait NotifyTransactionAcceptance {

    /**
      | Notifies of every mempool acceptance
      |
      */
    fn notify_transaction_acceptance(&mut self, 
            transaction:      &Transaction,
            mempool_sequence: u64) -> bool;
}

pub trait NotifyTransactionRemoval {

    /**
      | Notifies of every mempool removal,
      | except inclusion in blocks
      |
      */
    fn notify_transaction_removal(&mut self, 
            transaction:      &Transaction,
            mempool_sequence: u64) -> bool;
}

pub trait NotifyTransaction {

    /**
      | Notifies of transactions added to mempool
      | or appearing in blocks
      |
      */
    fn notify_transaction(&mut self, transaction: &Transaction) -> bool;
}
