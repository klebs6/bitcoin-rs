// ---------------- [ File: bitcoinleveldb-dbimpl/src/maybe_ignore_error.rs ]
crate::ix!();

impl DBImpl {
    pub fn maybe_ignore_error(&self, s: *mut Status) {
        unsafe {
            if (*s).is_ok() || self.options.paranoid_checks() {
                // No change needed
            } else {
                tracing::warn!(status = %(*s).to_string(), "Ignoring error");
                *s = Status::ok();
            }
        }
    }
}
