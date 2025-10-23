// ---------------- [ File: bitcoin-fees/tests/get_fee.rs ]
use bitcoin_fees::*;
use bitcoin_imports::*;

#[traced_test]
fn get_fee_test() {
    // feeRate = CFeeRate(0);
    let mut fee_rate = FeeRate::new(0);
    // Must always return 0
    assert_eq!(fee_rate.get_fee(0), 0);
    assert_eq!(fee_rate.get_fee(100_000), 0);

    // feeRate = CFeeRate(1000);
    fee_rate = FeeRate::new(1000);
    // Must always just return the arg
    assert_eq!(fee_rate.get_fee(0), 0);
    assert_eq!(fee_rate.get_fee(1), 1);
    assert_eq!(fee_rate.get_fee(121), 121);
    assert_eq!(fee_rate.get_fee(999), 999);
    assert_eq!(fee_rate.get_fee(1_000), 1_000);
    assert_eq!(fee_rate.get_fee(9_000), 9_000);

    // feeRate = CFeeRate(-1000);
    fee_rate = FeeRate::new(-1000);
    // Must always just return -1 * arg
    assert_eq!(fee_rate.get_fee(0), 0);
    assert_eq!(fee_rate.get_fee(1), -1);
    assert_eq!(fee_rate.get_fee(121), -121);
    assert_eq!(fee_rate.get_fee(999), -999);
    assert_eq!(fee_rate.get_fee(1_000), -1_000);
    assert_eq!(fee_rate.get_fee(9_000), -9_000);

    // feeRate = CFeeRate(123);
    fee_rate = FeeRate::new(123);
    // Truncates the result, if not integer (with the ±1 special-case bumps)
    assert_eq!(fee_rate.get_fee(0), 0);
    assert_eq!(fee_rate.get_fee(8), 1);   // Special case: returns 1 instead of 0
    assert_eq!(fee_rate.get_fee(9), 1);
    assert_eq!(fee_rate.get_fee(121), 14);
    assert_eq!(fee_rate.get_fee(122), 15);
    assert_eq!(fee_rate.get_fee(999), 122);
    assert_eq!(fee_rate.get_fee(1_000), 123);
    assert_eq!(fee_rate.get_fee(9_000), 1107);

    // feeRate = CFeeRate(-123);
    fee_rate = FeeRate::new(-123);
    // Truncates the result, if not integer
    assert_eq!(fee_rate.get_fee(0), 0);
    assert_eq!(fee_rate.get_fee(8), -1);  // Special case: returns -1 instead of 0
    assert_eq!(fee_rate.get_fee(9), -1);

    // check alternate constructor (copy ctor analogue)
    fee_rate = FeeRate::new(1000);
    let alt_fee_rate = fee_rate; // Copy
    assert_eq!(fee_rate.get_fee(100), alt_fee_rate.get_fee(100));

    // Check full constructor (nFeePaid, num_bytes)
    assert_eq!(FeeRate::new_with_fee_paid(&(-1i64), 0), FeeRate::new(0));
    assert_eq!(FeeRate::new_with_fee_paid(&0i64, 0),    FeeRate::new(0));
    assert_eq!(FeeRate::new_with_fee_paid(&1i64, 0),    FeeRate::new(0));
    // default value
    assert_eq!(FeeRate::new_with_fee_paid(&(-1i64), 1000), FeeRate::new(-1));
    assert_eq!(FeeRate::new_with_fee_paid(&0i64,    1000), FeeRate::new(0));
    assert_eq!(FeeRate::new_with_fee_paid(&1i64,    1000), FeeRate::new(1));
    // lost precision (can only resolve satoshis per kB)
    assert_eq!(FeeRate::new_with_fee_paid(&1i64, 1001), FeeRate::new(0));
    assert_eq!(FeeRate::new_with_fee_paid(&2i64, 1001), FeeRate::new(1));
    // some more integer checks
    assert_eq!(FeeRate::new_with_fee_paid(&26i64, 789), FeeRate::new(32));
    assert_eq!(FeeRate::new_with_fee_paid(&27i64, 789), FeeRate::new(34));

    // Maximum size in bytes, should not crash
    // (Using a very large fee instead of importing MAX_MONEY keeps this test self-contained.)
    let _ = FeeRate::new_with_fee_paid(&(9_223_372_036_854_775_000i64), u32::MAX).get_fee_perk();
}
