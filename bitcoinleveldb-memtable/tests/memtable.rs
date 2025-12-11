// ---------------- [ File: bitcoinleveldb-memtable/tests/memtable.rs ]
use bitcoinleveldb_memtable::*;
use bitcoinleveldb_coding::*;
use bitcoinleveldb_comparator::*;
use bitcoinleveldb_iterator::*;
use bitcoinleveldb_iteratorinner::*;
use bitcoinleveldb_key::*;
use bitcoinleveldb_options::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoin_imports::*;

fn build_default_internal_key_comparator() -> InternalKeyComparator {
    unsafe {
        let user_cmp_ptr = bytewise_comparator();
        InternalKeyComparator::new(user_cmp_ptr)
    }
}

fn new_memtable_for_tests() -> MemTable {
    let icmp = build_default_internal_key_comparator();
    MemTable::new(&icmp)
}

fn build_lookup_key_for_str(user_key: &str, seq: SequenceNumber) -> LookupKey {
    let key_slice = Slice::from(user_key.as_bytes());
    LookupKey::new(&key_slice, seq)
}

fn insert_user_key_value(
    memtable:   &mut MemTable,
    seq:        SequenceNumber,
    key_str:    &str,
    value_str:  &str,
    value_type: ValueType,
) {
    let key_slice   = Slice::from(key_str.as_bytes());
    let value_slice = Slice::from(value_str.as_bytes());
    memtable.add(seq, value_type, &key_slice, &value_slice);
}

fn assert_memtable_get_ok(
    memtable:       &mut MemTable,
    lookup:         &LookupKey,
    expected_value: &str,
) {
    let mut out    = String::new();
    let mut status = Status::ok();

    let found = memtable.get(
        lookup,
        &mut out as *mut String,
        &mut status as *mut Status,
    );

    assert!(found, "expected key lookup to succeed");
    assert!(
        status.is_ok(),
        "expected OK status, got {:?}",
        status.code()
    );
    assert_eq!(out, expected_value);
}

fn make_internal_key_for_user_key(user_key: &str, seq: SequenceNumber) -> InternalKey {
    let key_slice = Slice::from(user_key.as_bytes());
    InternalKey::new(&key_slice, seq, ValueType::TypeValue)
}


#[traced_test]
fn mem_table_simple_insert_and_read() {
    info!("mem_table_simple_insert_and_read: starting");

    unsafe {
        // Internal key comparator using the bytewise user comparator.
        let user_cmp_ptr = bytewise_comparator();
        let internal_cmp =
            InternalKeyComparator::new(user_cmp_ptr);

        // Stack‑allocated MemTable (we do not use Ref/Unref here).
        let mut memtable = MemTable::new(&internal_cmp);

        let entries = vec![
            ("k1", "v1"),
            ("k2", "v2"),
            ("k3", "v3"),
            ("largekey", "vlarge"),
        ];

        let base_seq: SequenceNumber = 100;
        let mut seq = base_seq + 1;

        // Populate memtable directly.
        for (k, v) in &entries {
            let k_slice = Slice::from(k.as_bytes());
            let v_slice = Slice::from(v.as_bytes());

            memtable.add(
                seq,
                ValueType::TypeValue,
                &k_slice,
                &v_slice,
            );

            seq += 1;
        }

        // Verify lookups via the public Get interface.
        seq = base_seq + 1;
        for (k, v_expected) in &entries {
            let k_slice = Slice::from(k.as_bytes());
            let lookup = LookupKey::new(&k_slice, seq);

            let mut out = String::new();
            let mut status = Status::ok();

            let found = memtable.get(
                &lookup,
                &mut out as *mut String,
                &mut status as *mut Status,
            );

            assert!(
                found,
                "expected key '{}' to be found",
                k
            );
            assert!(
                status.is_ok(),
                "expected OK status for key '{}', got {:?}",
                k,
                status.code()
            );
            assert_eq!(
                out, *v_expected,
                "value mismatch for key '{}'",
                k
            );

            seq += 1;
        }

        // Verify iteration order matches key order.
        let iter_raw = memtable.new_iterator();
        let iter_box: Box<LevelDBIterator> =
            Box::from_raw(iter_raw);
        let mut iter = *iter_box;

        iter.seek_to_first();

        let mut seen: Vec<(String, String)> = Vec::new();
        while iter.valid() {
            let internal_key = iter.key();
            let user_key =
                extract_user_key(&internal_key);

            let user = user_key.to_string();
            let val = iter.value().to_string();
            seen.push((user, val));
            iter.next();
        }

        let expected_sorted: Vec<(String, String)> =
            {
                let mut tmp: Vec<(String, String)> =
                    entries
                        .iter()
                        .map(|(k, v)| {
                            ((*k).to_string(),
                             (*v).to_string())
                        })
                        .collect();
                tmp.sort_by(|a, b| a.0.cmp(&b.0));
                tmp
            };

        assert_eq!(
            seen, expected_sorted,
            "iterator produced unexpected key/value sequence"
        );

        info!(
            "mem_table_simple_insert_and_read: completed successfully"
        );
    }
}

#[traced_test]
fn memtable_encoding_roundtrip_encode_key_and_get_length_prefixed_slice_for_various_lengths() {
    info!("starting memtable_encoding_roundtrip_encode_key_and_get_length_prefixed_slice_for_various_lengths");

    let lengths: [usize; 5] = [0, 1, 10, 128, 1024];

    for len in lengths.iter().copied() {
        let mut payload = Vec::with_capacity(len);
        for i in 0..len {
            payload.push((i % 251) as u8);
        }

        let slice = Slice::from(payload.as_slice());

        let mut scratch = String::new();
        let scratch_ptr: *mut String = &mut scratch;

        let encoded_ptr = encode_key(scratch_ptr, &slice);

        let decoded: Slice = unsafe {
            let encoded_len = scratch.len();
            let mut input = Slice::from_ptr_len(encoded_ptr, encoded_len);
            let mut result = Slice::default();

            let ok = get_length_prefixed_slice(
                &mut input as *mut Slice,
                &mut result as *mut Slice,
            );

            assert!(
                ok,
                "get_length_prefixed_slice should succeed for len={}",
                len
            );

            result
        };

        assert_eq!(
            *decoded.size(),
            len,
            "decoded length mismatch for len={}",
            len
        );

        let decoded_bytes = slice_as_bytes(&decoded);
        assert_eq!(
            decoded_bytes,
            payload.as_slice(),
            "decoded payload mismatch for len={}",
            len
        );
    }
}

#[traced_test]
fn memtable_encoding_varint32_roundtrip_matches_decode_and_length() {
    info!("starting memtable_encoding_varint32_roundtrip_matches_decode_and_length");

    let test_values: [u32; 7] = [
        0,
        1,
        127,
        128,
        255,
        16_384,
        u32::MAX,
    ];

    for &v in test_values.iter() {
        let mut buf = [0u8; 10];
        let dst = buf.as_mut_ptr();

        let end_ptr = unsafe { encode_varint32(dst, v) };
        let encoded_len =
            unsafe { end_ptr.offset_from(dst) as usize };

        assert!(
            encoded_len > 0,
            "encoded length must be positive for v={}",
            v
        );

        let expected_len_i32 = varint_length(v as u64);
        assert!(
            expected_len_i32 >= 0,
            "varint_length returned negative length for value {}",
            v
        );
        let expected_len: usize = expected_len_i32 as usize;

        assert_eq!(
            encoded_len,
            expected_len,
            "varint_length mismatch for value {}",
            v
        );

        let encoded_slice = &buf[..encoded_len];
        let (decoded, consumed_len) =
            decode_varint32(encoded_slice);

        assert_eq!(
            decoded, v,
            "roundtrip decode mismatch for value {}",
            v
        );
        assert_eq!(
            consumed_len, encoded_len,
            "decoder reported a different length for value {}",
            v
        );

        assert_eq!(
            encoded_slice[encoded_len - 1] & 0x80,
            0,
            "final byte must not have continuation bit set for value {}",
            v
        );
    }
}

#[traced_test]
fn memtable_encoding_get_length_prefixed_slice_decodes_manual_buffers() {
    info!("starting memtable_encoding_get_length_prefixed_slice_decodes_manual_buffers");

    let payloads: Vec<Vec<u8>> = vec![
        Vec::new(),
        b"x".to_vec(),
        b"short-bytes".to_vec(),
        (0u8..=50u8).collect(),
    ];

    for payload in payloads.iter() {
        let mut buf = Vec::new();
        let len_u32: u32 = payload.len() as u32;

        put_varint32_vec(&mut buf, len_u32);
        buf.extend_from_slice(payload.as_slice());

        let ptr = buf.as_ptr();

        let decoded: Slice = unsafe {
            let mut input =
                Slice::from_ptr_len(ptr, buf.len());
            let mut result = Slice::default();

            let ok = get_length_prefixed_slice(
                &mut input as *mut Slice,
                &mut result as *mut Slice,
            );

            assert!(
                ok,
                "get_length_prefixed_slice should succeed for payload {:?}",
                payload
            );

            result
        };

        assert_eq!(
            *decoded.size(),
            payload.len(),
            "decoded size mismatch for payload {:?}",
            payload
        );
        let decoded_bytes = slice_as_bytes(&decoded);
        assert_eq!(
            decoded_bytes,
            payload.as_slice(),
            "decoded payload mismatch for payload {:?}",
            payload
        );
    }
}

#[traced_test]
fn memtable_core_memory_usage_starts_at_zero_and_increases_after_inserts() {
    info!("starting memtable_core_memory_usage_starts_at_zero_and_increases_after_inserts");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    let initial_usage = memtable.approximate_memory_usage();
    assert_eq!(
        initial_usage, 0,
        "fresh MemTable should report zero Arena usage"
    );

    insert_user_key_value(
        &mut memtable,
        1,
        "k-memory",
        "value-memory",
        ValueType::TypeValue,
    );

    let after_usage = memtable.approximate_memory_usage();
    assert!(
        after_usage > initial_usage,
        "Arena usage should grow after inserts (before={}, after={})",
        initial_usage,
        after_usage
    );
}

#[traced_test]
fn memtable_core_reference_counting_balances_ref_and_unref_without_panic() {
    info!("starting memtable_core_reference_counting_balances_ref_and_unref_without_panic");

    let icmp = build_default_internal_key_comparator();
    let memtable = MemTable::new(&icmp);

    let raw: *mut MemTable = Box::into_raw(Box::new(memtable));

    unsafe {
        (*raw).ref_();
        (*raw).ref_();

        (*raw).unref();
        (*raw).unref();

        // After the second Unref the MemTable has deleted itself; do not touch `raw` again.
    }
}

#[traced_test]
fn memtable_api_single_insert_and_get_returns_expected_value() {
    info!("starting memtable_api_single_insert_and_get_returns_expected_value");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    insert_user_key_value(
        &mut memtable,
        100,
        "key-single",
        "value-single",
        ValueType::TypeValue,
    );

    let lookup = build_lookup_key_for_str("key-single", 100);
    assert_memtable_get_ok(&mut memtable, &lookup, "value-single");
}

#[traced_test]
fn memtable_api_multiple_inserts_iteration_yields_sorted_user_keys() {
    info!("starting memtable_api_multiple_inserts_iteration_yields_sorted_user_keys");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    let entries = vec![
        ("key3", "value3"),
        ("key1", "value1"),
        ("key2", "value2"),
        ("key0", "value0"),
    ];

    let mut seq: SequenceNumber = 1;
    for (k, v) in entries.iter() {
        insert_user_key_value(
            &mut memtable,
            seq,
            k,
            v,
            ValueType::TypeValue,
        );
        seq += 1;
    }

    let iter_raw = memtable.new_iterator();
    let mut iter_box: Box<LevelDBIterator> =
        unsafe { Box::from_raw(iter_raw) };

    iter_box.seek_to_first();

    let mut seen: Vec<(String, String)> = Vec::new();
    while iter_box.valid() {
        let internal_key = iter_box.key();
        let user_key     = extract_user_key(&internal_key);

        let user  = user_key.to_string();
        let value = iter_box.value().to_string();
        seen.push((user, value));

        iter_box.next();
    }

    let mut expected: Vec<(String, String)> = entries
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
    expected.sort_by(|a, b| a.0.cmp(&b.0));

    assert_eq!(
        seen, expected,
        "memtable iterator produced unexpected key/value sequence"
    );
}

#[traced_test]
fn memtable_get_returns_false_for_missing_key_and_preserves_status_and_value() {
    info!("starting memtable_get_returns_false_for_missing_key_and_preserves_status_and_value");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    insert_user_key_value(
        &mut memtable,
        1,
        "present-key",
        "present-value",
        ValueType::TypeValue,
    );

    let lookup = build_lookup_key_for_str("missing-key", 1000);

    let mut value_out = String::from("unchanged");
    let mut status    = Status::ok();

    let found = memtable.get(
        &lookup,
        &mut value_out as *mut String,
        &mut status as *mut Status,
    );

    assert!(
        !found,
        "get should return false when key is not present"
    );
    assert!(
        status.is_ok(),
        "status should remain OK when key is not present and no tombstone exists; got {:?}",
        status.code()
    );
    assert_eq!(
        value_out, "unchanged",
        "value output should remain untouched when key is not found"
    );
}

#[traced_test]
fn memtable_get_prefers_highest_sequence_number_for_same_user_key() {
    info!("starting memtable_get_prefers_highest_sequence_number_for_same_user_key");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    insert_user_key_value(
        &mut memtable,
        100,
        "shared-key",
        "value-old",
        ValueType::TypeValue,
    );
    insert_user_key_value(
        &mut memtable,
        200,
        "shared-key",
        "value-new",
        ValueType::TypeValue,
    );

    let lookup = build_lookup_key_for_str("shared-key", 300);

    let mut out    = String::new();
    let mut status = Status::ok();

    let found = memtable.get(
        &lookup,
        &mut out as *mut String,
        &mut status as *mut Status,
    );

    assert!(found, "expected shared-key to be found");
    assert!(
        status.is_ok(),
        "expected status OK for shared-key lookup, got {:?}",
        status.code()
    );
    assert_eq!(
        out, "value-new",
        "expected most recent value for shared-key"
    );
}

#[traced_test]
fn memtable_get_interprets_deletion_marker_as_not_found() {
    info!("starting memtable_get_interprets_deletion_marker_as_not_found");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    insert_user_key_value(
        &mut memtable,
        50,
        "tombstoned-key",
        "visible-before-delete",
        ValueType::TypeValue,
    );

    insert_user_key_value(
        &mut memtable,
        60,
        "tombstoned-key",
        "",
        ValueType::TypeDeletion,
    );

    let lookup_after_delete =
        build_lookup_key_for_str("tombstoned-key", 100);

    let mut out    = String::from("should-be-cleared");
    let mut status = Status::ok();

    let found = memtable.get(
        &lookup_after_delete,
        &mut out as *mut String,
        &mut status as *mut Status,
    );

    assert!(
        found,
        "get should return true when deletion marker is found for key"
    );
    assert!(
        status.is_not_found(),
        "status should be NotFound for tombstoned key; got {:?}",
        status.code()
    );
    assert!(
        out.is_empty(),
        "value output should be cleared for tombstoned key; got '{}'",
        out
    );
}

#[traced_test]
fn memtable_iterator_seek_to_first_and_seek_to_last_cover_all_entries() {
    info!("starting memtable_iterator_seek_to_first_and_seek_to_last_cover_all_entries");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    let entries = vec![
        ("a", "va"),
        ("b", "vb"),
        ("c", "vc"),
    ];

    let mut seq: SequenceNumber = 1;
    for (k, v) in entries.iter() {
        insert_user_key_value(
            &mut memtable,
            seq,
            k,
            v,
            ValueType::TypeValue,
        );
        seq += 1;
    }

    let iter_raw = memtable.new_iterator();
    let mut iter_box: Box<LevelDBIterator> =
        unsafe { Box::from_raw(iter_raw) };

    iter_box.seek_to_first();
    assert!(
        iter_box.valid(),
        "iterator must be valid after seek_to_first on non-empty memtable"
    );
    let first_internal = iter_box.key();
    let first_user     = extract_user_key(&first_internal).to_string();

    let mut forward_keys = Vec::new();
    while iter_box.valid() {
        let ikey = iter_box.key();
        let ukey = extract_user_key(&ikey).to_string();
        forward_keys.push(ukey);
        iter_box.next();
    }

    iter_box.seek_to_last();
    assert!(
        iter_box.valid(),
        "iterator must be valid after seek_to_last on non-empty memtable"
    );
    let last_internal = iter_box.key();
    let last_user     = extract_user_key(&last_internal).to_string();

    let mut backward_keys = Vec::new();
    while iter_box.valid() {
        let ikey = iter_box.key();
        let ukey = extract_user_key(&ikey).to_string();
        backward_keys.push(ukey);
        iter_box.prev();
    }

    let mut expected_sorted: Vec<String> =
        entries.iter().map(|(k, _)| (*k).to_string()).collect();
    expected_sorted.sort();

    assert_eq!(
        first_user,
        expected_sorted
            .first()
            .cloned()
            .expect("expected at least one key"),
        "first key after seek_to_first did not match sorted order"
    );
    assert_eq!(
        last_user,
        expected_sorted
            .last()
            .cloned()
            .expect("expected at least one key"),
        "last key after seek_to_last did not match sorted order"
    );
    assert_eq!(
        forward_keys, expected_sorted,
        "forward traversal did not match sorted user key order"
    );

    backward_keys.reverse();
    assert_eq!(
        backward_keys, expected_sorted,
        "backward traversal did not match sorted user key order"
    );
}

#[traced_test]
fn memtable_iterator_seek_and_prev_navigate_correctly_between_internal_keys() {
    info!("starting memtable_iterator_seek_and_prev_navigate_correctly_between_internal_keys");

    let icmp = build_default_internal_key_comparator();
    let mut memtable = MemTable::new(&icmp);

    insert_user_key_value(
        &mut memtable,
        10,
        "a-key",
        "va",
        ValueType::TypeValue,
    );
    insert_user_key_value(
        &mut memtable,
        20,
        "b-key",
        "vb",
        ValueType::TypeValue,
    );
    insert_user_key_value(
        &mut memtable,
        30,
        "c-key",
        "vc",
        ValueType::TypeValue,
    );

    let iter_raw = memtable.new_iterator();
    let mut iter_box: Box<LevelDBIterator> =
        unsafe { Box::from_raw(iter_raw) };

    let internal_b        = make_internal_key_for_user_key("b-key", 20);
    let internal_b_slice  = internal_b.encode();

    iter_box.seek(&internal_b_slice);

    assert!(
        iter_box.valid(),
        "iterator should be valid after seeking to an existing internal key"
    );
    let current_user = extract_user_key(&iter_box.key()).to_string();
    assert_eq!(current_user, "b-key");

    iter_box.prev();
    assert!(
        iter_box.valid(),
        "iterator should remain valid after prev from middle key"
    );
    let prev_user = extract_user_key(&iter_box.key()).to_string();
    assert_eq!(prev_user, "a-key");

    iter_box.prev();
    assert!(
        !iter_box.valid(),
        "iterator should be invalid after moving before the first entry"
    );
}

#[traced_test]
fn memtable_key_comparator_orders_by_user_key_then_sequence_number_descending() {
    info!("starting memtable_key_comparator_orders_by_user_key_then_sequence_number_descending");

    let icmp = build_default_internal_key_comparator();
    let cmp  = MemTableKeyComparator::new(&icmp);

    let low_seq_key  = make_internal_key_for_user_key("shared-key", 10);
    let high_seq_key = make_internal_key_for_user_key("shared-key", 20);

    let low_seq_slice  = low_seq_key.encode();
    let high_seq_slice = high_seq_key.encode();

    let mut scratch_low  = String::new();
    let mut scratch_high = String::new();

    let low_ptr =
        encode_key(&mut scratch_low as *mut String, &low_seq_slice);
    let high_ptr =
        encode_key(&mut scratch_high as *mut String, &high_seq_slice);

    let res_high_vs_low = cmp.invoke(high_ptr, low_ptr);
    assert!(
        res_high_vs_low < 0,
        "higher sequence internal key should sort before lower sequence (cmp={})",
        res_high_vs_low
    );

    let res_low_vs_high = cmp.invoke(low_ptr, high_ptr);
    assert!(
        res_low_vs_high > 0,
        "lower sequence internal key should sort after higher sequence (cmp={})",
        res_low_vs_high
    );

    let res_equal = cmp.invoke(low_ptr, low_ptr);
    assert_eq!(
        res_equal, 0,
        "comparator must report 0 when comparing a key with itself"
    );

    let user_a = Slice::from(b"a-key".as_ref());
    let user_b = Slice::from(b"b-key".as_ref());

    let internal_a =
        InternalKey::new(&user_a, 100, ValueType::TypeValue);
    let internal_b =
        InternalKey::new(&user_b, 100, ValueType::TypeValue);

    let internal_a_slice = internal_a.encode();
    let internal_b_slice = internal_b.encode();

    let mut scratch_a = String::new();
    let mut scratch_b = String::new();

    let ptr_a =
        encode_key(&mut scratch_a as *mut String, &internal_a_slice);
    let ptr_b =
        encode_key(&mut scratch_b as *mut String, &internal_b_slice);

    let cmp_ab = cmp.invoke(ptr_a, ptr_b);
    assert!(
        cmp_ab < 0,
        "user key 'a-key' should sort before 'b-key' (cmp={})",
        cmp_ab
    );

    let cmp_ba = cmp.invoke(ptr_b, ptr_a);
    assert!(
        cmp_ba > 0,
        "user key 'b-key' should sort after 'a-key' (cmp={})",
        cmp_ba
    );
}

#[traced_test]
fn memtable_constructor_builds_memtable_and_iterator_exposes_sorted_user_view() {
    info!("starting memtable_constructor_builds_memtable_and_iterator_exposes_sorted_user_view");

    let comparator: Box<dyn SliceComparator> =
        Box::new(BytewiseComparatorImpl::default());
    let mut constructor = MemTableConstructor::new(comparator);

    let mut kv = KVMap::default();
    kv.insert("delta".to_string(),  "4".to_string());
    kv.insert("alpha".to_string(),  "1".to_string());
    kv.insert("charlie".to_string(),"3".to_string());
    kv.insert("bravo".to_string(),  "2".to_string());

    let options = Options::default();

    let status = constructor.finish_impl(&options, &kv);
    assert!(
        status.is_ok(),
        "finish_impl should succeed for well-formed KVMap; got {:?}",
        status.code()
    );

    let raw_iter = constructor.new_iterator();
    let mut iter_box: Box<LevelDBIterator> =
        unsafe { Box::from_raw(raw_iter) };

    iter_box.seek_to_first();

    let mut actual: Vec<(String, String)> = Vec::new();
    while iter_box.valid() {
        let key_slice   = iter_box.key();
        let value_slice = iter_box.value();
        actual.push((key_slice.to_string(), value_slice.to_string()));
        iter_box.next();
    }

    let mut expected: Vec<(String, String)> = kv
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    expected.sort_by(|a, b| a.0.cmp(&b.0));

    assert_eq!(actual, expected);
}

#[traced_test]
fn memtable_constructor_replaces_existing_memtable_on_finish_impl() {
    info!("starting memtable_constructor_replaces_existing_memtable_on_finish_impl");

    let comparator: Box<dyn SliceComparator> =
        Box::new(BytewiseComparatorImpl::default());
    let mut constructor = MemTableConstructor::new(comparator);

    let mut first_map = KVMap::default();
    first_map.insert("first".to_string(), "one".to_string());

    let options = Options::default();

    let st1 = constructor.finish_impl(&options, &first_map);
    assert!(
        st1.is_ok(),
        "first finish_impl should succeed; got {:?}",
        st1.code()
    );

    let raw_iter1 = constructor.new_iterator();
    let mut iter1: Box<LevelDBIterator> =
        unsafe { Box::from_raw(raw_iter1) };

    iter1.seek_to_first();
    let mut seen_first = Vec::new();
    while iter1.valid() {
        seen_first.push(iter1.key().to_string());
        iter1.next();
    }

    assert_eq!(
        seen_first,
        vec!["first".to_string()],
        "first memtable should expose only the initial key"
    );

    let mut second_map = KVMap::default();
    second_map.insert("second".to_string(), "two".to_string());
    second_map.insert("another".to_string(), "three".to_string());

    let st2 = constructor.finish_impl(&options, &second_map);
    assert!(
        st2.is_ok(),
        "second finish_impl should succeed; got {:?}",
        st2.code()
    );

    let raw_iter2 = constructor.new_iterator();
    let mut iter2: Box<LevelDBIterator> =
        unsafe { Box::from_raw(raw_iter2) };

    iter2.seek_to_first();

    let mut seen_second = Vec::new();
    while iter2.valid() {
        seen_second.push(iter2.key().to_string());
        iter2.next();
    }

    let mut expected_second: Vec<String> =
        second_map.keys().cloned().collect();
    expected_second.sort();

    assert_eq!(
        seen_second, expected_second,
        "second memtable view did not match expected sorted keys"
    );
}

fn between(
    val:  u64,
    low:  u64,
    high: u64,
) -> bool {
    let result = (val >= low) && (val <= high);
    if !result {
        eprintln!(
            "Value {} is not in range [{}, {}]",
            val, low, high
        );
    }
    result
}
