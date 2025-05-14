// ---------------- [ File: bitcoin-psbt/src/finalize.rs ]
crate::ix!();

/**
  | Finalizes a PSBT if possible, combining
  | partial signatures.
  | 
  | -----------
  | @param[in,out] psbtx
  | 
  | PartiallySignedTransaction to finalize
  | return True if the PSBT is now complete,
  | false otherwise
  |
  */
pub fn finalizepsbt(psbtx: &mut PartiallySignedTransaction) -> bool {
    
    /*
      | Finalize input signatures -- in case we
      | have partial signatures that add up to
      | a complete signature, but have not combined
      | them yet (e.g. because the combiner that
      | created this PartiallySignedTransaction did
      | not understand them), this will combine
      | them into a final script.
      */
    let mut complete: bool = true;

    let txdata: PrecomputedTransactionData 
    = precompute_psbt_data(psbtx);

    let len = psbtx.tx.as_ref().unwrap().vin.len();

    for i in 0..len {

        complete &= sign_psbt_input(
            &DUMMY_SIGNING_PROVIDER,
            psbtx,
            i as i32,
            &txdata,
            Some(SIGHASH_ALL as i32),
            None
        );

    }

    complete
}

/**
  | Finalizes a PSBT if possible, and extracts
  | it to a CMutableTransaction if it could
  | be finalized.
  | 
  | -----------
  | @param[in] psbtx
  | 
  | PartiallySignedTransaction
  | ----------
  | @param[out] result
  | 
  | CMutableTransaction representing
  | the complete transaction, if successful
  | 
  | -----------
  | @return
  | 
  | True if we successfully extracted the
  | transaction, false otherwise
  |
  */
pub fn finalize_and_extractpsbt<'a>(
        psbtx:      &'a mut PartiallySignedTransaction,
        mut result: &'a mut MutableTransaction) -> bool {
    
    /**
      | It's not safe to extract a PSBT that
      | isn't finalized, and there's no easy way
      | to check whether a PSBT is finalized
      | without finalizing it, so we just do
      | this.
      */
    if !finalizepsbt(psbtx) {
        return false;
    }

    result = psbtx.tx.as_mut().unwrap();

    for i in 0..result.vin.len() {
        result.vin[i].script_sig     = psbtx.inputs[i].final_script_sig.clone();
        result.vin[i].script_witness = psbtx.inputs[i].final_script_witness.clone();
    }

    true
}
