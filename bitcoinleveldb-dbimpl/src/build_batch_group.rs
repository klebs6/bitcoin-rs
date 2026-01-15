// ---------------- [ File: bitcoinleveldb-dbimpl/src/build_batch_group.rs ]
crate::ix!();

impl DBImpl {
    /// REQUIRES: Writer list must be non-empty
    ///
    /// REQUIRES: First writer must have a non-null
    /// batch
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn build_batch_group(
        &mut self,
        last_writer: *mut *mut DBImplWriter,
    ) -> *mut WriteBatch {
        self.mutex.assert_held();
        assert!(!self.writers.is_empty());

        let first: *mut DBImplWriter = *self.writers.front().unwrap();
        let mut result: *mut WriteBatch = unsafe { *(*first).batch() };
        assert!(!result.is_null());

        let mut size: usize =
            unsafe { write_batch_internal::byte_size(result as *const WriteBatch) };

        // Allow the group to grow up to a maximum size, but if the
        // original write is small, limit the growth so we do not slow
        // down the small write too much.
        let mut max_size: usize = 1usize << 20;
        if size <= (128usize << 10) {
            max_size = size + (128usize << 10);
        }

        unsafe {
            *last_writer = first;
        }

        let mut iter = self.writers.iter();
        iter.next(); // Advance past "first"

        for wptr in iter {
            let w: *mut DBImplWriter = *wptr;

            if unsafe { *(*w).sync() } && !unsafe { *(*first).sync() } {
                // Do not include a sync write into a batch handled by a non-sync write.
                break;
            }

            let wbatch: *mut WriteBatch = unsafe { *(*w).batch() };
            if !wbatch.is_null() {
                size += unsafe {
                    write_batch_internal::byte_size(wbatch as *const WriteBatch)
                };
                if size > max_size {
                    // Do not make batch too big
                    break;
                }

                // Append to *result
                if result == unsafe { *(*first).batch() } {
                    // Switch to temporary batch instead of disturbing caller's batch
                    result = self.tmp_batch;
                    assert_eq!(
                        unsafe {
                            write_batch_internal::count(result as *const WriteBatch)
                        },
                        0
                    );
                    unsafe {
                        write_batch_internal::append(
                            result,
                            *(*first).batch() as *const WriteBatch,
                        );
                    }
                }

                unsafe {
                    write_batch_internal::append(
                        result,
                        wbatch as *const WriteBatch,
                    );
                }
            }

            unsafe {
                *last_writer = w;
            }
        }

        result
    }
}

#[cfg(test)]
mod build_batch_group_behavior_contract_suite {
    use super::*;

    enum BatchRecordSpec {
        Put { key_len: usize, value_len: usize },
        Delete { key_len: usize },
    }

    #[inline]
    fn encode_fixed32_le(value: u32) -> [u8; 4] {
        [
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
        ]
    }

    #[inline]
    fn encode_fixed64_le(value: u64) -> [u8; 8] {
        [
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            ((value >> 32) & 0xFF) as u8,
            ((value >> 40) & 0xFF) as u8,
            ((value >> 48) & 0xFF) as u8,
            ((value >> 56) & 0xFF) as u8,
        ]
    }

    #[inline]
    fn put_varint32_vec(dst: &mut Vec<u8>, mut value: u32) {
        while value >= 0x80 {
            dst.push(((value & 0x7F) as u8) | 0x80);
            value >>= 7;
        }
        dst.push(value as u8);
    }

    fn build_write_batch_rep_bytes(seq: u64, records: &[BatchRecordSpec]) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        // Header: sequence (fixed64) + count (fixed32)
        out.extend_from_slice(&encode_fixed64_le(seq));
        out.extend_from_slice(&encode_fixed32_le(records.len() as u32));

        for rec in records {
            match *rec {
                BatchRecordSpec::Put { key_len, value_len } => {
                    // Tag for value record (LevelDB: 1)
                    out.push(1u8);

                    let key: Vec<u8> = vec![b'k'; key_len];
                    put_varint32_vec(&mut out, key_len as u32);
                    out.extend_from_slice(&key);

                    let value: Vec<u8> = vec![b'v'; value_len];
                    put_varint32_vec(&mut out, value_len as u32);
                    out.extend_from_slice(&value);
                }
                BatchRecordSpec::Delete { key_len } => {
                    // Tag for deletion record (LevelDB: 0)
                    out.push(0u8);

                    let key: Vec<u8> = vec![b'd'; key_len];
                    put_varint32_vec(&mut out, key_len as u32);
                    out.extend_from_slice(&key);
                }
            }
        }

        out
    }

    fn set_write_batch_contents_from_records(batch: &mut WriteBatch, seq: u64, records: &[BatchRecordSpec]) {
        let bytes: Vec<u8> = build_write_batch_rep_bytes(seq, records);
        let slice: Slice = Slice::from_bytes(&bytes);
        write_batch_internal::set_contents(batch as *mut WriteBatch, &slice);

        tracing::debug!(
            seq,
            records = records.len() as u64,
            byte_size = write_batch_internal::byte_size(batch as *const WriteBatch) as u64,
            "Initialized WriteBatch contents for test"
        );
    }

    fn set_write_batch_to_empty(batch: &mut WriteBatch, seq: u64) {
        let records: [BatchRecordSpec; 0] = [];
        set_write_batch_contents_from_records(batch, seq, &records);
    }

    fn make_dbimpl_for_build_batch_group(
        writers: std::collections::VecDeque<*mut DBImplWriter>,
        tmp_batch_ptr: *mut WriteBatch,
    ) -> core::mem::ManuallyDrop<DBImpl> {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let dbname: String = "dbimpl-build-batch-group-test".to_string();

        let mut db: core::mem::ManuallyDrop<DBImpl> =
            core::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        unsafe {
            let db_mut: &mut DBImpl = &mut *(&mut db as *mut _ as *mut DBImpl);
            db_mut.writers = writers;
            db_mut.tmp_batch = tmp_batch_ptr;

            tracing::debug!(
                writers_len = db_mut.writers.len() as u64,
                tmp_batch_ptr = ?db_mut.tmp_batch,
                "Initialized DBImpl for build_batch_group tests"
            );
        }

        db
    }

    #[traced_test]
    fn build_batch_group_returns_first_batch_when_single_writer() {
        tracing::info!("Testing build_batch_group single-writer behavior");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut batch1: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch1,
            7,
            &[BatchRecordSpec::Put {
                key_len: 3,
                value_len: 5,
            }],
        );
        let batch1_ptr: *mut WriteBatch = &mut batch1 as *mut WriteBatch;

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(batch1_ptr);
        w1.set_sync(false);
        w1.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let result: *mut WriteBatch = unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
        };

        tracing::info!(
            result_ptr = ?result,
            expected_ptr = ?batch1_ptr,
            last_writer_ptr = ?last_writer,
            expected_last_writer = ?(&mut w1 as *mut DBImplWriter),
            "Observed build_batch_group outputs"
        );

        assert_eq!(result, batch1_ptr);
        assert_eq!(last_writer, &mut w1 as *mut DBImplWriter);

        let count: i32 = write_batch_internal::count(result as *const WriteBatch);
        assert_eq!(count, 1);
    }

    #[traced_test]
    fn build_batch_group_switches_to_tmp_batch_when_appending_additional_writer_batches() {
        tracing::info!("Testing build_batch_group tmp_batch switching and append behavior");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut batch1: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch1,
            10,
            &[BatchRecordSpec::Put {
                key_len: 4,
                value_len: 9,
            }],
        );
        let batch1_ptr: *mut WriteBatch = &mut batch1 as *mut WriteBatch;

        let batch1_before: Vec<u8> = {
            let c: Slice = write_batch_internal::contents(batch1_ptr as *const WriteBatch);
            c.as_bytes().to_vec()
        };

        let mut batch2: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch2,
            11,
            &[BatchRecordSpec::Delete { key_len: 7 }],
        );
        let batch2_ptr: *mut WriteBatch = &mut batch2 as *mut WriteBatch;

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(batch1_ptr);
        w1.set_sync(false);
        w1.set_done(false);

        let mut w2: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w2.set_batch(batch2_ptr);
        w2.set_sync(false);
        w2.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);
        writers.push_back(&mut w2 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let result: *mut WriteBatch = unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
        };

        tracing::info!(
            result_ptr = ?result,
            tmp_batch_ptr = ?tmp_batch_ptr,
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group outputs"
        );

        assert_eq!(result, tmp_batch_ptr);
        assert_eq!(last_writer, &mut w2 as *mut DBImplWriter);

        let first_after: Vec<u8> = {
            let c: Slice = write_batch_internal::contents(batch1_ptr as *const WriteBatch);
            c.as_bytes().to_vec()
        };

        assert_eq!(
            batch1_before, first_after,
            "First writer's batch must not be mutated when tmp_batch is used"
        );

        let grouped_count: i32 = write_batch_internal::count(result as *const WriteBatch);
        assert_eq!(grouped_count, 2);

        let grouped_size: usize = write_batch_internal::byte_size(result as *const WriteBatch);
        let size1: usize = write_batch_internal::byte_size(batch1_ptr as *const WriteBatch);
        let size2: usize = write_batch_internal::byte_size(batch2_ptr as *const WriteBatch);

        tracing::debug!(
            size1 = size1 as u64,
            size2 = size2 as u64,
            grouped_size = grouped_size as u64,
            "Grouped batch sizes"
        );

        assert!(
            grouped_size >= size1,
            "Grouped batch must be at least as large as the first batch"
        );
        assert!(
            grouped_size <= size1 + size2,
            "Grouped batch must not exceed the sum of input batch byte sizes"
        );
    }

    #[traced_test]
    fn build_batch_group_does_not_mix_sync_write_into_non_sync_group() {
        tracing::info!("Testing build_batch_group sync boundary: non-sync group must not include sync write");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut batch1: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch1,
            1,
            &[BatchRecordSpec::Put {
                key_len: 2,
                value_len: 2,
            }],
        );
        let batch1_ptr: *mut WriteBatch = &mut batch1 as *mut WriteBatch;

        let mut batch2: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch2,
            2,
            &[BatchRecordSpec::Put {
                key_len: 2,
                value_len: 2,
            }],
        );
        let batch2_ptr: *mut WriteBatch = &mut batch2 as *mut WriteBatch;

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(batch1_ptr);
        w1.set_sync(false);
        w1.set_done(false);

        let mut w2: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w2.set_batch(batch2_ptr);
        w2.set_sync(true);
        w2.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);
        writers.push_back(&mut w2 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let result: *mut WriteBatch = unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
        };

        tracing::info!(
            result_ptr = ?result,
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group outputs"
        );

        assert_eq!(result, batch1_ptr);
        assert_eq!(last_writer, &mut w1 as *mut DBImplWriter);

        let grouped_count: i32 = write_batch_internal::count(result as *const WriteBatch);
        assert_eq!(grouped_count, 1);
    }

    #[traced_test]
    fn build_batch_group_allows_non_sync_writes_to_follow_sync_leader() {
        tracing::info!("Testing build_batch_group: sync leader may batch subsequent non-sync writes");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut batch1: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch1,
            1,
            &[BatchRecordSpec::Put {
                key_len: 2,
                value_len: 3,
            }],
        );
        let batch1_ptr: *mut WriteBatch = &mut batch1 as *mut WriteBatch;

        let mut batch2: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch2,
            2,
            &[BatchRecordSpec::Delete { key_len: 5 }],
        );
        let batch2_ptr: *mut WriteBatch = &mut batch2 as *mut WriteBatch;

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(batch1_ptr);
        w1.set_sync(true);
        w1.set_done(false);

        let mut w2: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w2.set_batch(batch2_ptr);
        w2.set_sync(false);
        w2.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);
        writers.push_back(&mut w2 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let result: *mut WriteBatch = unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
        };

        tracing::info!(
            result_ptr = ?result,
            tmp_batch_ptr = ?tmp_batch_ptr,
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group outputs"
        );

        assert_eq!(result, tmp_batch_ptr);
        assert_eq!(last_writer, &mut w2 as *mut DBImplWriter);

        let grouped_count: i32 = write_batch_internal::count(result as *const WriteBatch);
        assert_eq!(grouped_count, 2);
    }

    #[traced_test]
    fn build_batch_group_stops_before_exceeding_max_size_for_small_first_batch() {
        tracing::info!("Testing build_batch_group max_size boundary for small first write");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut batch1: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch1,
            1,
            &[BatchRecordSpec::Put {
                key_len: 4,
                value_len: 8,
            }],
        );
        let batch1_ptr: *mut WriteBatch = &mut batch1 as *mut WriteBatch;

        let first_size: usize = write_batch_internal::byte_size(batch1_ptr as *const WriteBatch);
        tracing::debug!(first_size = first_size as u64, "First batch size");

        // Make second batch very large so first_size + second_size > first_size + 128KB.
        let mut batch2: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch2,
            2,
            &[BatchRecordSpec::Put {
                key_len: 16,
                value_len: 200_000,
            }],
        );
        let batch2_ptr: *mut WriteBatch = &mut batch2 as *mut WriteBatch;
        let second_size: usize = write_batch_internal::byte_size(batch2_ptr as *const WriteBatch);

        tracing::debug!(
            second_size = second_size as u64,
            "Second batch size (intentionally huge)"
        );

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(batch1_ptr);
        w1.set_sync(false);
        w1.set_done(false);

        let mut w2: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w2.set_batch(batch2_ptr);
        w2.set_sync(false);
        w2.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);
        writers.push_back(&mut w2 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let result: *mut WriteBatch = unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
        };

        tracing::info!(
            result_ptr = ?result,
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group outputs under max_size pressure"
        );

        assert_eq!(result, batch1_ptr);
        assert_eq!(last_writer, &mut w1 as *mut DBImplWriter);

        let grouped_count: i32 = write_batch_internal::count(result as *const WriteBatch);
        assert_eq!(grouped_count, 1);
    }

    #[traced_test]
    fn build_batch_group_skips_null_batches_and_can_still_include_later_batches() {
        tracing::info!("Testing build_batch_group handling of intermediate null batch writers");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut batch1: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch1,
            1,
            &[BatchRecordSpec::Put {
                key_len: 3,
                value_len: 3,
            }],
        );
        let batch1_ptr: *mut WriteBatch = &mut batch1 as *mut WriteBatch;

        let mut batch3: WriteBatch = WriteBatch::default();
        set_write_batch_contents_from_records(
            &mut batch3,
            3,
            &[BatchRecordSpec::Delete { key_len: 9 }],
        );
        let batch3_ptr: *mut WriteBatch = &mut batch3 as *mut WriteBatch;

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(batch1_ptr);
        w1.set_sync(false);
        w1.set_done(false);

        let mut w2: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w2.set_batch(core::ptr::null_mut()); // null batch should be skipped for append/size
        w2.set_sync(false);
        w2.set_done(false);

        let mut w3: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w3.set_batch(batch3_ptr);
        w3.set_sync(false);
        w3.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);
        writers.push_back(&mut w2 as *mut DBImplWriter);
        writers.push_back(&mut w3 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let result: *mut WriteBatch = unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
        };

        tracing::info!(
            result_ptr = ?result,
            tmp_batch_ptr = ?tmp_batch_ptr,
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group outputs with null intermediate writer"
        );

        assert_eq!(result, tmp_batch_ptr);
        assert_eq!(last_writer, &mut w3 as *mut DBImplWriter);

        let grouped_count: i32 = write_batch_internal::count(result as *const WriteBatch);
        assert_eq!(grouped_count, 2);
    }

    #[traced_test]
    fn build_batch_group_panics_on_empty_writer_queue_contract_violation() {
        tracing::info!("Testing build_batch_group contract: empty writer queue should panic");

        let writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = unsafe {
                (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
            };
        }))
        .is_err();

        tracing::debug!(
            panicked,
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group panic behavior on empty writer queue"
        );

        assert!(panicked);
    }

    #[traced_test]
    fn build_batch_group_panics_when_first_writer_batch_is_null_contract_violation() {
        tracing::info!("Testing build_batch_group contract: first writer batch must be non-null");

        let mut writer_mu: RawMutex = RawMutex::INIT;

        let mut w1: DBImplWriter = DBImplWriter::new(&mut writer_mu as *mut RawMutex);
        w1.set_batch(core::ptr::null_mut());
        w1.set_sync(false);
        w1.set_done(false);

        let mut writers: std::collections::VecDeque<*mut DBImplWriter> = Default::default();
        writers.push_back(&mut w1 as *mut DBImplWriter);

        let mut tmp_batch: WriteBatch = WriteBatch::default();
        set_write_batch_to_empty(&mut tmp_batch, 0);
        let tmp_batch_ptr: *mut WriteBatch = &mut tmp_batch as *mut WriteBatch;

        let mut db: core::mem::ManuallyDrop<DBImpl> =
            make_dbimpl_for_build_batch_group(writers, tmp_batch_ptr);

        let mut last_writer: *mut DBImplWriter = core::ptr::null_mut();

        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = unsafe {
                (&mut *(&mut db as *mut _ as *mut DBImpl)).build_batch_group(&mut last_writer)
            };
        }))
        .is_err();

        tracing::debug!(
            panicked,
            first_writer_ptr = ?(&mut w1 as *mut DBImplWriter),
            last_writer_ptr = ?last_writer,
            "Observed build_batch_group panic behavior when first writer batch is null"
        );

        assert!(panicked);
    }
}
