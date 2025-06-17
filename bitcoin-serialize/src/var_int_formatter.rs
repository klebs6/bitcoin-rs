// ---------------- [ File: bitcoin-serialize/src/var_int_formatter.rs ]
crate::ix!();

/**
  | Serialization wrapper class for integers
  | in VarInt format.
  |
  */
pub struct VarIntFormatter<const Mode: VarIntMode> {

}

impl<const Mode: VarIntMode> VarIntFormatter<Mode> {
    #[inline]
    pub fn ser<Stream, I>(&mut self, s: &mut Stream, v: I)
    where
        Stream: Write,
        I: Into<u128> + Copy + From<u8> + TryInto<u128>,
    {
        write_var_int::<Stream, I, Mode>(s, v);
    }

    #[inline]
    pub fn unser<Stream, I>(&mut self, s: &mut Stream, v: &mut I)
    where
        Stream: Read,
        I: TryFrom<u128> + Copy + Default + std::fmt::Debug,
        <I as TryFrom<u128>>::Error: std::fmt::Debug,
    {
        *v = read_var_int::<Stream, I, Mode>(s);
    }
}
