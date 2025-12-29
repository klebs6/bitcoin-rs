// ---------------- [ File: bitcoinleveldb-specialenv/src/counting_file.rs ]
crate::ix!();

pub struct CountingFile {
    pub(crate) target:  *mut Box<dyn RandomAccessFile>,
    pub(crate) counter: *const AtomicCounter,
}

impl Drop for CountingFile {
    fn drop(&mut self) {
        trace!("CountingFile::drop");
        unsafe {
            if !self.target.is_null() {
                drop(Box::from_raw(self.target));
                self.target = std::ptr::null_mut();
            }
        }
    }
}

impl RandomAccessFile for CountingFile {}

impl Named for CountingFile {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}

impl RandomAccessFileRead for CountingFile {
    fn read(
        &self,
        offset: u64,
        n: usize,
        result: *mut Slice,
        scratch: *mut u8,
    ) -> crate::Status {
        trace!(
            offset = offset,
            n = n,
            "CountingFile::read (incrementing random-read counter)"
        );

        unsafe {
            if !self.counter.is_null() {
                (&*self.counter).increment();
            }
        }

        unsafe { (&*self.target).read(offset, n, result, scratch) }
    }
}

#[cfg(test)]
mod counting_file_contract_suite {
    crate::ix!();

    use super::*;
    use std::borrow::Cow;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

    struct ReadTrackingState {
        read_calls:  AtomicUsize,
        drop_calls:  AtomicUsize,
        last_offset: AtomicU64,
        last_n:      AtomicUsize,
    }

    impl ReadTrackingState {
        fn new() -> Self {
            Self {
                read_calls: AtomicUsize::new(0),
                drop_calls: AtomicUsize::new(0),
                last_offset: AtomicU64::new(0),
                last_n: AtomicUsize::new(0),
            }
        }

        fn read_calls(&self) -> usize {
            self.read_calls.load(Ordering::SeqCst)
        }

        fn drop_calls(&self) -> usize {
            self.drop_calls.load(Ordering::SeqCst)
        }

        fn last_offset(&self) -> u64 {
            self.last_offset.load(Ordering::SeqCst)
        }

        fn last_n(&self) -> usize {
            self.last_n.load(Ordering::SeqCst)
        }
    }

    #[derive(Clone)]
    struct ReadTrackingRandomAccessFile {
        state: Arc<ReadTrackingState>,
        fill:  u8,
    }

    impl RandomAccessFile for ReadTrackingRandomAccessFile {}

    impl Named for ReadTrackingRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Borrowed("ReadTrackingRandomAccessFile")
        }
    }

    impl RandomAccessFileRead for ReadTrackingRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> crate::Status {
            debug!(offset, n, "ReadTrackingRandomAccessFile::read");
            self.state.read_calls.fetch_add(1, Ordering::SeqCst);
            self.state.last_offset.store(offset, Ordering::SeqCst);
            self.state.last_n.store(n, Ordering::SeqCst);

            if !result.is_null() {
                unsafe {
                    *result = Slice::default();
                }
            }

            if n > 0 && !scratch.is_null() && !result.is_null() {
                unsafe {
                    let buf = std::slice::from_raw_parts_mut(scratch, n);
                    for b in buf.iter_mut() {
                        *b = self.fill;
                    }
                    *result = Slice::from_ptr_len(scratch as *const u8, n);
                }
            }

            crate::Status::ok()
        }
    }

    impl Drop for ReadTrackingRandomAccessFile {
        fn drop(&mut self) {
            trace!("ReadTrackingRandomAccessFile::drop");
            self.state.drop_calls.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn allocate_random_access_file_ptr(
        state: Arc<ReadTrackingState>,
        fill: u8,
    ) -> *mut Box<dyn RandomAccessFile> {
        let inner: Box<dyn RandomAccessFile> = Box::new(ReadTrackingRandomAccessFile { state, fill });
        Box::into_raw(Box::new(inner))
    }

    #[traced_test]
    fn counting_file_increments_counter_on_each_read_and_delegates_to_target() {
        trace!("test: counting_file_increments_counter_on_each_read_and_delegates_to_target");

        let state = Arc::new(ReadTrackingState::new());
        let target_ptr = allocate_random_access_file_ptr(state.clone(), b'x');

        let counter = AtomicCounter::default();

        {
            let counting = CountingFile {
                target: target_ptr,
                counter: &counter as *const AtomicCounter,
            };

            let mut result = Slice::default();
            let mut scratch = vec![0u8; 4];

            let s1 = RandomAccessFileRead::read(
                &counting,
                123,
                4,
                &mut result as *mut Slice,
                scratch.as_mut_ptr(),
            );
            assert!(s1.is_ok());

            let observed1 = counter.read();
            debug!(observed1, "counter after first read");
            assert_eq!(observed1, 1);

            assert_eq!(state.read_calls(), 1);
            assert_eq!(state.last_offset(), 123);
            assert_eq!(state.last_n(), 4);

            let result_str = result.to_string();
            debug!(result_str = %result_str, "read result to_string");
            assert_eq!(result_str, "xxxx");

            let mut result2 = Slice::default();
            let mut scratch2 = vec![0u8; 2];

            let s2 = RandomAccessFileRead::read(
                &counting,
                999,
                2,
                &mut result2 as *mut Slice,
                scratch2.as_mut_ptr(),
            );
            assert!(s2.is_ok());

            assert_eq!(counter.read(), 2);
            assert_eq!(state.read_calls(), 2);
            assert_eq!(state.last_offset(), 999);
            assert_eq!(state.last_n(), 2);
            assert_eq!(result2.to_string(), "xx");
        }

        info!(
            target_drop_calls = state.drop_calls(),
            "target drop count after CountingFile drop"
        );
        assert_eq!(state.drop_calls(), 1);
    }

    #[traced_test]
    fn counting_file_delegates_reads_when_counter_is_null() {
        trace!("test: counting_file_delegates_reads_when_counter_is_null");

        let state = Arc::new(ReadTrackingState::new());
        let target_ptr = allocate_random_access_file_ptr(state.clone(), b'a');

        {
            let counting = CountingFile {
                target: target_ptr,
                counter: std::ptr::null(),
            };

            let mut result = Slice::default();
            let mut scratch = vec![0u8; 3];

            let s = RandomAccessFileRead::read(
                &counting,
                0,
                3,
                &mut result as *mut Slice,
                scratch.as_mut_ptr(),
            );
            assert!(s.is_ok());

            assert_eq!(state.read_calls(), 1);
            assert_eq!(state.last_offset(), 0);
            assert_eq!(state.last_n(), 3);
            assert_eq!(result.to_string(), "aaa");
        }

        assert_eq!(state.drop_calls(), 1);
    }

    #[traced_test]
    fn counting_file_name_is_empty_string_as_documented() {
        trace!("test: counting_file_name_is_empty_string_as_documented");

        let counting = CountingFile {
            target: std::ptr::null_mut(),
            counter: std::ptr::null(),
        };

        let name = Named::name(&counting);
        debug!(name = %name, "CountingFile::name");
        assert_eq!(name.as_ref(), "");
    }

    #[traced_test]
    fn counting_file_drop_is_noop_when_target_is_null() {
        trace!("test: counting_file_drop_is_noop_when_target_is_null");

        let counting = CountingFile {
            target: std::ptr::null_mut(),
            counter: std::ptr::null(),
        };

        drop(counting);
        assert!(true);
    }

    #[traced_test]
    fn counting_file_handles_zero_length_reads() {
        trace!("test: counting_file_handles_zero_length_reads");

        let state = Arc::new(ReadTrackingState::new());
        let target_ptr = allocate_random_access_file_ptr(state.clone(), b'z');

        let counter = AtomicCounter::default();

        {
            let counting = CountingFile {
                target: target_ptr,
                counter: &counter as *const AtomicCounter,
            };

            let mut result = Slice::default();
            let mut scratch: Vec<u8> = Vec::new();

            let s = RandomAccessFileRead::read(
                &counting,
                7,
                0,
                &mut result as *mut Slice,
                scratch.as_mut_ptr(),
            );
            assert!(s.is_ok());

            assert_eq!(counter.read(), 1);
            assert_eq!(state.read_calls(), 1);
            assert_eq!(state.last_offset(), 7);
            assert_eq!(state.last_n(), 0);
            assert_eq!(result.to_string(), "");
        }

        assert_eq!(state.drop_calls(), 1);
    }
}
