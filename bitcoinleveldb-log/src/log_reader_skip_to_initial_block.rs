// ---------------- [ File: bitcoinleveldb-log/src/log_reader_skip_to_initial_block.rs ]
crate::ix!();

impl LogReader {

    /**
      | Skips all blocks that are completely before
      | "initial_offset_".
      |
      | Returns true on success. Handles reporting.
      */
    pub fn skip_to_initial_block(&mut self) -> bool {
        
        todo!();
        /*
            const size_t offset_in_block = initial_offset_ % kBlockSize;
      uint64_t block_start_location = initial_offset_ - offset_in_block;

      // Don't search a block if we'd be in the trailer
      if (offset_in_block > kBlockSize - 6) {
        block_start_location += kBlockSize;
      }

      end_of_buffer_offset_ = block_start_location;

      // Skip to start of first block that can contain the initial record
      if (block_start_location > 0) {
        Status skip_status = file_->Skip(block_start_location);
        if (!skip_status.ok()) {
          ReportDrop(block_start_location, skip_status);
          return false;
        }
      }

      return true;
        */
    }
}
