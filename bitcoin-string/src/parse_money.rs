crate::ix!();

/**
  | Parse an amount denoted in full coins.
  | E.g. "0.0034" supplied on the command
  | line. *
  |
  */
pub fn parse_money(money_string: &String) -> Option<Amount> {
    
    todo!();
        /*
            if (!ValidAsCString(money_string)) {
            return std::nullopt;
        }
        const std::string str = TrimString(money_string);
        if (str.empty()) {
            return std::nullopt;
        }

        std::string strWhole;
        int64_t nUnits = 0;
        const char* p = str.c_str();
        for (; *p; p++)
        {
            if (*p == '.')
            {
                p++;
                int64_t nMult = COIN / 10;
                while (IsDigit(*p) && (nMult > 0))
                {
                    nUnits += nMult * (*p++ - '0');
                    nMult /= 10;
                }
                break;
            }
            if (IsSpace(*p))
                return std::nullopt;
            if (!IsDigit(*p))
                return std::nullopt;
            strWhole.insert(strWhole.end(), *p);
        }
        if (*p) {
            return std::nullopt;
        }
        if (strWhole.size() > 10) // guard against 63 bit overflow
            return std::nullopt;
        if (nUnits < 0 || nUnits > COIN)
            return std::nullopt;
        int64_t nWhole = LocaleIndependentAtoi<int64_t>(strWhole);
        CAmount value = nWhole * COIN + nUnits;

        if (!MoneyRange(value)) {
            return std::nullopt;
        }

        return value;
        */
}
