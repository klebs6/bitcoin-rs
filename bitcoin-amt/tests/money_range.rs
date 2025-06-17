// ---------------- [ File: bitcoin-amt/tests/money_range.rs ]
use bitcoin_amt::*;
use bitcoin_imports::*;

/// Validate that `money_range` accepts and rejects
/// the correct edge‑case values without panicking.
#[traced_test]
fn money_range_accepts_and_rejects_expected_values() {

    // ─── Arrange ──────────────────────────────────────────────────────────────
    let negative_one: Amount = -1;
    let zero: Amount         = 0;
    let one: Amount          = 1;
    let max_money: Amount    = MAX_MONEY;
    let above_max: Amount    = MAX_MONEY + 1;

    // ─── Act & Assert ────────────────────────────────────────────────────────
    info!("Verifying that negative amounts are rejected");
    assert!(
        !money_range(&negative_one),
        "money_range should reject negative values"
    );

    info!("Verifying that zero is accepted");
    assert!(
        money_range(&zero),
        "money_range should accept zero"
    );

    info!("Verifying that one satoshi is accepted");
    assert!(
        money_range(&one),
        "money_range should accept one satoshi"
    );

    info!("Verifying that MAX_MONEY is accepted");
    assert!(
        money_range(&max_money),
        "money_range should accept MAX_MONEY ({} satoshis)",
        MAX_MONEY
    );

    info!("Verifying that amounts above MAX_MONEY are rejected");
    assert!(
        !money_range(&above_max),
        "money_range should reject values greater than MAX_MONEY"
    );
}

