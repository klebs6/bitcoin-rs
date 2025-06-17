// ---------------- [ File: bitcoin-bech32m/src/decode.rs ]
crate::ix!();

#[derive(Debug,Getters)]
#[getset(get="pub")]
pub struct DecodeResult {

    /**
      | What encoding was detected in the result;
      | 
      | Encoding::INVALID if failed.
      |
      */
    encoding: Encoding,

    /**
      | The human readable part
      |
      */
    hrp:      String,

    /**
      | The payload (excluding checksum)
      |
      */
    data:     Vec<u8>,
}

impl Default for DecodeResult {
    
    fn default() -> Self {
        Self {
            encoding: Encoding::INVALID,
            hrp:      String::default(),
            data:     vec![],
        }
    }
}

impl DecodeResult {

    pub fn new(
        enc: Encoding,
        h:   String,
        d:   Vec<u8>) -> Self {
    
        Self {
            encoding: enc,
            hrp:      h,
            data:     d,
        }
    }
}

/**
  | Decode a Bech32 or Bech32m string.
  |
  */
pub fn decode(input: &str) -> DecodeResult {
    // ----------------------------------------------------------------
    //  1. ASCII‑range and case‑uniformity validation
    // ----------------------------------------------------------------
    let mut saw_lower = false;
    let mut saw_upper = false;

    for ch in input.chars() {
        match ch {
            'a'..='z' => saw_lower = true,
            'A'..='Z' => saw_upper = true,
            '\x21'..='\x7e' => (),
            _ => return DecodeResult::default(),
        }
    }
    if saw_lower && saw_upper {
        return DecodeResult::default(); // mixed case not allowed
    }

    // ----------------------------------------------------------------
    //  2. Locate the separator '1' and perform length checks
    // ----------------------------------------------------------------
    let sep = match input.rfind('1') {
        Some(pos) => pos,
        None => return DecodeResult::default(),
    };
    if input.len() > 90 || sep == 0 || sep + 7 > input.len() {
        return DecodeResult::default();
    }

    // ----------------------------------------------------------------
    //  3. Parse data section into 5‑bit values
    // ----------------------------------------------------------------
    let mut values: Vec<u8> = Vec::with_capacity(input.len() - sep - 1);
    for ch in input.chars().skip(sep + 1) {
        let idx = ch as u32;
        if idx > 0x7f {
            return DecodeResult::default();
        }
        let v = CHARSET_REV[idx as usize];
        if v == -1 {
            return DecodeResult::default();
        }
        values.push(v as u8);
    }

    // ----------------------------------------------------------------
    //  4. Normalize HRP to lower‑case
    // ----------------------------------------------------------------
    let mut hrp = String::with_capacity(sep);
    for ch in input.chars().take(sep) {
        hrp.push(ch.to_ascii_lowercase());
    }

    // ----------------------------------------------------------------
    //  5. Verify checksum and return result
    // ----------------------------------------------------------------
    let enc = verify_checksum(&hrp, &values);
    if enc == Encoding::INVALID {
        return DecodeResult::default();
    }

    let data = values[..values.len() - 6].to_vec();
    DecodeResult::new(enc, hrp, data)
}
