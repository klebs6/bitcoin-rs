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
        use libc::{self, c_void};
        use libc_stdhandle::stdout;
        use tracing::trace;

        trace!(len = data.size(), "StdoutPrinter::append: writing to stdout");

        unsafe {
            libc::fwrite(
                *data.data() as *const c_void,
                1,
                *data.size(),
                stdout(),
            );
        }

        crate::Status::ok()
    }
}

impl WritableFileFlush for StdoutPrinter {
    fn flush(&mut self) -> crate::Status {
        use tracing::debug;

        debug!("StdoutPrinter::flush -> OK");
        crate::Status::ok()
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

impl Named for StdoutPrinter {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[stdout]".to_string())
    }
}
