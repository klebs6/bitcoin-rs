crate::ix!();

pub struct LogReporter {
    info_log: *mut dyn Logger,
    fname:    String,
    status:   *mut Status,
}

impl LogReaderReporter for LogReporter {
    fn corruption(&mut self, bytes: usize, s: &Status) {
        todo!();
        /*
        if self.status.is_null() {
            tracing::warn!(
                file = %self.fname,
                dropped_bytes = bytes,
                status = %s.to_string(),
                "Corruption (ignoring error)"
            );
        } else {
            tracing::warn!(
                file = %self.fname,
                dropped_bytes = bytes,
                status = %s.to_string(),
                "Corruption"
            );
            unsafe {
                if (*self.status).is_ok() {
                    *self.status = s.clone();
                }
            }
        }
        */
    }
}
