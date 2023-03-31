crate::ix!();

/**
  | Return implementation of Wallet
  | interface. This function is defined in
  | dummywallet.cpp and throws if the wallet
  | component is not compiled.
  */
pub fn make_wallet_with_context(
    context: &mut WalletContext,
    wallet:  &Arc<Wallet>) -> Box<dyn WalletInterface> 
{
    todo!();
        /*
            return wallet ? std::make_unique<WalletImpl>(context, wallet) : nullptr;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/wallet.h]

/**
  | Called periodically by the schedule
  | thread. Prompts individual wallets
  | to resend their transactions. Actual
  | rebroadcast schedule is managed by
  | the wallets themselves.
  |
  */
pub fn maybe_resend_wallet_txs(context: &mut WalletContext)  {
    
    todo!();
        /*
        
        */
}

pub fn dummy_sign_input(
        provider:    &SigningProvider,
        tx_in:       &mut TxIn,
        txout:       &TxOut,
        use_max_sig: bool) -> bool {
    
    todo!();
        /*
        
        */
}
