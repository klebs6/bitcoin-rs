// ---------------- [ File: bitcoinleveldb-logwriter/src/create.rs ]
crate::ix!();

impl LogWriter {

    /// Create a writer that will append data to "*dest".
    ///
    /// "*dest" must have initial length "dest_length".
    ///
    /// "*dest" must remain live while this LogWriter is in use.
    pub fn new(
        dest: Rc<RefCell<dyn WritableFile>>,
        dest_length: u64,
    ) -> Self {
        info!(
            "LogWriter::new: dest_length={} block_size={}",
            dest_length,
            LOG_BLOCK_SIZE
        );

        let block_offset = Self::initial_block_offset_from_length(dest_length);
        let type_crc     = Self::build_type_crc_table();

        info!(
            "LogWriter::new: computed initial block_offset={} type_crc_entries={}",
            block_offset,
            type_crc.len()
        );

        LogWriterBuilder::default()
            .dest(dest)
            .block_offset(block_offset)
            .type_crc(type_crc)
            .build()
            .unwrap()
    }
}
