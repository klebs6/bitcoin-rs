// ---------------- [ File: bitcoin-random/src/rd_rand.rs ]
crate::ix!();

/**
  | Read 64 bits of entropy using rdrand.
  | 
  | Must only be called when RdRand is supported.
  |
  */
#[cfg(have_getcpuid)]
pub fn get_rd_rand() -> u64 {
    // 32-bit x86: stitch two 32-bit RDRAND results
    #[cfg(target_arch = "x86")]
    {
        // RdRand may very rarely fail. Invoke it
        // up to 10 times in a loop to reduce this
        // risk.
        let mut ok: u8 = 0;

        // Initialize to 0 to silence a compiler
        // warning that r1 or r2 may be used
        // uninitialized. Even if rdrand fails
        // (!ok) it will set the output to 0, but
        // there is no way that the compiler could
        // know that.
        let mut r1: u32 = 0;
        let mut r2: u32 = 0;

        for _ in 0..10 {
            unsafe {
                // rdrand %eax
                core::arch::asm!(".byte 0x0f, 0xc7, 0xf0; setc {ok}",
                                  out("eax") r1, ok = out(reg_byte) ok,
                                  options(nostack, preserves_flags));
            }
            if ok != 0 { break; }
        }
        for _ in 0..10 {
            unsafe {
                // rdrand %eax
                core::arch::asm!(".byte 0x0f, 0xc7, 0xf0; setc {ok}",
                                  out("eax") r2, ok = out(reg_byte) ok,
                                  options(nostack, preserves_flags));
            }
            if ok != 0 { break; }
        }
        return ((r2 as u64) << 32) | (r1 as u64);
    }

    // 64-bit x86_64: single 64-bit RDRAND
    #[cfg(target_arch = "x86_64")]
    {
        let mut ok: u8 = 0;

        // See above why we initialize to 0.
        let mut r: u64 = 0;

        for _ in 0..10 {

            unsafe {

                // rdrand %rax
                core::arch::asm!(".byte 0x48, 0x0f, 0xc7, 0xf0; setc {ok}",
                                  out("rax") r, ok = out(reg_byte) ok,
                                  options(nostack, preserves_flags));
            }

            if ok != 0 { break; }
        }
        return r;
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        panic!("RdRand is only supported on x86 and x86_64");
    }
}

#[cfg(test)]
mod rd_rand_spec {
    use super::*;

    #[traced_test]
    #[cfg(all(have_getcpuid, any(target_arch = "x86", target_arch = "x86_64")))]
    fn rd_rand_callable_if_supported() {
        // Ensure flags are initialized for this process.
        init_hardware_rand();
        if G_RDRAND_SUPPORTED.load(core::sync::atomic::Ordering::Relaxed) {
            let _ = get_rd_rand(); // should not panic on supported HW
        }
    }
}
