// ---------------- [ File: bitcoinleveldb-versionset/src/needs_compaction.rs ]
crate::ix!();

impl NeedsCompaction for VersionSet {

    /// Returns true iff some level needs a compaction.
    fn needs_compaction(&self) -> bool {
        let vptr: *mut Version = self.current();

        trace!(
            current_ptr = %format!("{:p}", vptr),
            "VersionSet::needs_compaction: enter"
        );

        if vptr.is_null() {
            debug!(
                "VersionSet::needs_compaction: current is null; returning false"
            );
            return false;
        }

        unsafe {
            let v: &Version = &*vptr;

            let score = *v.compaction_score();
            let file_to_compact_ptr = *v.file_to_compact();

            let needs = (score >= 1.0) || (!file_to_compact_ptr.is_null());

            debug!(
                compaction_score = score,
                file_to_compact_ptr = %format!("{:p}", file_to_compact_ptr),
                needs_compaction = needs,
                "VersionSet::needs_compaction: evaluated"
            );

            needs
        }
    }
}
