// ---------------- [ File: bitcoinleveldb-versionsetutil/src/max.rs ]
crate::ix!();

/**
  | Maximum bytes of overlaps in grandparent
  | (i.e., level+2) before we stop building
  | a single file in a level->level+1 compaction.
  |
  */
pub fn max_grand_parent_overlap_bytes(options: *const Options) -> i64 {

    unsafe {
        debug_assert!(
            !options.is_null(),
            "max_grand_parent_overlap_bytes: options pointer must not be null"
        );
        let tfs = target_file_size(options) as i64;
        let result = 10_i64 * tfs;
        trace!(
            target_file_size = tfs,
            result,
            "max_grand_parent_overlap_bytes"
        );
        result
    }
}

/// Note: the result for level zero is not really used since we set
/// the level-0 compaction threshold based on number of files.
pub fn max_bytes_for_level(options: *const Options, level: i32) -> f64 {

    // `options` is unused here, but kept for a faithful signature.
    let _ = options;

    let mut lvl = level;
    let mut result: f64 = 10.0 * 1048576.0;

    while lvl > 1 {
        result *= 10.0;
        lvl -= 1;
    }

    trace!(
        original_level = level,
        computed_bytes = result,
        "max_bytes_for_level"
    );

    result
}

pub fn max_file_size_for_level(options: *const Options, level: i32) -> u64 {

    // We could vary per level to reduce number of files?
    let size = target_file_size(options) as u64;

    trace!(
        level,
        max_file_size = size,
        "max_file_size_for_level"
    );

    size
}

#[cfg(test)]
mod max_level_util_spec {
    use super::*;

    #[traced_test]
    fn verify_max_grand_parent_overlap_bytes_scales_with_target_size() {
        let mut opts = Options::default();
        // Use a non-default value to ensure scaling is observed.
        opts.set_max_file_size(4 * 1024 * 1024);

        let opt_ptr: *const Options = &opts;

        let expected = 10_i64 * (target_file_size(opt_ptr) as i64);
        let actual = max_grand_parent_overlap_bytes(opt_ptr);

        debug!(
            expected = expected,
            actual = actual,
            "verify_max_grand_parent_overlap_bytes_scales_with_target_size"
        );

        assert_eq!(
            expected, actual,
            "max_grand_parent_overlap_bytes must be 10x the target file size"
        );
    }

    #[traced_test]
    fn verify_max_bytes_for_level_growth_pattern() {
        let opts = Options::default();
        let opt_ptr: *const Options = &opts;

        // Level 0 and 1 share the same base.
        let l0 = max_bytes_for_level(opt_ptr, 0);
        let l1 = max_bytes_for_level(opt_ptr, 1);
        let l2 = max_bytes_for_level(opt_ptr, 2);
        let l3 = max_bytes_for_level(opt_ptr, 3);

        debug!(l0, l1, l2, l3, "verify_max_bytes_for_level_growth_pattern");

        assert!(
            (l0 - 10.0 * 1048576.0).abs() < 0.5,
            "Level-0 base must be approximately 10MB"
        );
        assert!(
            (l1 - l0).abs() < 0.5,
            "Level-1 capacity should match level-0 in this helper"
        );
        assert!(
            (l2 - l1 * 10.0).abs() < 0.5,
            "Level-2 should be ~10x level-1"
        );
        assert!(
            (l3 - l2 * 10.0).abs() < 0.5,
            "Level-3 should be ~10x level-2"
        );
    }

    #[traced_test]
    fn verify_max_file_size_for_level_delegates_to_target_file_size() {
        let mut opts = Options::default();
        opts.set_max_file_size(8 * 1024 * 1024);
        let opt_ptr: *const Options = &opts;

        let l0 = max_file_size_for_level(opt_ptr, 0);
        let l5 = max_file_size_for_level(opt_ptr, 5);
        let expected = target_file_size(opt_ptr) as u64;

        debug!(l0, l5, expected, "verify_max_file_size_for_level_delegates_to_target_file_size");

        assert_eq!(
            expected, l0,
            "max_file_size_for_level(0) should equal target_file_size"
        );
        assert_eq!(
            expected, l5,
            "max_file_size_for_level should not vary by level in this helper"
        );
    }
}
