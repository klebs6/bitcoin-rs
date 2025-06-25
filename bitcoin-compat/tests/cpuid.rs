// ---------------- [ File: bitcoin-compat/tests/cpuid.rs ]
//! Interface‑level tests for the `getcpuid` wrapper
//! on x86/x86‑64.  
//!
//! The goal is **not** to validate vendor‑specific
//! feature bits – that would break on CI runners
//! with diverse hardware – but to ensure the basic
//! call path works and the register contract is
//! upheld.
use bitcoin_compat::*;
use bitcoin_imports::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod cpuid_tests {
    use super::*;

    /// Invoke `getcpuid` for `leaf 0 / subleaf 0` and
    /// ensure the returned maximum‑leaf value in `EAX`
    /// is at least `0x0000_0001` (all modern CPUs
    /// support basic feature leaf 1).
    #[traced_test]
    fn basic_cpuid_sanity() {
        let mut eax = 0u32;
        let mut ebx = 0u32;
        let mut ecx = 0u32;
        let mut edx = 0u32;

        getcpuid(0, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);

        // The CPUID specification guarantees that
        // EAX[31:0] for leaf 0 contains the maximum
        // supported basic leaf – this must be ≥ 1.
        assert!(
            eax >= 1,
            "unexpected max‑leaf value returned by CPUID: {:#x}",
            eax
        );

        // EBX/ECX/EDX contain the vendor string and
        // should therefore be *printable* ASCII.
        let vendor_bytes = [
            ebx.to_le_bytes(),
            edx.to_le_bytes(),
            ecx.to_le_bytes(),
        ]
        .concat();

        for &b in &vendor_bytes {
            assert!(
                (b as char).is_ascii_graphic(),
                "non‑ASCII vendor byte: 0x{b:02x}"
            );
        }
    }
}
