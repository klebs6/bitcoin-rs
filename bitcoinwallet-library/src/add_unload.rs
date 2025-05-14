// ---------------- [ File: bitcoinwallet-library/src/add_unload.rs ]
crate::ix!();

/**
  | Explicitly unload and delete the wallet.
  |
  | Blocks the current thread after signaling the
  | unload intent so that all wallet clients
  | release the wallet.
  |
  | Note that, when blocking is not required, the
  | wallet is implicitly unloaded by the shared
  | pointer deleter.
  */
pub fn unload_wallet(wallet: Arc<Wallet>)  {
    
    todo!();
        /*
        
        */
}

pub fn add_wallet(
    context: &mut WalletContext,
    wallet:  &Arc<Wallet>) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn get_wallets(context: &mut WalletContext) -> Vec<Arc<Wallet>> {
    
    todo!();
        /*
        
        */
}

pub fn get_wallet(
    context: &mut WalletContext,
    name:    &String) -> Arc<Wallet> {

    todo!();
        /*
        
        */
}

pub fn load_wallet(
    context:       &mut WalletContext,
    name:          &String,
    load_on_start: Option<bool>,
    options:       &DatabaseOptions,
    status:        &mut DatabaseStatus,
    error:         &mut BilingualStr,
    warnings:      &mut Vec<BilingualStr>) -> Arc<Wallet> {

    todo!();
        /*
        
        */
}

pub fn create_wallet(
    context:       &mut WalletContext,
    name:          &String,
    load_on_start: Option<bool>,
    options:       &mut DatabaseOptions,
    status:        &mut DatabaseStatus,
    error:         &mut BilingualStr,
    warnings:      &mut Vec<BilingualStr>) -> Arc<Wallet> {

    todo!();
    /*
        
        */
}

pub fn handle_load_wallet(
    context:     &mut WalletContext,
    load_wallet: LoadWalletFn) -> Box<dyn Handler> {
    
    todo!();
        /*
        
        */
}

pub fn make_wallet_database(
    name:    &String,
    options: &DatabaseOptions,
    status:  &mut DatabaseStatus,
    error:   &mut BilingualStr) -> Box<WalletDatabase> {
    
    todo!();
        /*
        
        */
}
