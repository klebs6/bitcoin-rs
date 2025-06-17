crate::ix!();

/// ShiftRows (forward)
#[inline(always)]
pub fn shift_rows(s: *mut AESState) {
    tracing::trace!(target: "aes", "shift_rows – entry {:p}", s);

    unsafe {
        for word in &mut (*s).slice {
            let v = *word;
            *word = (v & bit_range!(0, 4))
                | bit_range_left!(v, 4, 5, 3)
                | bit_range_right!(v, 5, 8, 1)
                | bit_range_left!(v, 8, 10, 2)
                | bit_range_right!(v, 10, 12, 2)
                | bit_range_left!(v, 12, 15, 1)
                | bit_range_right!(v, 15, 16, 3);
        }
    }

    tracing::trace!(target: "aes", "shift_rows – exit");
}

/// Inverse ShiftRows
#[inline(always)]
pub fn inv_shift_rows(s: *mut AESState) {
    tracing::trace!(target: "aes", "inv_shift_rows – entry {:p}", s);

    unsafe {
        for word in &mut (*s).slice {
            let v = *word;
            *word = (v & bit_range!(0, 4))
                | bit_range_left!(v, 4, 7, 1)
                | bit_range_right!(v, 7, 8, 3)
                | bit_range_left!(v, 8, 10, 2)
                | bit_range_right!(v, 10, 12, 2)
                | bit_range_left!(v, 12, 13, 3)
                | bit_range_right!(v, 13, 16, 1);
        }
    }

    tracing::trace!(target: "aes", "inv_shift_rows – exit");
}
