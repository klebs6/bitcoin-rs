// ---------------- [ File: bitcoin-bech32m/src/checksum.rs ]
crate::ix!();

/**
  | Convert to lower case.
  |
  */
#[inline] pub fn lower_case(c: u8) -> u8 {
    
    char::from(c).to_ascii_lowercase().try_into().unwrap()
}

/**
  | Expand a HRP for use in checksum computation.
  |
  */
pub fn expand_hrp(hrp: &String) -> Vec<u8> {
    
    let mut ret = Vec::<u8>::new();

    ret.reserve(hrp.len() + 90);
    ret.resize(hrp.len() * 2 + 1, 0);

    for i in 0..hrp.len() {
        let c: u8 = hrp.chars().nth(i).unwrap().try_into().unwrap();
        ret[i] = c >> 5;
        ret[i + hrp.len() + 1] = c & 0x1f;
    }

    ret[hrp.len()] = 0;
    return ret;
}

/**
  | Verify a checksum.
  |
  */
pub fn verify_checksum(
    hrp:    &String,
    values: &Vec<u8>) -> Encoding {

    // PolyMod computes what value to xor into the
    // final values to make the checksum
    // 0. However, if we required that the
    // checksum was 0, it would be the case that
    // appending a 0 to a valid list of values
    // would result in a new valid list. For that
    // reason, Bech32 requires the resulting
    // checksum to be 1 instead. In Bech32m, this
    // constant was amended. See
    // https://gist.github.com/sipa/14c248c288c3880a3b191f978a34508e
    // for details.
    let check: u32 = poly_mod(&cat(expand_hrp(hrp),values));

    if check == encoding_constant(Encoding::BECH32) {
        return Encoding::BECH32;
    }

    if check == encoding_constant(Encoding::BECH32M) {
        return Encoding::BECH32M;
    }

    Encoding::INVALID
}

/**
  | Create a checksum.
  |
  */
pub fn create_checksum(
        encoding: Encoding,
        hrp:      &String,
        values:   &Vec<u8>) -> Vec<u8> {
    
    let mut enc: Vec::<u8> = cat(expand_hrp(hrp),values);

    // Append 6 zeroes
    enc.resize(enc.len() + 6, 0); 

    // Determine what to XOR into those 6 zeroes.
    let mod_: u32 = poly_mod(&enc) ^ encoding_constant(encoding);

    let mut ret: Vec::<u8> = Vec::<u8>::with_capacity(6);

    for i in 0..6 {

        // Convert the 5-bit groups in mod to
        // checksum values.
        //
        ret[i] = ((mod_ >> (5 * (5 - i))) & 31).try_into().unwrap();
    }

    ret
}
