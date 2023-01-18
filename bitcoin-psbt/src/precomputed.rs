crate::ix!();

/**
  | Compute a PrecomputedTransactionData
  | object from a psbt.
  |
  */
pub fn precompute_psbt_data(psbt: &PartiallySignedTransaction) -> PrecomputedTransactionData {
    
    let tx: &MutableTransaction = psbt.tx.as_ref().unwrap();

    let mut have_all_spent_outputs: bool = true;

    let mut utxos: Vec::<TxOut> = Vec::<TxOut>::with_capacity(tx.vin.len());

    for idx in 0..tx.vin.len() {
        if !psbt.get_inpututxo(&mut utxos[idx], idx.try_into().unwrap()) {
            have_all_spent_outputs = false;
        }
    }

    let mut txdata = PrecomputedTransactionData::default();

    if have_all_spent_outputs {
        txdata.init(tx, utxos, Some(true));
    } else {
        txdata.init(tx, vec![], Some(true));
    }

    txdata
}

