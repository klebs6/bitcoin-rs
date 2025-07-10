// ---------------- [ File: bitcoin-sha256/src/transforms.rs ]
crate::ix!();

pub type TransformType    = unsafe fn(*mut u32, *const u8, usize);
pub type TransformD64Type = unsafe fn(*mut u8,  *const u8);

#[cfg(target_arch = "x86_64")]
use std::arch::is_x86_feature_detected;

/// Fallback block‑transform stub; used only when no optimised
/// backend is selected (kept for completeness).
pub unsafe fn transform_d64_stub(_out: *mut u8, _inp: *const u8) {}

/// Global dispatch table – *never* access directly outside this module.
#[allow(clippy::declare_interior_mutable_const)]
pub(crate) static mut TRANSFORM:            TransformType      = sha256_transform_wrapper;

#[allow(clippy::declare_interior_mutable_const)]
pub(crate) static mut TRANSFORM_D64: TransformD64Type = transform_d64_scalar;

#[allow(clippy::declare_interior_mutable_const)]
pub(crate) static mut TRANSFORM_D64_2WAY:   Option<TransformD64Type> = None;

#[allow(clippy::declare_interior_mutable_const)]
pub(crate) static mut TRANSFORM_D64_4WAY:   Option<TransformD64Type> = None;

#[allow(clippy::declare_interior_mutable_const)]
pub(crate) static mut TRANSFORM_D64_8WAY:   Option<TransformD64Type> = None;

/// Rust façade matching the C++ `sha256::Transform` (64‑byte blocks).
///
/// * `state` – pointer to 8‑word SHA‑256 state array.
/// * `chunk` – pointer to the first byte of the first 64‑byte block.
/// * `blocks` – number of 64‑byte blocks to process.
///
/// This simply calls the original single‑block transform in a loop.
#[inline]
pub unsafe fn sha256_transform_wrapper(
    state:  *mut u32,
    chunk:  *const u8,
    blocks: usize,
) {
    for i in 0..blocks {
        let off = chunk.add(i * 64);
        sha256_transform_block(state, off);
    }
}
