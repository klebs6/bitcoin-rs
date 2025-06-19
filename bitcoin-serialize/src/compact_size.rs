// ---------------- [ File: bitcoin-serialize/src/compact_size.rs ]
crate::ix!();

/**
  | Compact Size
  | 
  | Size < 253 -- 1 byte
  | 
  | Size <= USHRT_MAX -- 3 bytes (253 + 2 bytes)
  | 
  | Size <= UINT_MAX -- 5 bytes (254 + 4 bytes)
  | 
  | Size > UINT_MAX -- 9 bytes (255 + 8 bytes)
  |
  */
#[inline]
pub fn get_size_of_compact_size(n_size: u64) -> u32 {
    let sz = if n_size < 253 {
        1
    } else if n_size <= u16::MAX as u64 {
        1 + 2
    } else if n_size <= u32::MAX as u64 {
        1 + 4
    } else {
        1 + 8
    };
    trace!(n_size, sz, "get_size_of_compact_size");
    sz
}

pub fn write_compact_size<Stream: Write>(os: &mut Stream, n_size: u64) {
    trace!(n_size, "write_compact_size → start");
    if n_size < 253 {
        ser_writedata8(os, n_size as u8);
    } else if n_size <= u16::MAX as u64 {
        ser_writedata8(os, 253);
        ser_writedata16(os, n_size as u16);
    } else if n_size <= u32::MAX as u64 {
        ser_writedata8(os, 254);
        ser_writedata32(os, n_size as u32);
    } else {
        ser_writedata8(os, 255);
        ser_writedata64(os, n_size);
    }
    trace!("write_compact_size → done");
}

/**
  | Decode a CompactSize-encoded variable-length
  | integer.
  | 
  | As these are primarily used to encode
  | the size of vector-like serializations,
  | by default a range check is performed.
  | When used as a generic number encoding,
  | range_check should be set to false.
  |
  */
pub fn read_compact_size<Stream: Read>(is: &mut Stream, range_check: Option<bool>) -> u64 {
    let range_check = range_check.unwrap_or(true);
    let ch_size = ser_readdata8(is);
    let mut n_size_ret: u64 = match ch_size {
        0..=252 => ch_size as u64,
        253 => {
            let v = ser_readdata16(is) as u64;
            if v < 253 {
                error!("non‑canonical CompactSize (ch=253)");
                panic!("non‑canonical ReadCompactSize()");
            }
            v
        }
        254 => {
            let v = ser_readdata32(is) as u64;
            if v < 0x10000 {
                error!("non‑canonical CompactSize (ch=254)");
                panic!("non‑canonical ReadCompactSize()");
            }
            v
        }
        _ => {
            let v = ser_readdata64(is);
            if v < 0x1_0000_0000 {
                error!("non‑canonical CompactSize (ch=255)");
                panic!("non‑canonical ReadCompactSize()");
            }
            v
        }
    };

    if range_check && n_size_ret > MAX_SIZE {
        error!(n_size_ret, "CompactSize exceeds MAX_SIZE");
        panic!("ReadCompactSize(): size too large");
    }

    trace!(n_size_ret, "read_compact_size");
    n_size_ret
}

#[cfg(test)]
mod compact_size_tests {
    use super::*;
    use std::io::Cursor;

    /// Values that are *permitted* when `range_check = true`.
    const SAMPLES_WITHIN_MAX: &[u64] = &[
        0,
        1,
        252,
        253,
        254,
        255,
        32_768,
        65_536,
        crate::constants::MAX_SIZE, // 0x02_00_00_00
    ];

    /// Values that exceed `MAX_SIZE` and require `range_check = false`.
    const SAMPLES_ABOVE_MAX: &[u64] = &[
        crate::constants::MAX_SIZE + 1,
        (1_u64 << 33),
        (1_u64 << 40),
    ];

    /* -------- helpers -------- */

    fn roundtrip(value: u64, range_check: bool) {
        let mut buf = Cursor::new(Vec::<u8>::new());
        write_compact_size(&mut buf, value);

        buf.set_position(0);
        let decoded = read_compact_size(&mut buf, Some(range_check));
        assert_eq!(decoded, value, "value {value:#x} failed round‑trip");
        assert_eq!(buf.position() as usize, buf.get_ref().len());
    }

    /* -------- tests -------- */

    #[traced_test]
    fn get_size_matches_spec() {
        assert_eq!(get_size_of_compact_size(0),                    1);
        assert_eq!(get_size_of_compact_size(252),                  1);
        assert_eq!(get_size_of_compact_size(253),                  3);
        assert_eq!(get_size_of_compact_size(u16::MAX as u64),      3);
        assert_eq!(get_size_of_compact_size(u16::MAX as u64 + 1),  5);
        assert_eq!(get_size_of_compact_size(u32::MAX as u64),      5);
        assert_eq!(get_size_of_compact_size(u32::MAX as u64 + 1),  9);
    }

    /// Round‑trip **all** values ≤ `MAX_SIZE` with range‑checking **on**.
    #[traced_test]
    fn roundtrip_within_max() {
        for &n in SAMPLES_WITHIN_MAX {
            roundtrip(n, true);
        }
    }

    /// Round‑trip values *above* `MAX_SIZE` with the caller opting‑out of
    /// the safety check.
    #[traced_test]
    fn roundtrip_above_max_without_check() {
        for &n in SAMPLES_ABOVE_MAX {
            roundtrip(n, false);
        }
    }

    /// Ensure that enabling the range‑check for an excessive value
    /// triggers a panic.
    #[test]
    #[should_panic] // exact message is implementation detail
    fn reject_excessive_size_when_range_checked() {
        let val = crate::constants::MAX_SIZE + 1;
        let mut buf = Cursor::new(Vec::<u8>::new());
        write_compact_size(&mut buf, val);

        buf.set_position(0);
        let _ = read_compact_size(&mut buf, Some(true));
    }

    /// Encoding 252 using the 0xFD prefix is non‑canonical and must be
    /// rejected.
    #[test]
    #[should_panic]
    fn reject_non_canonical_encoding() {
        let bad = [0xFD, 0xFC, 0x00]; // canonical form would be single‑byte 0xFC
        let mut cur = Cursor::new(bad.as_slice());
        let _ = read_compact_size(&mut cur, Some(true));
    }
}
