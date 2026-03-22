crate::ix!();

/**
  | Store in *dst a random string of length
  | "len" and return a Slice that references
  | the generated data.
  |
  */
pub fn random_string(
    rnd: *mut Random,
    len: i32,
    dst: *mut String,
) -> Slice {
    trace!(
        target: "bitcoinleveldb_test::util",
        event = "random_string_entry",
        rnd_is_null = rnd.is_null(),
        dst_is_null = dst.is_null(),
        len = len
    );

    if rnd.is_null() || dst.is_null() {
        error!(
            target: "bitcoinleveldb_test::util",
            event = "random_string_null_input",
            rnd_is_null = rnd.is_null(),
            dst_is_null = dst.is_null()
        );

        return Slice::from_ptr_len(core::ptr::null::<u8>(), 0usize);
    }

    let target_len: usize = if len <= 0 { 0usize } else { len as usize };
    let mut bytes: Vec<u8> = Vec::with_capacity(target_len);

    let mut i: usize = 0usize;
    while i < target_len {
        // ' ' .. '~'
        let sample = unsafe { (&mut *rnd).uniform(95) } as u8;
        bytes.push(b' ' + sample);
        i += 1usize;
    }

    let generated = match String::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    };

    unsafe {
        *dst = generated;
    }

    let out = unsafe { Slice::from(&*dst) };

    trace!(
        target: "bitcoinleveldb_test::util",
        event = "random_string_exit",
        result_len = target_len
    );

    out
}


