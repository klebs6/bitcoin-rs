crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/walletmodeltransaction.h]

/**
  | Data model for a walletmodel transaction.
  |
  */
pub struct WalletModelTransaction {
    recipients: QList<SendCoinsRecipient>,
    wtx:        TransactionRef,
    fee:        Amount,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/walletmodeltransaction.cpp]
impl WalletModelTransaction {
    
    pub fn new(recipients: &QList<SendCoinsRecipient>) -> Self {
    
        todo!();
        /*
            :
        recipients(_recipients),
        fee(0)
        */
    }
    
    pub fn get_recipients(&self) -> QList<SendCoinsRecipient> {
        
        todo!();
        /*
            return recipients;
        */
    }
    
    pub fn get_wtx(&mut self) -> &mut TransactionRef {
        
        todo!();
        /*
            return wtx;
        */
    }
    
    pub fn set_wtx(&mut self, new_tx: &TransactionRef)  {
        
        todo!();
        /*
            wtx = newTx;
        */
    }
    
    pub fn get_transaction_size(&mut self) -> u32 {
        
        todo!();
        /*
            return wtx ? GetVirtualTransactionSize(*wtx) : 0;
        */
    }
    
    pub fn get_transaction_fee(&self) -> Amount {
        
        todo!();
        /*
            return fee;
        */
    }
    
    pub fn set_transaction_fee(&mut self, new_fee: &Amount)  {
        
        todo!();
        /*
            fee = newFee;
        */
    }
    
    /**
      | needed for the subtract-fee-from-amount
      | feature
      |
      */
    pub fn reassign_amounts(&mut self, n_change_pos_ret: i32)  {
        
        todo!();
        /*
            const CTransaction* walletTransaction = wtx.get();
        int i = 0;
        for (QList<SendCoinsRecipient>::iterator it = recipients.begin(); it != recipients.end(); ++it)
        {
            SendCoinsRecipient& rcp = (*it);
            {
                if (i == nChangePosRet)
                    i++;
                rcp.amount = walletTransaction->vout[i].nValue;
                i++;
            }
        }
        */
    }
    
    pub fn get_total_transaction_amount(&self) -> Amount {
        
        todo!();
        /*
            CAmount totalTransactionAmount = 0;
        for (const SendCoinsRecipient &rcp : recipients)
        {
            totalTransactionAmount += rcp.amount;
        }
        return totalTransactionAmount;
        */
    }
}
