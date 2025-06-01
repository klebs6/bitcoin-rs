// ---------------- [ File: bitcoin-u256/tests/u256.rs ]
use bitcoin_u256::*;
use bitcoin_u160::*;
use bitcoin_imports::*;
use tracing::{trace, debug, info};

/// Example: A 32-byte array for the 256-bit R1.
/// (We only keep the raw arrays as true constants.)
pub const R1ARRAY_256: [u8; 32] = [
    0x9c, 0x52, 0x4a, 0xdb, 0xcf, 0x56, 0x11, 0x12,
    0x2b, 0x29, 0x12, 0x5e, 0x5d, 0x35, 0xd2, 0xd2,
    0x22, 0x81, 0xaa, 0xb5, 0x33, 0xf0, 0x08, 0x32,
    0xd5, 0x56, 0xb1, 0xf9, 0xea, 0xe5, 0x1d, 0x7d,
];

/// For `u160`, we need only the first 20 bytes of R1:
pub const R1ARRAY_160: [u8; 20] = [
    0x9c, 0x52, 0x4a, 0xdb, 0xcf,
    0x56, 0x11, 0x12, 0x2b, 0x29,
    0x12, 0x5e, 0x5d, 0x35, 0xd2,
    0xd2, 0x22, 0x81, 0xaa, 0xb5,
];

/// A convenience if you want the hex representation as bytes:
pub const R1ARRAY_HEX: &[u8] =
    b"7D1DE5EAF9B156D53208F033B5AA8122D2D2355D5E12292B121156CFDB4A529C";

// Similarly for R2:
pub const R2ARRAY_256: [u8; 32] = [
    0x70, 0x32, 0x1d, 0x7c, 0x47, 0xa5, 0x6b, 0x40,
    0x26, 0x7e, 0x0a, 0xc3, 0xa6, 0x9c, 0xb6, 0xbf,
    0x13, 0x30, 0x47, 0xa3, 0x19, 0x2d, 0xda, 0x71,
    0x49, 0x13, 0x72, 0xf0, 0xb4, 0xca, 0x81, 0xd7,
];

pub const R2ARRAY_160: [u8; 20] = [
    0x70, 0x32, 0x1d, 0x7c, 0x47,
    0xa5, 0x6b, 0x40, 0x26, 0x7e,
    0x0a, 0xc3, 0xa6, 0x9c, 0xb6,
    0xbf, 0x13, 0x30, 0x47, 0xa3,
];

// Zero, One, Max as 32-byte and 20-byte arrays:
pub const ZERO_ARRAY_256: [u8; 32] = [0u8; 32];
pub const ZERO_ARRAY_160: [u8; 20] = [0u8; 20];

pub const ONE_ARRAY_256: [u8; 32] = [
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];
pub const ONE_ARRAY_160: [u8; 20] = [
    0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

pub const MAX_ARRAY_256: [u8; 32] = [0xFFu8; 32];
pub const MAX_ARRAY_160: [u8; 20] = [0xFFu8; 20];

/* ---- Now define helper functions that return u256 or u160 from the above arrays ---- */

fn r1l() -> u256 {
    let v = u256::from_bytes_32(R1ARRAY_256);
    trace!("r1l() => built R1L={}", v.to_string());
    v
}
fn r1s() -> u160 {
    let v = u160::from_bytes_20(R1ARRAY_160);
    trace!("r1s() => built R1S={}", v.to_string());
    v
}

fn r2l() -> u256 {
    let v = u256::from_bytes_32(R2ARRAY_256);
    trace!("r2l() => built R2L={}", v.to_string());
    v
}
fn r2s() -> u160 {
    let v = u160::from_bytes_20(R2ARRAY_160);
    trace!("r2s() => built R2S={}", v.to_string());
    v
}

fn zerol() -> u256 {
    let v = u256::from_bytes_32(ZERO_ARRAY_256);
    trace!("zerol() => built ZeroL={}", v.to_string());
    v
}
fn zeros() -> u160 {
    let v = u160::from_bytes_20(ZERO_ARRAY_160);
    trace!("zeros() => built ZeroS={}", v.to_string());
    v
}

fn onel() -> u256 {
    let v = u256::from_bytes_32(ONE_ARRAY_256);
    trace!("onel() => built OneL={}", v.to_string());
    v
}
fn ones() -> u160 {
    let v = u160::from_bytes_20(ONE_ARRAY_160);
    trace!("ones() => built OneS={}", v.to_string());
    v
}

fn maxl() -> u256 {
    let v = u256::from_bytes_32(MAX_ARRAY_256);
    trace!("maxl() => built MaxL={}", v.to_string());
    v
}
fn maxs() -> u160 {
    let v = u160::from_bytes_20(MAX_ARRAY_160);
    trace!("maxs() => built MaxS={}", v.to_string());
    v
}

/** 
  A helper that replicates the C++ reverse-hex printing, 
  to match old tests. 
*/
pub fn array_to_string(a: &[u8], width: u32) -> String {
    let mut s = String::new();
    for i in 0..width {
        let b = a[(width - 1 - i) as usize];
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// Log a hexdump of a slice (for debugging).
fn hexdump_slice(label: &str, data: &[u8]) {
    let mut line = String::new();
    for (i, &b) in data.iter().enumerate() {
        if i % 16 == 0 {
            if !line.is_empty() {
                trace!("{}: {}", label, line);
            }
            line.clear();
        }
        line.push_str(&format!("{:02X} ", b));
    }
    if !line.is_empty() {
        trace!("{}: {}", label, line);
    }
}

/// 1) "basics" test from the old C++ code
#[traced_test]
fn test_basics() {
    info!("== test_basics START ==");

    trace!("calling r1l(), r1s(), etc. to ensure they've been constructed:");
    let _ = r1l();
    let _ = r1s();
    let _ = r2l();
    let _ = r2s();
    let _ = zerol();
    let _ = zeros();
    let _ = onel();
    let _ = ones();
    let _ = maxl();
    let _ = maxs();

    trace!("Begin checks in test_basics...");

    assert_eq!(1, 0+1, "simple check: 1==0+1 just sanity");

    // Compare r1l().to_string() with array_to_string(R1ARRAY_256,32)
    {
        let got = r1l().to_string().to_lowercase();
        let expect = array_to_string(&R1ARRAY_256,32);
        trace!("r1l().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "R1L.ToString()");
    }

    // Similarly for r1s():
    {
        let got = r1s().to_string().to_lowercase();
        let expect = array_to_string(&R1ARRAY_160, 20);
        trace!("r1s().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "R1S.ToString()");
    }

    // r2l
    {
        let got = r2l().to_string().to_lowercase();
        let expect = array_to_string(&R2ARRAY_256,32);
        trace!("r2l().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "R2L.ToString()");
    }

    // r2s
    {
        let got = r2s().to_string().to_lowercase();
        let expect = array_to_string(&R2ARRAY_160,20);
        trace!("r2s().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "R2S.ToString()");
    }

    // zero-l
    {
        let got = zerol().to_string().to_lowercase();
        let expect = array_to_string(&ZERO_ARRAY_256,32);
        trace!("zerol().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "ZeroL.ToString()");
    }

    // zero-s
    {
        let got = zeros().to_string().to_lowercase();
        let expect = array_to_string(&ZERO_ARRAY_160,20);
        trace!("zeros().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "ZeroS.ToString()");
    }

    // one-l
    {
        let got = onel().to_string().to_lowercase();
        let expect = array_to_string(&ONE_ARRAY_256,32);
        trace!("onel().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "OneL.ToString()");
    }

    // one-s
    {
        let got = ones().to_string().to_lowercase();
        let expect = array_to_string(&ONE_ARRAY_160,20);
        trace!("ones().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "OneS.ToString()");
    }

    // max-l
    {
        let got = maxl().to_string().to_lowercase();
        let expect = array_to_string(&MAX_ARRAY_256,32);
        trace!("maxl().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "MaxL.ToString()");
    }

    // max-s
    {
        let got = maxs().to_string().to_lowercase();
        let expect = array_to_string(&MAX_ARRAY_160,20);
        trace!("maxs().to_string() => {}, expect => {}", got, expect);
        assert_eq!(got, expect, "MaxS.ToString()");
    }

    // check onel() != zero-l array
    {
        let o = onel().to_string().to_lowercase();
        let z = array_to_string(&ZERO_ARRAY_256,32);
        trace!("Check that onel() != zero-arr => {} vs {}", o, z);
        assert_ne!(o, z, "OneL != ZeroL");
    }

    // Additional
    info!("== test_basics END ==");
}

fn set_bit_u256(x: &mut u256, bit_index: usize) {
    trace!("set_bit_u256: before => x={}, bit_index={}", x.to_string(), bit_index);

    // This exactly replicates the old C++ code:
    //   *(TmpL.begin() + (i >> 3)) |= 1 << (7 - (i & 7));
    // except here, `begin()` is equivalent to x.as_slice_mut()[0].
    let offset = bit_index >> 3;
    let shift = 7 - (bit_index & 7);

    // Safety check: for 256 bits => valid bit_index is [0..255].
    if offset >= 32 {
        panic!("bit_index={} is out of range for a 256-bit value", bit_index);
    }

    let slice = x.as_slice_mut();
    slice[offset] |= 1 << shift;

    trace!("set_bit_u256: after => x={}", x.to_string());
}

fn set_bit_u160(x: &mut u160, bit_index: usize) {
    trace!("set_bit_u160: before => x={}, bit_index={}", x.to_string(), bit_index);

    // Same approach for 160 bits:
    //   *(TmpS.begin() + (i >> 3)) |= 1 << (7 - (i & 7));
    let offset = bit_index >> 3;
    let shift = 7 - (bit_index & 7);

    // Safety check: for 160 bits => valid bit_index is [0..159].
    if offset >= 20 {
        panic!("bit_index={} is out of range for a 160-bit value", bit_index);
    }

    let slice = x.as_slice_mut();
    slice[offset] |= 1 << shift;

    trace!("set_bit_u160: after => x={}", x.to_string());
}

/// 2) "test_comparison"
#[traced_test]
fn test_comparison() {
    info!("== test_comparison START ==");

    // for 256 bits
    let mut last_l = zerol();
    trace!("Initial last_l=zerol() => {}", last_l.to_string());
    for i in (0..=255).rev() {
        // In each iteration, create a new zero-l, set bit i, then compare
        let mut tmp_l = zerol();
        set_bit_u256(&mut tmp_l, i);
        trace!("Comparing last_l < tmp_l => i={}", i);
        if !(last_l < tmp_l) {
            error!("FAIL: last_l >= tmp_l at i={}", i);
            trace!("  last_l => {}", last_l.to_string());
            hexdump_slice("  last_l bytes", last_l.as_slice());
            trace!("  tmp_l  => {}", tmp_l.to_string());
            hexdump_slice("  tmp_l  bytes", tmp_l.as_slice());

            panic!("last_l < tmp_l at i={}", i);
        }
        last_l = tmp_l;
    }
    assert!(zerol() < r1l());
    assert!(r2l() < r1l());
    assert!(zerol() < onel());
    assert!(onel() < maxl());
    assert!(r1l() < maxl());
    assert!(r2l() < maxl());

    // for 160 bits
    let mut last_s = zeros();
    trace!("Initial last_s=zeros() => {}", last_s.to_string());
    for i in (0..=159).rev() {
        let mut tmp_s = zeros();
        set_bit_u160(&mut tmp_s, i);
        trace!("Comparing last_s < tmp_s => i={}", i);
        if !(last_s < tmp_s) {
            error!("FAIL: last_s >= tmp_s at i={}", i);
            trace!("  last_s => {}", last_s.to_string());
            hexdump_slice("  last_s bytes", last_s.as_slice());
            trace!("  tmp_s => {}", tmp_s.to_string());
            hexdump_slice("  tmp_s bytes", tmp_s.as_slice());

            panic!("last_s < tmp_s at i={}", i);
        }
        last_s = tmp_s;
    }
    assert!(zeros() < r1s());
    assert!(r2s() < r1s());
    assert!(zeros() < ones());
    assert!(ones() < maxs());
    assert!(r1s() < maxs());
    assert!(r2s() < maxs());

    info!("== test_comparison END ==");
}

/// 3) "test_methods"
#[traced_test]
fn test_methods() {
    info!("== test_methods START ==");

    // Checking get_hex vs to_string for 256 bits
    assert_eq!(r1l().get_hex(), r1l().to_string());
    assert_eq!(r2l().get_hex(), r2l().to_string());
    assert_eq!(onel().get_hex(), onel().to_string());
    assert_eq!(maxl().get_hex(), maxl().to_string());

    let mut tmp_l = r1l();
    tmp_l.set_hex_from_str(&r2l().to_string());
    assert_eq!(tmp_l, r2l());
    tmp_l.set_hex_from_str(&zerol().to_string());
    assert_eq!(tmp_l, zerol());

    assert_eq!(r1l().as_slice(), &R1ARRAY_256[..]);
    assert_eq!(r2l().as_slice(), &R2ARRAY_256[..]);
    assert_eq!(zerol().as_slice(), &ZERO_ARRAY_256[..]);
    assert_eq!(onel().as_slice(), &ONE_ARRAY_256[..]);
    assert_eq!(r1l().byte_len(), 32);

    // for 160 bits
    assert_eq!(r1s().get_hex(), r1s().to_string());
    assert_eq!(r2s().get_hex(), r2s().to_string());
    assert_eq!(ones().get_hex(), ones().to_string());
    assert_eq!(maxs().get_hex(), maxs().to_string());

    let mut tmp_s = r1s();
    tmp_s.set_hex_from_str(&r2s().to_string());
    assert_eq!(tmp_s, r2s());
    tmp_s.set_hex_from_str(&zeros().to_string());
    assert_eq!(tmp_s, zeros());

    assert_eq!(r1s().as_slice(), &R1ARRAY_160[..]);
    assert_eq!(r2s().as_slice(), &R2ARRAY_160[..]);
    assert_eq!(zeros().as_slice(), &ZERO_ARRAY_160[..]);
    assert_eq!(ones().as_slice(), &ONE_ARRAY_160[..]);
    assert_eq!(r1s().byte_len(), 20);

    let lw = r1l().low64();
    trace!("r1l().low64() => 0x{:016X}", lw);

    info!("== test_methods END ==");
}

/// 4) "test_conversion"
#[traced_test]
fn test_conversion() {
    info!("== test_conversion START ==");

    let r1_arith = uint_to_arith256(&r1l());
    let r1_back = arith_to_uint256(&r1_arith);
    assert_eq!(r1_back, r1l());

    let zero_ar = uint_to_arith256(&zerol());
    assert_eq!(zero_ar, 0u64.into());
    let from0 = arith_to_uint256(&0u64.into());
    assert_eq!(from0, zerol());

    let one_ar = uint_to_arith256(&onel());
    assert_eq!(one_ar, 1u64.into());
    let from1 = arith_to_uint256(&1u64.into());
    assert_eq!(from1, onel());

    // string alignment
    assert_eq!(r1l().to_string(), uint_to_arith256(&r1l()).get_hex());
    assert_eq!(r2l().to_string(), uint_to_arith256(&r2l()).get_hex());

    info!("== test_conversion END ==");
}

/// 5) "test_operator_with_self"
#[traced_test]
fn test_operator_with_self() {
    info!("== test_operator_with_self START ==");

    let two_hex = "02".to_string();
    let two_u256 = u256::from(&two_hex);
    let mut v = uint_to_arith256(&two_u256);

    // v *= v => 4
    v *= &v.clone();
    let four = uint_to_arith256(&u256::from(&"04".to_string()));
    assert_eq!(v, four);

    // v /= v => 1
    v /= &v.clone();
    let one = uint_to_arith256(&u256::from(&"01".to_string()));
    assert_eq!(v, one);

    // v += v => 2
    v += &v.clone();
    let two_ar = uint_to_arith256(&u256::from(&"02".to_string()));
    assert_eq!(v, two_ar);

    // v -= v => 0
    v -= &v.clone();
    let z = uint_to_arith256(&zerol());
    assert_eq!(v, z);

    info!("== test_operator_with_self END ==");
}

/// 6) "test_check_one"
#[traced_test]
fn test_check_one() {
    info!("== test_check_one START ==");

    let cxx_str = "0000000000000000000000000000000000000000000000000000000000000001";
    let one_from_cxx = u256::from(&cxx_str.to_string());
    assert_eq!(one_from_cxx, onel(), "Matches ONEL");

    info!("== test_check_one END ==");
}
