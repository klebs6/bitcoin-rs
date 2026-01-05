// ---------------- [ File: bitcoinleveldb-dbinterface/src/get_approximate_sizes.rs ]
crate::ix!();

pub trait DBGetApproximateSizes {

    /// For each i in [0,n-1], store in "sizes[i]", the approximate file system space used by keys
    /// in "[range[i].start .. range[i].limit)".
    /// 
    /// Note that the returned sizes measure file system space usage, so if the user data
    /// compresses by a factor of ten, the returned sizes will be one-tenth the size of the
    /// corresponding user data size.
    /// 
    /// The results may not include the sizes of recently written data.
    ///
    fn get_approximate_sizes(&mut self, 
            range: *const bitcoinleveldb_slice::Range,
            n:     i32,
            sizes: *mut u64);
}

#[cfg(test)]
mod get_approximate_sizes_pointer_contract_suite {
    use super::*;
    use core::ptr;
    use tracing::{debug, error, info, trace, warn};

    struct DeterministicSizer;

    impl GetApproximateSizes for DeterministicSizer {
        fn get_approximate_sizes(
            &mut self,
            _range: *const bitcoinleveldb_slice::Range,
            n: i32,
            sizes: *mut u64,
        ) {
            let mut i: i32 = 0;

            // C++-style loop semantics: if n <= 0, do nothing.
            while i < n {
                unsafe {
                    *sizes.add(i as usize) = i as u64;
                }
                i += 1;
            }
        }
    }

    #[traced_test]
    fn get_approximate_sizes_writes_n_values_to_sizes_pointer() {
        let mut sizer = DeterministicSizer;

        let ranges = [
            bitcoinleveldb_slice::Range::new(Slice::from("a"), Slice::from("b")),
            bitcoinleveldb_slice::Range::new(Slice::from("c"), Slice::from("d")),
            bitcoinleveldb_slice::Range::new(Slice::from("e"), Slice::from("f")),
        ];

        let mut sizes: [u64; 3] = [u64::MAX, u64::MAX, u64::MAX];

        trace!("calling get_approximate_sizes for n=3");
        sizer.get_approximate_sizes(
            ranges.as_ptr(),
            3,
            #[allow(clippy::as_ptr_cast_mut)]
            sizes.as_mut_ptr(),
        );

        assert_eq!(sizes, [0, 1, 2]);

        info!("verified sizes[i] is written for each i in [0,n-1]");
    }

    #[traced_test]
    fn get_approximate_sizes_with_n_zero_does_not_touch_memory_and_allows_null_pointers() {
        let mut sizer = DeterministicSizer;

        trace!("calling get_approximate_sizes with n=0 and null pointers");
        sizer.get_approximate_sizes(
            ptr::null::<bitcoinleveldb_slice::Range>(),
            0,
            ptr::null_mut::<u64>(),
        );

        info!("verified n=0 results in no dereference of input pointers");
    }

    #[traced_test]
    fn get_approximate_sizes_with_negative_n_is_a_no_op_like_cplusplus_loop() {
        let mut sizer = DeterministicSizer;

        trace!("calling get_approximate_sizes with n=-1 and null pointers");
        sizer.get_approximate_sizes(
            ptr::null::<bitcoinleveldb_slice::Range>(),
            -1,
            ptr::null_mut::<u64>(),
        );

        info!("verified negative n is treated as no-op via i32 loop condition");
    }
}
