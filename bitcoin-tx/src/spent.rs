crate::ix!();

/**
  | Compute the (single) SHA256 of the concatenation
  | of all amounts spent by a tx.
  |
  */
pub fn get_spent_amountssha256(outputs_spent: &Vec<TxOut>) -> u256 {
    
    todo!();
        /*
            CHashWriter ss(SER_GETHASH, 0);
        for (const auto& txout : outputs_spent) {
            ss << txout.nValue;
        }
        return ss.GetSHA256();
        */
}

/**
  | Compute the (single) SHA256 of the concatenation
  | of all scriptPubKeys spent by a tx.
  |
  */
pub fn get_spent_scriptssha256(outputs_spent: &Vec<TxOut>) -> u256 {
    
    todo!();
        /*
            CHashWriter ss(SER_GETHASH, 0);
        for (const auto& txout : outputs_spent) {
            ss << txout.scriptPubKey;
        }
        return ss.GetSHA256();
        */
}

