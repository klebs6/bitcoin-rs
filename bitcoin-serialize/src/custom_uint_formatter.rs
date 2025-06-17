// ---------------- [ File: bitcoin-serialize/src/custom_uint_formatter.rs ]
crate::ix!();

/**
  | Serialization wrapper class for custom
  | integers and enums.
  | 
  | It permits specifying the serialized
  | size (1 to 8 bytes) and endianness.
  | 
  | Use the big endian mode for values that
  | are stored in memory in native byte order,
  | but serialized in big endian notation.
  | This is only intended to implement serializers
  | that are compatible with existing formats,
  | and its use is not recommended for new
  | data structures.
  |
  */
pub struct CustomUintFormatter<'a, T,const Bytes: i32,const BigEndian: bool = false> 
where If<{inclusive_range_1_to_8::<Bytes>()}>: True 
{
    pub item: &'a mut T 
}

impl<'a, T, const Bytes: i32,const BigEndian: bool> CustomUintFormatter<'a, T,Bytes,BigEndian> 
where If<{inclusive_range_1_to_8::<Bytes>()}>: True
{
    pub const MAX: u64 = 0xffffffffffffffff >> (8 * (8 - Bytes));

    pub fn ser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: I)  {
    
        todo!();
        /*
            if (v < 0 || v > MAX) throw std::ios_base::failure("CustomUintFormatter value out of range");
            if (BigEndian) {
                uint64_t raw = htobe64(v);
                s.write(((const char*)&raw) + 8 - Bytes, Bytes);
            } else {
                uint64_t raw = htole64(v);
                s.write((const char*)&raw, Bytes);
            }
        */
    }
    
    
    pub fn unser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: &mut I)  {
    
        todo!();
        /*
            using U = typename std::conditional<std::is_enum<I>::value, std::underlying_type<I>, std::common_type<I>>::type::type;
            const_assert(std::numeric_limits<U>::max() >= MAX && std::numeric_limits<U>::min() <= 0, "Assigned type too small");
            uint64_t raw = 0;
            if (BigEndian) {
                s.read(((char*)&raw) + 8 - Bytes, Bytes);
                v = static_cast<I>(be64toh(raw));
            } else {
                s.read((char*)&raw, Bytes);
                v = static_cast<I>(le64toh(raw));
            }
        */
    }
}

pub type BigEndianFormatter<'a, T, const Bytes: i32> = CustomUintFormatter<'a, T,Bytes,true>;
