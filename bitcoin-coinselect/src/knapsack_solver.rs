// ---------------- [ File: bitcoin-coinselect/src/knapsack_solver.rs ]
crate::ix!();

/**
  | Original coin selection algorithm
  | as a fallback
  |
  */
pub fn knapsack_solver(
    n_target_value: &Amount,
    groups:         &mut Vec<OutputGroup>,
    set_coins_ret:  &mut HashSet<InputCoin>,
    n_value_ret:    &mut Amount) -> bool {
    
    todo!();
        /*
            setCoinsRet.clear();
        nValueRet = 0;

        // List of values less than target
        std::optional<OutputGroup> lowest_larger;
        std::vector<OutputGroup> applicable_groups;
        CAmount nTotalLower = 0;

        Shuffle(groups.begin(), groups.end(), FastRandomContext());

        for (const OutputGroup& group : groups) {
            if (group.GetSelectionAmount() == nTargetValue) {
                util::insert(setCoinsRet, group.m_outputs);
                nValueRet += group.m_value;
                return true;
            } else if (group.GetSelectionAmount() < nTargetValue + MIN_CHANGE) {
                applicable_groups.push_back(group);
                nTotalLower += group.GetSelectionAmount();
            } else if (!lowest_larger || group.GetSelectionAmount() < lowest_larger->GetSelectionAmount()) {
                lowest_larger = group;
            }
        }

        if (nTotalLower == nTargetValue) {
            for (const auto& group : applicable_groups) {
                util::insert(setCoinsRet, group.m_outputs);
                nValueRet += group.m_value;
            }
            return true;
        }

        if (nTotalLower < nTargetValue) {
            if (!lowest_larger) return false;
            util::insert(setCoinsRet, lowest_larger->m_outputs);
            nValueRet += lowest_larger->m_value;
            return true;
        }

        // Solve subset sum by stochastic approximation
        std::sort(applicable_groups.begin(), applicable_groups.end(), descending);
        std::vector<char> vfBest;
        CAmount nBest;

        ApproximateBestSubset(applicable_groups, nTotalLower, nTargetValue, vfBest, nBest);
        if (nBest != nTargetValue && nTotalLower >= nTargetValue + MIN_CHANGE) {
            ApproximateBestSubset(applicable_groups, nTotalLower, nTargetValue + MIN_CHANGE, vfBest, nBest);
        }

        // If we have a bigger coin and (either the stochastic approximation didn't find a good solution,
        //                                   or the next bigger coin is closer), return the bigger coin
        if (lowest_larger &&
            ((nBest != nTargetValue && nBest < nTargetValue + MIN_CHANGE) || lowest_larger->GetSelectionAmount() <= nBest)) {
            util::insert(setCoinsRet, lowest_larger->m_outputs);
            nValueRet += lowest_larger->m_value;
        } else {
            for (unsigned int i = 0; i < applicable_groups.size(); i++) {
                if (vfBest[i]) {
                    util::insert(setCoinsRet, applicable_groups[i].m_outputs);
                    nValueRet += applicable_groups[i].m_value;
                }
            }

            if (LogAcceptCategory(BCLog::SELECTCOINS)) {
                std::string log_message{"Coin selection best subset: "};
                for (unsigned int i = 0; i < applicable_groups.size(); i++) {
                    if (vfBest[i]) {
                        log_message += strprintf("%s ", FormatMoney(applicable_groups[i].m_value));
                    }
                }
                LogPrint(BCLog::SELECTCOINS, "%stotal %s\n", log_message, FormatMoney(nBest));
            }
        }

        return true;
        */
}
