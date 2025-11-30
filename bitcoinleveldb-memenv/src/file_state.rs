// ---------------- [ File: bitcoinleveldb-memenv/src/file_state.rs ]
crate::ix!();

pub const FILE_STATE_BLOCK_SIZE: usize = 8 * 1024;

#[no_copy]
pub struct FileStateRefs {
    refs: i32,
}

#[no_copy]
pub struct FileStateBlocks {
    blocks: Vec<*mut u8>,
    size:   u64,
}

//----------------------------------------
#[no_copy]
pub struct FileState {
    refs_mutex:   Mutex<FileStateRefs>,
    blocks_mutex: RefCell<Mutex<FileStateBlocks>>,
}

impl Default for FileState {
    
    /**
      | FileStates are reference counted.
      | The initial reference count is zero
      | and the caller must call Ref() at least
      | once.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : refs(0),
        : size(0),

        
        */
    }
}

impl Drop for FileState {

    /**
      | Private since only Unref() should be
      | used to delete it.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            Truncate();
        */
    }
}

impl FileState {

    /**
       Increase the reference count.
      */
    pub fn ref_(&mut self)  {
        
        todo!();
        /*
            MutexLock lock(&refs_mutex_);
        ++refs_;
        */
    }

    /**
      | Decrease the reference count. Delete
      | if this is the last reference.
      |
      */
    pub fn unref(&mut self)  {
        
        todo!();
        /*
            bool do_delete = false;

        {
          MutexLock lock(&refs_mutex_);
          --refs_;
          assert(refs_ >= 0);
          if (refs_ <= 0) {
            do_delete = true;
          }
        }

        if (do_delete) {
          delete this;
        }
        */
    }
    
    pub fn size(&self) -> u64 {
        
        todo!();
        /*
            MutexLock lock(&blocks_mutex_);
        return size_;
        */
    }
    
    pub fn truncate(&mut self)  {
        
        todo!();
        /*
            MutexLock lock(&blocks_mutex_);
        for (char*& block : blocks_) {
          delete[] block;
        }
        blocks_.clear();
        size_ = 0;
        */
    }
    
    pub fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&blocks_mutex_);
        if (offset > size_) {
          return Status::IOError("Offset greater than file size.");
        }
        const uint64_t available = size_ - offset;
        if (n > available) {
          n = static_cast<size_t>(available);
        }
        if (n == 0) {
          *result = Slice();
          return Status::OK();
        }

        assert(offset / kBlockSize <= std::numeric_limits<size_t>::max());
        size_t block = static_cast<size_t>(offset / kBlockSize);
        size_t block_offset = offset % kBlockSize;
        size_t bytes_to_copy = n;
        char* dst = scratch;

        while (bytes_to_copy > 0) {
          size_t avail = kBlockSize - block_offset;
          if (avail > bytes_to_copy) {
            avail = bytes_to_copy;
          }
          memcpy(dst, blocks_[block] + block_offset, avail);

          bytes_to_copy -= avail;
          dst += avail;
          block++;
          block_offset = 0;
        }

        *result = Slice(scratch, n);
        return Status::OK();
        */
    }
    
    pub fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            const char* src = data.data();
        size_t src_len = data.size();

        MutexLock lock(&blocks_mutex_);
        while (src_len > 0) {
          size_t avail;
          size_t offset = size_ % kBlockSize;

          if (offset != 0) {
            // There is some room in the last block.
            avail = kBlockSize - offset;
          } else {
            // No room in the last block; push new one.
            blocks_.push_back(new char[kBlockSize]);
            avail = kBlockSize;
          }

          if (avail > src_len) {
            avail = src_len;
          }
          memcpy(blocks_.back() + offset, src, avail);
          src_len -= avail;
          src += avail;
          size_ += avail;
        }

        return Status::OK();
        */
    }
}
