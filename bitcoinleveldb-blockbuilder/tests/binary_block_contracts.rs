use bitcoinleveldb_blockbuilder::BlockBuilder;
use bitcoinleveldb_options::Options;
use bitcoinleveldb_slice::Slice;

#[test]
fn binary_prefix_entries_and_restart_trailer_match_literal_wire_bytes() {
    let mut options = Box::new(Options::default());
    options.set_block_restart_interval(2);
    let mut builder = BlockBuilder::new(&*options);
    builder.add(&Slice::from(&[0x00, 0xff, 0x80][..]), &Slice::from(&[0xff, 0x00][..]));
    builder.add(&Slice::from(&[0x00, 0xff, 0x81][..]), &Slice::from(&[0xc3, 0x28, 0x80][..]));
    builder.add(&Slice::from(&[0x80, 0x00][..]), &Slice::from(&[0xfe][..]));
    assert_eq!(builder.restarts_slice(), &[0, 15]);
    assert_eq!(builder.current_size_estimate(), 33);
    let expected = [
        0x00, 0x03, 0x02, 0x00, 0xff, 0x80, 0xff, 0x00,
        0x02, 0x01, 0x03, 0x81, 0xc3, 0x28, 0x80,
        0x00, 0x02, 0x01, 0x80, 0x00, 0xfe,
        0x00, 0x00, 0x00, 0x00,
        0x0f, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00,
    ];
    let finished = builder.finish();
    assert_eq!(finished.as_bytes(), &expected);
    assert_eq!(builder.buffer_bytes(), &expected);
    assert_eq!(builder.last_key().as_slice(), &[0x80, 0x00]);
    assert!(builder.is_finished());
}

#[test]
fn binary_long_key_and_value_keep_literal_multibyte_length_prefixes() {
    let options = Box::new(Options::default());
    let mut builder = BlockBuilder::new(&*options);
    let key = vec![0xff; 128];
    let value = vec![0x80; 129];
    builder.add(&Slice::from(key.as_slice()), &Slice::from(value.as_slice()));
    let mut expected = vec![0x00, 0x80, 0x01, 0x81, 0x01];
    expected.extend_from_slice(&key);
    expected.extend_from_slice(&value);
    expected.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]);
    assert_eq!(builder.finish().as_bytes(), expected.as_slice());
    assert_eq!(expected.len(), 270);
}

#[test]
fn add_snapshots_key_and_value_views_of_previous_builder_allocations() {
    let options = Box::new(Options::default());
    let mut builder = BlockBuilder::new(&*options);
    builder.add(&Slice::from(&[0x00, 0xff][..]), &Slice::from(&[0x80, 0x00, 0xff][..]));
    builder.buffer_mut().shrink_to_fit();
    builder.last_key_mut().shrink_to_fit();
    let key = Slice::from(&builder.last_key()[1..]);
    let value = Slice::from(&builder.buffer_bytes()[5..8]);
    builder.add(&key, &value);
    let expected = [
        0x00, 0x02, 0x03, 0x00, 0xff, 0x80, 0x00, 0xff,
        0x00, 0x01, 0x03, 0xff, 0x80, 0x00, 0xff,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    ];
    assert_eq!(builder.finish().as_bytes(), &expected);
    assert_eq!(builder.last_key().as_slice(), &[0xff]);
}

#[test]
fn empty_block_empty_entry_and_reset_have_distinct_literal_formats() {
    let options = Box::new(Options::default());
    let mut builder = BlockBuilder::new(&*options);
    assert_eq!(builder.finish().as_bytes(), &[0, 0, 0, 0, 1, 0, 0, 0]);
    builder.reset();
    assert!(builder.empty());
    assert!(builder.last_key().is_empty());
    assert_eq!(builder.restarts_slice(), &[0]);
    assert_eq!(*builder.counter(), 0);
    assert!(!builder.is_finished());
    builder.add(&Slice::default(), &Slice::default());
    assert_eq!(builder.finish().as_bytes(), &[0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0]);
}
