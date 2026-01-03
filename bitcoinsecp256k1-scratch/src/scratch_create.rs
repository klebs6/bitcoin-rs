// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch_create.rs ]
crate::ix!();

pub fn scratch_create(
        error_callback: *const Callback,
        max_size:       usize) -> *mut Scratch {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::scratch",
            error_callback = error_callback as usize,
            max_size,
            "scratch_create"
        );

        let size: usize = max_size;

        let base_alloc: usize = {
            let sz = core::mem::size_of::<Scratch>();
            let tmp = sz.wrapping_add(ALIGNMENT.wrapping_sub(1));
            (tmp / ALIGNMENT).wrapping_mul(ALIGNMENT)
        };

        let alloc: *mut libc::c_void = checked_malloc(
            error_callback,
            base_alloc.wrapping_add(size),
        );

        let ret: *mut Scratch = alloc as *mut Scratch;

        if !ret.is_null() {
            libc::memset(
                ret as *mut libc::c_void,
                0,
                core::mem::size_of::<Scratch>(),
            );

            libc::memcpy(
                (*ret).magic.as_mut_ptr() as *mut libc::c_void,
                b"scratch\0".as_ptr() as *const libc::c_void,
                8,
            );

            (*ret).data = (alloc as *mut u8).add(base_alloc) as *mut libc::c_void;
            (*ret).max_size = size;

            debug!(
                target: "bitcoinsecp256k1_scratch::scratch",
                scratch = ret as usize,
                data = (*ret).data as usize,
                base_alloc,
                size,
                "scratch_create: created"
            );
        } else {
            warn!(
                target: "bitcoinsecp256k1_scratch::scratch",
                base_alloc,
                size,
                "scratch_create: allocation failed"
            );
        }

        ret
    }
}

#[cfg(test)]
mod scratch_create_and_basic_use_test_suite {
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
    fn scratch_create_produces_valid_scratch_space_and_data_is_alignment_aligned() {
        let capture = CallbackCapture::default();
        let cb = mk_error_callback(&capture);

        let scratch = scratch_create((&cb as *const Callback), ALIGNMENT * 2);
        assert!(!scratch.is_null());

        let data_ptr = scratch_alloc((&cb as *const Callback), scratch, 0);
        assert!(!data_ptr.is_null());

        assert_eq!(
            (data_ptr as usize) % ALIGNMENT,
            0,
            "scratch data pointer must be ALIGNMENT-aligned"
        );

        scratch_destroy((&cb as *const Callback), scratch);

        assert_eq!(capture.calls.load(Ordering::SeqCst), 0);
    }
}
