// ---------------- [ File: bitcoin-coinselect/src/input_coin.rs ]
crate::ix!();

/**
  | A UTXO under consideration for use in
  | funding a new transaction.
  |
  */
pub struct InputCoin {

    outpoint:        OutPoint,
    txout:           TxOut,
    effective_value: Amount,
    fee:             Amount, // default = { 0 }
    long_term_fee:   Amount, // default = { 0 }

    /**
      | Pre-computed estimated size of this
      | output as a fully-signed input in a transaction.
      | Can be -1 if it could not be calculated
      |
      */
    input_bytes:     i32, // default = { -1 }
}

impl Ord for InputCoin {
    
    #[inline] fn cmp(&self, other: &InputCoin) -> Ordering {
        todo!();
        /*
            return outpoint < rhs.outpoint;
        */
    }
}

impl PartialOrd<InputCoin> for InputCoin {
    #[inline] fn partial_cmp(&self, other: &InputCoin) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq<InputCoin> for InputCoin {
    
    #[inline] fn eq(&self, other: &InputCoin) -> bool {
        todo!();
        /*
            return outpoint == rhs.outpoint;
        */
    }
}

impl Eq for InputCoin {}

impl InputCoin {
    
    pub fn new_with_txref(
        tx: &TransactionRef,
        i:  u32) -> Self {
    
        todo!();
        /*


            if (!tx)
                throw std::invalid_argument("tx should not be null");
            if (i >= tx->vout.size())
                throw std::out_of_range("The output index is out of range");

            outpoint = OutPoint(tx->GetHash(), i);
            txout = tx->vout[i];
            effective_value = txout.nValue;
        */
    }
    
    pub fn new_with_txref_and_input_bytes(
        tx:          &TransactionRef,
        i:           u32,
        input_bytes: i32) -> Self {
    
        todo!();
        /*
        : input_coin(tx, i),

            m_input_bytes = input_bytes;
        */
    }
    
    pub fn new_wth_outpoint(
        outpoint_in: &OutPoint,
        txout_in:    &TxOut) -> Self {
    
        todo!();
        /*


            outpoint = outpoint_in;
            txout = txout_in;
            effective_value = txout.nValue;
        */
    }
    
    pub fn new_with_input_bytes(
        outpoint_in: &OutPoint,
        txout_in:    &TxOut,
        input_bytes: i32) -> Self {
    
        todo!();
        /*
        : input_coin(outpoint_in, txout_in),

            m_input_bytes = input_bytes;
        */
    }
}
