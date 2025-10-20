// ---------------- [ File: bitcoin-random/src/seed.rs ]
crate::ix!();

/**
  | A note on the use of in the seeding functions
  | below:
  | 
  | None of the RNG code should ever throw
  | any exception.
  |
  */
pub fn seed_timestamp(hasher: &mut Sha512)  {
    
    let mut perfcounter: i64 = get_performance_counter();

    hasher.write(
        &mut perfcounter as *mut _ as *mut u8,  
        size_of_val(&perfcounter)
    );
}

pub fn seed_fast(hasher: &mut Sha512)  {

    let mut buffer: [u8; 32] = [0; 32];

    /*
      | Stack pointer to indirectly commit
      | to thread/callstack
      |
      */
    let mut ptr = buffer.as_mut_ptr();

    hasher.write(ptr as *mut u8, size_of_val(&ptr));

    /*
      | Hardware randomness is very fast when
      | available; use it always.
      |
      */
    seed_hardware_fast(hasher);

    /*
      | High-precision timestamp
      |
      */
    seed_timestamp(hasher);
}

pub fn seed_slow(
        hasher: &mut Sha512,
        rng:    &mut RNGState)  {

    let mut buffer: [u8; 32] = [0; 32];

    /*
      | Everything that the 'fast' seeder includes
      |
      */
    seed_fast(hasher);

    /*
      | OS randomness
      |
      */
    get_os_rand(buffer.as_mut_ptr());

    hasher.write(
        buffer.as_mut_ptr(), 
        size_of_val(&buffer)
    );

    /*
      | Add the events hasher into the mix
      |
      */
    rng.seed_events(hasher);

    /*
      | High-precision timestamp.
      |
      | Note that we also commit to a timestamp in
      | the Fast seeder, so we indirectly commit to
      | a benchmark of all the entropy gathering
      | sources in this function).
      */
    seed_timestamp(hasher);
}

/**
  | Extract entropy from rng, strengthen
  | it, and feed it into hasher.
  |
  */
pub fn seed_strengthen(
        hasher:       &mut Sha512,
        rng:          &mut RNGState,
        microseconds: i32)  {

    /*
      | Generate 32 bytes of entropy from the
      | RNG, and a copy of the entropy already
      | in hasher.
      |
      */
    let mut strengthen_seed: [u8; 32] = [0; 32];

    let len = strengthen_seed.len();

    rng.mix_extract(
        unsafe { std::slice::from_raw_parts_mut(strengthen_seed.as_mut_ptr(), len) }, 
        size_of_val(&strengthen_seed), 
        hasher.clone(), 
        false
    );

    /*
      | Strengthen the seed, and feed it into
      | hasher.
      |
      */
    strengthen(&strengthen_seed, microseconds, hasher);
}

pub fn seed_periodic(
        hasher: &mut Sha512,
        rng:    &mut RNGState)  {

    /*
      | Everything that the 'fast' seeder includes
      |
      */
    seed_fast(hasher);

    /*
      | High-precision timestamp
      |
      */
    seed_timestamp(hasher);

    /*
      | Add the events hasher into the mix
      |
      */
    rng.seed_events(hasher);

    /*
      | Dynamic environment data (performance
      | monitoring, ...)
      |
      */
    let old_size = hasher.size();

    rand_add_dynamic_env(hasher);

    log_print!{
        LogFlags::RAND, 
        "Feeding {} bytes of dynamic environment data into RNG\n", 
        hasher.size() - old_size
    };

    /*
      | Strengthen for 10 ms
      |
      */
    seed_strengthen(hasher, rng, 10000);
}

pub fn seed_startup(
        hasher: &mut Sha512,
        rng:    &mut RNGState)  {

    /*
      | Gather 256 bits of hardware randomness,
      | if available
      |
      */
    seed_hardware_slow(hasher);

    /*
      | Everything that the 'slow' seeder includes.
      |
      */
    seed_slow(hasher, rng);

    /*
      | Dynamic environment data (performance
      | monitoring, ...)
      |
      */
    let old_size = hasher.size();

    rand_add_dynamic_env(hasher);

    // Static environment data
    rand_add_static_env(hasher);

    log_print!{
        LogFlags::RAND, 
        "Feeding {} bytes of environment data into RNG\n", 
        hasher.size() - old_size
    };

    /*
      | Strengthen for 100 ms
      |
      */
    seed_strengthen(hasher, rng, 100000);
}
