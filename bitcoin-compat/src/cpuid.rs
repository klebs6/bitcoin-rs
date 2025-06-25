//! Minimal, **safe‑ish** wrapper around the `CPUID`
//! instruction for x86/x86‑64 platforms.
//!
//! The original C++ helper supported sub‑leaf
//! invocations; in Rust we achieve the same with
//! `core::arch::{x86,x86_64}::__cpuid_count` while
//! adding *robust* `tracing` instrumentation.
//!
//! On non‑x86 targets the function degrades
//! gracefully: it zeroes the output registers and
//! emits a `warn!` so that any accidental usage is
//! immediately obvious in production logs.

// ---------------- [ File: bitcoin-compat/src/cpuid.rs ]
crate::ix!();

/// Execute `CPUID` with the given `leaf` & `subleaf`
/// and return the raw register values in `a`, `b`,
/// `c`, and `d`.
///
/// # Safety
/// *Exactly* the same contract as the original C++:
/// the function has no pre‑conditions beyond the
/// obvious validity of the output pointers.
///
/// The wrapper itself is **safe** to call – the
/// `unsafe` boundary is fully contained inside.
#[inline]
pub fn getcpuid(
    leaf: u32,
    subleaf: u32,
    a: &mut u32,
    b: &mut u32,
    c: &mut u32,
    d: &mut u32,
) {
    trace!(
        target: "compat::cpuid",
        leaf = leaf,
        subleaf = subleaf,
        "enter getcpuid"
    );

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // SAFETY: CPUID is guaranteed to be available
        // on all x86‑64 CPUs and all 32‑bit CPUs that
        // support SSE2 (Bitcoin's minimum baseline).
        unsafe {
            #[cfg(target_arch = "x86")]
            use core::arch::x86::__cpuid_count as __cpuid_count_inner;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64::__cpuid_count as __cpuid_count_inner;

            let r = __cpuid_count_inner(leaf, subleaf);
            *a = r.eax;
            *b = r.ebx;
            *c = r.ecx;
            *d = r.edx;
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        warn!(
            target: "compat::cpuid",
            "getcpuid invoked on unsupported architecture – zeroing outputs"
        );
        *a = 0;
        *b = 0;
        *c = 0;
        *d = 0;
    }

    trace!(
        target: "compat::cpuid",
        eax = *a,
        ebx = *b,
        ecx = *c,
        edx = *d,
        "exit getcpuid"
    );
}
