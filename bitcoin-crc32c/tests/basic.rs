//! Rust port of crc32c_unittest.cc – high‑level convenience API tests
// ---------------- [ File: bitcoin-crc32c/tests/basic.rs ]
use bitcoin_crc32c::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_unittest.cc]

static SAMPLE48: [u8; 48] = [
    0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
    0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/* --------------------------------------------------------------------- */
/* 1.  byte‑slice helper (was C array)                                    */
/* --------------------------------------------------------------------- */
#[traced_test]
fn crc32c_slice_vectors() {
    unsafe {
        /* zeros */
        let buf = [0u8; 32];
        assert_eq!(crc32c_value(buf.as_ptr(), 32), 0x8a91_36aa);

        /* 0xFF */
        let buf = [0xFFu8; 32];
        assert_eq!(crc32c_value(buf.as_ptr(), 32), 0x62a8_ab43);

        /* ascending */
        let mut buf = [0u8; 32];
        for (i, b) in buf.iter_mut().enumerate() {
            *b = i as u8;
        }
        assert_eq!(crc32c_value(buf.as_ptr(), 32), 0x46dd_794e);

        /* descending */
        for (i, b) in buf.iter_mut().enumerate() {
            *b = (31 - i) as u8;
        }
        assert_eq!(crc32c_value(buf.as_ptr(), 32), 0x113f_db5c);

        /* 48‑byte sample */
        assert_eq!(crc32c_value(SAMPLE48.as_ptr(), 48), 0xd996_3a56);
    }
}

/* --------------------------------------------------------------------- */
/* 2.  “char pointer” analogue – identical to slice test                  */
/* --------------------------------------------------------------------- */
#[traced_test]
fn crc32c_char_pointer_vectors() {
    // same data & expectations as above (char == u8)
    crc32c_slice_vectors();
}

#[traced_test]
fn crc32c_string_vectors() {
    // same four vectors the C++ code uses ------------------------------
    let mut buf = vec![0u8; 32];

    // 1. all‑zero bytes
    buf.fill(0);
    assert_eq!(
        unsafe { crc32c_value(buf.as_ptr(), buf.len()) },
        0x8a91_36aa,
        "zeros"
    );

    // 2. all‑0xFF bytes
    buf.fill(0xFF);
    assert_eq!(
        unsafe { crc32c_value(buf.as_ptr(), buf.len()) },
        0x62a8_ab43,
        "0xFF"
    );

    // 3. ascending 0‥31
    buf.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8);
    assert_eq!(
        unsafe { crc32c_value(buf.as_ptr(), buf.len()) },
        0x46dd_794e,
        "ascending"
    );

    // 4. descending 31‥0
    buf.iter_mut()
        .enumerate()
        .for_each(|(i, b)| *b = (31 - i) as u8);
    assert_eq!(
        unsafe { crc32c_value(buf.as_ptr(), buf.len()) },
        0x113f_db5c,
        "descending"
    );
}

/// Convert an arbitrary byte‑slice into a `&str` without UTF‑8 checks.
/// Lifetime‑correct because `'a` is carried through.
#[inline(always)]
unsafe fn as_str<'a>(b: &'a [u8]) -> &'a str {
    std::str::from_utf8_unchecked(b)
}

#[traced_test]
fn crc32c_str_view_vectors() {
    let mut buf = [0u8; 32];

    // zeros
    buf.fill(0);
    assert_eq!(crc32c_with_str(unsafe { as_str(&buf) }), 0x8a91_36aa);

    // 0xFF
    buf.fill(0xFF);
    assert_eq!(crc32c_with_str(unsafe { as_str(&buf) }), 0x62a8_ab43);

    // ascending
    buf.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8);
    assert_eq!(crc32c_with_str(unsafe { as_str(&buf) }), 0x46dd_794e);

    // descending
    buf.iter_mut()
        .enumerate()
        .for_each(|(i, b)| *b = (31 - i) as u8);
    assert_eq!(crc32c_with_str(unsafe { as_str(&buf) }), 0x113f_db5c);
}
