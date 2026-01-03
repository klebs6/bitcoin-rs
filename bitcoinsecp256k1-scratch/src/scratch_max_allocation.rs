// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch_max_allocation.rs ]
crate::ix!();

/// Returns the maximum allocation the scratch space will allow
/// 
pub fn scratch_max_allocation(
        error_callback: *const Callback,
        scratch:        *const Scratch,
        n_objects:      usize) -> usize {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            error_callback = error_callback as usize,
            scratch = scratch as usize,
            n_objects,
            "scratch_max_allocation"
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
                "scratch_max_allocation: invalid scratch space (bad magic)"
            );
            callback_call(error_callback, b"invalid scratch space\0".as_ptr());
            return 0;
        }

        let objects: usize = n_objects;

        /* Ensure that multiplication will not wrap around */
        if ALIGNMENT > 1 && objects > (usize::MAX / (ALIGNMENT - 1)) {
            debug!(
                target: "bitcoinsecp256k1_scratch::scratch",
                objects,
                alignment = ALIGNMENT,
                "scratch_max_allocation: overflow guard triggered"
            );
            return 0;
        }

        let overhead: usize = objects.wrapping_mul(ALIGNMENT - 1);
        let available: usize = (*scratch).max_size.wrapping_sub((*scratch).alloc_size);

        if available <= overhead {
            debug!(
                target: "bitcoinsecp256k1_scratch::scratch",
                available,
                overhead,
                "scratch_max_allocation: insufficient space"
            );
            return 0;
        }

        let result: usize = available.wrapping_sub(overhead);

        debug!(
            target: "bitcoinsecp256k1_scratch::scratch",
            available,
            overhead,
            result,
            "scratch_max_allocation: computed"
        );

        result
    }
}

#[cfg(test)]
mod scratch_max_allocation_behavior_test_suite {
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
    fn scratch_max_allocation_computes_available_minus_overhead() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 10);
        assert!(!scratch.is_null());

        let _p = scratch_alloc((&cb as *const Callback), scratch, 1);
        let available = (ALIGNMENT * 10) - ALIGNMENT;

        let objects = 3usize;
        let overhead = objects * (ALIGNMENT - 1);
        let expected = if available <= overhead { 0 } else { available - overhead };

        let got = scratch_max_allocation((&cb as *const Callback), scratch, objects);

        assert_eq!(got, expected);

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);
        scratch_destroy((&cb as *const Callback), scratch);

        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn scratch_max_allocation_returns_zero_when_overhead_exceeds_available() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 2);
        assert!(!scratch.is_null());

        let _p = scratch_alloc((&cb as *const Callback), scratch, 1);

        let objects = ALIGNMENT + 10;
        let got = scratch_max_allocation((&cb as *const Callback), scratch, objects);
        assert_eq!(got, 0);

        scratch_apply_checkpoint((&cb as *const Callback), scratch, 0);
        scratch_destroy((&cb as *const Callback), scratch);

        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn scratch_max_allocation_overflow_guard_returns_zero() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        let got = scratch_max_allocation((&cb as *const Callback), scratch, usize::MAX);
        assert_eq!(got, 0);

        scratch_destroy((&cb as *const Callback), scratch);

        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn scratch_max_allocation_invokes_callback_on_invalid_magic() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT);
        assert!(!scratch.is_null());

        unsafe {
            (*scratch).magic[0] ^= 0x01;
        }

        let got = scratch_max_allocation((&cb as *const Callback), scratch, 1);
        assert_eq!(got, 0);

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
