crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/arith_u256.h]

/**
  | Template base class for unsigned big
  | integers.
  |
  */
#[derive(Clone,Debug)]
pub struct BaseUInt<const BITS: usize> 
where [(); BITS / 32 ]: 
{
    pn: [u32; BITS / 32],
}

impl<const BITS: usize> Default for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    fn default() -> Self {
        todo!();
        /*


            const_assert(BITS/32 > 0 && BITS%32 == 0, "Template parameter BITS must be a positive multiple of 32.");

            for (int i = 0; i < WIDTH; i++)
                pn[i] = 0;
        */
    }
}

impl<const BITS: usize> Not for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = BaseUInt<BITS>;

    #[inline] fn not(self) -> Self::Output {
        todo!();
        /*
            BaseUInt ret;
            for (int i = 0; i < WIDTH; i++)
                ret.pn[i] = ~pn[i];
            return ret;
        */
    }
}

impl<const BITS: usize> Neg for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = Self;
    
    #[inline] fn neg(self) -> Self::Output {
        todo!();
        /*
            BaseUInt ret;
            for (int i = 0; i < WIDTH; i++)
                ret.pn[i] = ~pn[i];
            ++ret;
            return ret;
        */
    }
}

impl<const BITS: usize> BitAndAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn bitand_assign(&mut self, b: &BaseUInt<BITS>) {
        todo!();
        /*
            for (int i = 0; i < WIDTH; i++)
                pn[i] &= b.pn[i];
            return *this;
        */
    }
}

impl<const BITS: usize> BitOrAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn bitor_assign(&mut self, b: &BaseUInt<BITS>) {
        todo!();
        /*
            for (int i = 0; i < WIDTH; i++)
                pn[i] |= b.pn[i];
            return *this;
        */
    }
}

impl<const BITS: usize> BitOrAssign<u64> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn bitor_assign(&mut self, b: u64) {
        todo!();
        /*
            pn[0] |= (unsigned int)b;
            pn[1] |= (unsigned int)(b >> 32);
            return *this;
        */
    }
}

impl<const BITS: usize> AddAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline]fn add_assign(&mut self, other: &BaseUInt<BITS>) {
        todo!();
        /*
            uint64_t carry = 0;
            for (int i = 0; i < WIDTH; i++)
            {
                uint64_t n = carry + pn[i] + b.pn[i];
                pn[i] = n & 0xffffffff;
                carry = n >> 32;
            }
            return *this;
        */
    }
}

impl<const BITS: usize> SubAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline]fn sub_assign(&mut self, other: &BaseUInt<BITS>) {
        todo!();
        /*
            *this += -b;
            return *this;
        */
    }
}

impl<const BITS: usize> AddAssign<u64> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline]fn add_assign(&mut self, other: u64) {
        todo!();
        /*
            BaseUInt b;
            b = b64;
            *this += b;
            return *this;
        */
    }
}

impl<const BITS: usize> SubAssign<u64> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline]fn sub_assign(&mut self, other: u64) {
        todo!();
        /*
            BaseUInt b;
            b = b64;
            *this += -b;
            return *this;
        */
    }
}

impl<const BITS: usize> BitXorAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn bitxor_assign(&mut self, b: &BaseUInt<BITS>) {
        todo!();
        /*
            for (int i = 0; i < WIDTH; i++)
                pn[i] ^= b.pn[i];
            return *this;
        */
    }
}

impl<const BITS: usize> BitXorAssign<u64> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn bitxor_assign(&mut self, b: u64) {
        todo!();
        /*
            pn[0] ^= (unsigned int)b;
            pn[1] ^= (unsigned int)(b >> 32);
            return *this;
        */
    }
}

impl<const BITS: usize> Add<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{

    type Output = BaseUInt<BITS>;
    
    fn add(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) += b;
        */
    }
}

impl<const BITS: usize> Sub<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{

    type Output = BaseUInt<BITS>;
    
    fn sub(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) -= b;
        */
    }
}

impl<const BITS: usize> Mul<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{

    type Output = BaseUInt<BITS>;
    
    fn mul(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) *= b;
        */
    }
}

impl<const BITS: usize> Div<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{

    type Output = BaseUInt<BITS>;

    fn div(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) /= b;
        */
    }
}

impl<const BITS: usize> BitOr<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = BaseUInt<BITS>;
    
    fn bitor(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) |= b;
        */
    }
}

impl<const BITS: usize> BitAnd<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = BaseUInt<BITS>;
    
    fn bitand(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) &= b;
        */
    }
}

impl<const BITS: usize> BitXor<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = BaseUInt<BITS>;
    
    fn bitxor(self, other: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) ^= b;
        */
    }
}

impl<const BITS: usize> Shr<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = BaseUInt<BITS>;

    fn shr(self, rhs: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) >>= shift;
        */
    }
}

impl<const BITS: usize> Shl<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    type Output = BaseUInt<BITS>;

    fn shl(self, rhs: &BaseUInt<BITS>) -> Self::Output {
        todo!();
        /*
            return BaseUInt(a) <<= shift;
        */
    }
}

impl<const BITS: usize> PartialEq<BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    fn eq(&self, other: &BaseUInt<BITS>) -> bool {
        todo!();
        /*
            return memcmp(a.pn, b.pn, sizeof(a.pn)) == 0;
        */
    }
}

impl<const BITS: usize> Eq for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{}

impl<const BITS: usize> Ord for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    fn cmp(&self, other: &BaseUInt<BITS>) -> std::cmp::Ordering {
        todo!();
        /*
            return a.CompareTo(b) < 0;
        */
    }
}

impl<const BITS: usize> PartialOrd<BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    fn partial_cmp(&self, other: &BaseUInt<BITS>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const BITS: usize> BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    pub const WIDTH: usize = BITS / 32;

    pub fn new_from_other(b: &BaseUInt<BITS>) -> Self {
    
        todo!();
        /*
            const_assert(BITS/32 > 0 && BITS%32 == 0, "Template parameter BITS must be a positive multiple of 32.");

            for (int i = 0; i < WIDTH; i++)
                pn[i] = b.pn[i];
        */
    }
    
    pub fn assign_from_other(&mut self, b: &BaseUInt<BITS>) -> &mut BaseUInt<BITS> {
        
        todo!();
        /*
            for (int i = 0; i < WIDTH; i++)
                pn[i] = b.pn[i];
            return *this;
        */
    }
    
    pub fn new_from_u64(b: u64) -> Self {
    
        todo!();
        /*
            const_assert(BITS/32 > 0 && BITS%32 == 0, "Template parameter BITS must be a positive multiple of 32.");

            pn[0] = (unsigned int)b;
            pn[1] = (unsigned int)(b >> 32);
            for (int i = 2; i < WIDTH; i++)
                pn[i] = 0;
        */
    }
    
    pub fn assign_from_u64(&mut self, b: u64) -> &mut BaseUInt<BITS> {
        
        todo!();
        /*
            pn[0] = (unsigned int)b;
            pn[1] = (unsigned int)(b >> 32);
            for (int i = 2; i < WIDTH; i++)
                pn[i] = 0;
            return *this;
        */
    }
    
    pub fn prefix_increment(&mut self) -> &mut BaseUInt<BITS> {
        
        todo!();
        /*
            // prefix operator
            int i = 0;
            while (i < WIDTH && ++pn[i] == 0)
                i++;
            return *this;
        */
    }
    
    pub fn postfix_increment(&mut self, _0: i32) -> BaseUInt<BITS> {
        
        todo!();
        /*
            // postfix operator
            const BaseUInt ret = *this;
            ++(*this);
            return ret;
        */
    }
    
    pub fn prefix_decrement(&mut self) -> &mut BaseUInt<BITS> {
        
        todo!();
        /*
            // prefix operator
            int i = 0;
            while (i < WIDTH && --pn[i] == std::numeric_limits<uint32_t>::max())
                i++;
            return *this;
        */
    }
    
    pub fn postfix_decrement(&mut self, _0: i32) -> BaseUInt<BITS> {
        
        todo!();
        /*
            // postfix operator
            const BaseUInt ret = *this;
            --(*this);
            return ret;
        */
    }
       
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            return sizeof(pn);
        */
    }

    pub fn get_low64(&self) -> u64 {
        
        todo!();
        /*
            const_assert(WIDTH >= 2, "Assertion WIDTH >= 2 failed (WIDTH = BITS / 32). BITS is a template parameter.");
            return pn[0] | (uint64_t)pn[1] << 32;
        */
    }
}

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

impl<const BITS: usize> From<&str> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{

    fn from(str_: &str) -> Self {
    
        todo!();
        /*
           const_assert(
               BITS/32 > 0 && BITS%32 == 0, 
               "Template parameter BITS must be a positive multiple of 32."
           );

           SetHex(str);
        */
    }
}

impl<const BITS: usize> ShlAssign<u32> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn shl_assign(&mut self, shift: u32) {
        todo!();
        /*
            BaseUInt<BITS> a(*this);
        for (int i = 0; i < WIDTH; i++)
            pn[i] = 0;
        int k = shift / 32;
        shift = shift % 32;
        for (int i = 0; i < WIDTH; i++) {
            if (i + k + 1 < WIDTH && shift != 0)
                pn[i + k + 1] |= (a.pn[i] >> (32 - shift));
            if (i + k < WIDTH)
                pn[i + k] |= (a.pn[i] << shift);
        }
        return *this;
        */
    }
}

impl<const BITS: usize> ShrAssign<u32> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn shr_assign(&mut self, shift: u32) {
        todo!();
        /*
            BaseUInt<BITS> a(*this);
        for (int i = 0; i < WIDTH; i++)
            pn[i] = 0;
        int k = shift / 32;
        shift = shift % 32;
        for (int i = 0; i < WIDTH; i++) {
            if (i - k - 1 >= 0 && shift != 0)
                pn[i - k - 1] |= (a.pn[i] << (32 - shift));
            if (i - k >= 0)
                pn[i - k] |= (a.pn[i] >> shift);
        }
        return *this;
        */
    }
}

impl<const BITS: usize> MulAssign<u32> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn mul_assign(&mut self, b32: u32) {
        todo!();
        /*
            uint64_t carry = 0;
        for (int i = 0; i < WIDTH; i++) {
            uint64_t n = carry + (uint64_t)b32 * pn[i];
            pn[i] = n & 0xffffffff;
            carry = n >> 32;
        }
        return *this;
        */
    }
}

impl<const BITS: usize> MulAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn mul_assign(&mut self, b: &BaseUInt<BITS>) {
        todo!();
        /*
            BaseUInt<BITS> a;
        for (int j = 0; j < WIDTH; j++) {
            uint64_t carry = 0;
            for (int i = 0; i + j < WIDTH; i++) {
                uint64_t n = carry + a.pn[i + j] + (uint64_t)pn[j] * b.pn[i];
                a.pn[i + j] = n & 0xffffffff;
                carry = n >> 32;
            }
        }
        *this = a;
        return *this;
        */
    }
}

impl<const BITS: usize> DivAssign<&BaseUInt<BITS>> for BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{
    
    #[inline] fn div_assign(&mut self, b: &BaseUInt<BITS>) {
        todo!();
        /*
            BaseUInt<BITS> div = b;     // make a copy, so we can shift.
        BaseUInt<BITS> num = *this; // make a copy, so we can subtract.
        *this = 0;                   // the quotient.
        int num_bits = num.bits();
        int div_bits = div.bits();
        if (div_bits == 0)
            throw uint_error("Division by zero");
        if (div_bits > num_bits) // the result is certainly 0.
            return *this;
        int shift = num_bits - div_bits;
        div <<= shift; // shift so that div and num align.
        while (shift >= 0) {
            if (num >= div) {
                num -= div;
                pn[shift / 32] |= (1 << (shift & 31)); // set a bit of the result.
            }
            div >>= 1; // shift back.
            shift--;
        }
        // num now contains the remainder of the division.
        return *this;
        */
    }
}

impl<const BITS: usize> BaseUInt<BITS> 
where [(); BITS / 32 ]: 
{

    pub fn compare_to(&self, b: &BaseUInt<BITS>) -> i32 {
        
        todo!();
        /*
            for (int i = WIDTH - 1; i >= 0; i--) {
            if (pn[i] < b.pn[i])
                return -1;
            if (pn[i] > b.pn[i])
                return 1;
        }
        return 0;
        */
    }
    
    pub fn equal_to(&self, b: u64) -> bool {
        
        todo!();
        /*
            for (int i = WIDTH - 1; i >= 2; i--) {
            if (pn[i])
                return false;
        }
        if (pn[1] != (b >> 32))
            return false;
        if (pn[0] != (b & 0xfffffffful))
            return false;
        return true;
        */
    }
    
    pub fn getdouble(&self) -> f64 {
        
        todo!();
        /*
            double ret = 0.0;
        double fact = 1.0;
        for (int i = 0; i < WIDTH; i++) {
            ret += fact * pn[i];
            fact *= 4294967296.0;
        }
        return ret;
        */
    }
    
    pub fn get_hex(&self) -> String {
        
        todo!();
        /*
            return ArithToUint256(*this).GetHex();
        */
    }
    
    pub fn set_hex(&mut self, psz: *const u8)  {
        
        todo!();
        /*
            *this = UintToArith256(uint256S(psz));
        */
    }
    
    pub fn set_hex_with_str(&mut self, str_: &str) {
        
        todo!();
        /*
            SetHex(str.c_str());
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return (GetHex());
        */
    }
    
    /**
      | Returns the position of the highest
      | bit set plus one, or zero if the value
      | is zero.
      |
      */
    pub fn bits(&self) -> u32 {
        
        todo!();
        /*
            for (int pos = WIDTH - 1; pos >= 0; pos--) {
            if (pn[pos]) {
                for (int nbits = 31; nbits > 0; nbits--) {
                    if (pn[pos] & 1U << nbits)
                        return 32 * pos + nbits + 1;
                }
                return 32 * pos + 1;
            }
        }
        return 0;
        */
    }
}

lazy_static!{
    /*
    // Explicit instantiations for BaseUInt<256>
    template BaseUInt<256>::BaseUInt(const std::string&);
    template BaseUInt<256>& BaseUInt<256>::operator<<=(unsigned int);
    template BaseUInt<256>& BaseUInt<256>::operator>>=(unsigned int);
    template BaseUInt<256>& BaseUInt<256>::operator*=(uint32_t b32);
    template BaseUInt<256>& BaseUInt<256>::operator*=(const BaseUInt<256>& b);
    template BaseUInt<256>& BaseUInt<256>::operator/=(const BaseUInt<256>& b);
    template int BaseUInt<256>::CompareTo(const BaseUInt<256>&) const;
    template bool BaseUInt<256>::EqualTo(uint64_t) const;
    template double BaseUInt<256>::getdouble() const;
    template std::string BaseUInt<256>::GetHex() const;
    template std::string BaseUInt<256>::ToString() const;
    template c_void BaseUInt<256>::SetHex(const char*);
    template c_void BaseUInt<256>::SetHex(const std::string&);
    template unsigned int BaseUInt<256>::bits() const;
    */
}

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

pub fn uint_to_arith256(a: &u256) -> ArithU256 {
    
    todo!();
    /*
    let mut b = ArithU256::default();

    for x in 0..b.WIDTH {
        b.base.pn[x] = readle32(a.as_ptr().offset(x * 4));
    }

    b
    */
}
