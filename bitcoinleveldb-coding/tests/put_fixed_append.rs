// ---------------- [ File: bitcoinleveldb-coding/tests/put_fixed_append.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn put_fixed32_and_64_append_after_prefix() {
    info!("put_fixed32_and_64_append_after_prefix: start");

    let mut s = String::from("prefix");
    let initial_len = s.len();
    let dst = &mut s as *mut String;

    let v32: u32 = 0x0a0b_0c0d;
    let v64: u64 = 0x0102_0304_0506_0708u64;

    put_fixed32(dst, v32);
    put_fixed64(dst, v64);

    let bytes = s.as_bytes();
    let total_len = initial_len
        + core::mem::size_of::<u32>()
        + core::mem::size_of::<u64>();

    assert_eq!(
        total_len,
        bytes.len(),
        "put_fixed32_and_64_append_after_prefix: unexpected final length"
    );

    unsafe {
        let ptr32 = bytes.as_ptr().add(initial_len);
        let decoded32 = decode_fixed32(ptr32);
        assert_eq!(v32, decoded32);

        let ptr64 = ptr32.add(core::mem::size_of::<u32>());
        let decoded64 = decode_fixed64(ptr64);
        assert_eq!(v64, decoded64);
    }

    info!("put_fixed32_and_64_append_after_prefix: success");
}
