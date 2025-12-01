crate::ix!();

impl FileState {
   
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

    pub fn append(&mut self, data: &Slice) -> crate::Status {
        use std::alloc::{alloc, Layout};
        use std::ptr;

        let src_len_total = data.size();
        trace!(
            "FileState::append: appending {} bytes to in‑memory file",
            src_len_total
        );

        if src_len_total == 0 {
            debug!("FileState::append: nothing to append (length=0)");
            return crate::Status::ok();
        }

        unsafe {
            let mut src = data.data();
            let mut src_len = src_len_total;

            let mut blocks_ref = self.blocks_mutex.borrow_mut();
            let mut guard = match blocks_ref.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    warn!("FileState::append: blocks_mutex poisoned; recovering");
                    poisoned.into_inner()
                }
            };

            let layout = match Layout::array::<u8>(FILE_STATE_BLOCK_SIZE) {
                Ok(layout) => layout,
                Err(err) => {
                    error!(
                        "FileState::append: failed to construct allocation layout: {:?}",
                        err
                    );
                    let msg = Slice::from(
                        "Allocation failure in FileState::append()".as_bytes(),
                    );
                    return crate::Status::io_error(&msg, None);
                }
            };

            while src_len > 0 {
                let offset_in_block =
                    (guard.size as usize) % FILE_STATE_BLOCK_SIZE;

                let mut avail = if offset_in_block != 0 {
                    FILE_STATE_BLOCK_SIZE - offset_in_block
                } else {
                    debug!(
                        "FileState::append: allocating new block, current blocks={}",
                        guard.blocks.len()
                    );
                    let block_ptr = alloc(layout);
                    if block_ptr.is_null() {
                        error!("FileState::append: alloc returned null");
                        let msg = Slice::from(
                            "Allocation failure in FileState::append()".as_bytes(),
                        );
                        return crate::Status::io_error(&msg, None);
                    }
                    guard.blocks.push(block_ptr);
                    FILE_STATE_BLOCK_SIZE
                };

                if avail > src_len {
                    avail = src_len;
                }

                if let Some(&block_ptr) = guard.blocks.last() {
                    if block_ptr.is_null() {
                        error!(
                            "FileState::append: last block pointer is null (index={})",
                            guard.blocks.len() - 1
                        );
                        let msg =
                            Slice::from("Corrupt in‑memory append state.".as_bytes());
                        return crate::Status::corruption(&msg, None);
                    }

                    let dst = block_ptr.add(offset_in_block);
                    ptr::copy_nonoverlapping(src, dst, avail);
                } else {
                    error!("FileState::append: no blocks available after allocation");
                    let msg =
                        Slice::from("Internal append state error.".as_bytes());
                    return crate::Status::corruption(&msg, None);
                }

                src = src.add(avail);
                src_len -= avail;
                guard.size += avail as u64;
            }

            debug!(
                "FileState::append: successfully appended {} bytes; new size={}",
                src_len_total,
                guard.size
            );
        }

        crate::Status::ok()
    }
}
