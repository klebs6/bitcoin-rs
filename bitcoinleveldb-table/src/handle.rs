// ---------------- [ File: bitcoinleveldb-table/src/handle.rs ]
crate::ix!();

/**
  | 1-byte type + 32-bit crc
  |
  */
pub const BLOCK_TRAILER_SIZE: usize = 5;

pub struct BlockContents {

    /**
      | Actual contents of data
      |
      */
    data:           Slice,

    /**
      | True iff data can be cached
      |
      */
    cachable:       bool,

    /**
      | True iff caller should delete[] data.data()
      |
      */
    heap_allocated: bool,
}

/**
  | BlockHandle is a pointer to the extent
  | of a file that stores a data block or a
  | meta block.
  |
  */
#[derive(Default)]
pub struct BlockHandle {
    offset: u64,
    size:   u64,
}

/**
  | Maximum encoding length of a BlockHandle
  |
  */
pub const BLOCK_HANDLE_MAX_ENCODED_LENGTH: usize = 10 + 10;

impl BlockHandle {

    /**
      | The offset of the block in the file.
      |
      */
    pub fn offset(&self) -> u64 {
        
        todo!();
        /*
            return offset_;
        */
    }
    
    pub fn set_offset(&mut self, offset: u64)  {
        
        todo!();
        /*
            offset_ = offset;
        */
    }

    /**
       The size of the stored block
      */
    pub fn size(&self) -> u64 {
        
        todo!();
        /*
            return size_;
        */
    }
    
    pub fn set_size(&mut self, size: u64)  {
        
        todo!();
        /*
            size_ = size;
        */
    }
    
    /**
      | Implementation details follow. Clients
      | should ignore,
      |
      */
    pub fn new() -> Self {

        todo!();
        /*
           : offset_(~static_cast<uint64_t>(0)), size_(~static_cast<uint64_t>(0))
           */
    }
    
    pub fn encode_to(&self, dst: *mut String)  {
        
        todo!();
        /*
            // Sanity check that all fields have been set
      assert(offset_ != ~static_cast<uint64_t>(0));
      assert(size_ != ~static_cast<uint64_t>(0));
      PutVarint64(dst, offset_);
      PutVarint64(dst, size_);
        */
    }
    
    pub fn decode_from(&mut self, input: *mut Slice) -> crate::Status {
        
        todo!();
        /*
            if (GetVarint64(input, &offset_) && GetVarint64(input, &size_)) {
        return Status::OK();
      } else {
        return Status::Corruption("bad block handle");
      }
        */
    }
}
