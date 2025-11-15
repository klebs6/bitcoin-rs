// ---------------- [ File: bitcoinleveldb-hash/tests/issue.rs ]
use bitcoinleveldb_hash::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/hash_test.cc]

#[traced_test]
fn hash_signed_unsigned_issue() {
    info!("hash_signed_unsigned_issue: starting");

    let data1: [u8; 1] = [0x62];
    let data2: [u8; 2] = [0xc3, 0x97];
    let data3: [u8; 3] = [0xe2, 0x99, 0xa5];
    let data4: [u8; 4] = [0xe1, 0x80, 0xb9, 0x32];
    let data5: [u8; 48] = [
        0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    const SEED1: u32 = 0xbc9f_1d34;
    const SEED2: u32 = 0x1234_5678;

    let h0 = leveldb_hash(core::ptr::null(), 0, SEED1);
    info!("hash_signed_unsigned_issue: h0=0x{:08x}", h0);
    assert_eq!(h0, 0xbc9f_1d34);

    let h1 = leveldb_hash(data1.as_ptr(), data1.len(), SEED1);
    info!("hash_signed_unsigned_issue: h1=0x{:08x}", h1);
    assert_eq!(h1, 0xef13_45c4);

    let h2 = leveldb_hash(data2.as_ptr(), data2.len(), SEED1);
    info!("hash_signed_unsigned_issue: h2=0x{:08x}", h2);
    assert_eq!(h2, 0x5b66_3814);

    let h3 = leveldb_hash(data3.as_ptr(), data3.len(), SEED1);
    info!("hash_signed_unsigned_issue: h3=0x{:08x}", h3);
    assert_eq!(h3, 0x323c_078f);

    let h4 = leveldb_hash(data4.as_ptr(), data4.len(), SEED1);
    info!("hash_signed_unsigned_issue: h4=0x{:08x}", h4);
    assert_eq!(h4, 0xed21_633a);

    let h5 = leveldb_hash(data5.as_ptr(), data5.len(), SEED2);
    info!("hash_signed_unsigned_issue: h5=0x{:08x}", h5);
    assert_eq!(h5, 0xf333_dabb);

    info!("hash_signed_unsigned_issue: completed successfully");
}
