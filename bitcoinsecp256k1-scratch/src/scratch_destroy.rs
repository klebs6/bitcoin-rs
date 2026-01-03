// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch_destroy.rs ]
crate::ix!();

pub fn scratch_destroy(
        error_callback: *const Callback,
        scratch:        *mut Scratch)  {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            error_callback = error_callback as usize,
            scratch = scratch as usize,
            "scratch_destroy"
        );

        if scratch.is_null() {
            debug!(
                target: "bitcoinsecp256k1_scratch::scratch",
                "scratch_destroy: scratch is null (no-op)"
            );
            return;
        }

        VERIFY_CHECK!((*scratch).alloc_size == 0);

        if memcmp_var(
            (*scratch).magic.as_ptr() as *const libc::c_void,
            b"scratch\0".as_ptr() as *const libc::c_void,
            8,
        ) != 0
        {
            warn!(
                target: "bitcoinsecp256k1_scratch::scratch",
                scratch = scratch as usize,
                "scratch_destroy: invalid scratch space (bad magic)"
            );
            callback_call(error_callback, b"invalid scratch space\0".as_ptr());
            return;
        }

        libc::memset(
            (*scratch).magic.as_mut_ptr() as *mut libc::c_void,
            0,
            (*scratch).magic.len(),
        );

        libc::free(scratch as *mut libc::c_void);

        debug!(
            target: "bitcoinsecp256k1_scratch::scratch",
            "scratch_destroy: destroyed"
        );
    }
}

#[cfg(test)]
mod scratch_destroy_behavior_test_suite {
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
    fn scratch_destroy_is_noop_when_scratch_pointer_is_null() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        scratch_destroy((&cb as *const Callback), core::ptr::null_mut());

        assert_eq!(
            capture.calls.load(Ordering::SeqCst),
            0,
            "destroying a null scratch must not invoke callback"
        );
    }

    #[traced_test]
    fn scratch_destroy_invokes_callback_on_invalid_magic_and_does_not_free() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        unsafe {
            (*scratch).magic[0] ^= 0x01;
        }

        scratch_destroy((&cb as *const Callback), scratch);

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
