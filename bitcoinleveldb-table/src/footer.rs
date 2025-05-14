// ---------------- [ File: bitcoinleveldb-table/src/footer.rs ]
crate::ix!();

/**
  | Footer encapsulates the fixed information
  | stored at the tail end of every table
  | file.
  |
  */
#[derive(Default)]
pub struct Footer {
    metaindex_handle: BlockHandle,
    index_handle:     BlockHandle,
}

/**
  | Note:
  | 
  | The serialization of a Footer will always
  | occupy exactly this many bytes. It consists
  | of two block handles and a magic number.
  |
  */
pub const FOOTER_ENCODED_LENGTH: usize = 2 * BLOCK_HANDLE_MAX_ENCODED_LENGTH + 8;

impl Footer {

    /**
      | The block handle for the metaindex block
      | of the table
      |
      */
    pub fn metaindex_handle(&self) -> &BlockHandle {
        
        todo!();
        /*
            return metaindex_handle_;
        */
    }
    
    pub fn set_metaindex_handle(&mut self, h: &BlockHandle)  {
        
        todo!();
        /*
            metaindex_handle_ = h;
        */
    }

    /**
      | The block handle for the index block
      | of the table
      |
      */
    pub fn index_handle(&self) -> &BlockHandle {
        
        todo!();
        /*
            return index_handle_;
        */
    }
    
    pub fn set_index_handle(&mut self, h: &BlockHandle)  {
        
        todo!();
        /*
            index_handle_ = h;
        */
    }
    
    pub fn encode_to(&self, dst: *mut String)  {
        
        todo!();
        /*
            const size_t original_size = dst->size();
      metaindex_handle_.EncodeTo(dst);
      index_handle_.EncodeTo(dst);
      dst->resize(2 * BlockHandle::kMaxEncodedLength);  // Padding
      PutFixed32(dst, static_cast<uint32_t>(kTableMagicNumber & 0xffffffffu));
      PutFixed32(dst, static_cast<uint32_t>(kTableMagicNumber >> 32));
      assert(dst->size() == original_size + kEncodedLength);
      (c_void)original_size;  // Disable unused variable warning.
        */
    }
    
    pub fn decode_from(&mut self, 
        input: *mut Slice) -> Status {
        
        todo!();
        /*
            const char* magic_ptr = input->data() + kEncodedLength - 8;
      const uint32_t magic_lo = DecodeFixed32(magic_ptr);
      const uint32_t magic_hi = DecodeFixed32(magic_ptr + 4);
      const uint64_t magic = ((static_cast<uint64_t>(magic_hi) << 32) |
                              (static_cast<uint64_t>(magic_lo)));
      if (magic != kTableMagicNumber) {
        return Status::Corruption("not an sstable (bad magic number)");
      }

      Status result = metaindex_handle_.DecodeFrom(input);
      if (result.ok()) {
        result = index_handle_.DecodeFrom(input);
      }
      if (result.ok()) {
        // We skip over any leftover data (just padding for now) in "input"
        const char* end = magic_ptr + 8;
        *input = Slice(end, input->data() + input->size() - end);
      }
      return result;
        */
    }
}
