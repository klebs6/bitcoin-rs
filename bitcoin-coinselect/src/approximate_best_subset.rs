crate::ix!();

pub fn approximate_best_subset(
        groups:         &Vec<OutputGroup>,
        n_total_lower:  &Amount,
        n_target_value: &Amount,
        vf_best:        &mut Vec<u8>,
        n_best:         &mut Amount,
        iterations:     Option<i32>)  {

    let iterations: i32 = iterations.unwrap_or(1000);

    todo!();
        /*
            std::vector<char> vfIncluded;

        vfBest.assign(groups.size(), true);
        nBest = nTotalLower;

        FastRandomContext insecure_rand;

        for (int nRep = 0; nRep < iterations && nBest != nTargetValue; nRep++)
        {
            vfIncluded.assign(groups.size(), false);
            CAmount nTotal = 0;
            bool fReachedTarget = false;
            for (int nPass = 0; nPass < 2 && !fReachedTarget; nPass++)
            {
                for (unsigned int i = 0; i < groups.size(); i++)
                {
                    //The solver here uses a randomized algorithm,
                    //the randomness serves no real security purpose but is just
                    //needed to prevent degenerate behavior and it is important
                    //that the rng is fast. We do not use a constant random sequence,
                    //because there may be some privacy improvement by making
                    //the selection random.
                    if (nPass == 0 ? insecure_rand.randbool() : !vfIncluded[i])
                    {
                        nTotal += groups[i].GetSelectionAmount();
                        vfIncluded[i] = true;
                        if (nTotal >= nTargetValue)
                        {
                            fReachedTarget = true;
                            if (nTotal < nBest)
                            {
                                nBest = nTotal;
                                vfBest = vfIncluded;
                            }
                            nTotal -= groups[i].GetSelectionAmount();
                            vfIncluded[i] = false;
                        }
                    }
                }
            }
        }
        */
}

