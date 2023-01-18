/*!
  | Bech32 and Bech32m are string encoding formats
  | used in newer address types. 
  |
  | The outputs consist of a human-readable part
  | (alphanumeric), a separator character (1), and
  | a base32 data section, the last 6 characters
  | of which are a checksum. 
  |
  | The module is namespaced under bech32 for
  | historical reasons.
  |
  | For more information, see BIP 173 and BIP 350.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bech32.h]
//-------------------------------------------[.cpp/bitcoin/src/bech32.cpp]

/**
  | The Bech32 and Bech32m character set
  | for encoding.
  |
  */
pub const CHARSET: &'static str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

/**
  | The Bech32 and Bech32m character set
  | for decoding.
  |
  */
pub const CHARSET_REV: [i8; 128] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    15, -1, 10, 17, 21, 20, 26, 30,  7,  5, -1, -1, -1, -1, -1, -1,
    -1, 29, -1, 24, 13, 25,  9,  8, 23, -1, 18, 22, 31, 27, 19, -1,
     1,  0,  3, 16, 11, 28, 12, 14,  6,  4,  2, -1, -1, -1, -1, -1,
    -1, 29, -1, 24, 13, 25,  9,  8, 23, -1, 18, 22, 31, 27, 19, -1,
     1,  0,  3, 16, 11, 28, 12, 14,  6,  4,  2, -1, -1, -1, -1, -1
];
