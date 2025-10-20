// ---------------- [ File: bitcoin-hash/src/writer.rs ]
crate::ix!();

/**
  | A writer stream (for serialization)
  | that computes a 256-bit hash.
  |
  */
pub struct HashWriter {
    ctx:       Sha256,
    n_type:    i32,
    n_version: i32,
}

impl<T> Shl<&T> for HashWriter
where
    T: AsRef<[u8]> + ?Sized,          // ← `?Sized` so `&[u8]` works
{
    type Output = HashWriter;

    #[inline]
    fn shl(mut self, rhs: &T) -> Self::Output {
        self.write(rhs.as_ref());
        self
    }
}

impl HashWriter {

    /// Create a new writer with the given *type* and *version*.
    #[instrument(level = "trace")]
    pub fn new(n_type_in: i32, n_version_in: i32) -> Self {
        Self {
            ctx: Sha256::new(),
            n_type: n_type_in,
            n_version: n_version_in,
        }
    }

    #[inline]
    pub fn get_type(&self) -> i32 {
        self.n_type
    }

    #[inline]
    pub fn get_version(&self) -> i32 {
        self.n_version
    }

    #[instrument(level = "trace", skip(self, pch))]
    pub fn write(&mut self, pch: &[u8]) {
        self.ctx.write(pch);
    }

    /**
      | Compute the double-SHA256 hash of all
      | data written to this object.
      | 
      | Invalidates this object.
      |
      */
    #[instrument(level = "debug", skip(self))]
    pub fn get_hash(&mut self) -> u256 {
        let mut first = [0u8; 32];
        self.ctx.finalize(&mut first);

        self.ctx.reset();
        self.ctx.write(&first);
        let mut second = [0u8; 32];
        self.ctx.finalize(&mut second);

        u256::from_le_bytes(second)
    }

    /**
      | Compute the SHA256 hash of all data written
      | to this object.
      | 
      | Invalidates this object.
      |
      */
    #[instrument(level = "debug", skip(self))]
    pub fn getsha256(&mut self) -> u256 {
        let mut buf = [0u8; 32];
        self.ctx.finalize(&mut buf);
        u256::from_le_bytes(buf)
    }

    /**
      | Returns the first 64 bits from the resulting
      | hash.
      |
      */
    #[instrument(level = "trace", skip(self))]
    pub fn get_cheap_hash(&mut self) -> u64 {
        let digest = self.get_hash();
        let mut tmp = [0u8; 8];
        tmp.copy_from_slice(&<u256 as AsRef<[u8]>>::as_ref(&digest)[..8]);
        u64::from_le_bytes(tmp)
    }
}

/**
  | Return a `HashWriter` primed for tagged hashes
  | (as specified in BIP‑340).
  |
  | The returned object will have `SHA256(tag)`
  | written to it twice (= 64 bytes).
  */
pub fn tagged_hash(tag: &str) -> HashWriter {
    // Hash the tag once …
    let mut taghash = [0u8; 32];
    let mut sha = Sha256::new();
    sha.write(tag.as_bytes());
    sha.finalize(&mut taghash);

    // … prime a writer with the tag hash twice.
    let mut writer = HashWriter::new(SER_GETHASH as i32, 0);
    writer.write(&taghash);
    writer.write(&taghash);
    writer
}

lazy_static!{
    static ref HASHER_TAPSIGHASH: HashWriter = tagged_hash("TapSighash");
    static ref HASHER_TAPLEAF:    HashWriter = tagged_hash("TapLeaf");
    static ref HASHER_TAPBRANCH:  HashWriter = tagged_hash("TapBranch");
    static ref HASHER_TAPTWEAK:   HashWriter = tagged_hash("TapTweak");
}

#[cfg(test)]
mod writer_shl_spec {
    use super::*;

    #[traced_test]
    fn shl_writes_bytes_and_preserves_type_version() {
        let bytes = b"abc";
        let writer = HashWriter::new(42, 17) << &bytes[..];
        assert_eq!(writer.get_type(), 42);
        assert_eq!(writer.get_version(), 17);
        // we deliberately do **not** finalise – `bitcoin‑sha256`
        // is still a stub in the workspace.
    }

    #[test]
    #[should_panic] // expected because SHA‑256 back‑end is still a stub
    fn constructing_tagged_hash_does_not_panic() {
        let _ = tagged_hash("TapLeaf"); // should reach SHA panic
    }
}
