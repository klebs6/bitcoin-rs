// ---------------- [ File: bitcoin-siphash/src/optimized.rs ]
crate::ix!();

/// Optimized SipHash-2-4 implementation for uint256.
/// 
/// It is identical to:
///   BitcoinSipHasher(k0, k1)
///     .Write(val.GetUint64(0))
///     .Write(val.GetUint64(1))
///     .Write(val.GetUint64(2))
///     .Write(val.GetUint64(3))
///     .Finalize()
/// 
/// Constant‑time, branch‑free SipHash‑2‑4 for a `u256`.
#[inline]
pub fn sip_hash_uint256(k0: u64, k1: u64, val: &u256) -> u64 {
    trace!(
        "sip_hash_uint256(k0={:016x}, k1={:016x})",
        k0,
        k1
    );

    let mut d  = val.get_uint64(0);
    let mut v0 = 0x736f6d6570736575u64 ^ k0;
    let mut v1 = 0x646f72616e646f6du64 ^ k1;
    let mut v2 = 0x6c7967656e657261u64 ^ k0;
    let mut v3 = 0x7465646279746573u64 ^ k1 ^ d;

    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    v0 ^= d;

    d = val.get_uint64(1);
    v3 ^= d;
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    v0 ^= d;

    d = val.get_uint64(2);
    v3 ^= d;
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    v0 ^= d;

    d = val.get_uint64(3);
    v3 ^= d;
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    v0 ^= d;

    v3 ^= (4u64) << 59; // 32‑byte message length tag
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    v0 ^= (4u64) << 59;

    v2 ^= 0xFF;
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);

    let out = v0 ^ v1 ^ v2 ^ v3;
    trace!("sip_hash_uint256 result {:016x}", out);
    out
}

#[cfg(test)]
mod optimized_uint256_tests {
    use super::*;

    /// The specialised path must match the generic hasher path for 32‑byte inputs.
    #[traced_test]
    fn optimised_matches_generic_path() {
        let k0 = 0x0123_4567_89ab_cdef;
        let k1 = 0xfedc_ba98_7654_3210;

        // 32 deterministic bytes 0..31.
        let mut raw = [0u8; 32];
        for i in 0..32 {
            raw[i] = i as u8;
        }
        let val = u256::from_le_bytes(raw);

        // Fast path
        let fast = sip_hash_uint256(k0, k1, &val);

        // Generic path
        let mut hasher = BitcoinSipHasher::new(k0, k1);
        for limb in 0..4 {
            hasher.write_u64(val.get_uint64(limb));
        }
        let generic = hasher.finalize();

        assert_eq!(fast, generic, "optimised and generic results diverge");
    }
}
