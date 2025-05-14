// ---------------- [ File: bitcoin-coinselect/src/select.rs ]
crate::ix!();

/**
  | This is the Branch and Bound Coin Selection
  | algorithm designed by Murch. It searches
  | for an input set that can pay for the spending
  | target and does not exceed the spending
  | target by more than the cost of creating
  | and spending a change output. The algorithm
  | uses a depth-first search on a binary
  | tree. In the binary tree, each node corresponds
  | to the inclusion or the omission of a
  | UTXO. UTXOs are sorted by their effective
  | values and the trees is explored deterministically
  | per the inclusion branch first. At each
  | node, the algorithm checks whether
  | the selection is within the target range.
  | 
  | While the selection has not reached
  | the target range, more UTXOs are included.
  | When a selection's value exceeds the
  | target range, the complete subtree
  | deriving from this selection can be
  | omitted.
  | 
  | At that point, the last included UTXO
  | is deselected and the corresponding
  | omission branch explored instead.
  | The search ends after the complete tree
  | has been searched or after a limited
  | number of tries.
  | 
  | The search continues to search for better
  | solutions after one solution has been
  | found. The best solution is chosen by
  | minimizing the waste metric. The waste
  | metric is defined as the cost to spend
  | the current inputs at the given fee rate
  | minus the long term expected cost to
  | spend the inputs, plus the amount the
  | selection exceeds the spending target:
  | 
  | waste = selectionTotal - target + inputs
  | × (currentFeeRate - longTermFeeRate)
  | 
  | The algorithm uses two additional optimizations.
  | A lookahead keeps track of the total
  | value of the unexplored UTXOs. A subtree
  | is not explored if the lookahead indicates
  | that the target range cannot be reached.
  | Further, it is unnecessary to test equivalent
  | combinations. This allows us to skip
  | testing the inclusion of UTXOs that
  | match the effective value and waste
  | of an omitted predecessor.
  | 
  | The Branch and Bound algorithm is described
  | in detail in Murch's Master Thesis:
  | https://murch.one/wp-content/uploads/2016/11/erhardt2016coinselection.pdf
  | 
  | -----------
  | @param const
  | 
  | std::vector<CInputCoin>& utxo_pool
  | The set of UTXOs that we are choosing
  | from.
  | 
  | These UTXOs will be sorted in descending
  | order by effective value and the CInputCoins'
  | values are their effective values.
  | ----------
  | @param const
  | 
  | CAmount& selection_target This is
  | the value that we want to select. It is
  | the lower bound of the range.
  | ----------
  | @param const
  | 
  | CAmount& cost_of_change This is the
  | cost of creating and spending a change
  | output.
  | 
  | This plus selection_target is the upper
  | bound of the range.
  | ----------
  | @param std
  | 
  | ::set<CInputCoin>& out_set -> This
  | is an output parameter for the set of
  | CInputCoins that have been selected.
  | ----------
  | @param CAmount
  | 
  | & value_ret -> This is an output parameter
  | for the total value of the CInputCoins
  | that were selected.
  |
  */
pub const TOTAL_TRIES: usize = 100000;

pub fn select_coins_bnb(
    utxo_pool:        &mut Vec<OutputGroup>,
    selection_target: &Amount,
    cost_of_change:   &Amount,
    out_set:          &mut HashSet<InputCoin>,
    value_ret:        &mut Amount) -> bool {
    
    todo!();
        /*
            out_set.clear();
        CAmount curr_value = 0;

        std::vector<bool> curr_selection; // select the utxo at this index
        curr_selection.reserve(utxo_pool.size());

        // Calculate curr_available_value
        CAmount curr_available_value = 0;
        for (const OutputGroup& utxo : utxo_pool) {
            // Assert that this utxo is not negative. It should never be negative, effective value calculation should have removed it
            assert(utxo.GetSelectionAmount() > 0);
            curr_available_value += utxo.GetSelectionAmount();
        }
        if (curr_available_value < selection_target) {
            return false;
        }

        // Sort the utxo_pool
        std::sort(utxo_pool.begin(), utxo_pool.end(), descending);

        CAmount curr_waste = 0;
        std::vector<bool> best_selection;
        CAmount best_waste = MAX_MONEY;

        // Depth First search loop for choosing the UTXOs
        for (size_t i = 0; i < TOTAL_TRIES; ++i) {
            // Conditions for starting a backtrack
            bool backtrack = false;
            if (curr_value + curr_available_value < selection_target ||                // Cannot possibly reach target with the amount remaining in the curr_available_value.
                curr_value > selection_target + cost_of_change ||    // Selected value is out of range, go back and try other branch
                (curr_waste > best_waste && (utxo_pool.at(0).fee - utxo_pool.at(0).long_term_fee) > 0)) { // Don't select things which we know will be more wasteful if the waste is increasing
                backtrack = true;
            } else if (curr_value >= selection_target) {       // Selected value is within range
                curr_waste += (curr_value - selection_target); // This is the excess value which is added to the waste for the below comparison
                // Adding another UTXO after this check could bring the waste down if the long term fee is higher than the current fee.
                // However we are not going to explore that because this optimization for the waste is only done when we have hit our target
                // value. Adding any more UTXOs will be just burning the UTXO; it will go entirely to fees. Thus we aren't going to
                // explore any more UTXOs to avoid burning money like that.
                if (curr_waste <= best_waste) {
                    best_selection = curr_selection;
                    best_selection.resize(utxo_pool.size());
                    best_waste = curr_waste;
                    if (best_waste == 0) {
                        break;
                    }
                }
                curr_waste -= (curr_value - selection_target); // Remove the excess value as we will be selecting different coins now
                backtrack = true;
            }

            // Backtracking, moving backwards
            if (backtrack) {
                // Walk backwards to find the last included UTXO that still needs to have its omission branch traversed.
                while (!curr_selection.empty() && !curr_selection.back()) {
                    curr_selection.pop_back();
                    curr_available_value += utxo_pool.at(curr_selection.size()).GetSelectionAmount();
                }

                if (curr_selection.empty()) { // We have walked back to the first utxo and no branch is untraversed. All solutions searched
                    break;
                }

                // Output was included on previous iterations, try excluding now.
                curr_selection.back() = false;
                OutputGroup& utxo = utxo_pool.at(curr_selection.size() - 1);
                curr_value -= utxo.GetSelectionAmount();
                curr_waste -= utxo.fee - utxo.long_term_fee;
            } else { // Moving forwards, continuing down this branch
                OutputGroup& utxo = utxo_pool.at(curr_selection.size());

                // Remove this utxo from the curr_available_value utxo amount
                curr_available_value -= utxo.GetSelectionAmount();

                // Avoid searching a branch if the previous UTXO has the same value and same waste and was excluded. Since the ratio of fee to
                // long term fee is the same, we only need to check if one of those values match in order to know that the waste is the same.
                if (!curr_selection.empty() && !curr_selection.back() &&
                    utxo.GetSelectionAmount() == utxo_pool.at(curr_selection.size() - 1).GetSelectionAmount() &&
                    utxo.fee == utxo_pool.at(curr_selection.size() - 1).fee) {
                    curr_selection.push_back(false);
                } else {
                    // Inclusion branch first (Largest First Exploration)
                    curr_selection.push_back(true);
                    curr_value += utxo.GetSelectionAmount();
                    curr_waste += utxo.fee - utxo.long_term_fee;
                }
            }
        }

        // Check for solution
        if (best_selection.empty()) {
            return false;
        }

        // Set output set
        value_ret = 0;
        for (size_t i = 0; i < best_selection.size(); ++i) {
            if (best_selection.at(i)) {
                util::insert(out_set, utxo_pool.at(i).m_outputs);
                value_ret += utxo_pool.at(i).m_value;
            }
        }

        return true;
        */
}

/**
  | Select coins by Single Random Draw.
  | OutputGroups are selected randomly
  | from the eligible outputs until the
  | target is satisfied
  | 
  | -----------
  | @param[in] utxo_pool
  | 
  | The positive effective value OutputGroups
  | eligible for selection
  | ----------
  | @param[in] target_value
  | 
  | The target value to select for
  | 
  | -----------
  | @return
  | 
  | If successful, a pair of set of outputs
  | and total selected value, otherwise,
  | std::nullopt
  |
  */
pub fn select_coinssrd(
        utxo_pool:    &Vec<OutputGroup>,
        target_value: Amount) -> Option<(HashSet<InputCoin>,Amount)> {
    
    todo!();
        /*
            std::set<CInputCoin> out_set;
        CAmount value_ret = 0;

        std::vector<size_t> indexes;
        indexes.resize(utxo_pool.size());
        std::iota(indexes.begin(), indexes.end(), 0);
        Shuffle(indexes.begin(), indexes.end(), FastRandomContext());

        CAmount selected_eff_value = 0;
        for (const size_t i : indexes) {
            const OutputGroup& group = utxo_pool.at(i);
            Assume(group.GetSelectionAmount() > 0);
            selected_eff_value += group.GetSelectionAmount();
            value_ret += group.m_value;
            util::insert(out_set, group.m_outputs);
            if (selected_eff_value >= target_value) {
                return std::make_pair(out_set, value_ret);
            }
        }
        return std::nullopt;
        */
}
