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
#[derive(ConstParamTy,PartialEq,Eq)]
pub enum VarIntMode { 
    Default, 
    NonNegativeSigned 
}

pub struct CheckVarIntMode<const Mode: VarIntMode> {

}

impl<const Mode: VarIntMode> CheckVarIntMode<Mode> {

    pub fn new<I>() -> Self {
    
        todo!();
        /*

            const_assert(Mode != VarIntMode::Default || std::is_unsigned<I>::value, "Unsigned type required with mode Default.");
            const_assert(Mode != VarIntMode::NonNegativeSigned || std::is_signed<I>::value, "Signed type required with mode NonNegativeSigned.");
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
