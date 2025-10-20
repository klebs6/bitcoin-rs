// ---------------- [ File: bitcoin-u160/src/u160_int.rs ]
crate::ix!();

/**
   160-bit opaque blob.

   ----------- @note

   This type is called u160 for historical reasons only. 
   It is an opaque blob of 160 bits and has no integer operations.
*/
#[derive(Clone, Default, PartialOrd,Ord,PartialEq, Eq, Hash)]
pub struct u160 {
    blob: BaseBlob160,
}

impl core::fmt::Debug for u160 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // match your test requirement: "Debug format should contain 'u160('"
        write!(f, "u160({})", self.to_string())
    }
}

impl From<&str> for u160 {
    fn from(x: &str) -> Self {
        let mut rv = Self::default();
        rv.set_hex_from_str(x);
        rv
    }
}

impl From<&String> for u160 {
    fn from(x: &String) -> Self {
        let mut rv = Self::default();
        rv.set_hex_from_str(x);
        rv
    }
}

impl From<&Vec<u8>> for u160 {
    fn from(vch: &Vec<u8>) -> Self {
        // 160 bits => 20 bytes total
        let required_len = 160 / 8;
        if vch.len() != required_len {
            panic!(
                "u160::from(&Vec<u8>): input must be {} bytes, got={}",
                required_len,
                vch.len()
            );
        }

        // Build a default BaseBlob<160> and copy the bytes
        let mut inner = BaseBlob160::default();
        // Use the public `as_mut_slice()` method instead of direct `data` field
        inner.as_mut_slice()[..required_len].copy_from_slice(&vch[..required_len]);

        Self { blob: inner }
    }
}

// Allow read-only dereferencing of `u160` to `BaseBlob<160>`
impl Deref for u160 {
    type Target = BaseBlob160;

    fn deref(&self) -> &Self::Target {
        &self.blob
    }
}

// Allow mutable dereferencing of `u160` to `BaseBlob<160>`
impl DerefMut for u160 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.blob
    }
}

impl AsRef<[u8]> for u160 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        // BaseBlob160 already exposes a slice
        self.blob.as_slice()
    }
}

impl AsMut<[u8]> for u160 {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.blob.as_mut_slice()
    }
}

impl core::fmt::Display for u160 {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Write::write_str(f, &self.to_string())
    }
}


impl u160 {

    delegate! {
        to self.blob {
            pub fn as_slice(&self) -> &[u8];
            pub fn as_slice_mut(&mut self) -> &mut [u8];
            pub fn get_hex(&self) -> String;
            pub fn set_hex_from_str(&mut self, s: &str);
        }
    }

    /// 20 bytes for 160-bit
    pub fn byte_len(&self) -> usize { 20 }
    /// Construct a `u160` from a 20-byte array **at compile time**.
    pub const fn from_bytes_20(arr: [u8; 20]) -> Self {
        Self {
            blob: BaseBlob160::from_bytes(arr),
        }
    }

    /// For convenience, same as `self.blob.to_string()`.
    pub fn to_string(&self) -> String {
        self.blob.to_string()
    }
}

#[cfg(test)]
mod u160_spec {

    use super::*;
    use tracing::{info, trace, debug};
    use traced_test::traced_test;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// A helper for logging a slice in hex form.
    fn hexdump(label: &str, data: &[u8]) {
        let mut line = String::new();
        for (i, &b) in data.iter().enumerate() {
            if i % 16 == 0 {
                if !line.is_empty() {
                    trace!("{}: {}", label, line);
                }
                line.clear();
            }
            line.push_str(&format!("{:02X} ", b));
        }
        if !line.is_empty() {
            trace!("{}: {}", label, line);
        }
    }

    /// Verify that default construction yields a zeroed out `u160`.
    #[traced_test]
    fn check_default_construction() {
        info!("Testing default construction of u160 (should be all-zero).");
        let val = u160::default();
        trace!("Created default val => {}", val.to_string());
        hexdump("val bytes", val.as_slice());

        // Expect all 20 bytes to be zero.
        for &byte in val.as_slice() {
            assert_eq!(byte, 0, "Expected default() to produce zero bytes");
        }
        trace!("check_default_construction passed.");
    }

    /// Test the 20-byte array constructor and `byte_len()`.
    #[traced_test]
    fn check_from_bytes() {
        info!("Testing from_bytes_20 constructor and byte_len().");

        let arr: [u8; 20] = [
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            0xFF, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88, 0x99,
            0x00, 0xAB, 0xBC, 0xCD, 0xEF,
        ];
        let val = u160::from_bytes_20(arr);
        trace!("Constructed val => {}", val.to_string());
        hexdump("val bytes", val.as_slice());

        assert_eq!(val.byte_len(), 20, "u160::byte_len() should be 20");
        assert_eq!(val.as_slice(), &arr, "Storage must match the given array");
        trace!("check_from_bytes passed.");
    }

    /// Test creating `u160` from a `&str` (hex string).
    #[traced_test]
    fn check_from_str() {
        info!("Testing From<&str> for u160.");

        let hex_str = "AABBCCDDEEFF11223344556677889900ABBCCDDE";
        let constructed = u160::from(hex_str);

        trace!("Constructed from hex_str => {}", constructed.to_string());
        hexdump("constructed bytes", constructed.as_slice());

        // The to_string() is also a hex, so it should match ignoring case.
        let rehex = constructed.to_string().to_uppercase();
        let input_uc = hex_str.to_uppercase();
        assert_eq!(rehex, input_uc, "Re-hexing the stored data must match the input string");
        trace!("check_from_str passed.");
    }

    /// Test creating `u160` from a `&String`.
    #[traced_test]
    fn check_from_string() {
        info!("Testing From<&String> for u160.");
        let input = String::from("00112233445566778899AABBCCDDEEFF00112233");
        let val = u160::from(&input);

        trace!("Constructed from &String => {}", val.to_string());
        hexdump("val bytes", val.as_slice());

        // Compare again ignoring case
        assert_eq!(val.to_string().to_lowercase(), input.to_lowercase());
        trace!("check_from_string passed.");
    }

    /// Test creating `u160` from a `&Vec<u8>`.
    #[traced_test]
    fn check_from_vec() {
        info!("Testing From<&Vec<u8>> for u160.");
        let input: Vec<u8> = vec![
            0xDE, 0xAD, 0xBE, 0xEF, 0x00,
            0x11, 0x22, 0x33, 0x44, 0x55,
            0x66, 0x77, 0x88, 0x99, 0xAA,
            0xBB, 0xCC, 0xDD, 0xEE, 0xFF
        ];
        let val = u160::from(&input);

        trace!("Constructed from Vec => {}", val.to_string());
        hexdump("val bytes", val.as_slice());

        // Should match byte-for-byte
        assert_eq!(val.as_slice(), &input[..]);
        trace!("check_from_vec passed.");
    }

    /// Confirm that `Ord` and `PartialOrd` work (lexicographic over bytes).
    #[traced_test]
    fn check_ord_partialord() {
        info!("Testing ordering among multiple u160 values.");

        let lower_hex = "0000000000000000000000000000000000000000";
        let higher_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";

        let lower = u160::from(lower_hex);
        let higher = u160::from(higher_hex);

        trace!("Lower => {}", lower.to_string());
        trace!("Higher => {}", higher.to_string());

        assert!(lower < higher, "Expected all-zero to be < all-FF");
        assert!(higher > lower, "Expected all-FF to be > all-zero");
        assert_eq!(lower, lower.clone(), "Cloning should remain equal");
        trace!("check_ord_partialord passed.");
    }

    /// Confirm hashing by storing in a HashMap or using a Hasher.
    #[traced_test]
    fn check_hashing() {
        info!("Testing hashing of u160 values via std::hash.");

        let val1 = u160::from("AABBCCDDEEFF00112233445566778899AABBCCDD");
        let val2 = val1.clone();
        let mut hasher1 = DefaultHasher::new();
        val1.hash(&mut hasher1);
        let h1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        val2.hash(&mut hasher2);
        let h2 = hasher2.finish();

        trace!("val1 => {}", val1.to_string());
        trace!("val2 => {}", val2.to_string());
        trace!("h1 => 0x{:X}, h2 => 0x{:X}", h1, h2);

        assert_eq!(h1, h2, "Hash of identical values must match");
        trace!("check_hashing passed.");
    }

    /// Confirm Debug formatting is something sensible (just ensuring no panic).
    #[traced_test]
    fn check_debug_format() {
        info!("Testing Debug format of u160.");
        let hex_str = "ABCDEF1234567890ABCDEF1234567890ABCDEF12";
        let val = u160::from(hex_str);
        debug!("Debug version => {:?}", val);

        // Just ensure that it includes the substring "u160("
        let dbg_str = format!("{:?}", val);
        assert!(dbg_str.contains("u160("), "Debug format should contain 'u160('");
        trace!("check_debug_format passed.");
    }
}
