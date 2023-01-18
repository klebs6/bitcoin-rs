crate::ix!();

/**
  | Combines PSBTs with the same underlying
  | transaction, resulting in a single
  | PSBT with all partial signatures from
  | each input.
  | 
  | -----------
  | @param[out] out
  | 
  | the combined PSBT, if successful
  | ----------
  | @param[in] psbtxs
  | 
  | the PSBTs to combine
  | 
  | -----------
  | @return
  | 
  | error (OK if we successfully combined
  | the transactions, other error if they
  | were not compatible)
  |
  */
pub fn combine_psb_ts(
        out:    &mut PartiallySignedTransaction,
        psbtxs: &Vec<PartiallySignedTransaction>) -> TransactionError {
    
    todo!();
        /*
            out = psbtxs[0]; // Copy the first one

        // Merge
        for (auto it = std::next(psbtxs.begin()); it != psbtxs.end(); ++it) {
            if (!out.Merge(*it)) {
                return TransactionError::PSBT_MISMATCH;
            }
        }
        return TransactionError::OK;
        */
}


