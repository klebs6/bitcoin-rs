crate::ix!();

/**
  | Compress amount.
  | 
  | nAmount is of type uint64_t and thus
  | cannot be negative. If you're passing
  | in a CAmount (int64_t), make sure to
  | properly handle the case where the amount
  | is negative before calling CompressAmount(...).
  | 
  | @pre Function defined only for 0 <=
  | nAmount <= MAX_MONEY.
  |
  ----------------------------
  | Amount compression:
  |
  | - If the amount is 0, output 0
  |
  | - first, divide the amount (in base units) by
  |   the largest power of 10 possible; call the
  |   exponent e (e is max 9)
  |
  | - if e<9, the last digit of the resulting
  |   number cannot be 0; store it as d, and drop it
  |   (divide by 10)
  |      - call the result n
  |      - output 1 + 10*(9*n + d - 1) + e
  |
  | - if e==9, we only know the resulting number is
  | not zero, so output 1 + 10*(n - 1) + 9 (this is
  | decodable, as d is in [1-9] and e is in [0-9])
  */
pub fn compress_amount(mut n: u64) -> u64 {
    
    if n == 0 {
        return 0;
    }

    let mut e: i32 = 0;

    while ((n % 10) == 0) && e < 9 {
        n /= 10;
        e += 1;
    }

    if e < 9 {

        let d: i32 = (n % 10).try_into().unwrap();

        assert!(d >= 1 && d <= 9);

        n /= 10;

        return {

            let n9 = n * 9;

            let p = u64_plus_i32(n9, d - 1);

            u64_plus_i32(1 + p * 10, e)
        };

    } else {

        return {

            let p = (n - 1);

            1 + p * 10 + 9
        };
    }
}
