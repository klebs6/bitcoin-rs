// ---------------- [ File: bitcoinsecp256k1-scratch/src/callback_call.rs ]
crate::ix!();

#[inline] pub fn callback_call(
        cb:   *const Callback,
        text: *const u8)  {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            cb = cb as usize,
            text = text as usize,
            "callback_call"
        );
        ((*cb).fn_)(text, (*cb).data as *mut libc::c_void);
    }
}

#[derive(Builder, CopyGetters, Clone, Copy, Debug)]
#[getset(get_copy = "pub")]
#[builder(build_fn(error = "CallbackBuilderError"))]
#[repr(C)]
pub struct Callback {
    fn_: fn(text: *const u8, data: *mut libc::c_void),

    #[builder(default = "core::ptr::null()")]
    data: *const libc::c_void,
}

unsafe impl core::marker::Sync for Callback {}
unsafe impl core::marker::Send for Callback {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallbackBuilderError {
    MissingFn,
}

impl From<UninitializedFieldError> for CallbackBuilderError {
    fn from(_value: UninitializedFieldError) -> Self {
        CallbackBuilderError::MissingFn
    }
}

impl Callback {
    #[inline]
    pub const fn new(
        fn_: fn(text: *const u8, data: *mut libc::c_void),
        data: *const libc::c_void,
    ) -> Self {
        Self { fn_, data }
    }
}

#[cfg(test)]
mod callback_call_invocation_contract_suite {
    crate::ix!();

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
            trace!(
                target: "bitcoinsecp256k1_scratch::tests::callback_call",
                text = text as usize,
                data = data as usize,
                "recording_callback: invoked"
            );

            let capture: &CallbackCapture = &*(data as *const CallbackCapture);

            let msg = if text.is_null() {
                "<null>".to_owned()
            } else {
                let cstr = CStr::from_ptr(text as *const libc::c_char);
                cstr.to_string_lossy().into_owned()
            };

            capture.calls.fetch_add(1, Ordering::SeqCst);
            capture.messages.lock().expect("mutex poisoned").push(msg);
        }
    }

    #[traced_test]
    fn callback_call_forwards_text_and_data_exactly_once() {
        let capture = CallbackCapture::default();

        let cb = Callback::new(
            recording_callback,
            (&capture as *const CallbackCapture).cast::<libc::c_void>(),
        );

        info!(
            target: "bitcoinsecp256k1_scratch::tests::callback_call",
            cb = (&cb as *const Callback) as usize,
            "calling callback_call"
        );

        callback_call((&cb as *const Callback), b"hello\0".as_ptr());

        let calls = capture.calls.load(Ordering::SeqCst);
        let messages = capture.messages.lock().expect("mutex poisoned").clone();

        assert_eq!(calls, 1);
        assert_eq!(messages, vec!["hello".to_owned()]);
    }
}
