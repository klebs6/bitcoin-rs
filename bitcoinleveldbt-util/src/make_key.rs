// ---------------- [ File: bitcoinleveldbt-util/src/make_key.rs ]
crate::ix!();

/// Invariant: the returned key is exactly 16 ASCII decimal digits, left-padded with `0`.
/// Postcondition: `make_key(num).len() == 16` for all `num`.
pub fn make_key(num: u32) -> String {
    trace!(
        target: "bitcoinleveldbt-dbtest",
        label = "make_key.entry",
        num
    );

    let s = format!("{:016}", num);

    trace!(
        target: "bitcoinleveldbt-dbtest",
        label = "make_key.exit",
        num,
        out_len = s.len()
    );

    s
}
