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
