// ---------------- [ File: bitcoinleveldb-testutil/src/compressible_string.rs ]
crate::ix!();

/**
  | Store in *dst a string of length "len" that
  | will compress to "N*compressed_fraction" bytes
  | and return a Slice that references the
  | generated data.
  */
pub fn compressible_string(
    rnd:                 *mut Random,
    compressed_fraction: f64,
    len:                 usize,
    dst:                 *mut String,
) -> Slice {
    trace!(
        target: "bitcoinleveldb_test::util",
        event = "compressible_string_entry",
        rnd_is_null = rnd.is_null(),
        dst_is_null = dst.is_null(),
        compressed_fraction = compressed_fraction,
        len = len
    );

    if dst.is_null() {
        error!(
            target: "bitcoinleveldb_test::util",
            event = "compressible_string_null_dst"
        );

        return Slice::from_ptr_len(core::ptr::null::<u8>(), 0usize);
    }

    let mut raw: i32 = (len as f64 * compressed_fraction) as i32;
    if raw < 1 {
        raw = 1;
    }

    let mut raw_data = String::new();
    let _raw_slice = random_string(
        rnd,
        raw,
        (&mut raw_data) as *mut String,
    );

    // Duplicate the random data until we have filled "len" bytes
    unsafe {
        (*dst).clear();

        while (&(*dst)).len() < len {
            (*dst).push_str(raw_data.as_str());
        }

        (*dst).truncate(len);
    }

    let out = unsafe { Slice::from(&*dst) };

    trace!(
        target: "bitcoinleveldb_test::util",
        event = "compressible_string_exit",
        raw_len = raw,
        result_len = len
    );

    out
}
