// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch_alloc.rs ]
crate::ix!();

/// Returns a pointer into the most recently allocated frame, or NULL if there is insufficient
/// available space
/// 
pub fn scratch_alloc(
        error_callback: *const Callback,
        scratch:        *mut Scratch,
        size:           usize) -> *mut libc::c_void {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            error_callback = error_callback as usize,
            scratch = scratch as usize,
            size,
            "scratch_alloc"
        );

        let rounded_size: usize = {
            let tmp = size.wrapping_add(ALIGNMENT.wrapping_sub(1));
            (tmp / ALIGNMENT).wrapping_mul(ALIGNMENT)
        };

        /* Check that rounding did not wrap around */
        if rounded_size < size {
            debug!(
                target: "bitcoinsecp256k1_scratch::scratch",
                size,
                rounded_size,
                "scratch_alloc: rounding wrapped"
            );
            return core::ptr::null_mut();
        }

        let size: usize = rounded_size;

        if memcmp_var(
            (*scratch).magic.as_ptr() as *const libc::c_void,
            b"scratch\0".as_ptr() as *const libc::c_void,
            8,
        ) != 0
        {
            warn!(
                target: "bitcoinsecp256k1_scratch::scratch",
                scratch = scratch as usize,
                "scratch_alloc: invalid scratch space (bad magic)"
            );
            callback_call(error_callback, b"invalid scratch space\0".as_ptr());
            return core::ptr::null_mut();
        }

        if size > (*scratch).max_size.wrapping_sub((*scratch).alloc_size) {
            debug!(
                target: "bitcoinsecp256k1_scratch::scratch",
                size,
                alloc_size = (*scratch).alloc_size,
                max_size = (*scratch).max_size,
                "scratch_alloc: insufficient space"
            );
            return core::ptr::null_mut();
        }

        let ret: *mut libc::c_void = ((*scratch).data as *mut u8)
            .add((*scratch).alloc_size) as *mut libc::c_void;

        libc::memset(ret, 0, size);

        (*scratch).alloc_size = (*scratch).alloc_size.wrapping_add(size);

        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            ret = ret as usize,
            new_alloc_size = (*scratch).alloc_size,
            "scratch_alloc: allocated"
        );

        ret
    }
}

#[cfg(test)]
mod scratch_alloc_behavior_test_suite {
    use super::*;

    use std::ffi::CStr;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    #[derive(Default)]
    struct CallbackCapture {
        calls: AtomicUsize,
        messages: Mutex<Vec<String>>,
    }

    fn recording_callback(text: *const u8, data: *mut libc::c_void) {
        unsafe {
            let capture: &CallbackCapture = &*(data as *const CallbackCapture);

            let msg = if text.is_null() {
                "<null>".to_owned()
            } else {
                CStr::from_ptr(text as *const libc::c_char)
                    .to_string_lossy()
                    .into_owned()
            };

            capture.calls.fetch_add(1, Ordering::SeqCst);
            capture.messages.lock().expect("mutex poisoned").push(msg);
        }
    }

    fn mk_error_callback(capture: &CallbackCapture) -> Callback {
        Callback::new(
            recording_callback,
            (capture as *const CallbackCapture).cast::<libc::c_void>(),
        )
    }

    #[traced_test]
    fn scratch_alloc_returns_zeroed_aligned_regions_and_advances_in_alignment_steps() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 3);
        assert!(!scratch.is_null(), "scratch_create unexpectedly returned null");

        let p1 = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert!(!p1.is_null());

        unsafe {
            let bytes = std::slice::from_raw_parts(p1 as *const u8, ALIGNMENT);
            assert!(bytes.iter().all(|&b| b == 0));
        }

        let p2 = scratch_alloc((&cb as *const Callback), scratch, ALIGNMENT);
        assert!(!p2.is_null());
        assert_eq!(
            (p2 as usize).wrapping_sub(p1 as usize),
            ALIGNMENT,
            "second allocation must start ALIGNMENT bytes after the first"
        );

        let p3 = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert!(!p3.is_null());
        assert_eq!(
            (p3 as usize).wrapping_sub(p2 as usize),
            ALIGNMENT,
            "third allocation must start ALIGNMENT bytes after the second"
        );

        let p4 = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert!(
            p4.is_null(),
            "expected allocation to fail due to insufficient remaining space"
        );
        assert_eq!(
            capture.calls.load(Ordering::SeqCst),
            0,
            "insufficient space must not invoke callback"
        );

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);
        scratch_destroy((&cb as *const Callback), scratch);
    }

    #[traced_test]
    fn scratch_alloc_detects_rounding_wrap_and_returns_null_without_callback() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        let size = usize::MAX - (ALIGNMENT - 2);
        let p = scratch_alloc((&cb as *const Callback), scratch, size);
        assert!(p.is_null());

        assert_eq!(
            capture.calls.load(Ordering::SeqCst),
            0,
            "rounding wrap must not invoke callback"
        );

        scratch_destroy((&cb as *const Callback), scratch);
    }

    #[traced_test]
    fn scratch_alloc_invokes_callback_on_invalid_magic() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        unsafe {
            (*scratch).magic[0] ^= 0x01;
        }

        let p = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert!(p.is_null());

        let calls = capture.calls.load(Ordering::SeqCst);
        let messages = capture.messages.lock().expect("mutex poisoned").clone();

        assert_eq!(calls, 1);
        assert_eq!(messages.last().map(String::as_str), Some("invalid scratch space"));

        unsafe {
            libc::memcpy(
                (*scratch).magic.as_mut_ptr().cast::<libc::c_void>(),
                b"scratch\0".as_ptr().cast::<libc::c_void>(),
                8,
            );
        }

        scratch_destroy((&cb as *const Callback), scratch);
    }
}
