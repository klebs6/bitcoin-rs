// ---------------- [ File: bitcoinleveldb-log/src/append_number_to.rs ]
crate::ix!();

/**
  | Append a human-readable printout of
  | "num" to *str
  |
  */
pub fn append_number_to(str_: *mut String, num: u64) {
    trace!("append_number_to: num={}", num);

    if str_.is_null() {
        error!("append_number_to: received null String pointer");
        return;
    }

    unsafe {
        let s: &mut String = &mut *str_;
        use std::fmt::Write as _;
        let _ = write!(s, "{}", num);
    }
}
