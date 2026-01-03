// ---------------- [ File: bitcoinsecp256k1-scratch/src/checked_realloc.rs ]
crate::ix!();

#[inline] pub fn checked_realloc(
        cb:   *const Callback,
        ptr:  *mut libc::c_void,
        size: usize) -> *mut libc::c_void {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            cb = cb as usize,
            ptr = ptr as usize,
            size,
            "checked_realloc"
        );

        let ret: *mut libc::c_void = libc::realloc(ptr, size);

        if ret.is_null() {
            error!(
                target: "bitcoinsecp256k1_scratch::util",
                ptr = ptr as usize,
                size,
                "checked_realloc: out of memory"
            );
            callback_call(cb, b"Out of memory\0".as_ptr());
        } else {
            debug!(
                target: "bitcoinsecp256k1_scratch::util",
                old_ptr = ptr as usize,
                new_ptr = ret as usize,
                size,
                "checked_realloc: reallocated"
            );
        }

        ret
    }
}

#[cfg(test)]
mod checked_realloc_contract_test_suite {
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

            warn!(
                target: "bitcoinsecp256k1_scratch::tests::checked_realloc",
                msg = %msg,
                "recording_callback: observed"
            );

            capture.calls.fetch_add(1, Ordering::SeqCst);
            capture.messages.lock().expect("mutex poisoned").push(msg);
        }
    }

    #[traced_test]
    fn checked_realloc_grow_preserves_prefix_on_success() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        let original = unsafe { libc::malloc(16) };
        assert!(!original.is_null(), "malloc(16) unexpectedly returned null");

        unsafe {
            let s = std::slice::from_raw_parts_mut(original as *mut u8, 16);
            for b in s.iter_mut() {
                *b = 0xAA;
            }
        }

        let resized = checked_realloc((&cb as *const Callback), original, 32);

        if resized.is_null() {
            let calls = capture.calls.load(Ordering::SeqCst);
            assert_eq!(calls, 1);
            unsafe {
                let s = std::slice::from_raw_parts(original as *const u8, 16);
                assert!(s.iter().all(|&b| b == 0xAA));
                libc::free(original);
            }
        } else {
            assert_eq!(capture.calls.load(Ordering::SeqCst), 0);

            unsafe {
                let s = std::slice::from_raw_parts(resized as *const u8, 16);
                assert!(s.iter().all(|&b| b == 0xAA));
                libc::free(resized);
            }
        }
    }

    #[traced_test]
    fn checked_realloc_with_null_pointer_behaves_like_malloc() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        let p = checked_realloc((&cb as *const Callback), core::ptr::null_mut(), 32);
        assert!(!p.is_null(), "realloc(NULL, 32) returned null unexpectedly");

        unsafe { libc::free(p) };
        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }

    #[traced_test]
    fn checked_realloc_reports_out_of_memory_when_allocator_returns_null_and_preserves_original() {
        let capture = CallbackCapture::default();
        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        let original = unsafe { libc::malloc(16) };
        assert!(!original.is_null(), "malloc(16) unexpectedly returned null");

        unsafe {
            let s = std::slice::from_raw_parts_mut(original as *mut u8, 16);
            s[0] = 0x5A;
        }

        let resized = checked_realloc((&cb as *const Callback), original, usize::MAX);

        if resized.is_null() {
            let calls = capture.calls.load(Ordering::SeqCst);
            let messages = capture.messages.lock().expect("mutex poisoned").clone();

            assert_eq!(calls, 1);
            assert_eq!(messages.last().map(String::as_str), Some("Out of memory"));

            unsafe {
                let s = std::slice::from_raw_parts(original as *const u8, 16);
                assert_eq!(s[0], 0x5A, "original allocation should remain valid on realloc failure");
                libc::free(original);
            }
        } else {
            assert_eq!(
                capture.calls.load(Ordering::SeqCst),
                0,
                "callback should not be invoked if realloc unexpectedly succeeded"
            );
            unsafe { libc::free(resized) };
        }
    }
}
