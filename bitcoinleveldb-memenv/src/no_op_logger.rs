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
