crate::ix!();



/**
   | Read the hardware time stamp counter when
   | available.
   |
   | See
   | https://en.wikipedia.org/wiki/Time_Stamp_Counter
   | for more information.
   */
#[cfg(all(_MSC_VER,any(_M_IX86,_M_X64)))]
#[inline] pub fn get_performance_counter() -> i64 {
    __rdtsc()
}

#[cfg(i386_but_not_windows)]
#[inline] pub fn get_performance_counter() -> i64 {
    let r: u64 = 0;

    // Constrain the r variable to the eax:edx pair.
    asm!{"rdtsc" : "=A" (r) : "volatile"}; 

    r
}

#[cfg(all(not(_MSC_VER),any(__x86_64__,__amd64__)))]
#[inline] pub fn get_performance_counter() -> i64 {

    let r1: u64 = 0; 
    let r2: u64 = 0;

    // Constrain r1 to rax and r2 to rdx.
    asm!{"rdtsc" : "=a"(r1), "=d"(r2) : "volatile"}; 

    (r2 << 32) | r1
}

/**
   | Fall back to using C++11 clock (usually
   | microsecond or nanosecond precision)
   |
   */
#[cfg(not(any(
            all(_MSC_VER,any(_M_IX86,_M_X64)),
            i386_but_not_windows,
            all(not(_MSC_VER),any(__x86_64__,__amd64__)),
)))]
#[inline] pub fn get_performance_counter() -> i64 {

    quanta::Instant::now().as_u64().try_into().unwrap()
}
