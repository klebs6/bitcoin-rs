// ---------------- [ File: bitcoin-random/src/sanity.rs ]
crate::ix!();

/**
  | Check that OS randomness is available
  | and returning the requested number
  | of bytes.
  |
  */
pub fn random_sanity_check() -> bool {

    let mut start: u64 = get_performance_counter().try_into().unwrap();

    /*
      | This does not measure the quality of
      | randomness, but it does test that
      | GetOSRand() overwrites all 32 bytes
      | of the output given a maximum number
      | of tries.
      |
      */
    const MAX_TRIES: i32 = 1024;

    let mut data: [u8; NUM_OS_RANDOM_BYTES as usize] = [0; NUM_OS_RANDOM_BYTES as usize];

    /*
      | Tracks which bytes have been overwritten
      | at least once
      |
      */
    let mut overwritten: [bool; NUM_OS_RANDOM_BYTES as usize] = [false; NUM_OS_RANDOM_BYTES as usize];

    let mut num_overwritten: i32 = 0;
    let mut tries:           i32 = 0;

    /*
      | Loop until all bytes have been overwritten
      | at least once, or max number tries reached
      |
      */
    loop {

        unsafe {
            libc::memset(
                data.as_mut_ptr() as *mut c_void, 
                0, 
                NUM_OS_RANDOM_BYTES as usize
            );
        }

        get_os_rand(data.as_mut_ptr());

        for x in 0..NUM_OS_RANDOM_BYTES {
            overwritten[x as usize] |= data[x as usize] != 0;
        }

        num_overwritten = 0;

        for x in 0..NUM_OS_RANDOM_BYTES {
            if overwritten[x as usize] {
                num_overwritten += 1;
            }
        }

        tries += 1;

        let onward: bool = num_overwritten < NUM_OS_RANDOM_BYTES && tries < MAX_TRIES;

        if !onward {
            break;
        }
    } 

    if num_overwritten != NUM_OS_RANDOM_BYTES {

        /*
          | If this failed, bailed out after too
          | many tries
          |
          */
        return false; 
    }

    /*
      | Check that GetPerformanceCounter
      | increases at least during a GetOSRand()
      | call + 1ms sleep.
      |
      */
    std::thread::sleep(ONE_MILLISECOND.try_into().unwrap());

    let mut stop: u64 = get_performance_counter().try_into().unwrap();

    if stop == start {
        return false;
    }

    /*
      | We called GetPerformanceCounter.
      | Use it as entropy.
      |
      */
    let mut to_add: Sha512 = Sha512::default();

    to_add.write(&mut start as *mut u64 as *mut u8, size_of_val(&start));
    to_add.write(&mut stop  as *mut u64 as *mut u8, size_of_val(&stop));

    G_RNG.lock().mix_extract(&mut [], 0, to_add, false);

    true
}
