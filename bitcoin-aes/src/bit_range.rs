crate::ix!();

/// Inclusive lower‑bound, exclusive upper‑bound bit‑range mask.
#[macro_export]
macro_rules! bit_range {
    ($from:expr, $to:expr) => {
        (((1u16 << ($to - $from)) - 1) << $from)
    };
}

/// Mask + left‑shift a sub‑range of bits.
#[macro_export]
macro_rules! bit_range_left {
    ($x:expr, $from:expr, $to:expr, $shift:expr) => {
        (($x & bit_range!($from, $to)) << $shift)
    };
}

/// Mask + right‑shift a sub‑range of bits.
#[macro_export]
macro_rules! bit_range_right {
    ($x:expr, $from:expr, $to:expr, $shift:expr) => {
        (($x & bit_range!($from, $to)) >> $shift)
    };
}

/// Rotate 4‑bit “nibbles” inside a 16‑bit word.
#[macro_export]
macro_rules! rot {
    ($x:expr, $b:expr) => {
        (($x >> ($b * 4)) | ($x << ((4 - $b) * 4)))
    };
}
