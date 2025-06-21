// ---------------- [ File: bitcoin-string/src/format_money.rs ]
/*!
  | Money parsing/formatting utilities.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/moneystr.h]
//-------------------------------------------[.cpp/bitcoin/src/util/moneystr.cpp]

/**
  | Do not use these functions to represent
  | or parse monetary amounts to or from
  | 
  | JSON but use AmountFromValue and ValueFromAmount
  | for that.
  |
  */
pub fn format_money(n: Amount) -> String {
    trace!("format_money: input = {}", n);
    const_assert!(COIN > 1);

    let mut quotient = n / COIN;
    let mut remainder = n % COIN;

    /* work with positive values for string‑formatting */
    let negative = n < 0;
    if negative {
        quotient = -quotient;
        remainder = -remainder;
    }

    let mut out = format!("{quotient}.{remainder:08}");

    /* trim super‑fluous trailing ‘0’s, but never past the decimal point */
    while out.ends_with('0') {
        let len = out.len();
        /* stop if the previous character is the decimal point */
        if len >= 2 && out.as_bytes()[len - 2] == b'.' {
            break;
        }
        out.pop();
    }

    if negative {
        out.insert(0, '-');
    }
    trace!("format_money: output = {}", out);
    out
}

// --------------[ bitcoin-string/src/format_money.rs ]--------------

#[cfg(test)]
mod tests_format_money {
    use super::*;

    /// A satoshi is the smallest unit; `COIN` == 100 000 000.
    const ONE_BTC: Amount = COIN as Amount;

    #[traced_test]
    fn renders_full_coin() {
        assert_eq!(format_money(ONE_BTC), "1.0");
    }

    #[traced_test]
    fn keeps_sign_and_decimal_alignment() {
        let amt = -(ONE_BTC * 3 - 2500); // −2.999975 BTC
        assert_eq!(format_money(amt), "-2.999975");
    }

    #[traced_test]
    fn zero_is_single_zero() {
        assert_eq!(format_money(0), "0.0");
    }

    #[traced_test]
    fn trims_trailing_zeros() {
        // 42 BTC + 123 450 sat == 42.0012345 BTC
        let amt = ONE_BTC * 42 + 123_450;
        assert_eq!(format_money(amt), "42.0012345");
    }
}
