// ---------------- [ File: bitcoinsecp256k1-scratch/src/memczero.rs ]
crate::ix!();

/// Zero memory if flag == 1. Flag must be 0 or 1. Constant time.
#[inline] pub fn memczero(
        s:    *mut libc::c_void,
        len:  usize,
        flag: i32)  {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            s = s as usize,
            len,
            flag,
            "memczero"
        );

        let mut p: *mut u8 = s as *mut u8;
        let mut remaining: usize = len;

        /* Access flag with a volatile-qualified lvalue.
           This prevents clang from figuring out (after inlining) that flag can
           take only be 0 or 1, which leads to variable time code. */
        let vflag: i32 = core::ptr::read_volatile(&flag);

        let mask: u8 = (0u8).wrapping_sub(vflag as u8);

        while remaining != 0 {
            *p &= !mask;
            p = p.add(1);
            remaining = remaining.wrapping_sub(1);
        }
    }
}

#[cfg(test)]
mod memczero_behavior_test_suite {
    use super::*;

    #[traced_test]
    fn memczero_zeros_memory_when_flag_is_one() {
        let mut buf = [0xAAu8; 64];

        memczero(buf.as_mut_ptr().cast::<libc::c_void>(), buf.len(), 1);

        assert!(buf.iter().all(|&b| b == 0));
    }

    #[traced_test]
    fn memczero_preserves_memory_when_flag_is_zero() {
        let mut buf = [0xAAu8; 64];

        memczero(buf.as_mut_ptr().cast::<libc::c_void>(), buf.len(), 0);

        assert!(buf.iter().all(|&b| b == 0xAA));
    }

    #[traced_test]
    fn memczero_with_zero_length_does_not_touch_pointer() {
        memczero(core::ptr::null_mut(), 0, 1);
        memczero(core::ptr::null_mut(), 0, 0);
    }
}
