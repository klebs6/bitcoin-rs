// ---------------- [ File: bitcoin-network/src/netmask_bits.rs ]
crate::ix!();

/**
  | @return
  | 
  | The number of 1-bits in the prefix of
  | the specified subnet mask. If the specified
  | subnet mask is not a valid one, -1.
  |
  */
#[inline] pub fn netmask_bits(x: u8) -> i32 {
    
    match x {
        0x00  =>  0,
        0x80  =>  1,
        0xc0  =>  2,
        0xe0  =>  3,
        0xf0  =>  4,
        0xf8  =>  5,
        0xfc  =>  6,
        0xfe  =>  7,
        0xff  =>  8,
        _     => -1,
    }
}

#[cfg(test)]
mod netmask_bits_validation_spec {
    use super::*;

    #[traced_test]
    fn valid_masks_map_to_expected_prefix_lengths() {
        let cases: &[(u8, i32)] = &[
            (0x00, 0),
            (0x80, 1),
            (0xC0, 2),
            (0xE0, 3),
            (0xF0, 4),
            (0xF8, 5),
            (0xFC, 6),
            (0xFE, 7),
            (0xFF, 8),
        ];
        for (mask, want) in cases {
            let got = netmask_bits(*mask);
            info!(mask = format_args!("{:#04x}", mask), got, want, "Checking valid netmask");
            assert_eq!(got, *want);
        }
    }

    #[traced_test]
    fn invalid_masks_return_minus_one() {
        // NOTE: 0xFF is a valid /8 mask, so it must NOT be listed here.
        let invalids: &[u8] = &[
            0x01, 0x7F, 0xBF, 0xFD, 0xEF, 0x55, 0xAA, 0x81, 0xF7,
        ];
        for m in invalids {
            let got = netmask_bits(*m);
            debug!(mask = format_args!("{:#04x}", m), got, "Invalid netmask should be -1");
            assert_eq!(got, -1);
        }
    }

    #[traced_test]
    fn valid_masks_map_to_prefix_lengths() {
        let table: &[(u8, i32)] = &[
            (0x00, 0),
            (0x80, 1),
            (0xC0, 2),
            (0xE0, 3),
            (0xF0, 4),
            (0xF8, 5),
            (0xFC, 6),
            (0xFE, 7),
            (0xFF, 8),
        ];
        for (mask, expected) in table {
            let got = netmask_bits(*mask);
            info!(mask = format_args!("{:#010b}", mask), expected, got, "Checking netmask prefix length");
            assert_eq!(got, *expected, "Mask {:#x} expected {}", mask, expected);
        }
    }

    #[traced_test]
    fn invalid_masks_yield_neg_one() {
        let invalids = [0x7F, 0x81, 0xF7, 0xFD, 0x01, 0xAA, 0x5E];
        for m in invalids {
            let got = netmask_bits(m);
            debug!(mask = format_args!("{:#010b}", m), got, "Invalid mask case");
            assert_eq!(got, -1, "Mask {:#x} should be invalid", m);
        }
    }
}
