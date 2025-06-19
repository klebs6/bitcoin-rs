crate::ix!();


/** 
 | Upper bound for mantissa.
 |
 | 10^18-1 is the largest arbitrary decimal that
 | will fit in a signed 64-bit integer.
 |
 | Larger integers cannot consist of arbitrary
 | combinations of 0-9:
 |
 |   999999999999999999  1^18-1
 |  9223372036854775807  (1<<63)-1  (max int64_t)
 |  9999999999999999999  1^19-1     (would overflow)
 */
pub const UPPER_BOUND: i64 = 1000000000000000000 - 1;

/**
  | Helper function for ParseFixedPoint
  |
  */
#[inline] pub fn process_mantissa_digit(
        ch:              u8,
        mantissa:        &mut i64,
        mantissa_tzeros: &mut i32) -> bool {
    
    todo!();
        /*
            if(ch == '0')
            ++mantissa_tzeros;
        else {
            for (int i=0; i<=mantissa_tzeros; ++i) {
                if (mantissa > (UPPER_BOUND / 10LL))
                    return false; /* overflow */
                mantissa *= 10;
            }
            mantissa += ch - '0';
            mantissa_tzeros = 0;
        }
        return true;
        */
}
