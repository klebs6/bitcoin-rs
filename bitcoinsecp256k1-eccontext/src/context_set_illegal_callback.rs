// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_set_illegal_callback.rs ]
crate::ix!();

pub fn context_set_illegal_callback(
    ctx: *mut Secp256k1Context,
    fun: Option<fn(message: *const u8, data: *mut c_void)>,
    data: *const c_void,
) {
    unsafe {
        arg_check_no_return!(
            ctx != (&CONTEXT_NO_PRECOMP as *const Secp256k1Context as *mut Secp256k1Context)
        );

        let mut fun = fun;
        if fun.is_none() {
            fun = Some(default_illegal_callback_fn);
        }

        (*ctx).illegal_callback = Callback::new(fun.unwrap(), data);
    }
}
