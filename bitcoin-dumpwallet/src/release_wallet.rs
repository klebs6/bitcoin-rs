// ---------------- [ File: bitcoin-dumpwallet/src/release_wallet.rs ]
crate::ix!();

/**
  | The standard wallet deleter function blocks on
  | the validation interface queue, which doesn't
  | exist for the bitcoin-wallet. Define our own
  | deleter here.
  */
pub fn wallet_tool_release_wallet(wallet: &mut Wallet)  {
    
    todo!();
        /*
            wallet->WalletLogPrintf("Releasing wallet\n");
        wallet->Close();
        delete wallet;
        */
}
