// ---------------- [ File: bitcoinleveldb-log/tests/logging.rs ]
use bitcoinleveldb_log::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_crc32::*;
use bitcoinleveldb_coding::*;
use bitcoin_support::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/logging_test.cc]

struct Logging {}

fn consume_decimal_number_roundtrip_test(number: u64, padding: Option<&str>) {
    let padding_str = padding.unwrap_or("");
    trace!(
        "consume_decimal_number_roundtrip_test: number={} padding='{}'",
        number,
        padding_str
    );

    let decimal_number = number_to_string(number);
    let mut input_string = decimal_number.clone();
    input_string.push_str(padding_str);

    let mut slice = Slice::from(&input_string);
    let original_len = input_string.len();

    let mut result: u64 = 0;
    let ok = consume_decimal_number(
        &mut slice as *mut Slice,
        &mut result as *mut u64,
    );

    assert!(ok, "consume_decimal_number should succeed");
    assert_eq!(number, result);

    unsafe {
        let remaining_len = *slice.size();
        let consumed = original_len - remaining_len;
        assert_eq!(
            decimal_number.len(),
            consumed,
            "unexpected number of digits consumed"
        );

        let data_ptr_ptr = slice.data();
        let remaining = if data_ptr_ptr.is_null() {
            String::new()
        } else {
            let data_ptr = *data_ptr_ptr;
            if data_ptr.is_null() {
                String::new()
            } else {
                let bytes =
                    core::slice::from_raw_parts(data_ptr, remaining_len);
                String::from_utf8_lossy(bytes).to_string()
            }
        };

        assert_eq!(
            padding_str,
            remaining,
            "remaining input after parsing does not match padding"
        );
    }
}

fn consume_decimal_number_overflow_test(input_string: &str) {
    trace!(
        "consume_decimal_number_overflow_test: input='{}'",
        input_string
    );

    let mut slice = Slice::from(&input_string.to_owned());
    let mut result: u64 = 0;
    let ok = consume_decimal_number(
        &mut slice as *mut Slice,
        &mut result as *mut u64,
    );
    assert!(
        !ok,
        "consume_decimal_number should fail on overflow input '{}'",
        input_string
    );
}

fn consume_decimal_number_no_digits_test(input_string: &str) {
    trace!(
        "consume_decimal_number_no_digits_test: input='{:?}'",
        input_string
    );

    let mut slice    = Slice::from(&input_string.to_owned());
    let slice_before = Slice::from(&input_string.to_owned());
    let mut result: u64 = 0;

    let ok = consume_decimal_number(
        &mut slice as *mut Slice,
        &mut result as *mut u64,
    );
    assert!(
        !ok,
        "consume_decimal_number should fail when no leading digits"
    );

    unsafe {
        let before_size = *slice_before.size();
        let after_size = *slice.size();
        let before_ptr_ptr = slice_before.data();
        let after_ptr_ptr = slice.data();

        assert_eq!(before_size, after_size, "size changed unexpectedly");
        if !before_ptr_ptr.is_null() && !after_ptr_ptr.is_null() {
            let before_ptr = *before_ptr_ptr;
            let after_ptr = *after_ptr_ptr;
            assert_eq!(
                before_ptr, after_ptr,
                "data pointer changed unexpectedly"
            );
        }
    }
}

#[traced_test]
fn logging_number_to_string() {
    assert_eq!("0", number_to_string(0));
    assert_eq!("1", number_to_string(1));
    assert_eq!("9", number_to_string(9));

    assert_eq!("10", number_to_string(10));
    assert_eq!("11", number_to_string(11));
    assert_eq!("19", number_to_string(19));
    assert_eq!("99", number_to_string(99));

    assert_eq!("100", number_to_string(100));
    assert_eq!("109", number_to_string(109));
    assert_eq!("190", number_to_string(190));
    assert_eq!("123", number_to_string(123));
    assert_eq!("12345678", number_to_string(12345678));

    const_assert!(
        u64::MAX == 18_446_744_073_709_551_615u64 //, "Test consistency check"
    );
    assert_eq!(
        "18446744073709551000",
        number_to_string(18_446_744_073_709_551_000u64)
    );
    assert_eq!(
        "18446744073709551600",
        number_to_string(18_446_744_073_709_551_600u64)
    );
    assert_eq!(
        "18446744073709551610",
        number_to_string(18_446_744_073_709_551_610u64)
    );
    assert_eq!(
        "18446744073709551614",
        number_to_string(18_446_744_073_709_551_614u64)
    );
    assert_eq!(
        "18446744073709551615",
        number_to_string(18_446_744_073_709_551_615u64)
    );
}

#[traced_test]
fn logging_consume_decimal_number_roundtrip() {
    consume_decimal_number_roundtrip_test(0, None);
    consume_decimal_number_roundtrip_test(1, None);
    consume_decimal_number_roundtrip_test(9, None);

    consume_decimal_number_roundtrip_test(10, None);
    consume_decimal_number_roundtrip_test(11, None);
    consume_decimal_number_roundtrip_test(19, None);
    consume_decimal_number_roundtrip_test(99, None);

    consume_decimal_number_roundtrip_test(100, None);
    consume_decimal_number_roundtrip_test(109, None);
    consume_decimal_number_roundtrip_test(190, None);
    consume_decimal_number_roundtrip_test(123, None);

    assert_eq!("12345678", number_to_string(12_345_678));

    for i in 0u64..100 {
        let large_number = u64::MAX - i;
        consume_decimal_number_roundtrip_test(large_number, None);
    }
}

#[traced_test]
fn logging_consume_decimal_number_roundtrip_with_padding() {
    consume_decimal_number_roundtrip_test(0, Some(" "));
    consume_decimal_number_roundtrip_test(1, Some("abc"));
    consume_decimal_number_roundtrip_test(9, Some("x"));

    consume_decimal_number_roundtrip_test(10, Some("_"));
    consume_decimal_number_roundtrip_test(11, Some("\0\0\0"));
    consume_decimal_number_roundtrip_test(19, Some("abc"));
    consume_decimal_number_roundtrip_test(99, Some("padding"));

    consume_decimal_number_roundtrip_test(100, Some(" "));

    for i in 0u64..100 {
        let large_number = u64::MAX - i;
        consume_decimal_number_roundtrip_test(large_number, Some("pad"));
    }
}

#[traced_test]
fn logging_consume_decimal_number_overflow() {
    const_assert!(
        u64::MAX == 18_446_744_073_709_551_615u64 // , "Test consistency check"
    );

    consume_decimal_number_overflow_test("18446744073709551616");
    consume_decimal_number_overflow_test("18446744073709551617");
    consume_decimal_number_overflow_test("18446744073709551618");
    consume_decimal_number_overflow_test("18446744073709551619");
    consume_decimal_number_overflow_test("18446744073709551620");
    consume_decimal_number_overflow_test("18446744073709551621");
    consume_decimal_number_overflow_test("18446744073709551622");
    consume_decimal_number_overflow_test("18446744073709551623");
    consume_decimal_number_overflow_test("18446744073709551624");
    consume_decimal_number_overflow_test("18446744073709551625");
    consume_decimal_number_overflow_test("18446744073709551626");

    consume_decimal_number_overflow_test("18446744073709551700");
    consume_decimal_number_overflow_test("99999999999999999999");
}

#[traced_test]
fn logging_consume_decimal_number_no_digits() {
    consume_decimal_number_no_digits_test("");
    consume_decimal_number_no_digits_test(" ");
    consume_decimal_number_no_digits_test("a");
    consume_decimal_number_no_digits_test(" 123");
    consume_decimal_number_no_digits_test("a123");
    consume_decimal_number_no_digits_test("\0\0\0123");
    consume_decimal_number_no_digits_test("\u{7f}123");
    consume_decimal_number_no_digits_test("\u{ff}123");
}
