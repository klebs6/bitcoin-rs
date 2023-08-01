crate::ix!();

#[test] fn get_fee_test() {
    todo!();
    /*
    
        CFeeRate feeRate, altFeeRate;

        feeRate = CFeeRate(0);
        // Must always return 0
        BOOST_CHECK_EQUAL(feeRate.GetFee(0), CAmount(0));
        BOOST_CHECK_EQUAL(feeRate.GetFee(1e5), CAmount(0));

        feeRate = CFeeRate(1000);
        // Must always just return the arg
        BOOST_CHECK_EQUAL(feeRate.GetFee(0), CAmount(0));
        BOOST_CHECK_EQUAL(feeRate.GetFee(1), CAmount(1));
        BOOST_CHECK_EQUAL(feeRate.GetFee(121), CAmount(121));
        BOOST_CHECK_EQUAL(feeRate.GetFee(999), CAmount(999));
        BOOST_CHECK_EQUAL(feeRate.GetFee(1e3), CAmount(1e3));
        BOOST_CHECK_EQUAL(feeRate.GetFee(9e3), CAmount(9e3));

        feeRate = CFeeRate(-1000);
        // Must always just return -1 * arg
        BOOST_CHECK_EQUAL(feeRate.GetFee(0), CAmount(0));
        BOOST_CHECK_EQUAL(feeRate.GetFee(1), CAmount(-1));
        BOOST_CHECK_EQUAL(feeRate.GetFee(121), CAmount(-121));
        BOOST_CHECK_EQUAL(feeRate.GetFee(999), CAmount(-999));
        BOOST_CHECK_EQUAL(feeRate.GetFee(1e3), CAmount(-1e3));
        BOOST_CHECK_EQUAL(feeRate.GetFee(9e3), CAmount(-9e3));

        feeRate = CFeeRate(123);
        // Truncates the result, if not integer
        BOOST_CHECK_EQUAL(feeRate.GetFee(0), CAmount(0));
        BOOST_CHECK_EQUAL(feeRate.GetFee(8), CAmount(1)); // Special case: returns 1 instead of 0
        BOOST_CHECK_EQUAL(feeRate.GetFee(9), CAmount(1));
        BOOST_CHECK_EQUAL(feeRate.GetFee(121), CAmount(14));
        BOOST_CHECK_EQUAL(feeRate.GetFee(122), CAmount(15));
        BOOST_CHECK_EQUAL(feeRate.GetFee(999), CAmount(122));
        BOOST_CHECK_EQUAL(feeRate.GetFee(1e3), CAmount(123));
        BOOST_CHECK_EQUAL(feeRate.GetFee(9e3), CAmount(1107));

        feeRate = CFeeRate(-123);
        // Truncates the result, if not integer
        BOOST_CHECK_EQUAL(feeRate.GetFee(0), CAmount(0));
        BOOST_CHECK_EQUAL(feeRate.GetFee(8), CAmount(-1)); // Special case: returns -1 instead of 0
        BOOST_CHECK_EQUAL(feeRate.GetFee(9), CAmount(-1));

        // check alternate constructor
        feeRate = CFeeRate(1000);
        altFeeRate = CFeeRate(feeRate);
        BOOST_CHECK_EQUAL(feeRate.GetFee(100), altFeeRate.GetFee(100));

        // Check full constructor
        BOOST_CHECK(CFeeRate(CAmount(-1), 0) == CFeeRate(0));
        BOOST_CHECK(CFeeRate(CAmount(0), 0) == CFeeRate(0));
        BOOST_CHECK(CFeeRate(CAmount(1), 0) == CFeeRate(0));
        // default value
        BOOST_CHECK(CFeeRate(CAmount(-1), 1000) == CFeeRate(-1));
        BOOST_CHECK(CFeeRate(CAmount(0), 1000) == CFeeRate(0));
        BOOST_CHECK(CFeeRate(CAmount(1), 1000) == CFeeRate(1));
        // lost precision (can only resolve satoshis per kB)
        BOOST_CHECK(CFeeRate(CAmount(1), 1001) == CFeeRate(0));
        BOOST_CHECK(CFeeRate(CAmount(2), 1001) == CFeeRate(1));
        // some more integer checks
        BOOST_CHECK(CFeeRate(CAmount(26), 789) == CFeeRate(32));
        BOOST_CHECK(CFeeRate(CAmount(27), 789) == CFeeRate(34));
        // Maximum size in bytes, should not crash
        CFeeRate(MAX_MONEY, std::numeric_limits<uint32_t>::max()).GetFeePerK();

    */
}


