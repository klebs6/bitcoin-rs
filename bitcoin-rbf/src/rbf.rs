// ---------------- [ File: bitcoin-rbf/src/rbf.rs ]
crate::ix!();

pub trait IsRBFOptIn {

    /**
      | Check if transaction is RBF opt in.
      |
      */
    fn is_rbf_opt_in(&mut self, tx: &Transaction) -> RBFTransactionState;
}


//-------------------------------------------[.cpp/bitcoin/src/policy/rbf.h]

/**
  | Maximum number of transactions that
  | can be replaced by BIP125 RBF (Rule #5).
  | This includes all mempool conflicts
  | and their descendants.
  |
  */
pub const MAX_BIP125_REPLACEMENT_CANDIDATES: u32 = 100;

/**
  | The rbf state of unconfirmed transactions
  |
  */
pub enum RBFTransactionState {

    /**
      | Unconfirmed tx that does not signal
      | rbf and is not in the mempool
      |
      */
    UNKNOWN,

    /**
      | Either this tx or a mempool ancestor
      | signals rbf
      |
      */
    REPLACEABLE_BIP125,

    /**
      | Neither this tx nor a mempool ancestor
      | signals rbf
      |
      */
    FINAL,
}

//-------------------------------------------[.cpp/bitcoin/src/policy/rbf.cpp]

pub fn is_rbf_opt_in_empty_mempool(tx: &Transaction) -> RBFTransactionState {
    
    todo!();
        /*
            // If we don't have a local mempool we can only check the transaction itself.
        return SignalsOptInRBF(tx) ? RBFTransactionState::REPLACEABLE_BIP125 : RBFTransactionState::UNKNOWN;
        */
}

/**
  | Enforce BIP125 Rule #3 "The replacement
  | transaction pays an absolute fee of
  | at least the sum paid by the original
  | transactions." Enforce BIP125 Rule
  | #4 "The replacement transaction must
  | also pay for its own bandwidth at or above
  | the rate set by the node's minimum relay
  | fee setting."
  | 
  | -----------
  | @param[in] original_fees
  | 
  | Total modified fees of original transaction(s).
  | ----------
  | @param[in] replacement_fees
  | 
  | Total modified fees of replacement
  | transaction(s).
  | ----------
  | @param[in] replacement_vsize
  | 
  | Total virtual size of replacement transaction(s).
  | ----------
  | @param[in] relay_fee
  | 
  | The node's minimum feerate for transaction
  | relay.
  | ----------
  | @param[in] txid
  | 
  | Transaction ID, included in the error
  | message if violation occurs.
  | 
  | -----------
  | @return
  | 
  | error string if fees are insufficient,
  | otherwise std::nullopt.
  |
  */
pub fn pays_forrbf(
        original_fees:     Amount,
        replacement_fees:  Amount,
        replacement_vsize: usize,
        relay_fee:         FeeRate,
        txid:              &u256) -> Option<String> {
    
    todo!();
        /*
            // BIP125 Rule #3: The replacement fees must be greater than or equal to fees of the
        // transactions it replaces, otherwise the bandwidth used by those conflicting transactions
        // would not be paid for.
        if (replacement_fees < original_fees) {
            return strprintf("rejecting replacement %s, less fees than conflicting txs; %s < %s",
                             txid.ToString(), FormatMoney(replacement_fees), FormatMoney(original_fees));
        }

        // BIP125 Rule #4: The new transaction must pay for its own bandwidth. Otherwise, we have a DoS
        // vector where attackers can cause a transaction to be replaced (and relayed) repeatedly by
        // increasing the fee by tiny amounts.
        CAmount additional_fees = replacement_fees - original_fees;
        if (additional_fees < relay_fee.GetFee(replacement_vsize)) {
            return strprintf("rejecting replacement %s, not enough additional fees to relay; %s < %s",
                             txid.ToString(),
                             FormatMoney(additional_fees),
                             FormatMoney(relay_fee.GetFee(replacement_vsize)));
        }
        return std::nullopt;
        */
}

//-------------------------------------------[.cpp/bitcoin/src/util/rbf.h]
//-------------------------------------------[.cpp/bitcoin/src/util/rbf.cpp]

pub const MAX_BIP125_RBF_SEQUENCE: u32 = 0xfffffffd;

/**
  | Check whether the sequence numbers
  | on this transaction are signaling opt-in
  | to replace-by-fee, according to BIP
  | 125. Allow opt-out of transaction replacement
  | by setting nSequence >
  | 
  | MAX_BIP125_RBF_SEQUENCE (SEQUENCE_FINAL-2)
  | on all inputs.
  | 
  | SEQUENCE_FINAL-1 is picked to still
  | allow use of nLockTime by non-replaceable
  | transactions. All inputs rather than
  | just one is for the sake of multi-party
  | protocols, where we don't want a single
  | party to be able to disable replacement
  | by opting out in their own input.
  |
  */
pub fn signals_opt_inrbf(tx: &Transaction) -> bool {
    
    todo!();
        /*
            for (const CTxIn &txin : tx.vin) {
            if (txin.nSequence <= MAX_BIP125_RBF_SEQUENCE) {
                return true;
            }
        }
        return false;
        */
}
