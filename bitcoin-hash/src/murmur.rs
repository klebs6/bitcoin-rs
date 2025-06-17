// ---------------- [ File: bitcoin-hash/src/murmur.rs ]
crate::ix!();

#[instrument(level = "trace", skip_all)]
pub fn murmur_hash3(n_hash_seed: u32, data_to_hash: &[u8]) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;

    let mut h1 = n_hash_seed;
    let nblocks = data_to_hash.len() / 4;

    // body
    for chunk in data_to_hash[..nblocks * 4].chunks_exact(4) {
        let mut k1 = u32::from_le_bytes(chunk.try_into().expect("chunk length is 4"));
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1
            .wrapping_mul(5)
            .wrapping_add(0xe654_6b64);
    }

    // tail
    let mut k1 = 0u32;
    match data_to_hash.len() & 3 {
        3 => {
            k1 ^= (data_to_hash[nblocks * 4 + 2] as u32) << 16;
            k1 ^= (data_to_hash[nblocks * 4 + 1] as u32) << 8;
            k1 ^= data_to_hash[nblocks * 4] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (data_to_hash[nblocks * 4 + 1] as u32) << 8;
            k1 ^= data_to_hash[nblocks * 4] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= data_to_hash[nblocks * 4] as u32;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(C2);
            h1 ^= k1;
        }
        _ => {}
    }

    // finalization
    h1 ^= data_to_hash.len() as u32;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85eb_ca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2_ae35);
    h1 ^= h1 >> 16;

    h1
}

// ---------------- [ File: bitcoin-hash/src/murmur.rs ] (new test module)
#[cfg(test)]
mod murmur_spec {
    use super::*;

    #[traced_test]
    fn empty_payload_seed_zero_is_zero() {
        assert_eq!(murmur_hash3(0, &[]), 0);
    }

    #[traced_test]
    fn hello_world_vector_matches_reference() {
        // pre‑computed with the canonical C++ implementation
        const REF: u32 = 0x12da_77c8;
        assert_eq!(murmur_hash3(0, b"Hello"), REF);
    }

    #[traced_test]
    fn different_seeds_give_different_hashes() {
        let a = murmur_hash3(42, b"data");
        let b = murmur_hash3(43, b"data");
        assert_ne!(a, b);
    }
}
