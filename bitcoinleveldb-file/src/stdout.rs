// ---------------- [ File: bitcoinleveldb-file/src/stdout.rs ]
crate::ix!();

pub struct StdoutPrinter {

}

impl WritableFile for StdoutPrinter {

}

impl WritableFileClose for StdoutPrinter {
    fn close(&mut self) -> crate::Status {
        use tracing::debug;
        debug!("StdoutPrinter::close -> OK");
        crate::Status::ok()
    }
}

impl WritableFileAppend for StdoutPrinter {
    fn append(&mut self, data: &Slice) -> crate::Status {
        use std::io::{self, Write};
        use tracing::{debug, error, trace};

        // Best-effort write to stdout; errors are mapped to a generic IO error Status.
        // We avoid changing control flow relative to the C++ version (which ignored fwrite result),
        // but we still attempt to write for completeness and tracing.
        trace!("StdoutPrinter::append enter");
        let mut out = io::stdout();
        // SAFETY: Slice is assumed to provide a contiguous view over bytes.
        let bytes: &[u8] = data.as_ref();
        match out.write_all(bytes) {
            Ok(_) => {
                debug!(len = bytes.len(), "StdoutPrinter::append wrote bytes");
                crate::Status::ok()
            }
            Err(e) => {
                error!(error = %e, "StdoutPrinter::append write_all failed");
                crate::Status::io_error()
            }
        }
    }
}

impl WritableFileFlush for StdoutPrinter {
    fn flush(&mut self) -> crate::Status {
        use std::io::{self, Write};
        use tracing::{debug, error, trace};

        trace!("StdoutPrinter::flush enter");
        let mut out = io::stdout();
        match out.flush() {
            Ok(_) => {
                debug!("StdoutPrinter::flush -> OK");
                crate::Status::ok()
            }
            Err(e) => {
                error!(error = %e, "StdoutPrinter::flush failed");
                crate::Status::io_error()
            }
        }
    }
}

impl WritableFileSync for StdoutPrinter {
    fn sync(&mut self) -> crate::Status {
        use tracing::debug;
        // No-op for stdout; mirrors C++ which returned OK.
        debug!("StdoutPrinter::sync -> OK (no-op)");
        crate::Status::ok()
    }
}

impl GetName for StdoutPrinter {
    fn get_name(&self) -> &'static str {
        "[stdout]"
    }
}
