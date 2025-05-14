// ---------------- [ File: bitcoinwallet-client/src/client.rs ]
crate::ix!();

/**
  | Wallet chain client that in addition to having
  | chain client methods for starting up, shutting
  | down, and registering RPCs, also has
  | additional methods (called by the GUI) to load
  | and create wallets.
  */
pub trait WalletClient: 
ChainClient 
+ CreateWallet
+ WalletClientLoadWallet
+ GetWalletDir
+ ListWalletDir
+ GetWallets
+ HandleLoadWallet
+ GetWalletContext { }

pub trait GetWalletClient {

    fn get_wallet_client(&mut self) -> Rc<RefCell<dyn WalletClient>>;
}

pub trait CreateWallet {

    /**
      | Create new wallet.
      |
      */
    fn create_wallet(&mut self, 
        name:                  &String,
        passphrase:            &SecureString,
        wallet_creation_flags: u64,
        error:                 &mut BilingualStr,
        warnings:              &mut Vec<BilingualStr>) -> Box<dyn WalletInterface>;
}

pub trait WalletClientLoadWallet {

    /**
      | Load existing wallet.
      |
      */
    fn load_wallet(&mut self, 
        name:     &String,
        error:    &mut BilingualStr,
        warnings: &mut Vec<BilingualStr>) -> Box<dyn WalletInterface>;
}

pub trait GetWalletDir {

    /**
      | Return default wallet directory.
      |
      */
    fn get_wallet_dir(&mut self) -> String;
}

pub trait ListWalletDir {

    /**
      | Return available wallets in wallet
      | directory.
      |
      */
    fn list_wallet_dir(&mut self) -> Vec<String>;
}

pub trait GetWallets {

    /**
      | Return interfaces for accessing wallets
      | (if any).
      |
      */
    fn get_wallets(&mut self) -> Vec<Box<dyn WalletInterface>>;
}

pub trait HandleLoadWallet {

    /**
      | Register handler for load wallet
      | messages. This callback is triggered by
      | createWallet and loadWallet above, and also
      | triggered when wallets are loaded at
      | startup or by RPC.
      */
    fn handle_load_wallet(&mut self, fn_: LoadWalletFn) -> Box<dyn Handler>;
}
