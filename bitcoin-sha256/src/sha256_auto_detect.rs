// ---------------- [ File: bitcoin-sha256/src/sha256_auto_detect.rs ]
crate::ix!();

use std::sync::OnceLock;

pub(crate) struct Sha256Backend {
    pub transform: TransformType,
    pub d64: TransformD64Type,
    pub d64_2: Option<TransformD64Type>,
    pub d64_4: Option<TransformD64Type>,
    pub d64_8: Option<TransformD64Type>,
    pub desc: &'static str,
}

static BACKEND: OnceLock<Sha256Backend> = OnceLock::new();

#[inline]
pub(crate) fn backend() -> &'static Sha256Backend {
    BACKEND.get().expect("backend not initialised; call sha256auto_detect() first")
}

// Thin wrappers, so nobody else touches function pointers directly:
#[inline]
pub unsafe fn dispatch_transform(state: *mut u32, chunk: *const u8, blocks: usize) {
    (backend().transform)(state, chunk, blocks)
}
#[inline]
pub unsafe fn dispatch_d64(out: *mut u8, inp: *const u8) {
    (backend().d64)(out, inp)
}

/// Autodetect and configure the best available SHA‑256 backend.
///
/// Returns a human‑readable description mirroring Bitcoin Core’s string.
pub fn sha256auto_detect() -> String {

    if let Some(b) = BACKEND.get() {
        return b.desc.into();
    }

    let mut desc = "standard";
    let mut b = Sha256Backend {
        transform: sha256_transform_wrapper,
        d64:       transform_d64_scalar,
        d64_2:     None,
        d64_4:     None,
        d64_8:     None,
        desc,
    };

    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::is_x86_feature_detected;

        #[cfg(feature = "enable-shani")]
        if is_x86_feature_detected!("sha") {
            b.transform = sha256_shani::Transform;
            // FIX: single-lane should be the 1-way function, not 2-way
            b.d64       = sha256d64_shani::Transform;      // ← was Transform_2way
            b.d64_2     = Some(sha256d64_shani::Transform_2way);
            desc = "shani(1way,2way)";
        }

        #[cfg(feature = "enable-avx2")]
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("avx") {
            b.d64_8 = Some(sha256d64_avx2::Transform_8way);
            desc = if desc == "standard" { "avx2(8way)" } else { "shani(1way,2way),avx2(8way)" };
        }
    }

    b.desc = desc;
    let _ = BACKEND.set(b);

    // ← NEW: publish the selection to the global pointers used elsewhere
    unsafe {
        use crate::transforms::{
            TRANSFORM, TRANSFORM_D64,
            TRANSFORM_D64_2WAY, TRANSFORM_D64_4WAY, TRANSFORM_D64_8WAY
        };
        TRANSFORM          = backend().transform;
        TRANSFORM_D64      = backend().d64;
        TRANSFORM_D64_2WAY = backend().d64_2;
        TRANSFORM_D64_4WAY = backend().d64_4;
        TRANSFORM_D64_8WAY = backend().d64_8;
    }

    // This now self-tests the *installed* backend
    assert!(crate::self_test(), "backend failed self‑test");
    desc.into()
}
