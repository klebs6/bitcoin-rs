crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/policy_fee_tests.cpp]

#[cfg(test)]
pub mod policy_fee_tests {

    #[test] fn fee_rounder() {
        todo!();
        /*
        
            FeeFilterRounder fee_rounder{CFeeRate{1000}};

            // check that 1000 rounds to 974 or 1071
            std::set<CAmount> results;
            while (results.size() < 2) {
                results.emplace(fee_rounder.round(1000));
            }
            BOOST_CHECK_EQUAL(*results.begin(), 974);
            BOOST_CHECK_EQUAL(*++results.begin(), 1071);

            // check that negative amounts rounds to 0
            BOOST_CHECK_EQUAL(fee_rounder.round(-0), 0);
            BOOST_CHECK_EQUAL(fee_rounder.round(-1), 0);

            // check that MAX_MONEY rounds to 9170997
            BOOST_CHECK_EQUAL(fee_rounder.round(MAX_MONEY), 9170997);

        */
    }
}
