// ---------------- [ File: bitcoinleveldb-coding/src/varint_length.rs ]
crate::ix!();

/**
  | Returns the length of the varint32 or
  | varint64 encoding of "v"
  |
  */
pub fn varint_length(mut v: u64) -> i32 {
    let mut len: i32 = 1;
    trace!(
        value = v,
        "varint_length: computing varint length for value"
    );
    while v >= 128 {
        v >>= 7;
        len += 1;
    }
    debug!(len, "varint_length: computed varint length");
    len
}
