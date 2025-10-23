// ---------------- [ File: bitcoin-random/src/performance_counter.rs ]
crate::ix!();

/**
   | Read the hardware time stamp counter when
   | available.
   |
   | See
   | https://en.wikipedia.org/wiki/Time_Stamp_Counter
   | for more information.
   */
#[cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
#[inline]
pub fn get_performance_counter() -> i64 {
    // Use RDTSC where available
    unsafe {
        #[cfg(target_arch = "x86")]   { core::arch::x86::_rdtsc() as i64 }
        #[cfg(target_arch = "x86_64")]{ core::arch::x86_64::_rdtsc() as i64 }
    }
}

#[cfg(all(not(windows), target_arch = "x86"))]
#[inline]
pub fn get_performance_counter() -> i64 {
    let mut r_lo: u32 = 0;
    let mut r_hi: u32 = 0;
    unsafe {
        core::arch::asm!("rdtsc", out("eax") r_lo, out("edx") r_hi, options(nostack, preserves_flags));
    }
    ((r_hi as u64) << 32 | (r_lo as u64)) as i64
}

#[cfg(all(not(windows), target_arch = "x86_64"))]
#[inline]
pub fn get_performance_counter() -> i64 {
    let mut r: u64 = 0;
    unsafe {
        // Constrain r1 to rax and r2 to rdx.
        core::arch::asm!("rdtsc", out("rax") r, out("rdx") _, options(nostack, preserves_flags));
    }
    r as i64
}

/**
   | Fall back to using C++11 clock (usually
   | microsecond or nanosecond precision)
   |
   */
#[cfg(not(any(
    all(windows, any(target_arch = "x86", target_arch = "x86_64")),
    all(not(windows), target_arch = "x86"),
    all(not(windows), target_arch = "x86_64"),
)))]
#[inline]
pub fn get_performance_counter() -> i64 {
    quanta::Instant::now().as_u64() as i64
}

#[cfg(test)]
mod performance_counter_spec {
    use super::*;

    #[traced_test]
    fn counter_changes_over_time() {
        let a = get_performance_counter();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let b = get_performance_counter();
        assert_ne!(a, b, "performance counter did not change across 1ms sleep");
    }
}
