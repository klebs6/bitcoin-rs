// ---------------- [ File: bitcoinleveldb-versionset/src/get_level_summary.rs ]
crate::ix!();

impl GetLevelSummary for VersionSet {

    fn level_summary(&self, scratch: *mut VersionSetLevelSummaryStorage) -> *const u8 {
        let cur: *mut Version = VersionSet::current(self);

        trace!(
            scratch_ptr = %format!("{:p}", scratch),
            current_ptr = %format!("{:p}", cur),
            "VersionSet::level_summary: enter"
        );

        assert!(
            !scratch.is_null(),
            "VersionSet::level_summary: scratch must not be null"
        );

        // Update code if kNumLevels changes
        const_assert!(NUM_LEVELS == 7);

        let vptr: *mut Version = cur;

        let counts: [usize; 7] = if vptr.is_null() {
            warn!(
                "VersionSet::level_summary: current is null; reporting zeros"
            );
            [0, 0, 0, 0, 0, 0, 0]
        } else {
            unsafe {
                let v: &Version = &*vptr;
                [
                    v.files()[0].len(),
                    v.files()[1].len(),
                    v.files()[2].len(),
                    v.files()[3].len(),
                    v.files()[4].len(),
                    v.files()[5].len(),
                    v.files()[6].len(),
                ]
            }
        };

        let summary = format!(
            "files[ {} {} {} {} {} {} {} ]",
            counts[0], counts[1], counts[2], counts[3], counts[4], counts[5], counts[6]
        );

        unsafe {
            // VersionSetLevelSummaryStorage is a single-field struct containing [u8; 100].
            // Its only field is private, so we write through the raw pointer using the
            // known layout (buffer starts at offset 0).
            let buf: &mut [u8; 100] = &mut *(scratch as *mut [u8; 100]);

            buf.fill(0);

            let bytes = summary.as_bytes();
            let n = core::cmp::min(bytes.len(), buf.len().saturating_sub(1));
            buf[..n].copy_from_slice(&bytes[..n]);
            buf[n] = 0;

            debug!(
                summary = %summary,
                copied_len = n,
                "VersionSet::level_summary: wrote summary string into scratch"
            );

            scratch as *const u8
        }
    }
}
