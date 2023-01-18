crate::ix!();

/**
  | Encode a byte span as a base58-encoded
  | string
  |
  */
pub fn encode_base58(mut input: &[u8]) -> String {
    
    // Skip & count leading zeroes.
    let mut zeroes: usize = 0;
    let mut length: i32 = 0;

    while input.len() > 0 && input[0] == 0 {
        input = &input[1..];
        zeroes += 1;
    }

    /*
      | Allocate enough space in big-endian
      | base58 representation.
      |
      */

    // log(256) / log(58), rounded up.
    let mut size: i32 = (input.len() * 138 / 100 + 1).try_into().unwrap();

    let mut b58: Vec::<u8> = Vec::<u8>::with_capacity(size.try_into().unwrap());

    // Process the bytes.
    while input.len() > 0 {

        let mut carry: i32 = input[0].into();

        let mut i: i32 = 0;

        // Apply "b58 = b58 * 256 + ch".

        for it in b58.iter_mut().rev() {

            if carry == 0 && i >= length {
                break;
            }

            carry += 256 * ((*it) as i32);

            *it = (carry % 58).try_into().unwrap();

            carry /= 58;

            i += 1;
        }

        assert!(carry == 0);

        length = i;
        input  = &input[1..];
    }

    // Skip leading zeroes in base58 result.
    let mut it = b58.iter();

    it.advance_by((size - length).try_into().unwrap());

    while it.next() == Some(&0) {}

    // Translate the result into a string.
    let mut s: String = 
        String::with_capacity((zeroes as usize) + it.len());

    s += &"1".repeat(zeroes);

    while let Some(val) = it.next() {
        s += &String::from(PSZ_BASE58.chars().nth(*val as usize).unwrap());
    }

    s
}
