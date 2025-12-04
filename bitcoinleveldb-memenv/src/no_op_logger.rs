// ---------------- [ File: bitcoinleveldb-memenv/src/no_op_logger.rs ]
crate::ix!();

pub struct NoOpLogger {

}

impl Logger for NoOpLogger { }

impl Logv for NoOpLogger {

    fn logv(&mut self, 
        format: *const u8,
        ap:     &[&str])  { }
}

#[cfg(test)]
mod no_op_logger_tests {
    use super::*;

    #[traced_test]
    fn no_op_logger_accepts_logv_without_panic() {
        crate::ix!();

        let mut logger = NoOpLogger {};
        let fmt = "%s".as_ptr();
        let args: [&str; 1] = ["hello"];
        logger.logv(fmt, &args);
    }
}
