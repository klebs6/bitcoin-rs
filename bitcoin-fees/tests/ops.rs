// ---------------- [ File: bitcoin-fees/tests/ops.rs ]
use bitcoin_fees::*;
use bitcoin_imports::*;

#[traced_test]
fn binary_operator_test() {
    let mut a = FeeRate::new(1);
    let b = FeeRate::new(2);
    assert!(a < b);
    assert!(b > a);
    assert_eq!(a, a);
    assert!(a <= b);
    assert!(a <= a);
    assert!(b >= a);
    assert!(b >= b);

    // a should be 0.00000002 BTC/kvB now
    let a_copy = a;
    a += &a_copy;
    assert_eq!(a, b);
}

#[traced_test]
fn to_string_test() {
    let fee_rate = FeeRate::new(1);
    assert_eq!(fee_rate.to_string(None), "0.00000001 BTC/kvB");
    assert_eq!(fee_rate.to_string(Some(&FeeEstimateMode::BTC_KVB)), "0.00000001 BTC/kvB");
    assert_eq!(fee_rate.to_string(Some(&FeeEstimateMode::SAT_VB)),  "0.001 sat/vB");
}
