crate::ix!();

/**
  | Decode a base58-encoded string (str)
  | into a byte vector (vchRet). return
  | true if decoding is successful.
  |
  */
pub fn decode_base58(
        str_:        &str,
        vch_ret:     &mut Vec<u8>,
        max_ret_len: i32) -> bool {
    
    if !valid_as_cstring(str_) {
        return false
    }

    unsafe {
        decode_base58_raw(
            str_.as_ptr(),
            vch_ret,
            max_ret_len
        )
    }
}

pub unsafe fn decode_base58_raw(
        mut psz:     *const u8,
        vch:         &mut Vec<u8>,
        max_ret_len: i32) -> bool {
    
    // Skip leading spaces.
    while *psz != 0 && nom::character::is_space(*psz ){
        psz = psz.add(1);
    }

    // Skip and count leading '1's.
    let mut zeroes: i32 = 0;
    let mut length: i32 = 0;

    while char::from(*psz) == '1' {

        zeroes += 1;

        if zeroes > max_ret_len {
            return false;
        }

        psz = psz.add(1);
    }

    /**
      | Allocate enough space in big-endian
      | base256 representation.
      |
      */

    // log(58) / log(256), rounded up.
    let size: usize = libc::strlen(psz as *const i8) * 733 / 1000 + 1;

    let mut b256: Vec::<u8> = Vec::<u8>::with_capacity(size);

    // Process the characters.
    //
    // guarantee not out of range
    //
    // mapBase58.len() should be 256
    const_assert!(MAP_BASE58.len() == 256); 

    while *psz != 0 && !nom::character::is_space(*psz){

        //  Decode base58 character
        let mut carry: i32 = MAP_BASE58[*psz as usize] as i32;

        if carry == -1 {
            //  Invalid b58 character
            return false;
        }

        let mut i: i32 = 0;

        for val in b256.iter_mut().rev() {

            if carry == 0 && i >= length {
                break;
            }

            carry += (58 * (*val)) as i32;

            *val   = (carry % 256).try_into().unwrap();

            carry /= 256;

            i += 1;
        }

        assert!(carry == 0);

        length = i;

        if length + zeroes > max_ret_len {
            return false
        }

        psz = psz.add(1);
    }

    // Skip trailing spaces.
    while nom::character::is_space(*psz){
        psz = psz.add(1);
    }

    if *psz != 0 {
        return false
    }

    // Skip leading zeroes in b256.
    let mut it = b256.iter();

    let offset: usize = (size - length as usize).try_into().unwrap();

    it.advance_by(offset);

    // Copy result into output vector.
    vch.reserve(zeroes as usize + it.len());

    vch[0..zeroes as usize].fill(0);

    while let Some(val) = it.next() {
        vch.push(*val);
    }

    return true
}
