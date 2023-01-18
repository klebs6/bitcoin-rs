crate::ix!();

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
pub fn decode(str_: &String) -> DecodeResult {
    
    let mut lower: bool = false;
    let mut upper: bool = false;

    for i in 0..str_.len() {

        let c: char = str_.chars().nth(i).unwrap();

        if c >= 'a' && c <= 'z' {
            lower = true;

        } else if c >= 'A' && c <= 'Z' {
            upper = true;

        } else if c < char::from(33) || c > char::from(126) {
            return DecodeResult::default();
        }
    }

    if lower && upper {
        return DecodeResult::default();
    }

    let pos: Option<usize> = str_.rfind('1');

    if str_.len() > 90 
    || pos == None
    || pos == Some(0) 
    || pos.unwrap() + 7 > str_.len() 
    {
        return DecodeResult::default();
    }

    let pos: usize = pos.unwrap();

    let mut values: Vec::<u8> 
    = Vec::<u8>::with_capacity(str_.len() - 1 - pos);

    for i in 0..str_.len() - 1 - pos {

        let c:   u8 = str_.chars().nth(i + pos + 1).unwrap().try_into().unwrap();
        let rev: i8 = CHARSET_REV[c as usize];

        if rev == -1 {
            return DecodeResult::default();
        }

        values[i] = rev.try_into().unwrap();
    }

    let mut hrp = String::default();

    for i in 0..pos {
        hrp += &String::from(str_.chars().nth(i).unwrap().to_ascii_lowercase());
    }

    let result: Encoding = verify_checksum(&hrp,&values);

    if result == Encoding::INVALID {
        return DecodeResult::default();
    }

    todo!();
    /*
        return {result, std::move(hrp), data(values.begin(), values.end() - 6)};
    */
}
