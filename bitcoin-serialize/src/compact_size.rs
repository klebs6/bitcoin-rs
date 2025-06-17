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
