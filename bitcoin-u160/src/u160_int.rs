// ---------------- [ File: bitcoin-u160/src/u160_int.rs ]
crate::ix!();

/**
   160-bit opaque blob.

   ----------- @note

   This type is called u160 for historical reasons only. 
   It is an opaque blob of 160 bits and has no integer operations.
*/
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct u160 {
    blob: BaseBlob<160>,
}

impl From<&str> for u160 {
    fn from(x: &str) -> Self {
        // If your `BaseBlob` has a `set_hex()` method that takes a *const u8,
        // you can just call `self.set_hex(...)`. Because we `DerefMut` to
        // `BaseBlob<160>`, the code can do `rv.set_hex(...)`.
        let mut rv = Self::default();
        // This deref call is effectively `(&mut rv.blob).set_hex(...)`
        rv.set_hex(x.as_ptr());
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
        let mut inner = BaseBlob::<160>::default();
        // Use the public `as_mut_slice()` method instead of direct `data` field
        inner.as_mut_slice()[..required_len].copy_from_slice(&vch[..required_len]);

        Self { blob: inner }
    }
}

// Allow read-only dereferencing of `u160` to `BaseBlob<160>`
impl Deref for u160 {
    type Target = BaseBlob<160>;

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

impl u160 {
    /// Construct a `u160` from a 20-byte array **at compile time**.
    pub const fn from_bytes_20(arr: [u8; 20]) -> Self {
        Self {
            blob: BaseBlob::<160>::from_bytes(arr),
        }
    }
}
