// ---------------- [ File: bitcoin-random/src/proc_rand.rs ]
crate::ix!();

pub fn proc_rand(
    out:   &mut [u8],
    num:   i32,
    level: RNGLevel
) {

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

#[cfg(test)]
mod proc_rand_spec {
    use super::*;

    #[traced_test]
    fn proc_rand_fast_and_slow_produce_up_to_32_bytes() {
        let mut out = [0u8; 32];

        proc_rand(&mut out, 32, RNGLevel::FAST);
        assert!(out.iter().any(|&x| x != 0));

        let mut out2 = [0u8; 32];
        proc_rand(&mut out2, 32, RNGLevel::SLOW);
        assert!(out2.iter().any(|&x| x != 0));
    }

    #[traced_test]
    fn proc_rand_zero_len_is_allowed_and_does_not_panic() {
        let mut out = [0u8; 0];
        proc_rand(&mut out, 0, RNGLevel::FAST);
    }
}
