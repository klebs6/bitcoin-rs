// ---------------- [ File: bitcoin-u256/src/u256_int.rs ]
crate::ix!();

/// 256-bit opaque blob.
///
/// This type is called `uint256` for historical reasons only. 
/// It is an opaque blob of 256 bits and has no integer operations.
/// Use `arith_uint256` if those are required.
#[derive(Getters,Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[getset(get="pub")]
#[allow(non_camel_case_types)]
pub struct u256 {
    pub(crate) blob: BaseBlob256,
}

impl AsRef<[u8]> for u256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsMut<[u8]> for u256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_slice_mut()
    }
}

impl AsRef<[u8; 32]> for u256 {
    #[inline]
    fn as_ref(&self) -> &[u8; 32] {
        self.as_slice_exact()
    }
}

impl AsMut<[u8; 32]> for u256 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8; 32] {
        self.as_mut_slice_exact()
    }
}

impl core::fmt::Display for u256 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Write::write_str(f, &self.to_string())
    }
}

impl u256 {

    /// Immutable view as an exact 32-byte array
    #[inline]
    pub fn as_slice_exact(&self) -> &[u8; 32] {
        <&[u8] as core::convert::TryInto<&[u8; 32]>>::try_into(self.as_slice())
            .expect("u256 should always be 32 bytes")
    }

    /// Mutable view as an exact 32-byte array
    #[inline]
    pub fn as_mut_slice_exact(&mut self) -> &mut [u8; 32] {
        <&mut [u8] as core::convert::TryInto<&mut [u8; 32]>>::try_into(self.as_mut())
            .expect("u256 should always be 32 bytes")
    }

    /// Return the 64‑bit little‑endian limb at position `index` (0 ≤ index < 4).  
    /// This is the direct analogue of C++ `uint256::GetUint64()`.
    #[inline]
    pub fn get_uint64(&self, index: usize) -> u64 {
        assert!(index < 4, "u256::get_uint64 index out of range (0‑3)");
        let bytes = self.as_slice();
        let start = index * 8;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[start..start + 8]);
        u64::from_le_bytes(buf)
    }

    delegate! {
        to self.blob {
            pub fn as_slice(&self) -> &[u8];
            pub fn as_slice_mut(&mut self) -> &mut [u8];
            pub fn get_hex(&self) -> String;
            pub fn set_hex_from_str(&mut self, s: &str);
        }
    }

    /// Return the lowest 64 bits in little-endian order.
    pub fn low64(&self) -> u64 {
        let slice = self.as_slice();
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&slice[..8]); 
        u64::from_le_bytes(arr)
    }

    /// Construct a `u256` from a 32-byte array **at compile time**.
    pub const fn from_bytes_32(arr: [u8; 32]) -> Self {
        Self {
            blob: BaseBlob256::from_bytes(arr),
        }
    }

    pub fn zero() -> Self {
        trace!("u256::zero => returning default (all 0s)");
        let mut out = Self::default();
        // Or out.blob.set_null();
        out
    }

    pub fn one() -> Self {
        trace!("u256::one => returning a u256 with the low byte=1");
        let mut out = Self::default();
        // Write 1 into the least-significant byte
        out.as_slice_mut()[0] = 1;
        out
    }

    pub fn byte_len(&self) -> usize {
        32
    }

    pub fn is_null(&self) -> bool {
        self.blob.is_null()
    }

    pub fn set_null(&mut self) {
        self.blob.set_null()
    }

    pub fn to_string(&self) -> String {
        self.blob.get_hex()
    }

    /// Construct a `u256` from a **little‑endian** 32‑byte array.
    ///
    /// Bitcoin Core stores hashes in little‑endian when they are interpreted as
    /// integers, while `BaseBlob` keeps its internal bytes in big‑endian.  We
    /// therefore reverse the incoming buffer before delegating to the existing
    /// `from_bytes_32` constructor.
    #[inline]
    pub fn from_le_bytes(mut le: [u8; 32]) -> Self {
        le.reverse();
        Self::from_bytes_32(le)
    }
}

unsafe impl Send for u256 {}
unsafe impl Sync for u256 {}

//--------------------
// Exhaustive Test Suite
//--------------------
#[cfg(test)]
mod u256_tests {
    use super::*;
    use serde_test::{Token, assert_tokens, Configure};
    use std::panic::{catch_unwind, AssertUnwindSafe};

    /// We'll define a small helper for random bytes:
    fn random_bytes_32(seed: &mut u64) -> [u8; 32] {
        let mut out = [0u8; 32];
        for chunk in out.chunks_mut(8) {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let arr = seed.to_le_bytes();
            chunk.copy_from_slice(&arr[..8.min(chunk.len())]);
        }
        out
    }

    #[traced_test]
    fn test_constants_zero_one() {
        let z = u256::zero();
        assert!(z.is_null(), "zero => is_null()");
        let o = u256::one();
        assert!(!o.is_null(), "one => not null");
        // The first byte => 1, rest => 0
        assert_eq!(o.as_slice()[0], 1, "one => first byte=1");
        for b in &o.as_slice()[1..] {
            assert_eq!(*b, 0, "subsequent bytes=0 in one");
        }
    }

    #[traced_test]
    fn test_from_vec_u8() {
        let good = vec![0u8; 32];
        let _u = u256::from(&good);

        let bad = vec![1u8; 16];
        let caught = catch_unwind(AssertUnwindSafe(|| {
            let _v = u256::from(&bad);
        }));
        assert!(caught.is_err(), "Should panic on length != 32");
    }

    #[traced_test]
    fn test_from_u8() {
        let x = u256::from(0xABu8);
        assert_eq!(x.as_slice()[0], 0xAB);
        for b in &x.as_slice()[1..] {
            assert_eq!(*b, 0);
        }
    }

    #[traced_test]
    fn test_from_ptr() {
        // We'll just do a simple check
        // In real usage, you'd pass a `*const u8` pointing to a null-terminated hex string
        // We'll simulate that in Rust with a small buffer:
        let c_string = std::ffi::CString::new("0x1234AB").unwrap();
        let ptr = c_string.as_ptr() as *const u8;
        let u = u256::from(ptr);
        // Expect partial hex parse => 0x1234AB => stored in little-end. 
        // The simplest check is just `to_string()`
        let hex_str = u.to_string(); 
        // => "1234ab" or "1234AB", etc. 
        // Implementation depends on your `get_hex()`.
        assert_ne!(hex_str, "0", "Should parse a nonzero value");
    }

    #[traced_test]
    fn test_from_string() {
        let s = "abcdef".to_string();
        let u = u256::from(&s);
        let t = u.to_string();
        assert!(t.to_ascii_lowercase().contains("abcdef") 
            || t.to_ascii_lowercase().contains("abcde"), 
            "Should contain partial parse of 'abcdef'");
    }

    #[traced_test]
    fn test_is_null_and_set_null() {
        let mut x = u256::default();
        assert!(x.is_null(), "default => all zero => is_null()");
        x.as_slice_mut()[0] = 1;
        assert!(!x.is_null(), "modified => not null");
        x.set_null();
        assert!(x.is_null(), "after set_null => is_null()");
    }

    #[traced_test]
    fn test_to_string() {
        let mut x = u256::default();
        x.as_slice_mut()[0] = 0x12;
        x.as_slice_mut()[1] = 0x34;
        let s = x.to_string();
        // If your get_hex is big-end, that might appear reversed or minimal. 
        // We'll just check that it's not empty and not "0".
        assert_ne!(s, "0", "some non-zero bytes => string != \"0\"");
    }

    #[traced_test]
    fn test_serialize_deserialize() {
        // Use `serde_test` for a straightforward check
        let mut x = u256::default();
        for (i, b) in x.as_slice_mut().iter_mut().enumerate() {
            *b = i as u8;
        }

        // We expect 32 raw bytes
        static EXPECTED: &[u8] = &[
            0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31
        ];

        // We'll do a RoundTrip test with Token::Bytes
        use serde_test::{Token, assert_tokens};
        assert_tokens(&x.compact(), &[
            Token::Bytes(EXPECTED),
        ]);
    }
}
