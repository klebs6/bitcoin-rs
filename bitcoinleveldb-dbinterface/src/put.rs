// ---------------- [ File: bitcoinleveldb-dbinterface/src/put.rs ]
crate::ix!();

pub trait DBPut: DBWrite {

    /// Set the database entry for "key" to "value".
    ///
    /// Returns OK on success, and a non-OK status on error.
    /// 
    /// Note: consider setting options.sync = true.
    /// 
    /// Default implementations of convenience methods that subclasses of DB can call if they wish
    /// 
    fn put(&mut self, opt: &WriteOptions, key_: &Slice, value: &Slice) -> crate::Status
    {
        let mut batch = WriteBatch::new();
        batch.put(key_, value);
        self.write(opt, &mut batch as *mut WriteBatch)
    }
}

#[cfg(test)]
mod put_convenience_method_contract_suite {
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

        trace!(sequence, count, rep_len = rep.len(), "decoded write batch header");

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

    struct WriteCallSpy {
        write_calls: usize,
        last_sequence: Option<u64>,
        last_count: Option<u32>,
        last_records: Vec<DecodedWriteBatchRecord>,
        status_to_return: crate::Status,
    }

    impl WriteCallSpy {
        fn new(status_to_return: crate::Status) -> Self {
            Self {
                write_calls: 0,
                last_sequence: None,
                last_count: None,
                last_records: Vec::new(),
                status_to_return,
            }
        }
    }

    impl DBInterfaceWrite for WriteCallSpy {
        fn write(&mut self, _options: &WriteOptions, updates: *mut WriteBatch) -> crate::Status {
            trace!(updates_ptr = (updates as usize), "write invoked");

            self.write_calls += 1;

            assert!(!updates.is_null(), "updates must not be null");
            let batch = unsafe { &*updates };

            let rep_bytes = bytes_of_string(batch.rep());
            let (sequence, count, records) = decode_write_batch_rep(rep_bytes);

            debug!(
                sequence,
                count,
                record_len = records.len(),
                "write decoded records"
            );

            self.last_sequence = Some(sequence);
            self.last_count = Some(count);
            self.last_records = records;

            self.status_to_return.clone()
        }
    }

    impl Put for WriteCallSpy {}

    #[traced_test]
    fn put_builds_single_put_batch_and_invokes_write_once() {
        let mut db = WriteCallSpy::new(crate::Status::ok());

        let opt = WriteOptions::default();
        let key = Slice::from("alpha");
        let val = Slice::from("beta");

        trace!("calling Put::put convenience method");
        let s = db.put(&opt, &key, &val);

        assert!(s.is_ok());
        assert_eq!(db.write_calls, 1);

        assert_eq!(db.last_sequence, Some(0));
        assert_eq!(db.last_count, Some(1));
        assert_eq!(
            db.last_records,
            vec![DecodedWriteBatchRecord::Put {
                key: b"alpha".to_vec(),
                value: b"beta".to_vec()
            }]
        );

        info!("verified put() encodes exactly one put record and calls write() once");
    }

    #[traced_test]
    fn put_propagates_non_ok_status_from_write() {
        let err = crate::Status::io_error(&Slice::from("io"), None);

        let mut db = WriteCallSpy::new(err.clone());

        let opt = WriteOptions::default();
        let key = Slice::from("k");
        let val = Slice::from("v");

        trace!("calling put() expecting error propagation");
        let s = db.put(&opt, &key, &val);

        assert!(s.is_io_error());
        assert_eq!(s.code(), err.code());
        assert_eq!(db.write_calls, 1);

        info!("verified put() returns the exact status produced by write()");
    }

    #[traced_test]
    fn put_handles_empty_value_by_encoding_zero_length_value() {
        let mut db = WriteCallSpy::new(crate::Status::ok());

        let opt = WriteOptions::default();
        let key = Slice::from("k");
        let val = Slice::from("");

        trace!("calling put() with empty value");
        let s = db.put(&opt, &key, &val);

        assert!(s.is_ok());
        assert_eq!(db.write_calls, 1);
        assert_eq!(db.last_count, Some(1));
        assert_eq!(
            db.last_records,
            vec![DecodedWriteBatchRecord::Put {
                key: b"k".to_vec(),
                value: Vec::new()
            }]
        );

        info!("verified put() supports empty value encoding");
    }

    #[traced_test]
    fn put_creates_a_fresh_single_record_batch_per_call() {
        let mut db = WriteCallSpy::new(crate::Status::ok());
        let opt = WriteOptions::default();

        let k1 = Slice::from("k1");
        let v1 = Slice::from("v1");

        let k2 = Slice::from("k2");
        let v2 = Slice::from("v2");

        trace!("first put()");
        let s1 = db.put(&opt, &k1, &v1);
        assert!(s1.is_ok());
        assert_eq!(db.write_calls, 1);
        assert_eq!(db.last_count, Some(1));
        assert_eq!(
            db.last_records,
            vec![DecodedWriteBatchRecord::Put {
                key: b"k1".to_vec(),
                value: b"v1".to_vec()
            }]
        );

        trace!("second put()");
        let s2 = db.put(&opt, &k2, &v2);
        assert!(s2.is_ok());
        assert_eq!(db.write_calls, 2);
        assert_eq!(db.last_count, Some(1));
        assert_eq!(
            db.last_records,
            vec![DecodedWriteBatchRecord::Put {
                key: b"k2".to_vec(),
                value: b"v2".to_vec()
            }]
        );

        info!("verified put() produces a single-record batch each invocation");
    }
}
