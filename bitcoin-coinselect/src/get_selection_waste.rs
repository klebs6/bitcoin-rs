// ---------------- [ File: bitcoin-coinselect/src/get_selection_waste.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/coinselection.cpp]

/**
  | Compute the waste for this result given
  | the cost of change and the opportunity
  | cost of spending these inputs now vs
  | in the future.
  | 
  | If change exists, waste = change_cost
  | + inputs * (effective_feerate - long_term_feerate)
  | 
  | If no change, waste = excess + inputs
  | * (effective_feerate - long_term_feerate)
  | 
  | where excess = selected_effective_value
  | - target
  | 
  | change_cost = effective_feerate
  | * change_output_size + long_term_feerate
  | * change_spend_size
  | 
  | -----------
  | @param[in] inputs
  | 
  | The selected inputs
  | ----------
  | @param[in] change_cost
  | 
  | The cost of creating change and spending
  | it in the future.
  | 
  | Only used if there is change, in which
  | case it must be positive.
  | 
  | Must be 0 if there is no change.
  | ----------
  | @param[in] target
  | 
  | The amount targeted by the coin selection
  | algorithm.
  | ----------
  | @param[in] use_effective_value
  | 
  | Whether to use the input's effective
  | value (when true) or the real value (when
  | false).
  | 
  | -----------
  | @return
  | 
  | The waste
  |
  */
pub fn get_selection_waste(
    inputs:              &HashSet<InputCoin>,
    change_cost:         Amount,
    target:              Amount,
    use_effective_value: Option<bool>) -> Amount {

    let use_effective_value: bool = use_effective_value.unwrap_or(true);
    
    todo!();
        /*
            // This function should not be called with empty inputs as that would mean the selection failed
        assert(!inputs.empty());

        // Always consider the cost of spending an input now vs in the future.
        CAmount waste = 0;
        CAmount selected_effective_value = 0;
        for (const CInputCoin& coin : inputs) {
            waste += coin.m_fee - coin.m_long_term_fee;
            selected_effective_value += use_effective_value ? coin.effective_value : coin.txout.nValue;
        }

        if (change_cost) {
            // Consider the cost of making change and spending it in the future
            // If we aren't making change, the caller should've set change_cost to 0
            assert(change_cost > 0);
            waste += change_cost;
        } else {
            // When we are not making change (change_cost == 0), consider the excess we are throwing away to fees
            assert(selected_effective_value >= target);
            waste += selected_effective_value - target;
        }

        return waste;
        */
}
