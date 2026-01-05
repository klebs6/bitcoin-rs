// ---------------- [ File: bitcoinleveldb-dbinterface/src/write.rs ]
crate::ix!();

pub trait DBWrite {

    /// Apply the specified updates to the database.
    /// 
    /// Returns OK on success, non-OK on failure.
    /// 
    /// Note: consider setting options.sync = true.
    ///
    fn write(&mut self, 
            options: &WriteOptions,
            updates: *mut WriteBatch) -> crate::Status;
}

#[cfg(test)]
mod write_interface_contract_suite {
    use super::*;
    use tracing::{debug, error, info, trace, warn};

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum DecodedWriteBatchRecord {
        Put { key: Vec<u8>, value: Vec<u8> },
        Delete { key: Vec<u8> },
    }

    fn bytes_of_string(s: &String) -> &[u8] {
        unsafe { core::slice::from_raw_parts(s.as_ptr(), s.len()) }
    }

    fn decode_fixed32_le(input: &[u8]) -> u32 {
        assert!(input.len() >= 4);
        (input[0] as u32)
            | ((input[1] as u32) << 8)
            | ((input[2] as u32) << 16)
            | ((input[3] as u32) << 24)
    }

    fn decode_fixed64_le(input: &[u8]) -> u64 {
        assert!(input.len() >= 8);
        (input[0] as u64)
            | ((input[1] as u64) << 8)
            | ((input[2] as u64) << 16)
            | ((input[3] as u64) << 24)
            | ((input[4] as u64) << 32)
            | ((input[5] as u64) << 40)
            | ((input[6] as u64) << 48)
            | ((input[7] as u64) << 56)
    }

    fn decode_varint32(input: &[u8], index: &mut usize) -> Option<u32> {
        let mut result: u32 = 0;
        let mut shift: u32 = 0;

        while shift <= 28 {
            if *index >= input.len() {
                return None;
            }

            let byte = input[*index];
            *index += 1;

            result |= ((byte & 0x7f) as u32) << shift;

            if (byte & 0x80) == 0 {
                return Some(result);
            }

            shift += 7;
        }

        None
    }

    fn decode_write_batch_rep(rep: &[u8]) -> (u64, u32, Vec<DecodedWriteBatchRecord>) {
        assert!(
            rep.len() >= 12,
            "WriteBatch rep must contain at least the 12-byte header"
        );

        let sequence = decode_fixed64_le(&rep[0..8]);
        let count = decode_fixed32_le(&rep[8..12]);

        let mut idx: usize = 12;
        let mut records: Vec<DecodedWriteBatchRecord> = Vec::new();

        let mut i: u32 = 0;
        while i < count {
            assert!(idx < rep.len(), "unexpected end of WriteBatch rep");
            let tag = rep[idx];
            idx += 1;

            let key_len = decode_varint32(rep, &mut idx).expect("failed to decode key length");
            let key_len_usize: usize = key_len as usize;

            assert!(
                idx + key_len_usize <= rep.len(),
                "key length exceeds remaining rep bytes"
            );

            let key = rep[idx..(idx + key_len_usize)].to_vec();
            idx += key_len_usize;

            match tag {
                0x00 => {
                    records.push(DecodedWriteBatchRecord::Delete { key });
                }
                0x01 => {
                    let val_len =
                        decode_varint32(rep, &mut idx).expect("failed to decode value length");
                    let val_len_usize: usize = val_len as usize;

                    assert!(
                        idx + val_len_usize <= rep.len(),
                        "value length exceeds remaining rep bytes"
                    );

                    let value = rep[idx..(idx + val_len_usize)].to_vec();
                    idx += val_len_usize;

                    records.push(DecodedWriteBatchRecord::Put { key, value });
                }
                other => {
                    error!(tag = other, "unknown record tag in WriteBatch rep");
                    panic!("unknown WriteBatch record tag");
                }
            }

            i += 1;
        }

        (sequence, count, records)
    }

    struct RecordingWriteSink {
        calls: usize,
        last_sequence: Option<u64>,
        last_count: Option<u32>,
        last_records: Vec<DecodedWriteBatchRecord>,
        status_to_return: crate::Status,
    }

    impl RecordingWriteSink {
        fn new(status_to_return: crate::Status) -> Self {
            Self {
                calls: 0,
                last_sequence: None,
                last_count: None,
                last_records: Vec::new(),
                status_to_return,
            }
        }
    }

    impl DBInterfaceWrite for RecordingWriteSink {
        fn write(&mut self, _options: &WriteOptions, updates: *mut WriteBatch) -> crate::Status {
            self.calls += 1;

            assert!(!updates.is_null(), "updates must not be null");
            let batch = unsafe { &*updates };

            let rep = bytes_of_string(batch.rep());
            let (seq, count, records) = decode_write_batch_rep(rep);

            self.last_sequence = Some(seq);
            self.last_count = Some(count);
            self.last_records = records;

            self.status_to_return.clone()
        }
    }

    #[traced_test]
    fn write_can_be_called_through_trait_object_and_observes_batch_contents() {
        let mut sink = RecordingWriteSink::new(crate::Status::ok());

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("k"), &Slice::from("v"));

        let opt = WriteOptions::default();

        let w: &mut dyn DBInterfaceWrite = &mut sink;

        trace!("calling write() via trait object");
        let s = w.write(&opt, &mut batch as *mut WriteBatch);

        assert!(s.is_ok());
        assert_eq!(sink.calls, 1);
        assert_eq!(sink.last_sequence, Some(0));
        assert_eq!(sink.last_count, Some(1));
        assert_eq!(
            sink.last_records,
            vec![DecodedWriteBatchRecord::Put {
                key: b"k".to_vec(),
                value: b"v".to_vec()
            }]
        );

        info!("verified write() interface can be invoked and can observe batch payload via pointer");
    }

    #[traced_test]
    fn write_accepts_empty_batch_and_encodes_zero_count() {
        let mut sink = RecordingWriteSink::new(crate::Status::ok());

        let mut batch = WriteBatch::new();
        let opt = WriteOptions::default();

        trace!("calling write() with empty batch");
        let s = sink.write(&opt, &mut batch as *mut WriteBatch);

        assert!(s.is_ok());
        assert_eq!(sink.calls, 1);
        assert_eq!(sink.last_sequence, Some(0));
        assert_eq!(sink.last_count, Some(0));
        assert!(sink.last_records.is_empty());

        info!("verified empty batch carries count=0 and yields no records");
    }

    #[traced_test]
    fn write_observes_multiple_records_in_insertion_order() {
        let mut sink = RecordingWriteSink::new(crate::Status::ok());

        let mut batch = WriteBatch::new();
        batch.put(&Slice::from("k1"), &Slice::from("v1"));
        batch.delete(&Slice::from("k2"));

        let opt = WriteOptions::default();

        trace!("calling write() with multi-record batch");
        let s = sink.write(&opt, &mut batch as *mut WriteBatch);

        assert!(s.is_ok());
        assert_eq!(sink.calls, 1);
        assert_eq!(sink.last_count, Some(2));
        assert_eq!(
            sink.last_records,
            vec![
                DecodedWriteBatchRecord::Put {
                    key: b"k1".to_vec(),
                    value: b"v1".to_vec()
                },
                DecodedWriteBatchRecord::Delete {
                    key: b"k2".to_vec()
                }
            ]
        );

        info!("verified multi-record batches preserve record order");
    }
}
