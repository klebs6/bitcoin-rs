// ---------------- [ File: bitcoinleveldb-versionsetutil/src/meta.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set.cc]

pub fn target_file_size(options: *const Options) -> usize {
    unsafe {
        debug_assert!(
            !options.is_null(),
            "target_file_size: options pointer must not be null"
        );
        let size = *(*options).max_file_size();
        trace!(max_file_size = size, "target_file_size");
        size
    }
}

/// Maximum number of bytes in all compacted files.
///
/// We avoid expanding the lower level file set of a compaction if it would make
/// the total compaction cover more than this many bytes.
///
pub fn expanded_compaction_byte_size_limit(options: *const Options) -> i64 {
    unsafe {
        debug_assert!(
            !options.is_null(),
            "expanded_compaction_byte_size_limit: options pointer must not be null"
        );
        let tfs = target_file_size(options) as i64;
        let result = 25_i64 * tfs;
        trace!(
            target_file_size = tfs,
            result,
            "expanded_compaction_byte_size_limit"
        );
        result
    }
}

pub fn newest_first(a: *mut FileMetaData, b: *mut FileMetaData) -> bool {
    unsafe {
        debug_assert!(
            !a.is_null() && !b.is_null(),
            "newest_first: file pointers must not be null"
        );

        let na = *(*a).number();
        let nb = *(*b).number();

        let result = na > nb;

        trace!(
            a_number = na,
            b_number = nb,
            result,
            "newest_first"
        );

        result
    }
}

#[cfg(test)]
mod meta_level_util_spec {
    use super::*;

    #[traced_test]
    fn verify_target_file_size_reads_from_options() {
        let mut opts = Options::default();
        opts.set_max_file_size(16 * 1024 * 1024);
        let opt_ptr: *const Options = &opts;

        let result = target_file_size(opt_ptr);

        debug!(
            configured = *opts.max_file_size(),
            result = result,
            "verify_target_file_size_reads_from_options"
        );

        assert_eq!(
            *opts.max_file_size(),
            result,
            "target_file_size must read Options::max_file_size"
        );
    }

    #[traced_test]
    fn verify_expanded_compaction_byte_size_limit_is_25x_target() {
        let mut opts = Options::default();
        opts.set_max_file_size(3 * 1024 * 1024);
        let opt_ptr: *const Options = &opts;

        let tfs = target_file_size(opt_ptr) as i64;
        let expected = 25_i64 * tfs;
        let result = expanded_compaction_byte_size_limit(opt_ptr);

        debug!(
            target_file_size = tfs,
            expected = expected,
            result = result,
            "verify_expanded_compaction_byte_size_limit_is_25x_target"
        );

        assert_eq!(
            expected, result,
            "expanded_compaction_byte_size_limit must be 25x the target file size"
        );
    }

    #[traced_test]
    fn verify_newest_first_orders_by_descending_file_number() {
        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        let mut f2 = FileMetaData::default();
        f2.set_number(2);

        let p1: *mut FileMetaData = &mut f1;
        let p2: *mut FileMetaData = &mut f2;

        let r12 = newest_first(p1, p2);
        let r21 = newest_first(p2, p1);

        debug!(
            r12 = r12,
            r21 = r21,
            n1 = *unsafe { (*p1).number() },
            n2 = *unsafe { (*p2).number() },
            "verify_newest_first_orders_by_descending_file_number"
        );

        assert!(
            r12 == false && r21 == true,
            "newest_first(a,b) must be true iff a.number > b.number"
        );
    }
}
