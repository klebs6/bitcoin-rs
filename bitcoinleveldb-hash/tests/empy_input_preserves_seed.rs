// ---------------- [ File: bitcoinleveldb-hash/tests/empy_input_preserves_seed.rs ]
use bitcoinleveldb_hash::*;
use bitcoin_imports::*;

#[traced_test]
fn hash_empty_input_preserves_seed() {
    info!("hash_empty_input_preserves_seed: starting");

    let dummy: [u8; 4] = [0u8; 4];
    let seeds: [u32; 4] = [0u32, 1u32, 0xbc9f_1d34, 0xffff_ffff];

    for &seed in seeds.iter() {
        let h_null = leveldb_hash(core::ptr::null(), 0, seed);
        let h_non_null = leveldb_hash(dummy.as_ptr(), 0, seed);

        debug!(
            "hash_empty_input_preserves_seed: seed=0x{:08x}, h_null=0x{:08x}, h_non_null=0x{:08x}",
            seed,
            h_null,
            h_non_null
        );

        assert_eq!(
            h_null, seed,
            "hash() should return the seed for empty input with null pointer"
        );
        assert_eq!(
            h_non_null, seed,
            "hash() should return the seed for empty input with non-null pointer"
        );
    }

    info!("hash_empty_input_preserves_seed: completed successfully");
}
