// ---------------- [ File: bitcoin-bitstream/src/dummy_deserialize_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/streams.h]

/**
  | Dummy data type to identify deserializing
  | constructors.
  | 
  | By convention, a constructor of a type
  | T with signature
  | 
  | template <typename Stream> T::T(deserialize_type,
  | Stream& s)
  | 
  | is a deserializing constructor, which
  | builds the type by deserializing it
  | from s. If T contains const fields, this
  | is likely the only way to do so.
  |
  */
pub struct DeserializeType {}

lazy_static!{
    /*
    constexpr deserialize_type deserialize {}
    */
}
