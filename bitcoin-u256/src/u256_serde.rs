// ---------------- [ File: bitcoin-u256/src/u256_serde.rs ]
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

#[cfg(test)]
mod u256_serde_exhaustive_tests {
    use super::*;
    use serde_test::{
        Token,
        assert_tokens,
        assert_de_tokens_error,
        Configure
    };

    /// We'll define a small helper to produce `'static` slices by leaking them from a `Box`.
    /// This is necessary to satisfy the `'static` lifetime for `Token::Bytes(...)`.
    fn make_static_bytes(data: &[u8]) -> &'static [u8] {
        Box::leak(data.to_vec().into_boxed_slice())
    }

    /// 1) Check simple known patterns => zero, single-nonzero, etc.
    #[test]
    fn test_serialize_deserialize_basic() {
        // (a) Zero => 32 zero bytes
        let z = u256::default();
        let zero_32 = make_static_bytes(&[0u8; 32]);
        assert_tokens(
            &z.readable(), // calls serialize(), giving Token::Bytes
            &[Token::Bytes(zero_32)],
        );

        // (b) Single nonzero => e.g. as_slice_mut()[0] = 0xAB
        let mut x = u256::default();
        x.as_slice_mut()[0] = 0xAB;

        let mut local = [0u8; 32];
        local[0] = 0xAB;
        let static_ref = make_static_bytes(&local);

        let tokens = [Token::Bytes(static_ref)];
        assert_tokens(&x.readable(), &tokens);
    }

    /// 2) Round-trip test: random data => serialize => deserialize => match
    #[test]
    fn test_serialize_deserialize_random_roundtrip() {
        // We'll do 10 random patterns. We'll pick a valid 64-bit seed.
        // Previously, "0xABCDE_9999_FFFF_0000u64" was out of range.
        // Here we choose "0xABCD_9999_0000_0000u64" which is within 64 bits.
        let mut seed = 0xABCD_9999_0000_0000u64;

        for _i in 0..10 {
            // create a 32-byte array from seed
            let mut arr = [0u8; 32];
            for b in arr.iter_mut() {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                *b = (seed >> 24) as u8;
            }

            let mut x = u256::default();
            x.as_slice_mut().copy_from_slice(&arr);

            // Now check round-trip with Token::Bytes
            let static_arr = make_static_bytes(&arr);
            let tokens = [Token::Bytes(static_arr)];
            assert_tokens(&x.readable(), &tokens);
        }
    }

    #[test]
    fn test_deserialize_invalid_length() {
        // (a) only 16 bytes => must fail
        let short_16 = vec![0u8; 16];
        let static_16 = make_static_bytes(&short_16); // your helper to get `'static`
        let tokens_16 = [Token::Bytes(static_16)];

        assert_de_tokens_error::<u256>(
            &tokens_16,
            // updated expected:
            "invalid length 16, expected a 32-byte array representing u256",
        );

        // (b) 33 bytes => must fail
        let long_33 = vec![0xFFu8; 33];
        let static_33 = make_static_bytes(&long_33);
        let tokens_33 = [Token::Bytes(static_33)];

        assert_de_tokens_error::<u256>(
            &tokens_33,
            // updated expected:
            "invalid length 33, expected a 32-byte array representing u256",
        );
    }
}
