// ---------------- [ File: bitcoinwallet-interface/src/callbacks.rs ]
crate::ix!();

pub type WalletStatusChangedFn          = fn() -> ();
pub type WalletUnloadFn                 = fn() -> ();
pub type WalletShowProgressFn           = fn(title: &String, progress: i32) -> ();
pub type WalletTransactionChangedFn     = fn(txid: &u256, status: ChangeType) -> ();
pub type WalletWatchOnlyChangedFn       = fn(have_watch_only: bool) -> ();
pub type WalletCanGetAddressesChangedFn = fn() -> ();

pub type WalletAddressBookChangedFn = fn(
    address: &TxDestination,
    label:   &String,
    is_mine: bool,
    purpose: &String,
    status:  ChangeType
) -> ();
