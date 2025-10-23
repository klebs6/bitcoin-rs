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
    
    #[cfg(i386)]
    {
        // RdRand may very rarely fail. Invoke it
        // up to 10 times in a loop to reduce this
        // risk.
        let ok: u8 = 0;

        // Initialize to 0 to silence a compiler
        // warning that r1 or r2 may be used
        // uninitialized. Even if rdrand fails
        // (!ok) it will set the output to 0, but
        // there is no way that the compiler could
        // know that.
        let r1: u32 = 0; 
        let r2: u32 = 0;

        for i in 0..10 {

            // rdrand %eax
            asm!{".byte 0x0f, 0xc7, 0xf0; setc %1" 
                : "=a"(r1), "=q"(ok) :: "cc" : "volatile"}; 

            if ok != 0 {
                break;
            }
        }

        for i in 0..10 {

            // rdrand %eax
            asm!{".byte 0x0f, 0xc7, 0xf0; setc %1" 
                : "=a"(r2), "=q"(ok) :: "cc" : "volatile"}; 

            if ok != 0 {
                break;
            }
        }

        ((r2 as u64) << 32) | r1
    }

    #[cfg(x86_64_or_amd64)]
    {
        let ok: u8 = 0;

        // See above why we initialize to 0.
        let r1: u64 = 0; 

        for i in 0..10 {

            // rdrand %rax
            asm!{".byte 0x48, 0x0f, 0xc7, 0xf0; setc %1" 
                : "=a"(r1), "=q"(ok) :: "cc", "volatile"}; 

            if ok != 0 {
                break;
            }
        }

        return r1;
    }

    panic!{"RdRand is only supported on x86 and x86_64"};
}

#[cfg(test)]
mod rd_rand_spec {
    use super::*;

    #[traced_test]
    #[cfg(have_getcpuid)]
    fn rd_rand_callable_if_supported() {
        // Only invoke if the flag says it’s supported; otherwise, skip.
        if G_RDRAND_SUPPORTED.load(core::sync::atomic::Ordering::Relaxed) {
            let _ = get_rd_rand(); // no assertion beyond successful call
        }
    }
}
