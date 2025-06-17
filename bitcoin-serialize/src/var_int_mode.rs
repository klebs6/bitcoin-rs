// ---------------- [ File: bitcoin-serialize/src/var_int_mode.rs ]
crate::ix!();

/**
  | Mode for encoding VarInts.
  | 
  | Currently there is no support for signed
  | encodings. The default mode will not
  | compile with signed values, and the
  | legacy "nonnegative signed" mode will
  | accept signed values, but improperly
  | encode and decode them if they are negative.
  | In the future, the Default mode could
  | be extended to support negative numbers
  | in a backwards compatible way, and additional
  | modes could be added to support different
  | varint formats (e.g. zigzag encoding).
  |
  */
#[derive(Clone,Debug,ConstParamTy,PartialEq,Eq)]
pub enum VarIntMode { 
    Default, 
    NonNegativeSigned 
}

#[inline]
pub fn get_size_of_var_int<I, const Mode: VarIntMode>(mut n: I) -> u32
where
    (): ModeConstraint<Mode, I>,
    I: Into<u128> + Copy,
{
    let mut n128: u128 = n.into();
    let mut count = 1;
    while n128 > 0x7F {
        n128 = (n128 >> 7) - 1;
        count += 1;
    }
    count
}

pub fn write_var_int<Stream, I, const Mode: VarIntMode>(os: &mut Stream, n: I)
where
    (): ModeConstraint<Mode, I>,
    Stream: Write,
    I: Into<u128> + Copy,
{
    let mut val: u128 = n.into();
    let mut bytes = Vec::<u8>::new();

    loop {
        bytes.push(((val & 0x7F) as u8) | if bytes.is_empty() { 0 } else { 0x80 });
        if val <= 0x7F { break; }
        val = (val >> 7) - 1;
    }
    for b in bytes.iter().rev() { ser_writedata8(os, *b); }
}

pub fn read_var_int<Stream, I, const Mode: VarIntMode>(is: &mut Stream) -> I
where
    (): ModeConstraint<Mode, I>,
    Stream: Read,
    I: TryFrom<u128>,
    <I as TryFrom<u128>>::Error: core::fmt::Debug,
{
    let mut n: u128 = 0;
    loop {
        let ch = ser_readdata8(is);
        if n > (u128::MAX >> 7) { panic!("ReadVarInt(): size too large"); }
        n = (n << 7) | u128::from(ch & 0x7F);
        if ch & 0x80 != 0 {
            if n == u128::MAX { panic!("ReadVarInt(): size too large"); }
            n += 1;
        } else {
            break;
        }
    }
    I::try_from(n).expect("ReadVarInt(): value does not fit target type")
}
