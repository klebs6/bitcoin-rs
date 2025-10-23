// ---------------- [ File: bitcoin-random/src/seed_hardware.rs ]
crate::ix!();

/**
  | Add 64 bits of entropy gathered from
  | hardware to hasher. Do nothing if not
  | supported.
  |
  */
pub fn seed_hardware_fast(hasher: &mut Sha512)  {
    
    #[cfg(x86_64_or_amd64_or_i386)]
    {
        if G_RDRAND_SUPPORTED.load(Ordering::Relaxed) {

            let out: u64 = get_rd_rand();

            hasher.write(&out as *const _ as *const u8, size_of_val(&out));
        }
    }
}

/**
  | Add 256 bits of entropy gathered from
  | hardware to hasher. Do nothing if not
  | supported.
  |
  */
pub fn seed_hardware_slow(hasher: &mut Sha512)  {
    
    #[cfg(x86_64_or_amd64_or_i386)]
    {
        // When we want 256 bits of entropy,
        // prefer RdSeed over RdRand, as it's
        // guaranteed to produce independent
        // randomness on every call.
        if G_RDSEED_SUPPORTED.load(Ordering::Relaxed) {

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
        if G_RDRAND_SUPPORTED.load(Ordering::Relaxed) {

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
}

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
