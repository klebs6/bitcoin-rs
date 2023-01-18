crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/scriptnum10.h]

pub struct ScriptNum10Error {

}

impl RuntimeErrorInterface for ScriptNum10Error {}

impl From<&str> for ScriptNum10Error {
    
    fn from(str_: &str) -> Self {
    
        todo!();
        /*


            : std::runtime_error(str)
        */
    }
}

pub struct ScriptNum10 {
    value: i64,
}

pub mod script_num10 {
    pub const N_DEFAULT_MAX_NUM_SIZE: usize = 4;
}

impl PartialEq<i64> for ScriptNum10 {
    
    fn eq(&self, other: &i64) -> bool {
        todo!();
        /*
            return m_value == rhs;
        */
    }
}

impl Ord for ScriptNum10 {
    
    fn cmp(&self, other: &ScriptNum10) -> Ordering {
        todo!();
        /*
            return m_value <  rhs;
        */
    }
}

impl PartialOrd<i64> for ScriptNum10 {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(self.cmp(&ScriptNum10{ value: other.clone() }))
    }
}

impl PartialEq<ScriptNum10> for ScriptNum10 {
    
    fn eq(&self, other: &ScriptNum10) -> bool {
        todo!();
        /*
            return operator==(rhs.m_value);
        */
    }
}

impl Eq for ScriptNum10 {}

impl PartialOrd<ScriptNum10> for ScriptNum10 {
    fn partial_cmp(&self, other: &ScriptNum10) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<i64> for ScriptNum10 {

    type Output = ScriptNum10;
    
    fn add(self, other: i64) -> Self::Output {
        todo!();
        /*
            return CScriptNum10(m_value + rhs);
        */
    }
}

impl Sub<i64> for ScriptNum10 {

    type Output = ScriptNum10;
    
    fn sub(self, other: i64) -> Self::Output {
        todo!();
        /*
            return CScriptNum10(m_value - rhs);
        */
    }
}

impl Add<ScriptNum10> for ScriptNum10 {

    type Output = ScriptNum10;
    
    fn add(self, other: ScriptNum10) -> Self::Output {
        todo!();
        /*
            return operator+(rhs.m_value);
        */
    }
}

impl Sub<ScriptNum10> for ScriptNum10 {

    type Output = ScriptNum10;
    
    fn sub(self, other: ScriptNum10) -> Self::Output {
        todo!();
        /*
            return operator-(rhs.m_value);
        */
    }
}

impl AddAssign<ScriptNum10> for ScriptNum10 {

    fn add_assign(&mut self, other: ScriptNum10) {
        todo!();
        /*
            return operator+=(rhs.m_value);
        */
    }
}

impl SubAssign<ScriptNum10> for ScriptNum10 {
    
    fn sub_assign(&mut self, other: ScriptNum10) {
        todo!();
        /*
            return operator-=(rhs.m_value);
        */
    }
}

impl Neg for ScriptNum10 {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        todo!();
        /*
            assert(m_value != std::numeric_limits<int64_t>::min());
            return CScriptNum10(-m_value);
        */
    }
}

impl AddAssign<i64> for ScriptNum10 {

    fn add_assign(&mut self, other: i64) {
        todo!();
        /*
            assert(rhs == 0 || (rhs > 0 && m_value <= std::numeric_limits<int64_t>::max() - rhs) ||
                               (rhs < 0 && m_value >= std::numeric_limits<int64_t>::min() - rhs));
            m_value += rhs;
            return *this;
        */
    }
}

impl SubAssign<i64> for ScriptNum10 {
    
    fn sub_assign(&mut self, other: i64) {
        todo!();
        /*
            assert(rhs == 0 || (rhs > 0 && m_value >= std::numeric_limits<int64_t>::min() + rhs) ||
                               (rhs < 0 && m_value <= std::numeric_limits<int64_t>::max() + rhs));
            m_value -= rhs;
            return *this;
        */
    }
}

impl ScriptNum10 {
    
    #[inline] pub fn assign_from(&mut self, rhs: &i64) -> &mut ScriptNum10 {
        
        todo!();
        /*
            m_value = rhs;
            return *this;
        */
    }
    
    pub fn getint(&self) -> i32 {
        
        todo!();
        /*
            if (m_value > std::numeric_limits<int>::max())
                return std::numeric_limits<int>::max();
            else if (m_value < std::numeric_limits<int>::min())
                return std::numeric_limits<int>::min();
            return m_value;
        */
    }
    
    pub fn getvch(&self) -> Vec<u8> {
        
        todo!();
        /*
            return serialize(m_value);
        */
    }
    
    pub fn serialize(value: &i64) -> Vec<u8> {
        
        todo!();
        /*
            if(value == 0)
                return std::vector<unsigned char>();

            std::vector<unsigned char> result;
            const bool neg = value < 0;
            uint64_t absvalue = neg ? -value : value;

            while(absvalue)
            {
                result.push_back(absvalue & 0xff);
                absvalue >>= 8;
            }

    //    - If the most significant byte is >= 0x80 and the value is positive, push a
    //    new zero-byte to make the significant byte < 0x80 again.

    //    - If the most significant byte is >= 0x80 and the value is negative, push a
    //    new 0x80 byte that will be popped off when converting to an integral.

    //    - If the most significant byte is < 0x80 and the value is negative, add
    //    0x80 to it, since it will be subtracted and interpreted as a negative when
    //    converting to an integral.

            if (result.back() & 0x80)
                result.push_back(neg ? 0x80 : 0);
            else if (neg)
                result.back() |= 0x80;

            return result;
        */
    }
    
    pub fn set_vch(vch: &Vec<u8>) -> i64 {
        
        todo!();
        /*
            if (vch.empty())
              return 0;

          int64_t result = 0;
          for (size_t i = 0; i != vch.size(); ++i)
              result |= static_cast<int64_t>(vch[i]) << 8*i;

          // If the input vector's most significant byte is 0x80, remove it from
          // the result's msb and return a negative.
          if (vch.back() & 0x80)
              return -((int64_t)(result & ~(0x80ULL << (8 * (vch.size() - 1)))));

          return result;
        */
    }

    /**
      | The ScriptNum implementation from
      | Bitcoin Core 0.10.0, for cross-comparison.
      |
      */
    pub fn new_from_n(n: &i64) -> Self {
    
        todo!();
        /*


            m_value = n;
        */
    }
    
    pub fn new(
        vch:             &Vec<u8>,
        require_minimal: bool,
        n_max_num_size:  Option<usize>) -> Self {

        let n_max_num_size: usize =
                 n_max_num_size.unwrap_or(script_num::N_DEFAULT_MAX_NUM_SIZE);

        todo!();
        /*


            if (vch.size() > nMaxNumSize) {
                throw scriptnum10_error("script number overflow");
            }
            if (fRequireMinimal && vch.size() > 0) {
                // Check that the number is encoded with the minimum possible
                // number of bytes.
                //
                // If the most-significant-byte - excluding the sign bit - is zero
                // then we're not minimal. Note how this test also rejects the
                // negative-zero encoding, 0x80.
                if ((vch.back() & 0x7f) == 0) {
                    // One exception: if there's more than one byte and the most
                    // significant bit of the second-most-significant-byte is set
                    // it would conflict with the sign bit. An example of this case
                    // is +-255, which encode to 0xff00 and 0xff80 respectively.
                    // (big-endian).
                    if (vch.size() <= 1 || (vch[vch.size() - 2] & 0x80) == 0) {
                        throw scriptnum10_error("non-minimally encoded script number");
                    }
                }
            }
            m_value = set_vch(vch);
        */
    }
}
