// ---------------- [ File: bitcoin-string/src/to_integral.rs ]
crate::ix!();

/// Convert **`str_`** to an integral value of type **`T`**.
///
/// * Rejects leading or trailing whitespace.
/// * Rejects a leading **`+`**.
/// * Accepts a leading **`-`** *only* for signed integer targets.
/// * Rejects any trailing characters.
///
/// Returns `None` if parsing fails or the value is out‑of‑range.
pub fn to_integral<T>(str_: &str) -> Option<T>
where
    T: num_traits::PrimInt + std::str::FromStr + std::fmt::Debug,
{
    // Disallow leading/trailing whitespace.
    if str_.trim() != str_ {
        trace!(target: "to_integral", input = str_, "whitespace rejected");
        return None;
    }

    // Explicitly forbid a leading ‘+’.
    if str_.starts_with('+') {
        trace!(target: "to_integral", input = str_, "leading '+' rejected");
        return None;
    }

    match str_.parse::<T>() {
        Ok(val) => {
            debug!(target: "to_integral", input = str_, ?val, "parse successful");
            Some(val)
        }
        Err(_) => {
            trace!(target: "to_integral", input = str_, "parse failed");
            None
        }
    }
}

