crate::ix!();

impl FileState {

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

    pub fn read(
        &self,
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> crate::Status {
        use std::ptr;

        trace!(
            "FileState::read: offset={}, requested_bytes={}, scratch={:?}, result={:?}",
            offset,
            n,
            scratch,
            result
        );

        if scratch.is_null() {
            error!("FileState::read: scratch buffer pointer is null");
            let msg = Slice::from("Null scratch buffer in FileState::read()".as_bytes());
            return crate::Status::io_error(&msg, None);
        }

        let mut requested = n;

        {
            let blocks_ref = self.blocks_mutex.borrow();
            let mut guard = match blocks_ref.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    warn!("FileState::read: blocks_mutex poisoned; recovering");
                    poisoned.into_inner()
                }
            };

            let size = guard.size;
            if offset > size {
                error!(
                    "FileState::read: offset {} is greater than file size {}",
                    offset,
                    size
                );
                let msg =
                    Slice::from("Offset greater than file size.".as_bytes());
                return crate::Status::io_error(&msg, None);
            }

            let available = size - offset;
            if requested as u64 > available {
                requested = available as usize;
            }

            if requested == 0 {
                debug!("FileState::read: nothing to read (requested=0 after clipping)");
                unsafe {
                    if !result.is_null() {
                        *result = Slice::default();
                    } else {
                        warn!(
                            "FileState::read: result pointer is null while returning empty slice"
                        );
                    }
                }
                return crate::Status::ok();
            }

            let block_index_u64 = offset / (FILE_STATE_BLOCK_SIZE as u64);
            if block_index_u64 > usize::MAX as u64 {
                error!(
                    "FileState::read: block index {} exceeds usize::MAX",
                    block_index_u64
                );
                let msg =
                    Slice::from("Offset is too large for this system.".as_bytes());
                return crate::Status::invalid_argument(&msg, None);
            }

            let mut block = block_index_u64 as usize;
            let mut block_offset =
                (offset % (FILE_STATE_BLOCK_SIZE as u64)) as usize;

            let mut bytes_to_copy = requested;
            let mut dst = scratch;

            while bytes_to_copy > 0 {
                if block >= guard.blocks.len() {
                    error!(
                        "FileState::read: block index {} out of range ({} blocks)",
                        block,
                        guard.blocks.len()
                    );
                    let msg =
                        Slice::from("Read beyond end of in‑memory blocks.".as_bytes());
                    return crate::Status::io_error(&msg, None);
                }

                let mut avail = FILE_STATE_BLOCK_SIZE - block_offset;
                if avail > bytes_to_copy {
                    avail = bytes_to_copy;
                }

                let src_block_ptr = guard.blocks[block];
                if src_block_ptr.is_null() {
                    error!(
                        "FileState::read: null block pointer at index {}",
                        block
                    );
                    let msg =
                        Slice::from("Corrupt in‑memory file block.".as_bytes());
                    return crate::Status::corruption(&msg, None);
                }

                unsafe {
                    ptr::copy_nonoverlapping(
                        src_block_ptr.add(block_offset),
                        dst,
                        avail,
                    );
                    dst = dst.add(avail);
                }

                bytes_to_copy -= avail;
                block += 1;
                block_offset = 0;
            }

            debug!(
                "FileState::read: copied {} bytes from in‑memory blocks",
                requested
            );
        }

        unsafe {
            if !result.is_null() {
                *result = Slice::from_ptr_len(scratch as *const u8, requested);
            } else {
                warn!("FileState::read: result pointer is null after successful read");
            }
        }

        crate::Status::ok()
    }
}
