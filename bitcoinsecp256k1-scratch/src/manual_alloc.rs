// ---------------- [ File: bitcoinsecp256k1-scratch/src/manual_alloc.rs ]
crate::ix!();

/// Assume there is a contiguous memory object with bounds [base, base + max_size) of which the
/// memory range [base, *prealloc_ptr) is already allocated for usage, where *prealloc_ptr is an
/// aligned pointer. In that setting, this functions reserves the subobject [*prealloc_ptr,
/// *prealloc_ptr + alloc_size) of alloc_size bytes by increasing *prealloc_ptr accordingly, taking
/// into account alignment requirements.
/// 
/// The function returns an aligned pointer to the newly allocated subobject.
/// 
/// This is useful for manual memory management: if we're simply given a block [base, base
/// + max_size), the caller can use this function to allocate memory in this block and keep track
/// of the current allocation state with *prealloc_ptr.
/// 
/// It is verify_checked that there is enough space left in the memory object and *prealloc_ptr is
/// aligned relative to base.
/// 
#[inline] pub fn manual_alloc(
        prealloc_ptr: *mut *mut libc::c_void,
        alloc_size:   usize,
        base:         *mut libc::c_void,
        max_size:     usize) -> *mut libc::c_void {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            prealloc_ptr = prealloc_ptr as usize,
            alloc_size,
            base = base as usize,
            max_size,
            "manual_alloc"
        );

        let aligned_alloc_size: usize = {
            let tmp = alloc_size.wrapping_add(ALIGNMENT.wrapping_sub(1));
            (tmp / ALIGNMENT).wrapping_mul(ALIGNMENT)
        };

        verify_check!{!prealloc_ptr.is_null()};

        let cur: *mut libc::c_void = *prealloc_ptr;

        verify_check!{ !cur.is_null() };

        verify_check!{ !base.is_null() };

        let cur_u8: *mut u8 = cur as *mut u8;
        let base_u8: *mut u8 = base as *mut u8;

        verify_check!{ cur_u8 >= base_u8 };

        let diff: usize = (cur_u8 as usize).wrapping_sub(base_u8 as usize);

        verify_check!{ (diff % ALIGNMENT) == 0 };

        verify_check!{ diff.wrapping_add(aligned_alloc_size) <= max_size };

        let ret: *mut libc::c_void = cur;

        *prealloc_ptr = (cur_u8).add(aligned_alloc_size) as *mut libc::c_void;

        debug!(
            target: "bitcoinsecp256k1_scratch::util",
            ret = ret as usize,
            new_prealloc_ptr = (*prealloc_ptr) as usize,
            aligned_alloc_size,
            "manual_alloc: reserved"
        );

        ret
    }
}

#[cfg(test)]
mod manual_alloc_alignment_and_progression_test_suite {
    use super::*;

    #[traced_test]
    fn manual_alloc_rounds_up_to_alignment_and_advances_prealloc_ptr() {
        let max_size: usize = ALIGNMENT * 4;
        let base = unsafe { libc::malloc(max_size) };
        assert!(!base.is_null(), "malloc(max_size) unexpectedly returned null");

        let mut prealloc_ptr = base;

        let ret1 = manual_alloc(
            &mut prealloc_ptr as *mut *mut libc::c_void,
            1,
            base,
            max_size,
        );
        assert_eq!(ret1, base);

        let expected_after_1 = unsafe { (base as *mut u8).add(ALIGNMENT) as *mut libc::c_void };
        assert_eq!(prealloc_ptr, expected_after_1);

        let ret2 = manual_alloc(
            &mut prealloc_ptr as *mut *mut libc::c_void,
            ALIGNMENT,
            base,
            max_size,
        );
        assert_eq!(ret2, expected_after_1);

        let expected_after_2 =
            unsafe { (base as *mut u8).add(ALIGNMENT * 2) as *mut libc::c_void };
        assert_eq!(prealloc_ptr, expected_after_2);

        unsafe { libc::free(base) };
    }

    #[traced_test]
    fn manual_alloc_zero_sized_reservation_is_a_noop_on_prealloc_ptr() {
        let max_size: usize = ALIGNMENT * 2;
        let base = unsafe { libc::malloc(max_size) };
        assert!(!base.is_null(), "malloc(max_size) unexpectedly returned null");

        let mut prealloc_ptr = base;

        let ret = manual_alloc(
            &mut prealloc_ptr as *mut *mut libc::c_void,
            0,
            base,
            max_size,
        );

        assert_eq!(ret, base);
        assert_eq!(prealloc_ptr, base);

        unsafe { libc::free(base) };
    }
}
