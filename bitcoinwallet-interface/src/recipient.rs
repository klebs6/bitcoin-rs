// ---------------- [ File: bitcoinwallet-interface/src/recipient.rs ]
crate::ix!();

pub struct Recipient {

    script_pub_key:           Script,
    n_amount:                 Amount,
    subtract_fee_from_amount: bool,

    abort_rescan:           AtomicBool, // default = { false }

    /**
      | controlled by WalletRescanReserver
      |
      */
    scanning_wallet:        AtomicBool, // default = { false }

    scanning_start:         Atomic<i64>, // default = { 0 }
    scanning_progress:      Atomic<f64>, // default = { 0 }

    /**
      | The next scheduled rebroadcast of wallet
      | transactions.
      |
      */
    n_next_resend:          i64, // default = 0

    /**
      | Whether this wallet will submit newly
      | created transactions to the node's
      | mempool and prompt rebroadcasts (see
      | ResendWalletTransactions()).
      |
      */
    broadcast_transactions: bool, // default = false


    /**
      | Local time that the tip block was received.
      | Used to schedule wallet rebroadcasts.
      |
      */
    best_block_time:        Atomic<i64>, // default = { 0 }

    //TODO: #[GUARDED_BY(cs_wallet)]
    inner:                  RecipientInner,
}

pub struct RecipientInner {

    /**
      | the current wallet version: clients
      | below this version are not able to load
      | the wallet
      | 
      |
      */
    n_wallet_version:       i32, // default = { FEATURE_BASE }
    master_key:             KeyingMaterial,
}
