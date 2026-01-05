// ---------------- [ File: bitcoinsecp256k1-eccontext/src/context_preallocated_size.rs ]
crate::ix!();

pub fn context_preallocated_size(flags: u32) -> usize {
    let mut ret: usize = round_to_align!(core::mem::size_of::<Secp256k1Context>());
    /* A return value of 0 is reserved as an indicator for errors when we call this function internally. */
    verify_check!(ret != 0);

    if expect!((flags & FLAGS_TYPE_MASK) != FLAGS_TYPE_CONTEXT, 0) {
        callback_call(&*default_illegal_callback, b"Invalid flags\0".as_ptr());
        return 0;
    }

    if (flags & FLAGS_BIT_CONTEXT_SIGN) != 0 {
        ret += ECMULT_GEN_CONTEXT_PREALLOCATED_SIZE;
    }
    if (flags & FLAGS_BIT_CONTEXT_VERIFY) != 0 {
        ret += ECMULT_CONTEXT_PREALLOCATED_SIZE;
    }
    ret
}
