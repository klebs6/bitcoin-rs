// ---------------- [ File: bitcoinleveldb-compaction/src/compaction_lifecycle.rs ]
crate::ix!();

impl Drop for Compaction {

    fn drop(&mut self) {
        let raw_input_version_ptr: *mut Version = *self.input_version();

        trace!(
            "Compaction::drop: enter; input_version_ptr={:p}",
            raw_input_version_ptr
        );

        unsafe {
            if !raw_input_version_ptr.is_null() {
                trace!(
                    "Compaction::drop: calling Version::unref on {:p}",
                    raw_input_version_ptr
                );
                let v: &mut Version = &mut *raw_input_version_ptr;
                v.unref();
            } else {
                trace!(
                    "Compaction::drop: input_version_ is null; nothing to unref"
                );
            }
        }
    }
}

impl Compaction {

    pub fn new(options: *const Options, level: i32) -> Self {
        trace!(
            "Compaction::new: constructing compaction for level {} with options_ptr={:p}",
            level,
            options
        );

        let mut level_ptrs: [usize; NUM_LEVELS] = [0; NUM_LEVELS];

        for (idx, slot) in level_ptrs.iter_mut().enumerate() {
            *slot = 0;
            trace!(
                "Compaction::new: initialized level_ptrs[{}] to 0",
                idx
            );
        }

        CompactionBuilder::default()
            .level(level)
            .max_output_file_size(max_file_size_for_level(options,level))
            .input_version(core::ptr::null_mut())
            .edit(VersionEdit::default())
            .inputs([Vec::new(), Vec::new()])
            .grandparents(Vec::new())
            .grandparent_index(0)
            .seen_key_(false)
            .overlapped_bytes(0)
            .level_ptrs(level_ptrs)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod compaction_lifecycle_initialization_tests {
    use super::*;

    #[traced_test]
    fn compaction_new_initializes_accessors_consistently() {
        let mut opts = Options::default();
        opts.set_max_file_size(8 * 1024 * 1024);
        let level = 2;

        let c = Compaction::new(&opts as *const Options, level);

        assert_eq!(c.level(), level);
        assert_eq!(c.max_output_file_size(), 8 * 1024 * 1024);
    }

    #[traced_test]
    fn compaction_new_initializes_internal_tracking_state_to_zero() {
        let opts = Options::default();
        let level = 1;

        let c = Compaction::new(&opts as *const Options, level);

        assert_eq!(c.grandparents().len(), 0);
        assert_eq!(*c.grandparent_index(), 0);
        assert_eq!(*c.seen_key_(), false);
        assert_eq!(*c.overlapped_bytes(), 0);

        for (idx, ptr) in c.level_ptrs().iter().enumerate() {
            assert_eq!(
                *ptr,
                0,
                "expected level_ptrs[{}] to be initialized to 0",
                idx
            );
        }
    }

    #[traced_test]
    fn compaction_drop_with_null_input_version_is_safe() {
        let opts = Options::default();
        let level = 0;

        let c = Compaction::new(&opts as *const Options, level);

        drop(c);
    }
}
