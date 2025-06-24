// ---------------- [ File: bitcoin-base58/src/base58.rs ]
/*!
  | Why base-58 instead of standard base-64
  | encoding?
  | 
  | - Don't want 0OIl characters that look
  | the same in some fonts and could be used
  | to create visually identical looking
  | data.
  | 
  | - A string with non-alphanumeric characters
  | is not as easily accepted as input.
  | 
  | - E-mail usually won't line-break if
  | there's no punctuation to break at.
  | 
  | - Double-clicking selects the whole
  | string as one word if it's all alphanumeric.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/base58.h]
//-------------------------------------------[.cpp/bitcoin/src/base58.cpp]

/**
  | All alphanumeric characters except
  | for "0", "I", "O", and "l"
  |
  */
pub const PSZ_BASE58: &'static str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub const MAP_BASE58: [i8; 256] = [
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1, 0, 1, 2, 3, 4, 5, 6,  7, 8,-1,-1,-1,-1,-1,-1,
    -1, 9,10,11,12,13,14,15, 16,-1,17,18,19,20,21,-1,
    22,23,24,25,26,27,28,29, 30,31,32,-1,-1,-1,-1,-1,
    -1,33,34,35,36,37,38,39, 40,41,42,43,-1,44,45,46,
    47,48,49,50,51,52,53,54, 55,56,57,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1, -1,-1,-1,-1,-1,-1,-1,-1,
];

#[cfg(test)]
mod base58_spec {
    use super::*;

    /// Ensure the canonical **Base‑58 alphabet** is correct and unique.
    #[traced_test]
    fn alphabet_has_58_distinct_chars() {
        let mut seen = std::collections::HashSet::new();
        for (idx, ch) in PSZ_BASE58.chars().enumerate() {
            trace!(idx, %ch, "checking Base‑58 alphabet character");
            assert!(seen.insert(ch), "duplicate character {ch} in alphabet");
        }
        info!(len = PSZ_BASE58.len(), "alphabet length");
        assert_eq!(PSZ_BASE58.len(), 58, "alphabet must contain 58 symbols");
    }

    /// Validate that `MAP_BASE58` is **fully initialised** and agrees with `PSZ_BASE58`.
    #[traced_test]
    fn map_table_is_consistent() {
        assert_eq!(MAP_BASE58.len(), 256, "lookup table must be 256 bytes long");
        for (sym_idx, ch) in PSZ_BASE58.bytes().enumerate() {
            let mapped = MAP_BASE58[ch as usize];
            debug!(sym_idx, char = %char::from(ch), mapped, "verifying mapping");
            assert_eq!(
                mapped as usize, sym_idx,
                "character {} should map to {} but maps to {}",
                char::from(ch), sym_idx, mapped
            );
        }

        // Forbidden characters (0,O,I,l) must map to -1
        for &c in b"0OIl" {
            assert_eq!(
                MAP_BASE58[c as usize],
                -1,
                "forbidden character {} should map to -1",
                c as char
            );
        }
    }
}
