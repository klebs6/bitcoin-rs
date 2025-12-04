// ---------------- [ File: bitcoinleveldb-memenv/src/file_state_read.rs ]
crate::ix!();

impl FileState {

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
            let blocks_ref = self.blocks_mutex().borrow();
            let mut guard = blocks_ref.lock();

            let size = *guard.size();
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
                if block >= guard.blocks().len() {
                    error!(
                        "FileState::read: block index {} out of range ({} blocks)",
                        block,
                        guard.blocks().len()
                    );
                    let msg =
                        Slice::from("Read beyond end of in‑memory blocks.".as_bytes());
                    return crate::Status::io_error(&msg, None);
                }

                let mut avail = FILE_STATE_BLOCK_SIZE - block_offset;
                if avail > bytes_to_copy {
                    avail = bytes_to_copy;
                }

                let src_block_ptr = guard.blocks()[block];
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

#[cfg(test)]
mod file_state_read_tests {
    use super::*;

    fn append_bytes(file: &mut FileState, bytes: &[u8]) {
        let slice = Slice::from(bytes);
        let status = file.append(&slice);
        assert!(status.is_ok());
    }

    #[traced_test]
    fn read_from_empty_file_returns_empty_slice() {
        crate::ix!();

        let file = FileState::default();
        let mut scratch = vec![0_u8; 8];
        let mut result = Slice::default();

        let status = file.read(
            0,
            scratch.len(),
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );

        assert!(status.is_ok());
        assert_eq!(*result.size(), 0);
    }

    #[traced_test]
    fn read_with_offset_and_length_round_trips() {
        crate::ix!();

        let mut file = FileState::default();
        let payload = b"The quick brown fox jumps over the lazy dog";
        append_bytes(&mut file, payload);

        // Choose an offset in the middle and a shorter length.
        let offset = 10_u64;
        let read_len = 7usize;

        let mut scratch = vec![0_u8; read_len];
        let mut result = Slice::default();

        let status = file.read(
            offset,
            read_len,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(status.is_ok());
        assert_eq!(*result.size(), read_len);

        let start = offset as usize;
        let end = start + read_len;
        assert_eq!(&scratch[..read_len], &payload[start..end]);
    }

    #[traced_test]
    fn read_clamps_length_to_available_bytes() {
        crate::ix!();

        let mut file = FileState::default();
        let payload = b"short";
        append_bytes(&mut file, payload);

        // Request more bytes than available.
        let mut scratch = vec![0_u8; 32];
        let mut result = Slice::default();

        let status = file.read(
            0,
            scratch.len(),
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(status.is_ok());
        assert_eq!(*result.size(), payload.len());
        assert_eq!(&scratch[..payload.len()], &payload[..]);
    }

    #[traced_test]
    fn read_with_offset_greater_than_size_returns_io_error() {
        crate::ix!();

        let mut file = FileState::default();
        let payload = b"data";
        append_bytes(&mut file, payload);

        let mut scratch = vec![0_u8; 4];
        let mut result = Slice::default();

        let status = file.read(
            10, // offset > size
            scratch.len(),
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );

        assert!(status.is_io_error());
    }
}
