// ---------------- [ File: bitcoin-crc32c/src/round_up.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_round_up.h]

/// Round an integer value **up** to the next multiple of `N`.
///
/// `N` must be a power‑of‑two; enforced at **compile‑time**.
#[inline]
pub const fn round_up_with_uintptr<const N: usize>(pointer: usize) -> usize {
    ["N must be a power of two"][(N & (N - 1) != 0) as usize];
    (pointer + (N - 1)) & !(N - 1)
}

/// Round a raw byte pointer **up** to the next `N`‑byte‑aligned address.
///
/// *Not* a `const fn` (Rust forbids pointer→integer casts in CTFE).
#[inline]
pub fn round_up<const N: usize>(pointer: *const u8) -> *const u8 {
    let rounded = round_up_with_uintptr::<N>(pointer as usize) as *const u8;
    trace!(?pointer, ?rounded, alignment = N, "round_up(pointer)");
    rounded
}
