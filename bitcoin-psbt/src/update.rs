// ---------------- [ File: bitcoin-psbt/src/update.rs ]
crate::ix!();

/**
  | Updates a PSBTOutput with information
  | from provider.
  | 
  | This fills in the redeem_script, witness_script,
  | and hd_keypaths where possible.
  |
  */
pub fn update_psbt_output(
        provider: &SigningProvider,
        psbt:     &mut PartiallySignedTransaction,
        index:    i32) {
    
    let tx: &MutableTransaction = psbt.tx.as_ref().unwrap();

    let out: &TxOut = &tx.vout[index as usize];

    let psbt_out: &mut PSBTOutput = &mut psbt.outputs[index as usize];

    //  Fill a SignatureData with output info
    let mut sigdata = SignatureData::default();;

    psbt_out.fill_signature_data(&mut sigdata);

    /**
      | Construct a would-be spend of this
      | output, to update sigdata with.
      |
      | Note that ProduceSignature is used to
      | fill in metadata (not actual signatures),
      | so provider does not need to provide
      | any private keys (it can be
      | a HidingSigningProvider).
      */
    let creator: MutableTransactionSignatureCreator 
    = MutableTransactionSignatureCreator::new(
        tx, 
        0, 
        &out.n_value, 
        SIGHASH_ALL.try_into().unwrap()
    );

    produce_signature(
        provider, 
        &creator, 
        &out.script_pub_key, 
        &mut sigdata
    );

    // Put redeem_script, witness_script, key
    // paths, into PSBTOutput.
    psbt_out.from_signature_data(&sigdata);
}
