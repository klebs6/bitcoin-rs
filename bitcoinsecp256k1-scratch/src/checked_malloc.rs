// ---------------- [ File: bitcoinsecp256k1-scratch/src/checked_malloc.rs ]
crate::ix!();

#[inline] pub fn checked_malloc(
        cb:   *const Callback,
        size: usize) -> *mut libc::c_void {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            cb = cb as usize,
            size,
            "checked_malloc"
        );

        let ret: *mut libc::c_void = libc::malloc(size);

        if ret.is_null() {
            error!(
                target: "bitcoinsecp256k1_scratch::util",
                size,
                "checked_malloc: out of memory"
            );
            callback_call(cb, b"Out of memory\0".as_ptr());
        } else {
            debug!(
                target: "bitcoinsecp256k1_scratch::util",
                size,
                ret = ret as usize,
                "checked_malloc: allocated"
            );
        }

        ret
    }
}

#[cfg(test)]
mod checked_malloc_contract_test_suite {
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

            debug!(
                target: "bitcoinsecp256k1_scratch::tests::checked_malloc",
                msg = %msg,
                "recording_callback: observed"
            );

            capture.calls.fetch_add(1, Ordering::SeqCst);
            capture.messages.lock().expect("mutex poisoned").push(msg);
        }
    }

    #[traced_test]
    fn checked_malloc_allocates_small_blocks_without_triggering_callback() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        let p = checked_malloc((&cb as *const Callback), 32);

        assert!(
            !p.is_null(),
            "checked_malloc returned null for a small allocation"
        );

        unsafe {
            let bytes = std::slice::from_raw_parts_mut(p as *mut u8, 32);
            bytes[0] = 0xAB;
            bytes[31] = 0xCD;
            libc::free(p);
        }

        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn checked_malloc_reports_out_of_memory_when_allocator_returns_null() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        let p = checked_malloc((&cb as *const Callback), usize::MAX);

        if p.is_null() {
            let calls = capture.calls.load(Ordering::SeqCst);
            let messages = capture.messages.lock().expect("mutex poisoned").clone();

            assert_eq!(calls, 1);
            assert_eq!(messages.last().map(String::as_str), Some("Out of memory"));
        } else {
            unsafe { libc::free(p) };
            assert_eq!(
                capture.calls.load(Ordering::SeqCst),
                0,
                "callback should not be invoked if allocation succeeded"
            );
        }
    }

    #[traced_test]
    fn checked_malloc_size_zero_is_handled_consistently_with_allocator_behavior() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        let p = checked_malloc((&cb as *const Callback), 0);

        if p.is_null() {
            let calls = capture.calls.load(Ordering::SeqCst);
            assert_eq!(
                calls, 1,
                "if malloc(0) returned null, checked_malloc should report it as OOM"
            );
        } else {
            unsafe { libc::free(p) };
            assert_eq!(
                capture.calls.load(Ordering::SeqCst),
                0,
                "if malloc(0) returned non-null, checked_malloc must not report OOM"
            );
        }
    }
}
