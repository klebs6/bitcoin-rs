// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch_checkpoint.rs ]
crate::ix!();

/// Returns an opaque object used to "checkpoint" a scratch space. Used with
/// `scratch_apply_checkpoint` to undo allocations.
/// 
pub fn scratch_checkpoint(
        error_callback: *const Callback,
        scratch:        *const Scratch) -> usize {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            error_callback = error_callback as usize,
            scratch = scratch as usize,
            "scratch_checkpoint"
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
                "scratch_checkpoint: invalid scratch space (bad magic)"
            );
            callback_call(error_callback, b"invalid scratch space\0".as_ptr());
            return 0;
        }

        (*scratch).alloc_size
    }
}

#[cfg(test)]
mod scratch_checkpoint_behavior_test_suite {
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
    fn scratch_checkpoint_returns_current_allocation_offset() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 2);
        assert!(!scratch.is_null());

        let cp0 = scratch_checkpoint((&cb as *const Callback), scratch);
        assert_eq!(cp0, 0);

        let _p = scratch_alloc((&cb as *const Callback), scratch, 1);
        let cp1 = scratch_checkpoint((&cb as *const Callback), scratch);
        assert_eq!(cp1, ALIGNMENT);

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);
        scratch_destroy((&cb as *const Callback), scratch);

        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn scratch_checkpoint_invokes_callback_and_returns_zero_on_invalid_magic() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        unsafe {
            (*scratch).magic[0] ^= 0x01;
        }

        let cp = scratch_checkpoint((&cb as *const Callback), scratch);
        assert_eq!(cp, 0);

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
