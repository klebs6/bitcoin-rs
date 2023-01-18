crate::ix!();

/**
  | A group of UTXOs paid to the same output
  | script.
  |
  */
#[derive(Default)]
pub struct OutputGroup {

    /**
      | The list of UTXOs contained in this output
      | group.
      |
      */
    outputs:              Vec<InputCoin>,

    /**
      | Whether the UTXOs were sent by the wallet
      | to itself. This is relevant because
      | we may want at least a certain number
      | of confirmations on UTXOs received
      | from outside wallets while trusting
      | our own UTXOs more.
      |
      */
    from_me:              bool, // default = { true }

    /**
      | The total value of the UTXOs in sum.
      |
      */
    value:                Amount, // default = { 0 }

    /**
      | The minimum number of confirmations
      | the UTXOs in the group have. Unconfirmed
      | is 0.
      |
      */
    depth:                i32, // default = { 999 }

    /**
      | The aggregated count of unconfirmed
      | ancestors of all UTXOs in this group.
      | Not deduplicated and may overestimate
      | when ancestors are shared.
      |
      */
    ancestors:            usize, // default = { 0 }

    /**
      | The maximum count of descendants of
      | a single UTXO in this output group.
      |
      */
    descendants:          usize, // default = { 0 }

    /**
      | The value of the UTXOs after deducting
      | the cost of spending them at the effective
      | feerate.
      |
      */
    effective_value:      Amount, // default = { 0 }

    /**
      | The fee to spend these UTXOs at the effective
      | feerate.
      |
      */
    fee:                  Amount, // default = { 0 }

    /**
      | The target feerate of the transaction
      | we're trying to build.
      |
      */
    effective_feerate:    FeeRate, // default = { 0 }

    /**
      | The fee to spend these UTXOs at the long
      | term feerate.
      |
      */
    long_term_fee:        Amount, // default = { 0 }

    /**
      | The feerate for spending a created change
      | output eventually (i.e. not urgently,
      | and thus at a lower feerate). Calculated
      | using long term fee estimate. This is
      | used to decide whether it could be economical
      | to create a change output.
      |
      */
    long_term_feerate:    FeeRate, // default = { 0 }

    /**
      | Indicate that we are subtracting the
      | fee from outputs.
      | 
      | When true, the value that is used for
      | coin selection is the UTXO's real value
      | rather than effective value
      |
      */
    subtract_fee_outputs: bool, // default = { false }
}

impl From<&CoinSelectionParams> for OutputGroup {

    fn from(params: &CoinSelectionParams) -> Self {
    
        todo!();
        /*
        : effective_feerate(params.m_effective_feerate),
        : long_term_feerate(params.m_long_term_feerate),
        : subtract_fee_outputs(params.m_subtract_fee_outputs),
        */
    }
}

impl OutputGroup {

    pub fn insert(&mut self, 
        output:        &InputCoin,
        depth:         i32,
        from_me:       bool,
        ancestors:     usize,
        descendants:   usize,
        positive_only: bool)  {
        
        todo!();
        /*
            // Compute the effective value first
        const CAmount coin_fee = output.m_input_bytes < 0 ? 0 : m_effective_feerate.GetFee(output.m_input_bytes);
        const CAmount ev = output.txout.nValue - coin_fee;

        // Filter for positive only here before adding the coin
        if (positive_only && ev <= 0) return;

        m_outputs.push_back(output);
        CInputCoin& coin = m_outputs.back();

        coin.m_fee = coin_fee;
        fee += coin.m_fee;

        coin.m_long_term_fee = coin.m_input_bytes < 0 ? 0 : m_long_term_feerate.GetFee(coin.m_input_bytes);
        long_term_fee += coin.m_long_term_fee;

        coin.effective_value = ev;
        effective_value += coin.effective_value;

        m_from_me &= from_me;
        m_value += output.txout.nValue;
        m_depth = std::min(m_depth, depth);
        // ancestors here express the number of ancestors the new coin will end up having, which is
        // the sum, rather than the max; this will overestimate in the cases where multiple inputs
        // have common ancestors
        m_ancestors += ancestors;
        // descendants is the count as seen from the top ancestor, not the descendants as seen from the
        // coin itself; thus, this value is counted as the max, not the sum
        m_descendants = std::max(m_descendants, descendants);
        */
    }
    
    pub fn eligible_for_spending(&self, eligibility_filter: &CoinEligibilityFilter) -> bool {
        
        todo!();
        /*
            return m_depth >= (m_from_me ? eligibility_filter.conf_mine : eligibility_filter.conf_theirs)
            && m_ancestors <= eligibility_filter.max_ancestors
            && m_descendants <= eligibility_filter.max_descendants;
        */
    }
    
    pub fn get_selection_amount(&self) -> Amount {
        
        todo!();
        /*
            return m_subtract_fee_outputs ? m_value : effective_value;
        */
    }
}

