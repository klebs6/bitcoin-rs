// ---------------- [ File: bitcoin-hash/src/hash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/hash.h]

/**
  | Compute the 256-bit hash of an object.
  |
  */
#[inline] pub fn hash1<T: ?Sized>(in1: &T) -> u256 {

    todo!();
        /*
            uint256 result;
        Hash256().Write(MakeUCharSpan(in1)).Finalize(result);
        return result;
        */
}

/**
  | Compute the 256-bit hash of the concatenation
  | of two objects.
  |
  */
#[inline] pub fn hash2<T1, T2>(
        in1: &T1,
        in2: &T2) -> u256 {

    todo!();
        /*
            uint256 result;
        Hash256().Write(MakeUCharSpan(in1)).Write(MakeUCharSpan(in2)).Finalize(result);
        return result;
        */
}

/**
  | Compute the 256-bit hash of an object's
  | serialization.
  |
  */
pub fn serialize_hash<T>(
        obj:       &T,
        n_type:    Option<i32>,
        n_version: Option<i32>) -> u256 {

    let n_type:    i32 = n_type.unwrap_or(SER_GETHASH as i32);
    let n_version: i32 = n_version.unwrap_or(PROTOCOL_VERSION as i32);

    todo!();
        /*
            HashWriter ss(nType, nVersion);
        ss << obj;
        return ss.GetHash();
        */
}

//-------------------------------------------[.cpp/bitcoin/src/hash.cpp]

#[inline] pub fn rotl32(x: u32, r: i8) -> u32 {
    
    todo!();
        /*
            return (x << r) | (x >> (32 - r));
        */
}
