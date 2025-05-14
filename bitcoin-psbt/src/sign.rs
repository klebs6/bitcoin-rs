// ---------------- [ File: bitcoin-psbt/src/sign.rs ]
crate::ix!();

/**
  | Signs a PSBTInput, verifying that all
  | provided data matches what is being
  | signed.
  | 
  | txdata should be the output of PrecomputePSBTData
  | (which can be shared across multiple
  | SignPSBTInput calls). If it is nullptr,
  | a dummy signature will be created.
  |
  */
pub fn sign_psbt_input(
    provider:    &SigningProvider,
    psbt:        &mut PartiallySignedTransaction,
    index:       i32,
    txdata:      *const PrecomputedTransactionData,
    sighash:     Option<i32>,
    out_sigdata: Option<*mut SignatureData>) -> bool {

    let sighash: i32 = sighash.unwrap_or(SIGHASH_ALL.try_into().unwrap());
    
    let input: &mut PSBTInput = &mut psbt.inputs[index as usize];

    let tx: &MutableTransaction = psbt.tx.as_ref().unwrap();

    if psbt_input_signed(input) {
        return true;
    }

    // Fill SignatureData with input info
    let mut sigdata = SignatureData::default();

    input.fill_signature_data(&mut sigdata);

    // Get UTXO
    let mut require_witness_sig: bool = false;

    let mut utxo = TxOut::default();

    if Arc::<Transaction>::strong_count(&input.non_witness_utxo) != 0 {

        //  If we're taking our information
        //  from a non-witness UTXO, verify that it
        //  matches the prevout.
        let prevout: &OutPoint = &tx.vin[index as usize].prevout;;

        if prevout.n as usize >= (*input.non_witness_utxo).vout.len() {
            return false;
        }

        if (*input.non_witness_utxo).get_hash() != &prevout.hash {
            return false;
        }

        utxo = (*input.non_witness_utxo).vout[prevout.n as usize].clone();

    } else if !input.witness_utxo.is_null() {

        utxo = input.witness_utxo.clone();

        /*
          | When we're taking our information from
          | a witness UTXO, we can't verify it is
          | actually data from the output being
          | spent. This is safe in case a witness
          | signature is produced (which includes
          | this information directly in the hash),
          | but not for non-witness
          | signatures. Remember that we require
          | a witness signature in this situation.
          */
        require_witness_sig = true;

    } else {

        return false;
    }

    sigdata.witness = false;

    let mut sig_complete = bool::default();

    if txdata == std::ptr::null_mut() {

        sig_complete = produce_signature(
            provider,
            &**DUMMY_SIGNATURE_CREATOR.lock().unwrap(), 
            &utxo.script_pub_key,
            &mut sigdata
        );

    } else {

        let creator: MutableTransactionSignatureCreator 
        = MutableTransactionSignatureCreator::new_with_txdata(
            tx, 
            index.try_into().unwrap(), 
            &utxo.n_value, 
            txdata, 
            sighash
        );

        sig_complete = produce_signature(
            provider,
            &creator,
            &utxo.script_pub_key,
            &mut sigdata
        );
    }

    // Verify that a witness signature was
    // produced in case one was required.
    if require_witness_sig && !sigdata.witness {
        return false;
    }

    input.from_signature_data(&sigdata);

    /**
      | If we have a witness signature, put
      | a witness UTXO.
      |
      | TODO: For segwit v1, we should remove
      | the non_witness_utxo
      */
    if sigdata.witness {
        input.witness_utxo = utxo;
        //input.non_witness_utxo = std::ptr::null_mut();
    }

    let out_sigdata = out_sigdata.unwrap();

    // Fill in the missing info
    if out_sigdata != std::ptr::null_mut() {

        unsafe {
            (*out_sigdata).missing_pubkeys        = sigdata.missing_pubkeys;
            (*out_sigdata).missing_sigs           = sigdata.missing_sigs;
            (*out_sigdata).missing_redeem_script  = sigdata.missing_redeem_script;
            (*out_sigdata).missing_witness_script = sigdata.missing_witness_script;
        }
    }

    sig_complete
}
