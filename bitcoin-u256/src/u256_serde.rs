crate::ix!();

impl Serialize for u256 {
    /// Serialize as exactly 32 bytes in little-endian or big-endian?
    ///
    /// Typically for Bitcoin structures, we store raw bytes in **little-endian** 
    /// if we want to remain consistent with internal usage. 
    /// But you can choose big-endian if that’s your protocol format. 
    /// Here, we’ll do **raw internal** as is (the same memory layout as `BaseBlob<256>`).
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // The simplest approach: just serialize as a 32-byte byte-array
        let bytes = self.as_slice();
        // Return a byte array
        serializer.serialize_bytes(bytes)
    }
}

impl<'de> Deserialize<'de> for u256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct U256Visitor;

        impl<'de> de::Visitor<'de> for U256Visitor {
            type Value = u256;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a 32-byte array representing u256")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<u256, E>
            where
                E: de::Error,
            {
                if v.len() != 32 {
                    return Err(E::invalid_length(v.len(), &self));
                }
                let mut out = u256::default();
                out.as_slice_mut().copy_from_slice(v);
                Ok(out)
            }
        }

        deserializer.deserialize_bytes(U256Visitor)
    }
}
