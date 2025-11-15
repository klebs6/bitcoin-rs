// ---------------- [ File: bitcoinleveldb-bloom/src/next_length.rs ]
crate::ix!();

pub fn next_length(length: i32) -> i32 {
    let mut len = length;

    if len < 10 {
        len += 1;
    } else if len < 100 {
        len += 10;
    } else if len < 1_000 {
        len += 100;
    } else {
        len += 1_000;
    }

    len
}
