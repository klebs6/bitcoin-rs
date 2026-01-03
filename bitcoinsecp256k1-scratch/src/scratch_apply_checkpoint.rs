// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch_apply_checkpoint.rs ]
crate::ix!();

/// Applies a check point received from `scratch_checkpoint`, undoing all allocations since that
/// point.
/// 
pub fn scratch_apply_checkpoint(
        error_callback: *const Callback,
        scratch:        *mut Scratch,
        checkpoint:     usize)  {

    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            error_callback = error_callback as usize,
            scratch = scratch as usize,
            checkpoint,
            "scratch_apply_checkpoint"
        );

        if memcmp_var(
            (*scratch).magic.as_ptr() as *const libc::c_void,
            b"scratch\0".as_ptr() as *const libc::c_void,
            8,
        ) != 0
        {
            warn!(
                target: "bitcoinsecp256k1_scratch::scratch",
                scratch = scratch as usize,
                "scratch_apply_checkpoint: invalid scratch space (bad magic)"
            );
            callback_call(error_callback, b"invalid scratch space\0".as_ptr());
            return;
        }

        if checkpoint > (*scratch).alloc_size {
            warn!(
                target: "bitcoinsecp256k1_scratch::scratch",
                checkpoint,
                alloc_size = (*scratch).alloc_size,
                "scratch_apply_checkpoint: invalid checkpoint"
            );
            callback_call(error_callback, b"invalid checkpoint\0".as_ptr());
            return;
        }

        (*scratch).alloc_size = checkpoint;

        debug!(
            target: "bitcoinsecp256k1_scratch::scratch",
            scratch = scratch as usize,
            alloc_size = (*scratch).alloc_size,
            "scratch_apply_checkpoint: applied"
        );
    }
}

#[cfg(test)]
mod scratch_apply_checkpoint_behavior_test_suite {
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
    fn scratch_apply_checkpoint_rewinds_allocations_and_allows_reuse_with_zeroing() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 3);
        assert!(!scratch.is_null());

        let p1 = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert!(!p1.is_null());

        let cp = scratch_checkpoint((&cb as *const Callback), scratch);
        assert_eq!(cp, ALIGNMENT);

        let p2 = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert!(!p2.is_null());

        unsafe {
            let bytes = std::slice::from_raw_parts_mut(p2 as *mut u8, ALIGNMENT);
            bytes[0] = 0xCC;
            bytes[ALIGNMENT - 1] = 0xDD;
        }

        scratch_apply_checkpoint((&cb as *const Callback), scratch, cp);

        let p3 = scratch_alloc((&cb as *const Callback), scratch, 1);
        assert_eq!(p3, p2, "allocation after rewind should reuse the same address");

        unsafe {
            let bytes = std::slice::from_raw_parts(p3 as *const u8, ALIGNMENT);
            assert!(bytes.iter().all(|&b| b == 0), "reused region must be zeroed by scratch_alloc");
        }

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);
        scratch_destroy((&cb as *const Callback), scratch);
    }

    #[traced_test]
    fn scratch_apply_checkpoint_invokes_callback_on_invalid_checkpoint() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 2);
        assert!(!scratch.is_null());

        let _p = scratch_alloc((&cb as *const Callback), scratch, 1);
        let alloc_before = scratch_checkpoint((&cb as *const Callback), scratch);
        assert_eq!(alloc_before, ALIGNMENT);

        scratch_apply_checkpoint((&cb as *const Callback), scratch, alloc_before + 1);

        let calls = capture.calls.load(Ordering::SeqCst);
        let messages = capture.messages.lock().expect("mutex poisoned").clone();

        assert_eq!(calls, 1);
        assert_eq!(messages.last().map(String::as_str), Some("invalid checkpoint"));

        let alloc_after = scratch_checkpoint((&cb as *const Callback), scratch);
        assert_eq!(alloc_after, alloc_before, "invalid checkpoint must not modify alloc_size");

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);
        scratch_destroy((&cb as *const Callback), scratch);
    }

    #[traced_test]
    fn scratch_apply_checkpoint_invokes_callback_on_invalid_magic() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        unsafe {
            (*scratch).magic[0] ^= 0x01;
        }

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);

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
