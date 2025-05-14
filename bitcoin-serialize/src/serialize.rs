// ---------------- [ File: bitcoin-serialize/src/serialize.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/serialize.h]

/**
  | Map a value x that is uniformly distributed in
  | the range [0, 2^64) to a value uniformly
  | distributed in [0, n) by returning the upper 64
  | bits of x * n.
  |
  | See:
  | https://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/
  */
pub fn map_into_range(x: u64, n: u64) -> u64 {
    
    #[cfg(__SIZEOF_INT128__)]
    {
        return (x as u128 * n as u128) >> 64;
    }


    // To perform the calculation on 64-bit
    // numbers without losing the result to
    // overflow, split the numbers into the
    // most significant and least significant
    // 32 bits and perform multiplication
    // piece-wise.
    //
    // See:
    // https://stackoverflow.com/a/26855440
    let x_hi:    u64 = x >> 32;;
    let x_lo:    u64 = x & 0xFFFFFFFF;
    let n_hi:    u64 = n >> 32;
    let n_lo:    u64 = n & 0xFFFFFFFF;
    let ac:      u64 = x_hi * n_hi;
    let ad:      u64 = x_hi * n_lo;
    let bc:      u64 = x_lo * n_hi;
    let bd:      u64 = x_lo * n_lo;
    let mid34:   u64 = (bd >> 32) + (bc & 0xFFFFFFFF) + (ad & 0xFFFFFFFF);
    let upper64: u64 = ac + (bc >> 32) + (ad >> 32) + (mid34 >> 32);

    upper64
}

/**
  | The maximum size of a serialized object
  | in bytes or number of elements (for eg
  | vectors) when the size is encoded as
  | CompactSize.
  |
  */
pub const MAX_SIZE: u64 = 0x02000000;

/**
  | Maximum amount of memory (in bytes)
  | to allocate at once when deserializing
  | vectors.
  |
  */
pub const MAX_VECTOR_ALLOCATE: u32 = 5000000;

/**
  | Safely convert odd char pointer types
  | to standard ones.
  |
  */
#[inline] pub fn char_cast(c: *mut u8) -> *mut u8 {
    
    todo!();
        /*
            return (char*)c;
        */
}

/**
  | Lowest-level serialization and conversion.
  | 
  | -----------
  | @note
  | 
  | Sizes of these types are verified in
  | the tests
  |
  */
#[inline] pub fn ser_writedata8<Stream>(
        s:   &mut Stream,
        obj: u8)  {

    todo!();
        /*
            s.write((char*)&obj, 1);
        */
}

#[inline] pub fn ser_writedata16<Stream>(
        s:   &mut Stream,
        obj: u16)  {

    todo!();
        /*
            obj = htole16(obj);
        s.write((char*)&obj, 2);
        */
}

#[inline] pub fn ser_writedata_16be<Stream>(
        s:   &mut Stream,
        obj: u16)  {

    todo!();
        /*
            obj = htobe16(obj);
        s.write((char*)&obj, 2);
        */
}

#[inline] pub fn ser_writedata32<Stream>(
        s:   &mut Stream,
        obj: u32)  {

    todo!();
        /*
            obj = htole32(obj);
        s.write((char*)&obj, 4);
        */
}

#[inline] pub fn ser_writedata_32be<Stream>(
        s:   &mut Stream,
        obj: u32)  {

    todo!();
        /*
            obj = htobe32(obj);
        s.write((char*)&obj, 4);
        */
}

#[inline] pub fn ser_writedata64<Stream>(
        s:   &mut Stream,
        obj: u64)  {

    todo!();
        /*
            obj = htole64(obj);
        s.write((char*)&obj, 8);
        */
}

#[inline] pub fn ser_readdata8<Stream>(s: &mut Stream) -> u8 {

    todo!();
        /*
            uint8_t obj;
        s.read((char*)&obj, 1);
        return obj;
        */
}

#[inline] pub fn ser_readdata16<Stream: std::io::Read>(s: &mut Stream) -> u16 {

    todo!();
    /*
    let mut obj: u16 = 0;

    s.read(&mut obj as *mut u16 as *mut u8, 2);

    le_16toh(obj)
    */
}

#[inline] pub fn ser_readdata_16be<Stream: std::io::Read>(s: &mut Stream) -> u16 {

    todo!();

    /*
    let mut obj: u16 = 0;

    s.read(&mut obj as *mut u16 as *mut u8, 2);

    be_16toh(obj)
    */
}

#[inline] pub fn ser_readdata32<Stream>(s: &mut Stream) -> u32 {

    todo!();
        /*
            uint32_t obj;
        s.read((char*)&obj, 4);
        return le32toh(obj);
        */
}

#[inline] pub fn ser_readdata_32be<Stream>(s: &mut Stream) -> u32 {

    todo!();
        /*
            uint32_t obj;
        s.read((char*)&obj, 4);
        return be32toh(obj);
        */
}

#[inline] pub fn ser_readdata64<Stream>(s: &mut Stream) -> u64 {

    todo!();
        /*
            uint64_t obj;
        s.read((char*)&obj, 8);
        return le64toh(obj);
        */
}

/*
  | Templates for serializing to anything
  | that looks like a stream, i.e. anything
  | that supports .read(char*, size_t)
  | and .write(char*, size_t)
  |
  */
// primary actions
pub const SER_NETWORK: i32 = 1 << 0;
pub const SER_DISK:    i32 = 1 << 1;
pub const SER_GETHASH: i32 = 1 << 2;

/**
  | Convert the reference base type to X,
  | without changing constness or reference
  | type.
  |
  */
pub fn read_write_as_helper<X>(x: &mut X) -> &mut X {
    x
}

macro_rules! readwrite {
    ($($arg:ident),*) => {
        /*
                (::SerReadWriteMany(s, ser_action, __VA_ARGS__))
        */
    }
}

macro_rules! readwriteas {
    ($type:ident, $obj:ident) => {
        /*
                (::SerReadWriteMany(s, ser_action, ReadWriteAsHelper<type>(obj)))
        */
    }
}

macro_rules! ser_read {
    ($obj:ident, $code:ident) => {
        /*
                ::SerRead(s, ser_action, obj, [&](Stream& s, typename std::remove_const<Type>::type& obj) { code; })
        */
    }
}

macro_rules! ser_write {
    ($obj:ident, $code:ident) => {
        /*
                ::SerWrite(s, ser_action, obj, [&](Stream& s, const Type& obj) { code; })
        */
    }
}

/**
  | Implement the Ser and Unser methods
  | needed for implementing a formatter
  | (see Using below).
  | 
  | Both Ser and Unser are delegated to a
  | single static method SerializationOps,
  | which is polymorphic in the serialized/deserialized
  | type (allowing it to be const when serializing,
  | and non-const when deserializing).
  | 
  | Example use:
  | 
  | -----------
  | @code
  | 
  | struct FooFormatter {
  |   FORMATTER_METHODS(Class, obj) { READWRITE(obj.val1, VARINT(obj.val2)); }
  | }
  |
  | would define a class FooFormatter that
  | defines a serialization of Class objects
  | consisting of serializing its val1
  | member using the default serialization,
  | and its val2 member using
  | 
  | VARINT serialization. That FooFormatter
  | can then be used in statements like
  | 
  | READWRITE(Using<FooFormatter>(obj.bla)).
  |
  */
macro_rules! formatter_methods {
    ($cls:ident, $obj:ident) => {
        /*
        
            template<typename Stream> 
            static c_void Ser(Stream& s, const cls& obj) { SerializationOps(obj, s, CSerActionSerialize()); } 
            template<typename Stream> 
            static c_void Unser(Stream& s, cls& obj) { SerializationOps(obj, s, CSerActionUnserialize()); } 
            template<typename Stream, typename Type, typename Operation> 
            static inline c_void SerializationOps(Type& obj, Stream& s, Operation ser_action) 
        */
    }
}

/**
  | Implement the Serialize and Unserialize
  | methods by delegating to a single templated
  | static method that takes the to-be-(de)serialized
  | object as a parameter.
  | 
  | This approach has the advantage that
  | the constness of the object becomes
  | a template parameter, and thus allows
  | a single implementation that sees the
  | object as const for serializing and
  | non-const for deserializing, without
  | casts.
  |
  */
macro_rules! serialize_methods {
    ($cls:ident, $obj:ident) => {
        /*
        
            template<typename Stream>                                                       
            c_void Serialize(Stream& s) const                                                 
            {                                                                               
                const_assert(std::is_same<const cls&, decltype(*this)>::value, "Serialize type mismatch"); 
                Ser(s, *this);                                                              
            }                                                                               
            template<typename Stream>                                                       
            c_void Unserialize(Stream& s)                                                     
            {                                                                               
                const_assert(std::is_same<cls&, decltype(*this)>::value, "Unserialize type mismatch"); 
                Unser(s, *this);                                                            
            }                                                                               
            FORMATTER_METHODS(cls, obj)
        */
    }
}

pub trait Serialize<Stream> {
    fn serialize(&self, s: &mut Stream);
}

impl<Stream> Serialize<Stream> for i8 {
    #[inline] fn serialize(&self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata8(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for u8 {
    #[inline] fn serialize(&self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata8(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for i16 {

    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata16(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for u16 {
    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata16(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for i32 {
    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata32(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for u32 {
    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata32(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for i64 {

    #[inline] fn serialize(
            &self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata64(s, a);
            */
    }
}

impl<Stream> Serialize<Stream> for u64 {

    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                ser_writedata64(s, a);
            */
    }
}

impl<Stream, const N: usize> Serialize<Stream> for [u8; N] {
    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                s.write(a, N);
            */
    }
}

impl<Stream> Serialize<Stream> for &[u8] {
    #[inline] fn serialize(&self, s: &mut Stream)  {

        todo!();
            /*
                s.write(CharCast(span.data()), span.size());
            */
    }
}

pub trait Unserialize<Stream> {
    fn unserialize(&self, s: &mut Stream);
}

impl<Stream> Unserialize<Stream> for i8 {

    #[inline] fn unserialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata8(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for u8 {
    #[inline] fn unserialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata8(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for i16 {
    #[inline] fn unserialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata16(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for u16 {
    #[inline] fn unserialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata16(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for i32 {
    #[inline] fn unserialize(&self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata32(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for u32 {
    #[inline] fn unserialize(&self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata32(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for i64 {
    #[inline] fn unserialize(&self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata64(s);
            */
    }
}

impl<Stream> Unserialize<Stream> for u64 {
    #[inline] fn unserialize(&self, s: &mut Stream)  {

        todo!();
            /*
                a = ser_readdata64(s);
            */
    }
}

impl<Stream,const N: usize> Unserialize<Stream> for [u8;N] {

    #[inline] fn unserialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                s.read(a, N);
            */
    }
}

impl<Stream> Unserialize<Stream> for &[u8] {
    #[inline] fn unserialize(
        &self, s:    &mut Stream)  {

        todo!();
            /*
                s.read(CharCast(span.data()), span.size());
            */
    }
}

impl<Stream> Serialize<Stream> for bool {
    #[inline] fn serialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                uint8_t f = a; ser_writedata8(s, f);
            */
    }
}

impl<Stream> Unserialize<Stream> for bool {
    #[inline] fn unserialize(
        &self, s: &mut Stream)  {

        todo!();
            /*
                uint8_t f = ser_readdata8(s); a = f;
            */
    }
}

/**
  | Compact Size
  | 
  | Size < 253 -- 1 byte
  | 
  | Size <= USHRT_MAX -- 3 bytes (253 + 2 bytes)
  | 
  | Size <= UINT_MAX -- 5 bytes (254 + 4 bytes)
  | 
  | Size > UINT_MAX -- 9 bytes (255 + 8 bytes)
  |
  */
#[inline] pub fn get_size_of_compact_size(n_size: u64) -> u32 {
    
    todo!();
        /*
            if (nSize < 253)             return sizeof(unsigned char);
        else if (nSize <= std::numeric_limits<uint16_t>::max()) return sizeof(unsigned char) + sizeof(uint16_t);
        else if (nSize <= std::numeric_limits<unsigned int>::max())  return sizeof(unsigned char) + sizeof(unsigned int);
        else                         return sizeof(unsigned char) + sizeof(uint64_t);
        */
}

pub fn write_compact_size<Stream>(
        os:     &mut Stream,
        n_size: u64)  {

    todo!();
        /*
            if (nSize < 253)
        {
            ser_writedata8(os, nSize);
        }
        else if (nSize <= std::numeric_limits<uint16_t>::max())
        {
            ser_writedata8(os, 253);
            ser_writedata16(os, nSize);
        }
        else if (nSize <= std::numeric_limits<unsigned int>::max())
        {
            ser_writedata8(os, 254);
            ser_writedata32(os, nSize);
        }
        else
        {
            ser_writedata8(os, 255);
            ser_writedata64(os, nSize);
        }
        return;
        */
}

/**
  | Decode a CompactSize-encoded variable-length
  | integer.
  | 
  | As these are primarily used to encode
  | the size of vector-like serializations,
  | by default a range check is performed.
  | When used as a generic number encoding,
  | range_check should be set to false.
  |
  */
pub fn read_compact_size<Stream>(
        is:          &mut Stream,
        range_check: Option<bool>) -> u64 {

    let range_check: bool = range_check.unwrap_or(true);
    todo!();
        /*
            uint8_t chSize = ser_readdata8(is);
        uint64_t nSizeRet = 0;
        if (chSize < 253)
        {
            nSizeRet = chSize;
        }
        else if (chSize == 253)
        {
            nSizeRet = ser_readdata16(is);
            if (nSizeRet < 253)
                throw std::ios_base::failure("non-canonical ReadCompactSize()");
        }
        else if (chSize == 254)
        {
            nSizeRet = ser_readdata32(is);
            if (nSizeRet < 0x10000u)
                throw std::ios_base::failure("non-canonical ReadCompactSize()");
        }
        else
        {
            nSizeRet = ser_readdata64(is);
            if (nSizeRet < 0x100000000ULL)
                throw std::ios_base::failure("non-canonical ReadCompactSize()");
        }
        if (range_check && nSizeRet > MAX_SIZE) {
            throw std::ios_base::failure("ReadCompactSize(): size too large");
        }
        return nSizeRet;
        */
}

/*
  | Variable-length integers: bytes are
  | a MSB base-128 encoding of the number.
  | 
  | The high bit in each byte signifies whether
  | another digit follows. To make sure
  | the encoding is one-to-one, one is subtracted
  | from all but the last digit.
  | 
  | Thus, the byte sequence a[] with length
  | len, where all but the last byte has bit
  | 128 set, encodes the number:
  | 
  | -----------
  | @code
  | 
  | (a[len-1] & 0x7F) + sum(i=1..len-1, 128^i*((a[len-i-1] & 0x7F)+1))
  | 
  | Properties:
  | 
  | - Very small (0-127: 1 byte, 128-16511:
  | 2 bytes, 16512-2113663: 3 bytes)
  | 
  | - Every integer has exactly one encoding
  | 
  | - Encoding does not depend on size of
  | original integer type
  | 
  | - No redundancy: every (infinite) byte
  | sequence corresponds to a list of encoded
  | integers.
  | ----------
  | @code
  | 
  | 0:         [0x00]  256:        [0x81 0x00]
  | 1:         [0x01]  16383:      [0xFE 0x7F]
  | 127:       [0x7F]  16384:      [0xFF 0x00]
  | 128:  [0x80 0x00]  16511:      [0xFF 0x7F]
  | 255:  [0x80 0x7F]  65535: [0x82 0xFE 0x7F]
  | 2^32:           [0x8E 0xFE 0xFE 0xFF 0x00]
  |
  */

/**
  | Mode for encoding VarInts.
  | 
  | Currently there is no support for signed
  | encodings. The default mode will not
  | compile with signed values, and the
  | legacy "nonnegative signed" mode will
  | accept signed values, but improperly
  | encode and decode them if they are negative.
  | In the future, the DEFAULT mode could
  | be extended to support negative numbers
  | in a backwards compatible way, and additional
  | modes could be added to support different
  | varint formats (e.g. zigzag encoding).
  |
  */
#[derive(ConstParamTy,PartialEq,Eq)]
pub enum VarIntMode { 
    DEFAULT, 
    NONNEGATIVE_SIGNED 
}

pub struct CheckVarIntMode<const Mode: VarIntMode> {

}

impl<const Mode: VarIntMode> CheckVarIntMode<Mode> {

    pub fn new<I>() -> Self {
    
        todo!();
        /*

            const_assert(Mode != VarIntMode::DEFAULT || std::is_unsigned<I>::value, "Unsigned type required with mode DEFAULT.");
            const_assert(Mode != VarIntMode::NONNEGATIVE_SIGNED || std::is_signed<I>::value, "Signed type required with mode NONNEGATIVE_SIGNED.");
        */
    }
}

#[inline] pub fn get_size_of_var_int<I, const Mode: VarIntMode>(n: I) -> u32 {

    todo!();
        /*
            CheckVarIntMode<Mode, I>();
        int nRet = 0;
        while(true) {
            nRet++;
            if (n <= 0x7F)
                break;
            n = (n >> 7) - 1;
        }
        return nRet;
        */
}

pub fn write_var_int<Stream, I, const Mode: VarIntMode>(
        os: &mut Stream,
        n:  I)  {

    todo!();
        /*
            CheckVarIntMode<Mode, I>();
        unsigned char tmp[(sizeof(n)*8+6)/7];
        int len=0;
        while(true) {
            tmp[len] = (n & 0x7F) | (len ? 0x80 : 0x00);
            if (n <= 0x7F)
                break;
            n = (n >> 7) - 1;
            len++;
        }
        do {
            ser_writedata8(os, tmp[len]);
        } while(len--);
        */
}

pub fn read_var_int<Stream, I, const Mode: VarIntMode>(is: &mut Stream) -> I {

    todo!();
        /*
            CheckVarIntMode<Mode, I>();
        I n = 0;
        while(true) {
            unsigned char chData = ser_readdata8(is);
            if (n > (std::numeric_limits<I>::max() >> 7)) {
               throw std::ios_base::failure("ReadVarInt(): size too large");
            }
            n = (n << 7) | (chData & 0x7F);
            if (chData & 0x80) {
                if (n == std::numeric_limits<I>::max()) {
                    throw std::ios_base::failure("ReadVarInt(): size too large");
                }
                n++;
            } else {
                return n;
            }
        }
        */
}

/**
  | Simple wrapper class to serialize objects
  | using a formatter; used by Using().
  |
  */
pub struct Wrapper<'a, T> {
    object: &'a T,
}

impl<'a, T> Wrapper<'a, T> {

    pub fn new(obj: &'a T) -> Self {
    
        todo!();
        /*
        : object(obj),
        */
    }
    
    pub fn serialize<Formatter,Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            Formatter().Ser(s, m_object);
        */
    }
    
    pub fn unserialize<Formatter,Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            Formatter().Unser(s, m_object);
        */
    }
}

/**
  | Cause serialization/deserialization
  | of an object to be done using a specified
  | formatter class.
  | 
  | To use this, you need a class Formatter
  | that has public functions Ser(stream,
  | const object&) for serialization,
  | and Unser(stream, object&) for deserialization.
  | Serialization routines (inside
  | 
  | READWRITE, or directly with << and >>
  | operators), can then use Using<Formatter>(object).
  | 
  | This works by constructing a Wrapper<Formatter,
  | T>-wrapped version of object, where
  | T is const during serialization, and
  | non-const during deserialization,
  | which maintains const correctness.
  |
  */
#[inline] pub fn using<'a, Formatter, T>(t: T) -> Wrapper<'a, &'a mut T> {

    todo!();
        /*
            return Wrapper<Formatter, T&>(t);
        */
}

#[macro_export] macro_rules! varint_mode {
    ($obj:ident, $mode:ident) => {
        VarIntFormatter::<$mode>::new($obj)
    }
}

#[macro_export] macro_rules! varint {
    ($obj:ident) => {
        VarIntFormatter::<VarIntMode::DEFAULT>::new($obj)
    }
}

#[macro_export] macro_rules! compactsize {
    ($obj:ident) => {
        CompactSizeFormatter::<true>::new($obj)
    }
}

#[macro_export] macro_rules! limited_string {
    ($obj:expr, $n:ident) => {
        LimitedStringFormatter::<$n>{ item: $obj }
    }
}

/**
  | Serialization wrapper class for integers
  | in VarInt format.
  |
  */
pub struct VarIntFormatter<const Mode: VarIntMode> {

}

impl<const Mode: VarIntMode> VarIntFormatter<Mode> {
    
    pub fn ser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: I)  {
    
        todo!();
        /*
            WriteVarInt<Stream,Mode,typename std::remove_cv<I>::type>(s, v);
        */
    }
    
    
    pub fn unser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: &mut I)  {
    
        todo!();
        /*
            v = ReadVarInt<Stream,Mode,typename std::remove_cv<I>::type>(s);
        */
    }
}

pub struct If<const B: bool>;

pub trait True { }

impl True for If<{true}> { }

pub const fn inclusive_range_1_to_8<const Bytes: i32>() -> bool {
    Bytes > 0 && Bytes <= 8 
}

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

/**
  | Formatter for integers in CompactSize
  | format.
  |
  */
pub struct CompactSizeFormatter<const RangeCheck: bool> { }

impl<const RangeCheck: bool> CompactSizeFormatter<RangeCheck> {
    
    pub fn unser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: &mut I)  {
    
        todo!();
        /*
            uint64_t n = ReadCompactSize<Stream>(s, RangeCheck);
            if (n < std::numeric_limits<I>::min() || n > std::numeric_limits<I>::max()) {
                throw std::ios_base::failure("CompactSize exceeds limit of type");
            }
            v = n;
        */
    }
    
    
    pub fn ser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: I)  {
    
        todo!();
        /*
            const_assert(std::is_unsigned<I>::value, "CompactSize only supported for unsigned integers");
            const_assert(std::numeric_limits<I>::max() <= std::numeric_limits<uint64_t>::max(), "CompactSize only supports 64-bit integers and below");

            WriteCompactSize<Stream>(s, v);
        */
    }
}

///-------------------------------
pub struct LimitedStringFormatter<'a, const Limit: usize> { 
    pub item: &'a mut String,
}

impl<'a, const Limit: usize> LimitedStringFormatter<'a, Limit> {
    
    pub fn unser<Stream>(&mut self, 
        s: &mut Stream,
        v: &mut String)  {
    
        todo!();
        /*
            size_t size = ReadCompactSize(s);
            if (size > Limit) {
                throw std::ios_base::failure("String length limit exceeded");
            }
            v.resize(size);
            if (size != 0) s.read((char*)v.data(), size);
        */
    }
    
    
    pub fn ser<Stream>(&mut self, 
        s: &mut Stream,
        v: &String)  {
    
        todo!();
        /*
            s << v;
        */
    }
}

/**
  | Formatter to serialize/deserialize
  | vector elements using another formatter
  | 
  | Example:
  | 
  | -----------
  | @code
  | 
  | struct X {
  |   std::vector<uint64_t> v;
  |   SERIALIZE_METHODS(X, obj) { READWRITE(Using<VectorFormatter<VarInt>>(obj.v)); }
  | };
  |
  | will define a struct that contains a
  | vector of uint64_t, which is serialized
  | as a vector of VarInt-encoded integers.
  | 
  | V is not required to be an std::vector
  | type. It works for any class that exposes
  | a value_type, size, reserve, emplace_back,
  | back, and const iterators.
  |
  */
pub struct VectorFormatter<'a,T> {
    item: &'a mut T,
}

impl<'a,T> VectorFormatter<'a,T> {
    
    pub fn ser<Formatter,Stream, V>(&mut self, 
        s: &mut Stream,
        v: &V)  {
    
        todo!();
        /*
            Formatter formatter;
            WriteCompactSize(s, v.size());
            for (const typename V::value_type& elem : v) {
                formatter.Ser(s, elem);
            }
        */
    }
    
    pub fn unser<Formatter,Stream, V>(&mut self, 
        s: &mut Stream,
        v: &mut V)  {
    
        todo!();
        /*
            Formatter formatter;
            v.clear();
            size_t size = ReadCompactSize(s);
            size_t allocated = 0;
            while (allocated < size) {
                // For DoS prevention, do not blindly allocate as much as the stream claims to contain.
                // Instead, allocate in 5MiB batches, so that an attacker actually needs to provide
                // X MiB of data to make us allocate X+5 Mib.
                const_assert(sizeof(typename V::value_type) <= MAX_VECTOR_ALLOCATE, "Vector element size too large");
                allocated = std::min(size, allocated + MAX_VECTOR_ALLOCATE / sizeof(typename V::value_type));
                v.reserve(allocated);
                while (v.size() < allocated) {
                    v.emplace_back();
                    formatter.Unser(s, v.back());
                }
            }
        }{
        */
    }
}

/**
  | If none of the specialized versions
  | above matched, default to calling member
  | function.
  |
  */
/*
   impls conflict with concrete type impls

impl<Stream,T> Serialize<Stream> for T {
    #[inline] fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                a.Serialize(os);
            */
    }
}
*/

/* impls conflict with concrete type
 * impls
impl<Stream,T> Unserialize<Stream>  for T {
    #[inline] fn unserialize(&self, is: &mut Stream)  {
        todo!();
            /*
                a.Unserialize(is);
            */
    }
}
*/

/**
  | Default formatter. Serializes objects
  | as themselves.
  | 
  | The vector/prevector serialization
  | code passes this to VectorFormatter
  | to enable reusing that logic. It shouldn't
  | be needed elsewhere.
  |
  */
pub struct DefaultFormatter<'a,T> { 
    item: &'a mut T,
}

impl<'a,T> DefaultFormatter<'a,T> {
    
    pub fn ser<Stream>(
        s: &mut Stream,
        t: &T)  {
    
        todo!();
        /*
            Serialize(s, t);
        */
    }
    
    pub fn unser<Stream>(
        s: &mut Stream,
        t: &mut T)  {
    
        todo!();
        /*
            Unserialize(s, t);
        */
    }
}

impl<Stream> Serialize<Stream> for String {

    fn serialize(&self, os:   &mut Stream)  {

        todo!();
            /*
                WriteCompactSize(os, self.size());
            if (!self.empty())
                os.write((char*)self.data(), self.size() * sizeof(C));
            */
    }
}

impl<Stream> Unserialize<Stream> for String {

    fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
                unsigned int nSize = ReadCompactSize(is);
            str.resize(nSize);
            if (nSize != 0)
                is.read((char*)str.data(), nSize * sizeof(C));
            */
    }
}

impl<Stream, T: Default, const N: usize> Serialize<Stream> for PreVector<T,N> {

    #[inline] fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
            fn _serialize_impl_u8<Stream>(
                    os: &mut Stream,
                    v:  &PreVector<T,N>,
                    _2: &u8)  {

                todo!();
                    /*
                        WriteCompactSize(os, v.size());
                    if (!v.empty())
                        os.write((char*)v.data(), v.size() * sizeof(T));
                    */
            }

            fn _serialize_impl<Stream,V>(
                    os: &mut Stream,
                    v:  &PreVector<T,N>,
                    _2: &V)  {

                todo!();
                    /*
                        Serialize(os, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }
                Serialize_impl(os, v, T());
            */
    }
}

impl<Stream, T: Default, const N: usize> Unserialize<Stream> for PreVector<T,N> {

    #[inline] fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
            #[inline] fn _unserialize_impl_u8<Stream>(
                    is: &mut Stream,
                    v:  &mut PreVector<T,N>,
                    _2: &u8)  {

                todo!();
                    /*
                        // Limit size per read so bogus size value won't cause out of memory
                    v.clear();
                    unsigned int nSize = ReadCompactSize(is);
                    unsigned int i = 0;
                    while (i < nSize)
                    {
                        unsigned int blk = std::min(nSize - i, (unsigned int)(1 + 4999999 / sizeof(T)));
                        v.resize_uninitialized(i + blk);
                        is.read((char*)&v[i], blk * sizeof(T));
                        i += blk;
                    }
                    */
            }

            #[inline] fn _unserialize_impl<Stream,V>(
                    is: &mut Stream,
                    v:  &mut PreVector<T,N>,
                    _2: &V)  {

                todo!();
                    /*
                        Unserialize(is, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }

                Unserialize_impl(is, v, T());
            */
    }
}

impl<Stream, T, A: Allocator> Serialize<Stream> for Vec<T,A> {

    #[inline] fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
            #[inline] fn _serialize_impl_u8<Stream>(
                    os: &mut Stream,
                    v:  &Vec<T,A>,
                    _2: &u8)  {

                todo!();
                    /*
                        WriteCompactSize(os, v.size());
                    if (!v.empty())
                        os.write((char*)v.data(), v.size() * sizeof(T));
                    */
            }

            #[inline] fn _serialize_impl_bool<Stream>(
                    os: &mut Stream,
                    v:  &Vec<T,A>,
                    _2: &bool)  {

                todo!();
                    /*
                        // A special case for std::vector<bool>, as dereferencing
                    // std::vector<bool>::const_iterator does not result in a const bool&
                    // due to std::vector's special casing for bool arguments.
                    WriteCompactSize(os, v.size());
                    for (bool elem : v) {
                        ::Serialize(os, elem);
                    }
                    */
            }

            #[inline] fn _serialize_impl<Stream,V>(
                    os: &mut Stream,
                    v:  &Vec<T,A>,
                    _2: &V)  {

                todo!();
                    /*
                        Serialize(os, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }
                Serialize_impl(os, v, T());
            */
    }
}

impl<Stream,T,A: Allocator> Unserialize<Stream> for Vec<T,A> {

    #[inline] fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
            fn _unserialize_impl_u8<Stream>(
                    is: &mut Stream,
                    v:  &mut Vec<T,A>,
                    _2: &u8)  {

                todo!();
                    /*
                        // Limit size per read so bogus size value won't cause out of memory
                    v.clear();
                    unsigned int nSize = ReadCompactSize(is);
                    unsigned int i = 0;
                    while (i < nSize)
                    {
                        unsigned int blk = std::min(nSize - i, (unsigned int)(1 + 4999999 / sizeof(T)));
                        v.resize(i + blk);
                        is.read((char*)&v[i], blk * sizeof(T));
                        i += blk;
                    }
                    */
            }

            fn _unserialize_impl<Stream,V>(
                    is: &mut Stream,
                    v:  &mut Vec<T,A>,
                    _2: &V)  {

                todo!();
                    /*
                        Unserialize(is, Using<VectorFormatter<DefaultFormatter>>(v));
                    */
            }

                Unserialize_impl(is, v, T());
            */
    }
}

impl<Stream,K,T> Serialize<Stream> for (K,T) {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                Serialize(os, item.first);
            Serialize(os, item.second);
            */
    }
}

impl<Stream,K,T> Unserialize<Stream> for (K,T) {
    fn unserialize(&self, is:   &mut Stream)  {

        todo!();
            /*
                Unserialize(is, item.first);
            Unserialize(is, item.second);
            */
    }
}

impl<Stream,K,T> Serialize<Stream> for HashMap<K,T> {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                WriteCompactSize(os, m.size());
            for (const auto& entry : m)
                Serialize(os, entry);
            */
    }
}

impl<Stream,K,V> Unserialize<Stream> for HashMap<K,V> {
    fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
                m.clear();
            unsigned int nSize = ReadCompactSize(is);
            typename std::map<K, V, Pred, A>::iterator mi = m.begin();
            for (unsigned int i = 0; i < nSize; i++)
            {
                std::pair<K, V> item;
                Unserialize(is, item);
                mi = m.insert(mi, item);
            }
            */
    }
}

impl<Stream,K> Serialize<Stream> for HashSet<K> {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                WriteCompactSize(os, m.size());
            for (typename std::set<K, Pred, A>::const_iterator it = m.begin(); it != m.end(); ++it)
                Serialize(os, (*it));
            */
    }
}

impl<Stream,K> Unserialize<Stream> for HashSet<K> {
    fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
                m.clear();
            unsigned int nSize = ReadCompactSize(is);
            typename std::set<K, Pred, A>::iterator it = m.begin();
            for (unsigned int i = 0; i < nSize; i++)
            {
                K key;
                Unserialize(is, key);
                it = m.insert(it, key);
            }
            */
    }
}

impl<Stream,T> Serialize<Stream> for Box<T> {

    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                Serialize(os, *p);
            */
    }
}

impl<Stream,T> Unserialize<Stream> for Box<T> {
    fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
                p.reset(new T(deserialize, is));
            */
    }
}

impl<Stream,T> Serialize<Stream> for Arc<T> {
    fn serialize(&self, os: &mut Stream)  {

        todo!();
            /*
                Serialize(os, *p);
            */
    }

}

impl<Stream,T> Unserialize<Stream> for Arc<T> {
    fn unserialize(&self, is: &mut Stream)  {

        todo!();
            /*
                p = std::make_shared<const T>(deserialize, is);
            */
    }
}

/**
  | Support for SERIALIZE_METHODS and
  | READWRITE macro.
  |
  */
pub struct SerActionSerialize { }

impl SerActionSerialize {
    
    pub fn for_read(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

pub struct SerActionUnserialize { }

impl SerActionUnserialize {
    
    pub fn for_read(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

/**
  | ::GetSerializeSize implementations
  | 
  | Computing the serialized size of objects
  | is done through a special stream object
  | of type CSizeComputer, which only records
  | the number of bytes written to it.
  | 
  | If your Serialize or SerializationOp
  | method has non-trivial overhead for
  | serialization, it may be worthwhile
  | to implement a specialized version
  | for
  | 
  | CSizeComputer, which uses the s.seek()
  | method to record bytes that would be
  | written instead.
  |
  */
pub struct SizeComputer {
    n_size:    usize,
    n_version: i32,
}

impl<T> Shl<&T> for SizeComputer {
    type Output = SizeComputer;
    
    #[inline] fn shl(self, rhs: &T) -> Self::Output {
        todo!();
        /*
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl SizeComputer {
    
    pub const fn new(n_version_in: i32) -> Self {
        Self {
            n_size:    0,
            n_version: n_version_in,
        }
    }
    
    pub fn write(&mut self, 
        psz:    *const u8,
        n_size: usize)  {
        
        todo!();
        /*
            this->nSize += _nSize;
        */
    }

    /**
      | Pretend _nSize bytes are written, without
      | specifying them.
      |
      */
    pub fn seek(&mut self, n_size: usize)  {
        
        todo!();
        /*
            this->nSize += _nSize;
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return nSize;
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
}

pub fn serialize_many_base<Stream>(s: &mut Stream)  { }

pub fn serialize_many<Stream, Arg, Args>(
        s:    &mut Stream,
        arg:  &Arg,
        args: &Args)  {

    todo!();
        /*
            ::Serialize(s, arg);
        ::SerializeMany(s, args...);
        */
}

#[inline] pub fn unserialize_many_base<Stream>(s: &mut Stream)  { }

#[inline] pub fn unserialize_many<Stream, Arg, Args>(
        s:    &mut Stream,
        arg:  Arg,
        args: Args)  {

    todo!();
        /*
            ::Unserialize(s, arg);
        ::UnserializeMany(s, args...);
        */
}

#[inline] pub fn ser_read_write_many_with_action_serialize<Stream, Args>(
        s:          &mut Stream,
        ser_action: SerActionSerialize,
        args:       &Args)  {

    todo!();
        /*
            ::SerializeMany(s, args...);
        */
}

#[inline] pub fn ser_read_write_many_with_action_unserialize<Stream, Args>(
        s:          &mut Stream,
        ser_action: SerActionUnserialize,
        args:       Args)  {

    todo!();
        /*
            ::UnserializeMany(s, args...);
        */
}

#[inline] pub fn ser_read_with_action_serialize<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionSerialize,
        _2:         Type,
        _3:         Fn)  {

    todo!();
        /*
        
        */
}

#[inline] pub fn ser_read<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionUnserialize,
        obj:        Type,
        fn_:        Fn)  {

    todo!();
        /*
            fn(s, std::forward<Type>(obj));
        */
}

#[inline] pub fn ser_write_with_action_serialize<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionSerialize,
        obj:        Type,
        fn_:        Fn)  {

    todo!();
        /*
            fn(s, std::forward<Type>(obj));
        */
}

#[inline] pub fn ser_write_with_action_unserialize<Stream, Type, Fn>(
        s:          &mut Stream,
        ser_action: SerActionUnserialize,
        _2:         Type,
        _3:         Fn)  {

    todo!();
        /*
        
        */
}

#[inline] pub fn write_var_int_with_size_computer<I>(
        s: &mut SizeComputer,
        n: I)  {

    todo!();
        /*
            s.seek(GetSizeOfVarInt<I>(n));
        */
}

#[inline] pub fn write_compact_size_with_size_computer(
        s:      &mut SizeComputer,
        n_size: u64)  {
    
    todo!();
        /*
            s.seek(GetSizeOfCompactSize(nSize));
        */
}

pub fn get_serialize_size<T>(
        t:         &T,
        n_version: Option<i32>) -> usize {

    let n_version: i32 = n_version.unwrap_or(0);
    (SizeComputer::new(n_version) << &t).size()
}

pub fn get_serialize_size_many<T>(
        n_version: i32,
        t:         &T) -> usize {

    todo!();
        /*
            CSizeComputer sc(nVersion);
        SerializeMany(sc, t...);
        return sc.size();
        */
}
