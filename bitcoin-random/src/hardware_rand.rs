// ---------------- [ File: bitcoin-random/src/hardware_rand.rs ]
crate::ix!();

///--------------------------------------
#[cfg(have_getcpuid)]
pub fn init_hardware_rand()  {

    let mut eax: u32 = 0;
    let mut ebx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;

    getcpuid(1, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);

    if (ecx & CPUID_F1_ECX_RDRAND) != 0 {
        G_RDRAND_SUPPORTED.store(true, atomic::Ordering::Relaxed);
    }

    getcpuid(7, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);

    if (ebx & CPUID_F7_EBX_RDSEED) != 0 {
        G_RDSEED_SUPPORTED.store(true, atomic::Ordering::Relaxed);
    }
}

/**
  | Access to other hardware random number
  | generators could be added here later,
  | assuming it is sufficiently fast (in
  | the order of a few hundred CPU cycles).
  | 
  | Slower sources should probably be invoked
  | separately, and/or only from
  | 
  | RandAddPeriodic (which is called once
  | a minute).
  |
  */
#[cfg(not(have_getcpuid))] pub fn init_hardware_rand() { }

///-----------------------------------
#[cfg(have_getcpuid)]
pub fn report_hardware_rand()  {

    /*
      | This must be done in a separate function,
      | as InitHardwareRand() may be indirectly
      | called from global constructors, before
      | logging is initialized.
      */
    if G_RDSEED_SUPPORTED.load(atomic::Ordering::Relaxed) {
        log_printf!("Using RdSeed as additional entropy source\n");
    }

    if G_RDRAND_SUPPORTED.load(atomic::Ordering::Relaxed) {
        log_printf!("Using RdRand as an additional entropy source\n");
    }
}

#[cfg(not(have_getcpuid))]
pub fn report_hardware_rand()  { }
