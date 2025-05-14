// ---------------- [ File: bitcoin-random/src/strengthen.rs ]
crate::ix!();

/**
  | Use repeated SHA512 to strengthen the
  | randomness in seed32, and feed into
  | hasher.
  |
  */
pub fn strengthen(
        seed:         &[u8; 32],
        microseconds: i32,
        hasher:       &mut Sha512)  {

    let microseconds: i64 = microseconds.into();

    let mut inner_hasher: Sha512 = Sha512::default();

    inner_hasher.write(seed.as_ptr() as *const u8, size_of_val(seed));

    // Hash loop
    let mut buffer: [u8; 64] = unsafe { std::mem::zeroed() };

    let stop: Instant = get_time() + Duration::microseconds(microseconds);

    loop {

        for i in 0..1000 {
            inner_hasher.finalize(buffer);
            inner_hasher.reset();
            inner_hasher.write(buffer.as_mut_ptr() as *const u8, size_of_val(&buffer));
        }

        // Benchmark operation and feed it into outer hasher.
        let perf: i64 = get_performance_counter();

        hasher.write(&perf as *const _ as *const u8, size_of_val(&perf));

        if get_time() >= stop {
            break;
        }
    }

    // Produce output from inner state and feed it to outer hasher.
    inner_hasher.finalize(buffer);
    hasher.write(buffer.as_mut_ptr() as *const u8, size_of_val(&buffer));

    // Try to clean up.
    inner_hasher.reset();

    memory_cleanse(buffer.as_mut_ptr() as *mut c_void, size_of_val(&buffer));
}
