crate::ix!();

/**
  | convert hexadecimal string to unsigned
  | integer
  |
  */
pub fn hatoui(
        first: *const u8,
        last:  *const u8,
        out:   &mut u32) -> *const u8 {
    
    let mut result: u32 = 0;

    todo!();
        /*

        for (; first != last; ++first)
        {
            int digit;
            if (json_isdigit(*first))
                digit = *first - '0';

            else if (*first >= 'a' && *first <= 'f')
                digit = *first - 'a' + 10;

            else if (*first >= 'A' && *first <= 'F')
                digit = *first - 'A' + 10;

            else
                break;

            result = 16 * result + digit;
        }
        out = result;

        */

    first
}
