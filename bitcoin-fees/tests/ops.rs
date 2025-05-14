// ---------------- [ File: bitcoin-fees/tests/ops.rs ]
crate::ix!();

#[test] fn binary_operator_test() {
    todo!();
    /*
    
        CFeeRate a, b;
        a = CFeeRate(1);
        b = CFeeRate(2);
        BOOST_CHECK(a < b);
        BOOST_CHECK(b > a);
        BOOST_CHECK(a == a);
        BOOST_CHECK(a <= b);
        BOOST_CHECK(a <= a);
        BOOST_CHECK(b >= a);
        BOOST_CHECK(b >= b);
        // a should be 0.00000002 BTC/kvB now
        a += a;
        BOOST_CHECK(a == b);

    */
}

#[test] fn to_string_test() {
    todo!();
    /*
    
        CFeeRate feeRate;
        feeRate = CFeeRate(1);
        BOOST_CHECK_EQUAL(feeRate.ToString(), "0.00000001 BTC/kvB");
        BOOST_CHECK_EQUAL(feeRate.ToString(FeeEstimateMode::BTC_KVB), "0.00000001 BTC/kvB");
        BOOST_CHECK_EQUAL(feeRate.ToString(FeeEstimateMode::SAT_VB), "0.001 sat/vB");

    */
}
