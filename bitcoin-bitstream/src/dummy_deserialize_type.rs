// ---------------- [ File: bitcoin-bitstream/src/dummy_deserialize_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/streams.h]

/// A dummy data type to identify deserializing constructors.
///
/// By convention, a constructor of T with signature:
///    `T::T(deserialize_type, Stream& s)`
/// is a deserializing constructor in C++ code.
pub struct DeserializeType {}

// If you want the "constexpr deserialize {}" analog, you could do:
pub const DESERIALIZE: DeserializeType = DeserializeType {};
