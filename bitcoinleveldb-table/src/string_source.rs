// ---------------- [ File: bitcoinleveldb-table/src/string_source.rs ]
crate::ix!();

pub struct StringSource {
    contents: String,
}

impl RandomAccessFile for StringSource { }

impl RandomAccessFileRead for StringSource {
    fn read(
        &self,
        offset:  u64,
        mut n:   usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> crate::Status {
        unsafe {
            assert!(
                !result.is_null(),
                "StringSource::read: result pointer is null"
            );
            assert!(
                !scratch.is_null(),
                "StringSource::read: scratch pointer is null"
            );

            let len = self.contents.len() as u64;
            trace!(
                "StringSource::read: offset={}, requested_n={}, total_len={}",
                offset,
                n,
                len
            );

            if offset >= len {
                let msg = b"invalid Read offset";
                let msg_slice = Slice::from(&msg[..]);

                error!(
                    "StringSource::read: invalid offset {}; len={}",
                    offset,
                    len
                );

                return crate::Status::invalid_argument(
                    &msg_slice,
                    None,
                );
            }

            let available = (len - offset) as usize;
            if n > available {
                n = available;
            }

            let src_bytes = self.contents.as_bytes();
            let src_ptr = src_bytes.as_ptr().add(offset as usize);

            core::ptr::copy_nonoverlapping(src_ptr, scratch, n);

            *result =
                Slice::from_ptr_len(scratch as *const u8, n);

            trace!(
                "StringSource::read: fulfilled {} bytes from offset {}",
                n,
                offset
            );

            crate::Status::ok()
        }
    }
}

impl Named for StringSource {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Owned(String::from("StringSource"))
    }
}

impl StringSource {

    pub fn new(contents: &Slice) -> Self {
        unsafe {
            let ptr = *contents.data();
            let len = *contents.size();
            let bytes = core::slice::from_raw_parts(ptr, len);
            let s = String::from_utf8_lossy(bytes).to_string();

            trace!(
                "StringSource::new: initialized with {} bytes",
                len
            );

            StringSource { contents: s }
        }
    }

    pub fn size(&self) -> u64 {
        self.contents.len() as u64
    }
}

#[cfg(test)]
mod string_source_behavior_tests {
    use super::*;

    #[traced_test]
    fn string_source_new_and_size_match_input_slice() {
        let data = b"0123456789abcdef";
        let src_slice = Slice::from(&data[..]);

        let src = StringSource::new(&src_slice);
        assert_eq!(src.size(), data.len() as u64);
    }

    #[traced_test]
    fn string_source_read_from_start_returns_requested_bytes() {
        let data = b"abcdef";
        let src_slice = Slice::from(&data[..]);
        let src = StringSource::new(&src_slice);

        let mut result = Slice::default();
        let mut scratch = vec![0u8; 16];

        let status = src.read(
            0,
            3,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );

        assert!(status.is_ok());
        assert_eq!(*result.size(), 3);

        unsafe {
            let out_bytes = core::slice::from_raw_parts(
                *result.data(),
                *result.size(),
            );
            assert_eq!(out_bytes, b"abc");
        }
    }

    #[traced_test]
    fn string_source_read_clamps_at_end_of_contents() {
        let data = b"abcdef";
        let src_slice = Slice::from(&data[..]);
        let src = StringSource::new(&src_slice);

        let mut result = Slice::default();
        let mut scratch = vec![0u8; 16];

        let status = src.read(
            3,
            10,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );

        assert!(status.is_ok());
        assert_eq!(*result.size(), 3);

        unsafe {
            let out_bytes = core::slice::from_raw_parts(
                *result.data(),
                *result.size(),
            );
            assert_eq!(out_bytes, b"def");
        }
    }

    #[traced_test]
    fn string_source_read_with_invalid_offset_returns_error() {
        let data = b"abc";
        let src_slice = Slice::from(&data[..]);
        let src = StringSource::new(&src_slice);

        let mut result = Slice::default();
        let mut scratch = vec![0u8; 8];

        let status = src.read(
            10,
            1,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );

        assert!(
            !status.is_ok(),
            "string_source_read_with_invalid_offset_returns_error: expected non‑OK status"
        );
    }
}
