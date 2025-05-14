// ---------------- [ File: bitcoin-base58/src/decode_check.rs ]
crate::ix!();

pub fn decode_base_58check_raw(
        psz:         *const u8,
        vch_ret:     &mut Vec<u8>,
        max_ret_len: i32) -> bool {
    
    let b = match max_ret_len > i32::MAX - 4
    {
        true  => i32::MAX,
        false => max_ret_len + 4,
    };

    if !unsafe { decode_base58_raw(psz, vch_ret, b) } 
        || vch_ret.len() < 4
    {
        vch_ret.clear();
        return false;
    }

    // re-calculate the checksum, ensure it
    // matches the included 4-byte checksum
    let hash: u256 
        = hash1(&vch_ret[0..vch_ret.len() - 4]);

    if unsafe { 
        libc::memcmp(
            &hash as *const _ as *const libc::c_void, 
            &vch_ret[vch_ret.len() - 4] as *const _ as *const libc::c_void, 
            4) 
    } != 0 
    {
        vch_ret.clear();
        return false;
    }


    vch_ret.resize(vch_ret.len() - 4, 0);
    return true
}

/**
  | Decode a base58-encoded string (str)
  | that includes a checksum into a byte
  | vector (vchRet), return true if decoding
  | is successful
  |
  */
pub fn decode_base_58check(
        str_:    &str,
        vch_ret: &mut Vec<u8>,
        max_ret: i32) -> bool {


    if !valid_as_cstring(str_) {
        return false
    }

    decode_base_58check_raw(
        str_.as_ptr(),
        vch_ret,
        max_ret
    )
}
