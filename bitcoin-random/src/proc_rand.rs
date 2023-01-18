crate::ix!();

pub fn proc_rand(
        out:   &mut [u8],
        num:   i32,
        level: RNGLevel)  {

    // Make sure the RNG is initialized first
    // (as all Seed* function possibly need
    // hwrand to be available).
    let rng: &mut RNGState = &mut G_RNG.lock();

    assert!{num <= 32};

    let mut hasher = Sha512::default();

    match level {
        RNGLevel::FAST => {
            seed_fast(&mut hasher);
        }
        RNGLevel::SLOW => {
            seed_slow(&mut hasher, rng);
        }
        RNGLevel::PERIODIC => {
            seed_periodic(&mut hasher, rng);
        }
    }

    let num: usize = num.try_into().unwrap();

    // Combine with and update state
    if !rng.mix_extract(out, num, hasher, false) {

        // On the first invocation, also seed with SeedStartup().
        let mut startup_hasher = Sha512::default();
        seed_startup(&mut startup_hasher, rng);
        rng.mix_extract(out, num, startup_hasher, true);
    }
}
