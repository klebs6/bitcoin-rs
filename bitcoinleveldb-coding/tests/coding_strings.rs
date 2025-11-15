// ---------------- [ File: bitcoinleveldb-coding/tests/coding_strings.rs ]
use bitcoinleveldb_coding::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn coding_strings() {
    info!("coding_strings: start");

    let mut s = String::new();
    let dst = &mut s as *mut String;

    let empty_bytes: [u8; 0] = [];
    let foo_bytes = b"foo";
    let bar_bytes = b"bar";
    let long_bytes: Vec<u8> = vec![b'x'; 200];

    let empty_slice =
        Slice::from_ptr_len(empty_bytes.as_ptr(), empty_bytes.len());
    let foo_slice = Slice::from_ptr_len(foo_bytes.as_ptr(), foo_bytes.len());
    let bar_slice = Slice::from_ptr_len(bar_bytes.as_ptr(), bar_bytes.len());
    let long_slice =
        Slice::from_ptr_len(long_bytes.as_ptr(), long_bytes.len());

    put_length_prefixed_slice(dst, &empty_slice);
    put_length_prefixed_slice(dst, &foo_slice);
    put_length_prefixed_slice(dst, &bar_slice);
    put_length_prefixed_slice(dst, &long_slice);

    let bytes = s.as_bytes();
    let mut input_slice =
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len());

    let mut v = Slice::from_ptr_len(core::ptr::null(), 0);

    assert!(
        get_length_prefixed_slice(&mut input_slice, &mut v),
        "coding_strings: failed to get first slice"
    );
    assert_eq!(
        "",
        slice_to_utf8(&v),
        "coding_strings: first slice mismatch"
    );

    assert!(
        get_length_prefixed_slice(&mut input_slice, &mut v),
        "coding_strings: failed to get second slice"
    );
    assert_eq!(
        "foo",
        slice_to_utf8(&v),
        "coding_strings: second slice mismatch"
    );

    assert!(
        get_length_prefixed_slice(&mut input_slice, &mut v),
        "coding_strings: failed to get third slice"
    );
    assert_eq!(
        "bar",
        slice_to_utf8(&v),
        "coding_strings: third slice mismatch"
    );

    assert!(
        get_length_prefixed_slice(&mut input_slice, &mut v),
        "coding_strings: failed to get fourth slice"
    );
    assert_eq!(
        "x".repeat(200),
        slice_to_utf8(&v),
        "coding_strings: long slice mismatch"
    );

    assert_eq!(
        "",
        slice_to_utf8(&input_slice),
        "coding_strings: input slice should be empty at end"
    );

    info!("coding_strings: success");
}
