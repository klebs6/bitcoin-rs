// ---------------- [ File: bitcoin-crc32c/tests/extend.rs ]
// Common test cases for all implementations of CRC32C_Extend().

use bitcoin_crc32c::*;
use bitcoin_imports::*;

type ExtendFn = unsafe fn(u32, *const u8, usize) -> u32;

/// Collect every back‑end that is both **compiled‑in** *and* **enabled
/// at run‑time**, mirroring the C++ `TypeParam` instantiations.
fn available_impls() -> Vec<(&'static str, ExtendFn)> {
    let mut v: Vec<(&'static str, ExtendFn)> = vec![
        ("dispatcher", crc32c_extend),           // “Public” in the C++ test
        ("portable",   crc32c_extend_portable),  // Portable fallback
    ];

    #[cfg(target_arch = "aarch64")]
    if can_use_arm64_crc32() {
        v.push(("arm64‑hw", crc32c_extend_arm64));
    }

    #[cfg(target_arch = "x86_64")]
    if can_use_sse42() {
        v.push(("sse42‑hw", crc32c_extend_sse42));
    }

    v
}

unsafe fn extend(f: ExtendFn, crc: u32, buf: &[u8]) -> u32 {
    f(crc, buf.as_ptr(), buf.len())
}

fn assert_all<F>(msg: &str, f: F)
where
    F: Fn((&'static str, ExtendFn)) + Copy,
{
    for impl_ in available_impls() {
        f(impl_);
    }
    println!("✓ {msg} – all back‑ends");
}


#[traced_test]
fn extend_standard_results() {
    const VEC_ZERO:      u32 = 0x8a91_36aa;
    const VEC_FF:        u32 = 0x62a8_ab43;
    const VEC_INC:       u32 = 0x46dd_794e;
    const VEC_DEC:       u32 = 0x113f_db5c;
    const VEC_SAMPLE48:  u32 = 0xd996_3a56;

    // From rfc3720 section B.4.
    static SAMPLE48: [u8; 48] = [
        0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    assert_all("RFC‑3720 vectors", |(name, fun)| {
        unsafe {
            /* 32 × 0x00 */
            let buf = [0u8; 32];
            assert_eq!(
                extend(fun, 0, &buf), VEC_ZERO,
                "{name}: all‑zero vector"
            );

            /* 32 × 0xFF */
            let buf = [0xFFu8; 32];
            assert_eq!(extend(fun, 0, &buf), VEC_FF, "{name}: all‑0xFF vector");

            /* 0,1,…,31 */
            let mut buf = [0u8; 32];
            for (i, b) in buf.iter_mut().enumerate() {
                *b = i as u8;
            }
            assert_eq!(extend(fun, 0, &buf), VEC_INC, "{name}: ascending");

            /* 31,30,…,0 */
            for (i, b) in buf.iter_mut().enumerate() {
                *b = (31 - i) as u8;
            }
            assert_eq!(extend(fun, 0, &buf), VEC_DEC, "{name}: descending");

            /* 48‑byte sample */
            assert_eq!(
                extend(fun, 0, &SAMPLE48),
                VEC_SAMPLE48,
                "{name}: 48‑byte sample"
            );
        }
    });
}

#[traced_test]
fn extend_hello_world() {
    const HELLO: &[u8] = b"hello ";
    const WORLD: &[u8] = b"world";
    const HW:    &[u8] = b"hello world";

    assert_all("hello world incremental", |(name, fun)| {
        unsafe {
            let one_shot = extend(fun, 0, HW);
            let incr     = extend(fun, extend(fun, 0, HELLO), WORLD);
            assert_eq!(incr, one_shot, "{name}: incremental mismatch");
        }
    });
}

#[test]
fn extend_buffer_slicing_48() {
    const EXPECT: u32 = 0xd996_3a56;
    let mut buffer = [0u8; 48];
    buffer.copy_from_slice(&[
        0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);

    assert_all("48‑byte exhaustive slicing", |(name, fun)| {
        unsafe {
            for i in 0..48 {
                for j in i + 1..=48 {
                    let mut crc = 0u32;
                    if i > 0 {
                        crc = extend(fun, crc, &buffer[..i]);
                    }
                    crc = extend(fun, crc, &buffer[i..j]);
                    if j < 48 {
                        crc = extend(fun, crc, &buffer[j..]);
                    }
                    assert_eq!(
                        crc, EXPECT,
                        "{name}: slice ({i},{j}) mismatch"
                    );
                }
            }
        }
    });
}

#[traced_test]
#[cfg_attr(not(feature = "slow-tests"), ignore)]
fn extend_buffer_slicing_2048() {
    use bitcoin_crc32c::crc32c_extend;  // dispatcher – fastest for host

    const EXPECT: u32 = 0x36dc_c753;
    let mut buf = [0u8; 2048];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = ((3 * i * i + 7 * i + 11) & 0xff) as u8;
    }

    unsafe {
        for i in 0..2048 {
            for j in i + 1..=2048 {
                let mut crc = 0;
                if i > 0      { crc = crc32c_extend(crc, buf.as_ptr(),      i); }
                crc = crc32c_extend(crc, buf[i..].as_ptr(), j - i);
                if j < 2048  { crc = crc32c_extend(crc, buf[j..].as_ptr(), 2048 - j); }
                assert_eq!(crc, EXPECT, "slice ({i},{j}) mismatch");
            }
        }
    }
}
