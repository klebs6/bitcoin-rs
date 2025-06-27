// ---------------- [ File: bitcoin-crc32c/src/prefetch.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_prefetch.h]

/// Platform‑agnostic “best‑effort” pre‑fetch.  
///
/// On targets where an appropriate intrinsic is unavailable (or disabled at compile‑time) the
/// call degrades to a harmless no‑op.
///
/// Ask the hardware to prefetch the data at the given address into the L1 cache.
///
#[inline]
pub fn request_prefetch(address: *const u8) {
    trace!(?address, "request_prefetch()");
    #[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
    unsafe {
        core::arch::x86_64::_mm_prefetch(address as *const i8,
                                         core::arch::x86_64::_MM_HINT_NTA);
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    unsafe {
        // `pldl1keep` equivalent – the exact intrinsic name may vary
        // between toolchains; this variant is supported by nightly rustc.
        core::arch::aarch64::prefetch_read_data(address, 0);
    }

    // Other targets: nothing to do (silences “unused” warnings).
    #[cfg(not(any(
        all(target_arch = "x86_64", target_feature = "sse"),
        all(target_arch = "aarch64", target_feature = "neon")
    )))]
    {
        let _ = address;
    }
}
