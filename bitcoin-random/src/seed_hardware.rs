// ---------------- [ File: bitcoin-random/src/seed_hardware.rs ]
crate::ix!();

/**
  | Add 64 bits of entropy gathered from
  | hardware to hasher. Do nothing if not
  | supported.
  |
  */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn seed_hardware_fast(hasher: &mut Sha512)  {
    
    if G_RDRAND_SUPPORTED.load(atomic::Ordering::Relaxed) {

        let out: u64 = get_rd_rand();

        hasher.write(&out as *const _ as *const u8, size_of_val(&out));
    }

}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn seed_hardware_fast(_hasher: &mut Sha512) {}

/**
  | Add 256 bits of entropy gathered from
  | hardware to hasher. Do nothing if not
  | supported.
  |
  */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn seed_hardware_slow(hasher: &mut Sha512)  {
    
    // When we want 256 bits of entropy,
    // prefer RdSeed over RdRand, as it's
    // guaranteed to produce independent
    // randomness on every call.
    //
    if G_RDSEED_SUPPORTED.load(atomic::Ordering::Relaxed) {

        for i in 0..4 {

            let out: u64 = get_rd_seed();

            hasher.write(&out as *const _ as *const u8, size_of_val(&out));
        }

        return;
    }

    // When falling back to RdRand, XOR the
    // result of 1024 results.
    //
    // This guarantees a reseeding occurs
    // between each.
    if G_RDRAND_SUPPORTED.load(atomic::Ordering::Relaxed) {

        for i in 0..4 {

            let mut out: u64 = 0;

            for j in 0..1024 {
                out ^= get_rd_rand();
            }

            hasher.write(&out as *const _ as *const u8, size_of_val(&out));
        }

        return;
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn seed_hardware_slow(_hasher: &mut Sha512) {}

#[cfg(test)]
mod seed_hardware_spec {
    use super::*;

    #[traced_test]
    fn calling_seed_hardware_functions_is_safe_under_all_cfgs() {
        let mut h = Sha512::default();
        let before = h.size();
        seed_hardware_fast(&mut h);
        seed_hardware_slow(&mut h);
        // Not asserting size increased (depends on HW support); just ensure no panic.
        let _ = (before, h.size());
    }
}
