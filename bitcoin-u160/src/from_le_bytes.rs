// ---------------- [ File: bitcoin-u160/src/from_le_bytes.rs ]
crate::ix!();

impl u160 {
    /// Construct a `u160` directly from a **little‑endian** 20‑byte array.
    ///
    /// This mirrors the standard‑library convention (`u32::from_le_bytes`, etc.)
    /// and simply forwards to the existing `from_bytes_20` constructor,
    /// emitting a `trace!` for observability.
    #[inline]
    pub fn from_le_bytes(mut bytes: [u8; 20]) -> Self {
        tracing::trace!(
            "u160::from_le_bytes ⇒ {:02X?}",
            bytes
        );
        // Convert caller-provided little-endian bytes into the blob's
        // canonical big-endian representation (match u256 behavior).
        bytes.reverse();
        Self::from_bytes_20(bytes)
    }
}

#[cfg(test)]
mod u160_from_le_bytes_spec {
    use super::*;

    /// Ensure `from_le_bytes` round‑trips the data verbatim.
    #[traced_test]
    fn round_trip_le_bytes() {
        info!("Verifying u160::from_le_bytes → as_slice() round‑trip.");

        let original: [u8; 20] = [
            0x01, 0x02, 0x03, 0x04, 0x05,
            0x06, 0x07, 0x08, 0x09, 0x0A,
            0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14,
        ];
        let value = u160::from_le_bytes(original);
        trace!("Constructed value: {}", value.to_string());

        assert_eq!(
            value.as_slice(),
            &original,
            "Little‑endian bytes must be stored verbatim"
        );
    }
}
