// ---------------- [ File: bitcoinsecp256k1-scratch/src/memcmp_var.rs ]
crate::ix!();

/// Semantics like memcmp. Variable-time.
/// 
/// We use this to avoid possible compiler
/// bugs with memcmp, e.g. https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189
/// 
#[inline] pub fn memcmp_var(
        s1: *const libc::c_void,
        s2: *const libc::c_void,
        n:  usize) -> i32 {
    
    unsafe {
        trace!(
            target: "bitcoinsecp256k1_scratch::util",
            s1 = s1 as usize,
            s2 = s2 as usize,
            n,
            "memcmp_var"
        );

        let p1: *const u8 = s1 as *const u8;
        let p2: *const u8 = s2 as *const u8;

        let mut i: usize = 0;
        while i < n {
            let diff: i32 = (*p1.add(i) as i32) - (*p2.add(i) as i32);
            if diff != 0 {
                return diff;
            }
            i = i.wrapping_add(1);
        }
        0
    }
}

#[cfg(test)]
mod memcmp_var_semantics_test_suite {
    use super::*;

    #[traced_test]
    fn memcmp_var_returns_zero_for_zero_length_even_with_null_pointers() {
        let r = memcmp_var(core::ptr::null(), core::ptr::null(), 0);
        assert_eq!(r, 0);
    }

    #[traced_test]
    fn memcmp_var_matches_first_difference_delta_for_single_byte_inputs() {
        for a in 0u8..=255u8 {
            for b in 0u8..=255u8 {
                let r = unsafe {
                    memcmp_var(
                        (&a as *const u8).cast::<libc::c_void>(),
                        (&b as *const u8).cast::<libc::c_void>(),
                        1,
                    )
                };

                let want = (a as i32) - (b as i32);

                trace!(
                    target: "bitcoinsecp256k1_scratch::tests::memcmp_var",
                    a = a as u32,
                    b = b as u32,
                    r,
                    want,
                    "single-byte comparison"
                );

                assert_eq!(r, want);
            }
        }
    }

    #[traced_test]
    fn memcmp_var_detects_difference_at_various_positions() {
        let x: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut y: [u8; 8] = x;

        let r_eq = memcmp_var(
            x.as_ptr().cast::<libc::c_void>(),
            y.as_ptr().cast::<libc::c_void>(),
            x.len(),
        );
        assert_eq!(r_eq, 0);

        y[0] = 9;
        let r0 = memcmp_var(
            x.as_ptr().cast::<libc::c_void>(),
            y.as_ptr().cast::<libc::c_void>(),
            x.len(),
        );
        assert_eq!(r0, (0i32 - 9i32));

        y = x;
        y[4] = 9;
        let r4 = memcmp_var(
            x.as_ptr().cast::<libc::c_void>(),
            y.as_ptr().cast::<libc::c_void>(),
            x.len(),
        );
        assert_eq!(r4, (4i32 - 9i32));

        y = x;
        y[7] = 9;
        let r7 = memcmp_var(
            x.as_ptr().cast::<libc::c_void>(),
            y.as_ptr().cast::<libc::c_void>(),
            x.len(),
        );
        assert_eq!(r7, (7i32 - 9i32));
    }
}
