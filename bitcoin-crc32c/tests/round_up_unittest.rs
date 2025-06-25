// ---------------- [ File: bitcoin-crc32c/src/round_up_unittest.rs ]
use bitcoin_crc32c::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_round_up_unittest.cc]

#[traced_test]
fn crc32c_round_up_test_uintptr() {
    let zero: usize = 0;

    // === All original C++ assertions, 1‑for‑1 ===
    assert_eq!(zero, round_up_with_uintptr::<1>(zero));
    assert_eq!(1, round_up_with_uintptr::<1>(1));
    assert_eq!(2, round_up_with_uintptr::<1>(2));
    assert_eq!(3, round_up_with_uintptr::<1>(3));
    assert_eq!(usize::MAX, round_up_with_uintptr::<1>(usize::MAX));
    assert_eq!(usize::MAX - 1, round_up_with_uintptr::<1>(usize::MAX - 1));
    assert_eq!(usize::MAX - 2, round_up_with_uintptr::<1>(usize::MAX - 2));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<1>(usize::MAX - 3));

    assert_eq!(zero, round_up_with_uintptr::<2>(zero));
    assert_eq!(2, round_up_with_uintptr::<2>(1));
    assert_eq!(2, round_up_with_uintptr::<2>(2));
    assert_eq!(4, round_up_with_uintptr::<2>(3));
    assert_eq!(4, round_up_with_uintptr::<2>(4));
    assert_eq!(6, round_up_with_uintptr::<2>(5));
    assert_eq!(6, round_up_with_uintptr::<2>(6));
    assert_eq!(8, round_up_with_uintptr::<2>(7));
    assert_eq!(8, round_up_with_uintptr::<2>(8));
    assert_eq!(usize::MAX - 1, round_up_with_uintptr::<2>(usize::MAX - 1));
    assert_eq!(usize::MAX - 1, round_up_with_uintptr::<2>(usize::MAX - 2));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<2>(usize::MAX - 3));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<2>(usize::MAX - 4));

    assert_eq!(zero, round_up_with_uintptr::<4>(zero));
    assert_eq!(4, round_up_with_uintptr::<4>(1));
    assert_eq!(4, round_up_with_uintptr::<4>(2));
    assert_eq!(4, round_up_with_uintptr::<4>(3));
    assert_eq!(4, round_up_with_uintptr::<4>(4));
    assert_eq!(8, round_up_with_uintptr::<4>(5));
    assert_eq!(8, round_up_with_uintptr::<4>(6));
    assert_eq!(8, round_up_with_uintptr::<4>(7));
    assert_eq!(8, round_up_with_uintptr::<4>(8));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<4>(usize::MAX - 3));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<4>(usize::MAX - 4));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<4>(usize::MAX - 5));
    assert_eq!(usize::MAX - 3, round_up_with_uintptr::<4>(usize::MAX - 6));
    assert_eq!(usize::MAX - 7, round_up_with_uintptr::<4>(usize::MAX - 7));
    assert_eq!(usize::MAX - 7, round_up_with_uintptr::<4>(usize::MAX - 8));
    assert_eq!(usize::MAX - 7, round_up_with_uintptr::<4>(usize::MAX - 9));
}

#[traced_test]
fn crc32c_round_up_test_pointer() {
    let zero = 0usize as *const u8;
    let three = 3usize as *const u8;
    let four = 4usize as *const u8;
    let seven = 7usize as *const u8;
    let eight = 8usize as *const u8;

    // === Original pointer assertions ===
    assert_eq!(zero, round_up::<1>(zero));
    assert_eq!(zero, round_up::<4>(zero));
    assert_eq!(zero, round_up::<8>(zero));

    assert_eq!(three, round_up::<1>(three));
    assert_eq!(four, round_up::<4>(three));
    assert_eq!(eight, round_up::<8>(three));

    assert_eq!(four, round_up::<1>(four));
    assert_eq!(four, round_up::<4>(four));
    assert_eq!(eight, round_up::<8>(four));

    assert_eq!(seven, round_up::<1>(seven));
    assert_eq!(eight, round_up::<4>(seven));
    assert_eq!(eight, round_up::<8>(four)); // note: original C++ had this same repetition
}
