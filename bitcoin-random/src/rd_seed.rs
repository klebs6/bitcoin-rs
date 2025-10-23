// ---------------- [ File: bitcoin-random/src/rd_seed.rs ]
crate::ix!();

/**
  | Read 64 bits of entropy using rdseed.
  | 
  | Must only be called when RdSeed is supported.
  |
  */
#[cfg(have_getcpuid)]
pub fn get_rd_seed() -> u64 {

    // RdSeed may fail when the HW RNG is overloaded. 
    //
    // Loop indefinitely until enough entropy is gathered, but pause after every failure.
    //
    #[cfg(target_arch = "x86")]
    {
        let mut ok: u8 = 0;
        let mut r1: u32 = 0;
        let mut r2: u32 = 0;

        loop {
            unsafe {
                // rdseed %eax
                core::arch::asm!(".byte 0x0f, 0xc7, 0xf8; setc {ok}",
                                  out("eax") r1, ok = out(reg_byte) ok,
                                  options(nostack, preserves_flags));
            }
            if ok != 0 { break; }
            unsafe { core::arch::asm!("pause", options(nomem, nostack, preserves_flags)); }
        }
        loop {
            unsafe {
                // rdseed %eax
                core::arch::asm!(".byte 0x0f, 0xc7, 0xf8; setc {ok}",
                                  out("eax") r2, ok = out(reg_byte) ok,
                                  options(nostack, preserves_flags));
            }
            if ok != 0 { break; }
            unsafe { core::arch::asm!("pause", options(nomem, nostack, preserves_flags)); }
        }
        return ((r2 as u64) << 32) | (r1 as u64);
    }

    #[cfg(target_arch = "x86_64")]
    {
        let mut ok: u8 = 0;
        let mut r: u64 = 0;
        loop {
            unsafe {
                // rdseed %rax
                core::arch::asm!(".byte 0x48, 0x0f, 0xc7, 0xf8; setc {ok}",
                                  out("rax") r, ok = out(reg_byte) ok,
                                  options(nostack, preserves_flags));
            }
            if ok != 0 { break; }
            unsafe { core::arch::asm!("pause", options(nomem, nostack, preserves_flags)); }
        }
        return r;
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        panic!("RdSeed is only supported on x86 and x86_64");
    }
}

#[cfg(test)]
mod rd_seed_spec {
    use super::*;

    #[traced_test]
    #[cfg(all(have_getcpuid, any(target_arch = "x86", target_arch = "x86_64")))]
    fn rd_seed_callable_if_supported() {
        // Ensure flags are initialized for this process.
        init_hardware_rand();
        if G_RDSEED_SUPPORTED.load(core::sync::atomic::Ordering::Relaxed) {
            let _ = get_rd_seed(); // loops until success; should not panic on supported HW
        }
    }
}
