// ---------------- [ File: bitcoinsecp256k1-scratch/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{callback_call}
x!{checked_malloc}
x!{checked_realloc}
x!{ctz32_var}
x!{ctz32_var_debruijn}
x!{ctz64_var}
x!{ctz64_var_debruijn}
x!{int_cmov}
x!{manual_alloc}
x!{memcmp_var}
x!{memczero}
x!{scratch}
x!{scratch_alloc}
x!{scratch_apply_checkpoint}
x!{scratch_checkpoint}
x!{scratch_create}
x!{scratch_destroy}
x!{scratch_max_allocation}
x!{util}

#[cfg(test)]
mod crate_root_api_smoke_test_suite {
    use super::*;

    use std::ffi::CStr;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Default)]
    struct CallbackCapture {
        calls: AtomicUsize,
    }

    fn counting_callback(_text: *const u8, data: *mut libc::c_void) {
        unsafe {
            let capture: &CallbackCapture = &*(data as *const CallbackCapture);
            capture.calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[traced_test]
    fn crate_root_exports_are_callable_end_to_end_smoke() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            counting_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        callback_call((&cb as *const Callback), b"ping\0".as_ptr());
        assert_eq!(capture.calls.load(Ordering::SeqCst), 1);

        assert_eq!(ctz32_var(1), 0);
        assert_eq!(ctz64_var(1), 0);
        assert_eq!(ctz32_var_debruijn(8), 3);
        assert_eq!(ctz64_var_debruijn(8), 3);

        let mut r: i32 = 1;
        let a: i32 = 2;
        int_cmov(&mut r as *mut i32, &a as *const i32, 1);
        assert_eq!(r, 2);

        let p = checked_malloc((&cb as *const Callback), 8);
        if !p.is_null() {
            unsafe { libc::free(p) };
        } else {
            assert_eq!(
                capture.calls.load(Ordering::SeqCst),
                2,
                "allocation failure should have invoked callback"
            );
        }

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        if !scratch.is_null() {
            let q = scratch_alloc((&cb as *const Callback), scratch, 0);
            assert!(!q.is_null());

            scratch_destroy((&cb as *const Callback), scratch);
        }

        let msg = unsafe { CStr::from_ptr(b"scratch\0".as_ptr() as *const libc::c_char) }
            .to_string_lossy()
            .into_owned();
        assert_eq!(msg, "scratch");
    }
}
