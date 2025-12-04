// ---------------- [ File: bitcoinleveldb-memenv/src/file_state_append.rs ]
crate::ix!();

impl FileState {
   
    pub fn append(&mut self, data: &Slice) -> crate::Status {
        use std::alloc::{alloc, Layout};
        use std::ptr;

        let src_len_total: usize = *data.size();
        trace!(
            "FileState::append: appending {} bytes to in‑memory file",
            src_len_total
        );

        if src_len_total == 0 {
            debug!("FileState::append: nothing to append (length=0)");
            return crate::Status::ok();
        }

        unsafe {
            let mut src: *const u8 = *data.data();
            let mut src_len: usize = src_len_total;

            let mut blocks_ref = self.blocks_mutex().borrow_mut();
            let mut guard = blocks_ref.lock();

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
                    (*guard.size() as usize) % FILE_STATE_BLOCK_SIZE;

                let mut avail: usize = if offset_in_block != 0 {
                    // There is some room in the last block.
                    FILE_STATE_BLOCK_SIZE - offset_in_block
                } else {
                    // No room in the last block; push new one.
                    debug!(
                        "FileState::append: allocating new block, current blocks={}",
                        guard.blocks().len()
                    );
                    let block_ptr = alloc(layout);
                    if block_ptr.is_null() {
                        error!("FileState::append: alloc returned null");
                        let msg = Slice::from(
                            "Allocation failure in FileState::append()".as_bytes(),
                        );
                        return crate::Status::io_error(&msg, None);
                    }
                    guard.blocks_mut().push(block_ptr);
                    FILE_STATE_BLOCK_SIZE
                };

                if avail > src_len {
                    avail = src_len;
                }

                if let Some(&block_ptr) = guard.blocks().last() {
                    if block_ptr.is_null() {
                        error!(
                            "FileState::append: last block pointer is null (index={})",
                            guard.blocks().len() - 1
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
                *guard.size_mut() += avail as u64;
            }

            debug!(
                "FileState::append: successfully appended {} bytes; new size={}",
                src_len_total,
                guard.size()
            );
        }

        crate::Status::ok()
    }
}

#[cfg(test)]
mod file_state_append_tests {
    use super::*;

    fn make_slice_from_bytes(bytes: &[u8]) -> Slice {
        Slice::from(bytes)
    }

    #[traced_test]
    fn append_empty_slice_is_noop_and_ok() {
        crate::ix!();

        let mut file = FileState::default();
        let before_size = file.size();

        let empty: [u8; 0] = [];
        let slice = make_slice_from_bytes(&empty);

        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(file.size(), before_size);
    }

    #[traced_test]
    fn append_single_small_buffer_round_trips_via_read() {
        crate::ix!();

        let mut file = FileState::default();
        let data = b"hello in-memory world";
        let slice = make_slice_from_bytes(&data[..]);

        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(file.size(), data.len() as u64);

        let mut scratch = vec![0_u8; data.len()];
        let mut result = Slice::default();

        let status = file.read(
            0,
            data.len(),
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(status.is_ok());
        assert_eq!(*result.size(), data.len());
        assert_eq!(&scratch[..data.len()], &data[..]);
    }

    #[traced_test]
    fn append_large_buffer_spans_multiple_blocks_correctly() {
        crate::ix!();

        let mut file = FileState::default();
        let total_len = FILE_STATE_BLOCK_SIZE * 2 + 123;

        let mut payload = Vec::with_capacity(total_len);
        for i in 0..total_len {
            payload.push((i % 251) as u8);
        }

        let slice = Slice::from(payload.as_slice());
        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(file.size(), total_len as u64);

        // Read a window that straddles a block boundary.
        let offset = (FILE_STATE_BLOCK_SIZE - 10) as u64;
        let read_len = 64usize;

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
    fn append_multiple_calls_accumulate_size() {
        crate::ix!();

        let mut file = FileState::default();

        let part1 = b"first ";
        let part2 = b"second ";
        let part3 = b"third";

        for part in [&part1[..], &part2[..], &part3[..]].iter() {
            let slice = Slice::from(*part);
            let status = file.append(&slice);
            assert!(status.is_ok());
        }

        let expected = [&part1[..], &part2[..], &part3[..]].concat();

        assert_eq!(file.size(), expected.len() as u64);

        let mut scratch = vec![0_u8; expected.len()];
        let mut result = Slice::default();
        let status = file.read(
            0,
            expected.len(),
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(status.is_ok());
        assert_eq!(&scratch[..expected.len()], &expected[..]);
    }
}
