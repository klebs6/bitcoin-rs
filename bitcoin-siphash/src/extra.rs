// ---------------- [ File: bitcoin-siphash/src/extra.rs ]
crate::ix!();

/// Bitcoin‑Core‑compatible SipHash‑2‑4 of a 256‑bit value
/// with an additional 32‑bit tag (`extra`).  
///
/// This is the exact translation of `SipHashUint256Extra`
/// from the C++ reference code.
#[inline]
pub fn sip_hash_uint_256extra(
    k0:    u64,
    k1:    u64,
    val:   &u256,
    extra: u32,
) -> u64 {
    trace!(
        "sip_hash_uint_256extra(k0={:016x}, k1={:016x}, extra={:08x})",
        k0,
        k1,
        extra
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

    d = ((36u64) << 56) | extra as u64; // 32‑byte input + 4‑byte tag = 36
    v3 ^= d;
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    v0 ^= d;

    v2 ^= 0xFF;
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);
    sipround!(v0, v1, v2, v3);

    let out = v0 ^ v1 ^ v2 ^ v3;
    trace!("sip_hash_uint_256extra result {:016x}", out);
    out
}

#[cfg(test)]
mod uint256_extra_tests {
    use super::*;

    /// Cross‑check against the generic hasher path.
    #[traced_test]
    fn extra_matches_generic_path() {
        let k0 = 0x0706050403020100u64;
        let k1 = 0x0f0e0d0c0b0a0908u64;
        let extra: u32 = 0xdead_beef;

        // Build an easy deterministic uint256 value: 0x00…1f (LE).
        let mut raw = [0u8; 32];
        for i in 0..32 {
            raw[i] = i as u8;
        }
        let val = u256::from_le_bytes(raw);

        // Fast path:
        let fast = sip_hash_uint_256extra(k0, k1, &val, extra);

        // Generic path:
        let mut hasher = BitcoinSipHasher::new(k0, k1);
        for limb in 0..4 {
            hasher.write_u64(val.get_uint64(limb));
        }
        hasher.write(&extra.to_le_bytes());
        let generic = hasher.finalize();

        assert_eq!(fast, generic, "specialised and generic results diverge");
    }
}
