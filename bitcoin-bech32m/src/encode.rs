crate::ix!();

/// Concatenate two vectors.
///
#[inline] pub fn cat<T: Copy>(mut v1: Vec<T>, v2: &Vec<T>) -> Vec<T> {

    v1.reserve(v1.len() + v2.len());

    for arg in v2.iter() {
        v1.push(*arg);
    }

    v1
}

#[derive(PartialEq,Eq)]
pub enum Encoding {

    /**
      | Failed decoding
      |
      */
    INVALID, 

    /**
      | Bech32 encoding as defined in BIP173
      |
      */
    BECH32,  

    /**
      | Bech32m encoding as defined in BIP350
      |
      */
    BECH32M, 
}

/**
  | Determine the final constant to use
  | for the specified encoding.
  |
  */
pub fn encoding_constant(encoding: Encoding) -> u32 {
    
    assert!(encoding == Encoding::BECH32 || encoding == Encoding::BECH32M);

    if encoding == Encoding::BECH32 { 1 } else { 0x2bc830a3 }
}

/**
  | Encode a Bech32 or Bech32m string. If
  | hrp contains uppercase characters,
  | this will cause an assertion error.
  | Encoding must be one of BECH32 or BECH32M.
  |
  */
pub fn encode(
    encoding: Encoding,
    hrp:      &String,
    values:   &Vec<u8>) -> String 
{
    /*
      | First ensure that the HRP is all
      | lowercase. BIP-173 and BIP350 require an
      | encoder to return a lowercase
      | Bech32/Bech32m string, but if given an
      | uppercase HRP, the result will always be
      | invalid.
      */
    for c in hrp.chars() {
        assert!(c < 'A' || c > 'Z');
    }

    let checksum: Vec<u8> = 
    create_checksum(encoding, hrp, values);

    let combined: Vec<u8> = 
    cat(values.clone(), &checksum);

    let mut ret: String = hrp.to_owned() + "1";

    ret.reserve(ret.len() + combined.len());

    for c in combined.iter() {
        ret += 
        &String::from(CHARSET.chars().nth(*c as usize).unwrap());
    }

    ret
}
