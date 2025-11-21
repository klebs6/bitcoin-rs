// ---------------- [ File: bitcoinleveldb-logwriter/src/log_writer.rs ]
crate::ix!();
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_writer.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_writer.cc]

#[derive(Builder)]
#[builder(setter(into))]
pub struct LogWriter {
    dest:         Rc<RefCell<dyn WritableFile>>,
    /// Current offset in block
    block_offset: i32,
    /// crc32c values for all supported record types.
    ///
    /// These are pre-computed to reduce the overhead of computing the crc of
    /// the record type stored in the header.
    type_crc:     [u32; LOG_MAX_RECORD_TYPE as usize + 1],
}

impl LogWriter {

    /// Get the precomputed CRC seed for the given record type.
    pub fn type_crc_for(&self, record_type: LogRecordType) -> u32 {
        let crc = self.type_crc[record_type as usize];
        trace!(
            "LogWriter::type_crc_for: record_type={:?} crc={:#010x}",
            record_type,
            crc
        );
        crc
    }

    /// Accessor for the destination handle.
    pub fn dest_handle(&self) -> &Rc<RefCell<dyn WritableFile>> {
        trace!("LogWriter::dest_handle: returning destination handle");
        &self.dest
    }

    /// Current block offset.
    pub fn block_offset_value(&self) -> i32 {
        trace!(
            "LogWriter::block_offset_value: block_offset={}",
            self.block_offset
        );
        self.block_offset
    }

    /// Set the current block offset.
    pub fn set_block_offset_value(&mut self, value: i32) {
        let old = self.block_offset;
        self.block_offset = value;
        debug!(
            "LogWriter::set_block_offset_value: from {} to {}",
            old,
            self.block_offset
        );
    }

    /// Advance the current block offset by `delta`, saturating on overflow.
    pub fn advance_block_offset(&mut self, delta: i32) {
        let old = self.block_offset;
        self.block_offset = self.block_offset.saturating_add(delta);
        debug!(
            "LogWriter::advance_block_offset: {} + {} => {}",
            old,
            delta,
            self.block_offset
        );
    }
}

#[cfg(test)]
mod log_writer_core_tests {
    use super::*;

    #[traced_test]
    fn log_writer_new_sets_block_offset_and_type_crc() {
        let file        = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let dest_length = (LOG_BLOCK_SIZE as u64) * 3 + 5;

        let writer = LogWriter::new(file.clone(), dest_length);

        assert_eq!(
            writer.block_offset_value(),
            LogWriter::initial_block_offset_from_length(dest_length)
        );

        // Ensure at least one of the type CRC entries has been initialized.
        let full_crc = writer.type_crc_for(LogRecordType::Full);
        assert_ne!(full_crc, 0u32);
    }

    #[traced_test]
    fn log_writer_block_offset_helpers_behave_consistently() {
        let file  = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        assert_eq!(w.block_offset_value(), 0);

        w.set_block_offset_value(10);
        assert_eq!(w.block_offset_value(), 10);

        w.advance_block_offset(5);
        assert_eq!(w.block_offset_value(), 15);

        // Saturating behavior should not panic.
        w.advance_block_offset(i32::MAX);
        assert!(w.block_offset_value() >= 15);
    }
}
