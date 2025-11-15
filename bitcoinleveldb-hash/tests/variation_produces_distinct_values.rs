// ---------------- [ File: bitcoinleveldb-hash/tests/variation_produces_distinct_values.rs ]
use bitcoinleveldb_hash::*;
use bitcoin_imports::*;

#[traced_test]
fn hash_seed_variation_produces_distinct_values() {
    info!("hash_seed_variation_produces_distinct_values: starting");

    let data: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33,
        0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb,
        0xcc, 0xdd, 0xee, 0xff,
    ];

    let seeds: [u32; 4] = [0u32, 1u32, 0xdead_beef, 0xbc9f_1d34];
    let mut results: [u32; 4] = [0u32; 4];

    for (idx, seed) in seeds.iter().enumerate() {
        let h = leveldb_hash(data.as_ptr(), data.len(), *seed);
        results[idx] = h;

        debug!(
            "hash_seed_variation_produces_distinct_values: idx={}, seed=0x{:08x}, hash=0x{:08x}",
            idx,
            seed,
            h
        );
    }

    for i in 0..seeds.len() {
        for j in (i + 1)..seeds.len() {
            assert_ne!(
                results[i], results[j],
                "hash() should produce different outputs for different seeds (i={}, j={})",
                i,
                j
            );
        }
    }

    info!("hash_seed_variation_produces_distinct_values: completed successfully");
}
