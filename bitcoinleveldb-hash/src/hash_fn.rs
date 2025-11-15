// ---------------- [ File: bitcoinleveldb-hash/src/hash_fn.rs ]
/*!
  | Simple hash function used for internal
  | data structures
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/hash.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/hash.cc]

pub fn leveldb_hash(data: *const u8, n: usize, seed: u32) -> u32 {
    trace!(
        "leveldb_hash(): called with data_ptr={:?}, length={}, seed=0x{:08x}",
        data,
        n,
        seed
    );

    const M: u32 = 0xc6a4a793;
    const R: u32 = 24;

    // Initial mixing of the seed and length (wrapping arithmetic as in C++).
    let mut h: u32 = seed ^ ((n as u32).wrapping_mul(M));

    let mut offset: usize = 0;

    // Process 4 bytes (32 bits) at a time, little-endian, like DecodeFixed32.
    while offset + 4 <= n {
        let mut word_bytes = [0u8; 4];

        unsafe {
            let chunk_ptr = data.add(offset);
            core::ptr::copy_nonoverlapping(chunk_ptr, word_bytes.as_mut_ptr(), 4);
        }

        let w: u32 = u32::from_le_bytes(word_bytes);

        h = h.wrapping_add(w);
        h = h.wrapping_mul(M);
        h ^= h >> 16;

        trace!(
            "leveldb_hash(): processed 4-byte chunk at offset={}, word=0x{:08x}, h=0x{:08x}",
            offset,
            w,
            h
        );

        offset += 4;
    }

    let remaining: usize = n - offset;

    if remaining != 0 {
        unsafe {
            let tail_ptr = data.add(offset);

            trace!(
                "leveldb_hash(): processing tail bytes, remaining={}, offset={}",
                remaining,
                offset
            );

            match remaining {
                3 => {
                    h = h.wrapping_add((*tail_ptr.add(2) as u32) << 16);
                    h = h.wrapping_add((*tail_ptr.add(1) as u32) << 8);
                    h = h.wrapping_add(*tail_ptr as u32);
                    h = h.wrapping_mul(M);
                    h ^= h >> R;
                }
                2 => {
                    h = h.wrapping_add((*tail_ptr.add(1) as u32) << 8);
                    h = h.wrapping_add(*tail_ptr as u32);
                    h = h.wrapping_mul(M);
                    h ^= h >> R;
                }
                1 => {
                    h = h.wrapping_add(*tail_ptr as u32);
                    h = h.wrapping_mul(M);
                    h ^= h >> R;
                }
                _ => {
                    error!(
                        "leveldb_hash(): unreachable tail length encountered: remaining={}",
                        remaining
                    );
                    debug_assert!(false, "unreachable tail length in leveldb_hash()");
                }
            }
        }
    }

    debug!(
        "leveldb_hash(): completed with length={}, seed=0x{:08x}, result=0x{:08x}",
        n,
        seed,
        h
    );

    h
}
