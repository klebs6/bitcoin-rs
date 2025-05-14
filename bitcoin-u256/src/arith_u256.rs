// ---------------- [ File: bitcoin-u256/src/arith_u256.rs ]
crate::ix!();

/**
  | 256-bit unsigned big integer.
  |
  */
#[derive(Default,Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct ArithU256 {
    base: BaseUInt<256>,
}

unsafe impl Send for ArithU256 {}
unsafe impl Sync for ArithU256 {}

impl From<&BaseUInt<256>> for ArithU256 {

    fn from(b: &BaseUInt<256>) -> Self {
    
        todo!();
        /*
            : BaseUInt<_256>(b)
        */
    }
}

impl From<u64> for ArithU256 {
    
    fn from(b: u64) -> Self {
    
        todo!();
        /*
            : BaseUInt<_256>(b)
        */
    }
}

impl From<&str> for ArithU256 {
    
    fn from(str_: &str) -> Self {
    
        todo!();
        /*
            : BaseUInt<_256>(str)
        */
    }
}

impl MulAssign<u32> for ArithU256 
{
    #[inline] fn mul_assign(&mut self, b32: u32) {
        self.base.mul_assign(b32)
    }
}

impl MulAssign<i64> for ArithU256 
{
    #[inline] fn mul_assign(&mut self, b64: i64) {

        let u: u32 = b64.try_into().unwrap();

        self.base.mul_assign(u)
    }
}

impl MulAssign<&ArithU256> for ArithU256 
{
    
    #[inline] fn mul_assign(&mut self, b: &ArithU256) {
        self.base.mul_assign(&b.base)
    }
}

impl DivAssign<u32> for ArithU256 
{
    #[inline] fn div_assign(&mut self, b32: u32) {
        self.div_assign(b32)
    }
}

impl DivAssign<i64> for ArithU256 
{
    #[inline] fn div_assign(&mut self, b64: i64) {

        let u: u32 = b64.try_into().unwrap();

        self.div_assign(u)
    }
}

impl DivAssign<&ArithU256> for ArithU256 
{
    #[inline] fn div_assign(&mut self, b: &ArithU256) {
        self.base.div_assign(&b.base)
    }
}

//-------------------------------------------[.cpp/bitcoin/src/arith_u256.cpp]

impl ArithU256 {
    
    /**
      | This implementation directly uses
      | shifts instead of going through an intermediate
      | MPI representation.
      |
      | The "compact" format is a representation
      | of a whole number N using an unsigned
      | 32bit number similar to a floating point
      | format.
      | 
      | The most significant 8 bits are the unsigned
      | exponent of base 256.
      | 
      | This exponent can be thought of as "number
      | of bytes of N".
      | 
      | The lower 23 bits are the mantissa.
      | 
      | Bit number 24 (0x800000) represents
      | the sign of N.
      | 
      | N = (-1^sign) * mantissa * 256^(exponent-3)
      | 
      | Satoshi's original implementation
      | used BN_bn2mpi() and BN_mpi2bn().
      | 
      | MPI uses the most significant bit of
      | the first byte as sign.
      | 
      | Thus 0x1234560000 is compact (0x05123456)
      | 
      | And 0xc0de000000 is compact (0x0600c0de)
      | 
      | Bitcoin only uses this "compact" format
      | for encoding difficulty targets, which
      | are unsigned 256bit quantities. Thus,
      | all the complexities of the sign bit
      | and using base 256 are probably an implementation
      | accident.
      |
      */
    pub fn set_compact(&mut self, 
        n_compact:   u32,
        pf_negative: *mut bool,
        pf_overflow: *mut bool) -> &mut ArithU256 {
        
        todo!();
        /*
            int nSize = nCompact >> 24;
        uint32_t nWord = nCompact & 0x007fffff;
        if (nSize <= 3) {
            nWord >>= 8 * (3 - nSize);
            *this = nWord;
        } else {
            *this = nWord;
            *this <<= 8 * (nSize - 3);
        }
        if (pfNegative)
            *pfNegative = nWord != 0 && (nCompact & 0x00800000) != 0;
        if (pfOverflow)
            *pfOverflow = nWord != 0 && ((nSize > 34) ||
                                         (nWord > 0xff && nSize > 33) ||
                                         (nWord > 0xffff && nSize > 32));
        return *this;
        */
    }
    
    pub fn get_compact(&self, negative: Option<bool>) -> u32 {
        let negative: bool = negative.unwrap_or(false);
        
        todo!();
        /*
            int nSize = (bits() + 7) / 8;
        uint32_t nCompact = 0;
        if (nSize <= 3) {
            nCompact = GetLow64() << 8 * (3 - nSize);
        } else {
            ArithU256 bn = *this >> 8 * (nSize - 3);
            nCompact = bn.GetLow64();
        }
        // The 0x00800000 bit denotes the sign.
        // Thus, if it is already set, divide the mantissa by 256 and increase the exponent.
        if (nCompact & 0x00800000) {
            nCompact >>= 8;
            nSize++;
        }
        assert((nCompact & ~0x007fffff) == 0);
        assert(nSize < 256);
        nCompact |= nSize << 24;
        nCompact |= (fNegative && (nCompact & 0x007fffff) ? 0x00800000 : 0);
        return nCompact;
        */
    }
}

pub fn arith_to_uint256(a: &ArithU256) -> u256 {
    
    todo!();
    /*
    let mut b = u256::default();

    for x in 0..a.WIDTH {
        writele32(b.as_ptr().offset(x * 4), a.pn[x]);
    }

    b
    */
}

pub const fn uint_to_arith256(a: &u256) -> ArithU256 {
    
    todo!();
    /*
    let mut b = ArithU256::default();

    for x in 0..b.WIDTH {
        b.base.pn[x] = readle32(a.as_ptr().offset(x * 4));
    }

    b
    */
}
