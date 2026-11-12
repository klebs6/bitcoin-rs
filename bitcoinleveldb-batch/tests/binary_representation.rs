use bitcoinleveldb_batch::{
    write_batch_internal, WriteBatch, WriteBatchDelete, WriteBatchHandler,
    WriteBatchPut, HEADER,
};
use bitcoinleveldb_slice::Slice;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Put(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

#[derive(Default)]
struct Recorder {
    operations: Vec<Operation>,
}

impl WriteBatchHandler for Recorder {}

impl WriteBatchPut for Recorder {
    fn put(&mut self, key: &Slice, value: &Slice) {
        self.operations.push(Operation::Put(
            bitcoinleveldb_key::slice_as_bytes(key).to_vec(),
            bitcoinleveldb_key::slice_as_bytes(value).to_vec(),
        ));
    }
}

impl WriteBatchDelete for Recorder {
    fn delete(&mut self, key: &Slice) {
        self.operations.push(Operation::Delete(
            bitcoinleveldb_key::slice_as_bytes(key).to_vec(),
        ));
    }
}

fn operations(batch: &WriteBatch) -> Vec<Operation> {
    let mut recorder = Recorder::default();
    let status = batch.iterate(&mut recorder as *mut dyn WriteBatchHandler);
    assert!(status.is_ok());
    recorder.operations
}

// The format oracle uses standard fixed-width little-endian conversion and
// literal record bytes, rather than calling the production encoding helpers.
fn encoded(sequence: u64, count: i32, records: &[u8]) -> Vec<u8> {
    let mut expected = sequence.to_le_bytes().to_vec();
    expected.extend_from_slice(&(count as u32).to_le_bytes());
    expected.extend_from_slice(records);
    expected
}

#[test]
fn binary_records_preserve_literal_wire_format_and_handler_order() {
    let mut batch = WriteBatch::new();
    let key = [0x00, 0xff, 0x80];
    let value = [0xc3, 0x28, 0x00, 0xfe];
    let deleted = [0x80, 0x00, 0xff];
    batch.put(&Slice::from(&key[..]), &Slice::from(&value[..]));
    batch.delete(&Slice::from(&deleted[..]));
    write_batch_internal::set_sequence(&mut batch, 0xff80_00ff_0080_00ff);

    let expected = vec![
        0xff, 0x00, 0x80, 0x00, 0xff, 0x00, 0x80, 0xff,
        0x02, 0x00, 0x00, 0x00,
        0x01, 0x03, 0x00, 0xff, 0x80, 0x04, 0xc3, 0x28, 0x00, 0xfe,
        0x00, 0x03, 0x80, 0x00, 0xff,
    ];
    assert!(std::str::from_utf8(&expected).is_err());
    let representation: &Vec<u8> = batch.rep();
    assert_eq!(representation, &expected);
    assert_eq!(batch.approximate_size(), expected.len());
    assert_eq!(
        operations(&batch),
        vec![
            Operation::Put(key.to_vec(), value.to_vec()),
            Operation::Delete(deleted.to_vec()),
        ],
    );
}

#[test]
fn binary_varint_lengths_preserve_128_and_129_byte_payloads() {
    let mut batch = WriteBatch::new();
    let key = vec![0xff; 128];
    let value = vec![0x80; 129];
    batch.put(&Slice::from(&key[..]), &Slice::from(&value[..]));

    let mut record = vec![0x01, 0x80, 0x01];
    record.extend_from_slice(&key);
    record.extend_from_slice(&[0x81, 0x01]);
    record.extend_from_slice(&value);
    assert_eq!(batch.rep(), &encoded(0, 1, &record));
    assert_eq!(operations(&batch), vec![Operation::Put(key, value)]);
}

#[test]
fn binary_header_fields_preserve_non_utf8_fixed_width_bytes() {
    let mut batch = WriteBatch::new();
    let sequence = 0xff80_00ff_0080_00ff;
    let count = 0x00ff_0080;
    write_batch_internal::set_sequence(&mut batch, sequence);
    write_batch_internal::set_count(&mut batch, count);

    assert_eq!(
        batch.rep(),
        &vec![
            0xff, 0x00, 0x80, 0x00, 0xff, 0x00, 0x80, 0xff,
            0x80, 0x00, 0xff, 0x00,
        ],
    );
    assert_eq!(write_batch_internal::sequence(&batch), sequence);
    assert_eq!(write_batch_internal::count(&batch), count);
    // This checks raw header access, not validity of a record count with no
    // corresponding records. The iterator must still reject that mismatch.
    let mut recorder = Recorder::default();
    assert!(!batch.iterate(&mut recorder as *mut dyn WriteBatchHandler).is_ok());
    assert!(recorder.operations.is_empty());
}

#[test]
fn binary_append_set_contents_and_clear_round_trip_exact_bytes() {
    let mut destination = WriteBatch::new();
    let mut source = WriteBatch::new();
    destination.put(
        &Slice::from(&[0x00, 0xff, 0x80][..]),
        &Slice::from(&[0xc3, 0x28, 0x00, 0xfe][..]),
    );
    source.delete(&Slice::from(&[0x80, 0x00, 0xff][..]));
    source.put(&Slice::from(&[][..]), &Slice::from(&[0xff, 0x00][..]));
    write_batch_internal::set_sequence(&mut destination, 0xff80_00ff_0080_00ff);
    write_batch_internal::set_sequence(&mut source, 0x80ff);

    destination.append(&source);
    let expected = encoded(
        0xff80_00ff_0080_00ff,
        3,
        &[
            0x01, 0x03, 0x00, 0xff, 0x80, 0x04, 0xc3, 0x28, 0x00, 0xfe,
            0x00, 0x03, 0x80, 0x00, 0xff,
            0x01, 0x00, 0x02, 0xff, 0x00,
        ],
    );
    assert_eq!(destination.rep(), &expected);
    source.clear();
    assert_eq!(destination.rep(), &expected);

    let mut transport = expected.clone();
    let mut restored = WriteBatch::new();
    write_batch_internal::set_contents(&mut restored, &Slice::from(&transport[..]));
    transport.fill(0x5a);
    drop(transport);
    assert_eq!(restored.rep(), &expected);
    assert_eq!(
        operations(&restored),
        vec![
            Operation::Put(vec![0x00, 0xff, 0x80], vec![0xc3, 0x28, 0x00, 0xfe]),
            Operation::Delete(vec![0x80, 0x00, 0xff]),
            Operation::Put(vec![], vec![0xff, 0x00]),
        ],
    );

    restored.clear();
    assert_eq!(restored.rep(), &vec![0; HEADER]);
    assert!(operations(&restored).is_empty());
    restored.put(&Slice::from(&[][..]), &Slice::from(&[0xff][..]));
    assert_eq!(restored.rep(), &encoded(0, 1, &[0x01, 0x00, 0x01, 0xff]));
    assert_eq!(operations(&restored), vec![Operation::Put(vec![], vec![0xff])]);
}

#[test]
fn binary_set_contents_copies_a_view_of_its_current_owner() {
    let expected = encoded(0xff80, 1, &[0x01, 0x02, 0x00, 0xff, 0x02, 0x80, 0x00]);
    let mut batch = WriteBatch::new();
    write_batch_internal::set_contents(&mut batch, &Slice::from(&expected[..]));
    let borrowed_contents = write_batch_internal::contents(&batch);
    write_batch_internal::set_contents(&mut batch, &borrowed_contents);
    assert_eq!(batch.rep(), &expected);
    assert_eq!(
        operations(&batch),
        vec![Operation::Put(vec![0x00, 0xff], vec![0x80, 0x00])],
    );
}

#[test]
fn binary_internal_self_append_preserves_records_across_reallocation() {
    let mut batch = WriteBatch::new();
    let key = vec![0xff; 128];
    batch.put(&Slice::from(&key[..]), &Slice::from(&[0x80, 0x00][..]));
    write_batch_internal::set_sequence(&mut batch, 0xff80);
    batch.rep_mut().shrink_to_fit();
    let records = batch.rep()[HEADER..].to_vec();
    let batch_ptr = &mut batch as *mut WriteBatch;
    write_batch_internal::append(batch_ptr, batch_ptr as *const WriteBatch);

    let mut duplicated_records = records.clone();
    duplicated_records.extend_from_slice(&records);
    assert_eq!(batch.rep(), &encoded(0xff80, 2, &duplicated_records));
    assert_eq!(
        operations(&batch),
        vec![
            Operation::Put(key.clone(), vec![0x80, 0x00]),
            Operation::Put(key, vec![0x80, 0x00]),
        ],
    );
}
