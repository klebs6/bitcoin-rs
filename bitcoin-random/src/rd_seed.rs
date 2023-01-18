crate::ix!();

/**
  | Read 64 bits of entropy using rdseed.
  | 
  | Must only be called when RdSeed is supported.
  |
  */
#[cfg(have_getcpuid)]
pub fn get_rd_seed() -> u64 {
    
    // RdSeed may fail when the HW RNG is
    // overloaded. Loop indefinitely until enough
    // entropy is gathered, but pause after every
    // failure.
    #[cfg(i386)]
    {
        let ok:  u8 = 0;
        let r1: u32 = 0;
        let r2: u32 = 0;

        loop {

            // rdseed %eax
            asm!{".byte 0x0f, 0xc7, 0xf8; setc %1" 
                : "=a"(r1), "=q"(ok) :: "cc" : "volatile"}; 

            if ok != 0 {
                break;
            }

            asm!{"pause" : "volatile"};
        }

        loop {

            // rdseed %eax
            asm!{".byte 0x0f, 0xc7, 0xf8; setc %1" 
                : "=a"(r2), "=q"(ok) :: "cc" : "volatile" }; 

            if ok != 0 {
                break;
            }

            asm!{"pause" : "volatile"};
        }

        return ((r2 as u64) << 32) | r1;
    }

    #[cfg(x86_64_or_amd64)]
    {
        let ok:  u8 = 0;
        let r1: u64 = 0;

        loop {

            // rdseed %rax
            asm!{".byte 0x48, 0x0f, 0xc7, 0xf8; setc %1" 
                : "=a"(r1), "=q"(ok) :: "cc" : "volatile"}; 

            if ok != 0 {
                break;
            }

            asm!{"pause" : "volatile"};
        }

        return r1;
    }

    panic!{"RdSeed is only supported on x86 and x86_64"}
}
