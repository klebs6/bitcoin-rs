// ---------------- [ File: bitcoin-serialize/src/var_int_formatter.rs ]
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
crate::ix!();

/**
  | Serialization wrapper class for integers
  | in VarInt format.
  |
  */
pub struct VarIntFormatter<const Mode: VarIntMode> {

}

impl<const Mode: VarIntMode> VarIntFormatter<Mode> {
    /// Helper used by the `varint_mode!` / `varint!` macros:
    ///
    /// ```ignore
    /// READWRITE(varint!(nVersion));
    /// ```
    ///
    /// Returns a [`Wrapper`] tying the supplied `item` to this formatter so
    /// it can be fed directly into the usual `READWRITE` / `Serialize`
    /// machinery.
    #[inline]
    pub fn new<'a, T>(item: &'a mut T) -> crate::wrapper::Wrapper<'a, Self, T> {
        crate::wrapper::Wrapper::new(item)
    }
}

impl<const Mode: VarIntMode> Default for VarIntFormatter<Mode> {
    #[inline]
    fn default() -> Self {
        Self {}
    }
}

/* Blanket implementation so a `VarIntFormatter` **is** a formatter. */
impl<const Mode: VarIntMode, I> ValueFormatter<I> for VarIntFormatter<Mode>
where
    (): ModeConstraint<Mode, I>,
    I: Into<u128>
        + Copy
        + From<u8>
        + TryInto<u128>
        + TryFrom<u128>
        + std::fmt::Debug
        + Default,
    <I as TryFrom<u128>>::Error: std::fmt::Debug,
{
    #[inline]
    fn ser<S: Write>(&mut self, s: &mut S, value: &I) {
        write_var_int::<S, I, Mode>(s, *value);
    }

    #[inline]
    fn unser<S: Read>(&mut self, s: &mut S, value: &mut I) {
        *value = read_var_int::<S, I, Mode>(s);
    }
}
