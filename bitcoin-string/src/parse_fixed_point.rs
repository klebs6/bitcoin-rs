crate::ix!();

/**
  | Parse number as fixed point according
  | to JSON number syntax.
  | 
  | See https://json.org/number.gif
  | 
  | -----------
  | @note
  | 
  | The result must be in the range (-10^18,10^18),
  | otherwise an overflow error will trigger.
  | 
  | -----------
  | @return
  | 
  | true on success, false on error.
  |
  */
pub fn parse_fixed_point(
        val:        &str,
        decimals:   i32,
        amount_out: *mut i64) -> bool {
    
    todo!();
        /*
            int64_t mantissa = 0;
        int64_t exponent = 0;
        int mantissa_tzeros = 0;
        bool mantissa_sign = false;
        bool exponent_sign = false;
        int ptr = 0;
        int end = val.size();
        int point_ofs = 0;

        if (ptr < end && val[ptr] == '-') {
            mantissa_sign = true;
            ++ptr;
        }
        if (ptr < end)
        {
            if (val[ptr] == '0') {
                /* pass single 0 */
                ++ptr;
            } else if (val[ptr] >= '1' && val[ptr] <= '9') {
                while (ptr < end && IsDigit(val[ptr])) {
                    if (!ProcessMantissaDigit(val[ptr], mantissa, mantissa_tzeros))
                        return false; /* overflow */
                    ++ptr;
                }
            } else return false; /* missing expected digit */
        } else return false; /* empty string or loose '-' */
        if (ptr < end && val[ptr] == '.')
        {
            ++ptr;
            if (ptr < end && IsDigit(val[ptr]))
            {
                while (ptr < end && IsDigit(val[ptr])) {
                    if (!ProcessMantissaDigit(val[ptr], mantissa, mantissa_tzeros))
                        return false; /* overflow */
                    ++ptr;
                    ++point_ofs;
                }
            } else return false; /* missing expected digit */
        }
        if (ptr < end && (val[ptr] == 'e' || val[ptr] == 'E'))
        {
            ++ptr;
            if (ptr < end && val[ptr] == '+')
                ++ptr;
            else if (ptr < end && val[ptr] == '-') {
                exponent_sign = true;
                ++ptr;
            }
            if (ptr < end && IsDigit(val[ptr])) {
                while (ptr < end && IsDigit(val[ptr])) {
                    if (exponent > (UPPER_BOUND / 10LL))
                        return false; /* overflow */
                    exponent = exponent * 10 + val[ptr] - '0';
                    ++ptr;
                }
            } else return false; /* missing expected digit */
        }
        if (ptr != end)
            return false; /* trailing garbage */

        /* finalize exponent */
        if (exponent_sign)
            exponent = -exponent;
        exponent = exponent - point_ofs + mantissa_tzeros;

        /* finalize mantissa */
        if (mantissa_sign)
            mantissa = -mantissa;

        /* convert to one 64-bit fixed-point value */
        exponent += decimals;
        if (exponent < 0)
            return false; /* cannot represent values smaller than 10^-decimals */
        if (exponent >= 18)
            return false; /* cannot represent values larger than or equal to 10^(18-decimals) */

        for (int i=0; i < exponent; ++i) {
            if (mantissa > (UPPER_BOUND / 10LL) || mantissa < -(UPPER_BOUND / 10LL))
                return false; /* overflow */
            mantissa *= 10;
        }
        if (mantissa > UPPER_BOUND || mantissa < -UPPER_BOUND)
            return false; /* overflow */

        if (amount_out)
            *amount_out = mantissa;

        return true;
        */
}
