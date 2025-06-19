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
    
    todo!();
        /*
            // Note: not using straight sprintf here because we do NOT want
        // localized number formatting.
        const_assert(COIN > 1);
        int64_t quotient = n / COIN;
        int64_t remainder = n % COIN;
        if (n < 0) {
            quotient = -quotient;
            remainder = -remainder;
        }
        std::string str = strprintf("%d.%08d", quotient, remainder);

        // Right-trim excess zeros before the decimal point:
        int nTrim = 0;
        for (int i = str.size()-1; (str[i] == '0' && IsDigit(str[i-2])); --i)
            ++nTrim;
        if (nTrim)
            str.erase(str.size()-nTrim, nTrim);

        if (n < 0)
            str.insert((unsigned int)0, 1, '-');
        return str;
        */
}
