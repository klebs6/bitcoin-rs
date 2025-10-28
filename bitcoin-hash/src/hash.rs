// ---------------- [ File: bitcoin-hash/src/hash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/hash.h]

/**
  | Compute the 256-bit hash of an object.
  |
  */
#[inline]
pub fn hash1<T: AsRef<[u8]>>(in1: T) -> u256 {
    let mut buf = [0u8; 32];
    Hash256::default()
        .write(in1.as_ref())
        .finalize(&mut buf);
    u256::from_le_bytes(buf)
}

/**
  | Compute the 256-bit hash of the concatenation
  | of two objects.
  |
  */
#[inline]
pub fn hash2<T1: AsRef<[u8]>, T2: AsRef<[u8]>>(in1: T1, in2: T2) -> u256 {
    let mut buf = [0u8; 32];
    Hash256::default()
        .write(in1.as_ref())
        .write(in2.as_ref())
        .finalize(&mut buf);
    u256::from_le_bytes(buf)
}

/// Compute the 256‑bit hash of a binary blob that can expose its
/// bytes through `AsRef<[u8]>`. Type/version parameters are
/// currently ignored because the Rust port serialises directly from
/// the caller‑supplied slice.
#[inline]
pub fn serialize_hash<T>(obj: &T, _n_type: Option<i32>, _n_version: Option<i32>) -> u256
where
    T: AsRef<[u8]> + ?Sized,          // ← now accepts unsized inputs
{
    hash1(obj.as_ref())
}


//-------------------------------------------[.cpp/bitcoin/src/hash.cpp]

#[inline]
pub fn rotl32(x: u32, r: i8) -> u32 {
    x.rotate_left(r as u32)
}

// ---------------- [ File: bitcoin-hash/src/hash.rs ] (new test module)
#[cfg(test)]
mod hash_primitives_spec {
    use super::*;

    const EXPECTED_DOUBLE_SHA_EMPTY_BE: &str =
        "5df6e0e2761359d30a8275058e299fcc0381534545f55cf43e41983f5d4c9456";

    #[traced_test]
    fn rotl32_round_trip() {
        let x: u32 = 0xdead_beef;
        let r = 11;
        assert_eq!(rotl32(rotl32(x, r), 32 - r as i8), x);
    }

    #[traced_test]
    fn hash1_matches_reference_vector() {
        let h = hash1(&[]);
        assert_eq!(h.to_string(), EXPECTED_DOUBLE_SHA_EMPTY_BE);
    }

    #[traced_test]
    fn hash2_is_equivalent_to_manual_concatenation() {
        let lhs = b"foo";
        let rhs = b"bar";
        let via_two_calls = hash2(lhs, rhs);
        let mut concatenated = lhs.to_vec();
        concatenated.extend_from_slice(rhs);
        let via_one_call = hash1(concatenated);
        assert_eq!(via_two_calls, via_one_call);
    }

    #[test]
    fn serialize_hash_matches_hash1() {
        let data = b"payload";
        assert_eq!(serialize_hash(&data[..], None, None), hash1(data));
    }
}
