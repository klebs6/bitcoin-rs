// ---------------- [ File: bitcoin-scripting/src/script.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/script.h]

/**
  | Maximum number of bytes pushable to
  | the stack
  |
  */
pub const MAX_SCRIPT_ELEMENT_SIZE: usize = 520;

/**
  | Maximum number of non-push operations
  | per script
  |
  */
pub const MAX_OPS_PER_SCRIPT: usize = 201;

/**
  | Maximum number of public keys per multisig
  |
  */
pub const MAX_PUBKEYS_PER_MULTISIG: usize = 20;

/**
  | Maximum script length in bytes
  |
  */
pub const MAX_SCRIPT_SIZE: usize = 10000;

/**
  | Maximum number of values on script interpreter
  | stack
  |
  */
pub const MAX_STACK_SIZE: usize = 1000;

/**
  | Threshold for nLockTime: below this
  | value it is interpreted as block number,
  | otherwise as UNIX timestamp.
  |
  */
pub const LOCKTIME_THRESHOLD: u32 = 500000000; // Tue Nov  5 00:53:20 1985 UTC

/**
  | Maximum nLockTime. Since a lock time indicates
  | the last invalid timestamp, a transaction with
  | this lock time will never be valid unless lock
  | time checking is disabled (by setting all input
  | sequence numbers to SEQUENCE_FINAL).
  */
pub const LOCKTIME_MAX: u32 = 0xFFFFFFFF;

/**
  | Tag for input annex. If there are at least two
  | witness elements for a transaction input, and
  | the first byte of the last element is 0x50,
  | this last element is called annex, and has
  | meanings independent of the script
  */
pub const ANNEX_TAG: u32 = 0x50;

/**
  | Validation weight per passing signature
  | (Tapscript only, see BIP 342).
  |
  */
pub const VALIDATION_WEIGHT_PER_SIGOP_PASSED: u64 = 50;

/**
  | How much weight budget is added to the
  | witness size (Tapscript only, see BIP
  | 342).
  |
  */
pub const VALIDATION_WEIGHT_OFFSET: u64 = 50;

pub fn to_byte_vector<T>(in_: &T) -> Vec<u8> {

    todo!();
        /*
            return std::vector<unsigned char>(in.begin(), in.end());
        */
}

/**
  | Script opcodes
  |
  */
pub type OpcodeType = u8;

pub mod opcode_type
{
    /* ------------------ push value  ------------------ */
    pub const OP_0:                   u8 = 0x00;
    pub const OP_FALSE:               u8 = OP_0;
    pub const OP_PUSHDATA1:           u8 = 0x4c;
    pub const OP_PUSHDATA2:           u8 = 0x4d;
    pub const OP_PUSHDATA4:           u8 = 0x4e;
    pub const OP_1NEGATE:             u8 = 0x4f;
    pub const OP_RESERVED:            u8 = 0x50;
    pub const OP_1:                   u8 = 0x51;
    pub const OP_TRUE:                u8 = OP_1;
    pub const OP_2:                   u8 = 0x52;
    pub const OP_3:                   u8 = 0x53;
    pub const OP_4:                   u8 = 0x54;
    pub const OP_5:                   u8 = 0x55;
    pub const OP_6:                   u8 = 0x56;
    pub const OP_7:                   u8 = 0x57;
    pub const OP_8:                   u8 = 0x58;
    pub const OP_9:                   u8 = 0x59;
    pub const OP_10:                  u8 = 0x5a;
    pub const OP_11:                  u8 = 0x5b;
    pub const OP_12:                  u8 = 0x5c;
    pub const OP_13:                  u8 = 0x5d;
    pub const OP_14:                  u8 = 0x5e;
    pub const OP_15:                  u8 = 0x5f;
    pub const OP_16:                  u8 = 0x60;

    /* -------------------- control  -------------------- */
    pub const OP_NOP:                 u8 = 0x61;
    pub const OP_VER:                 u8 = 0x62;
    pub const OP_IF:                  u8 = 0x63;
    pub const OP_NOTIF:               u8 = 0x64;
    pub const OP_VERIF:               u8 = 0x65;
    pub const OP_VERNOTIF:            u8 = 0x66;
    pub const OP_ELSE:                u8 = 0x67;
    pub const OP_ENDIF:               u8 = 0x68;
    pub const OP_VERIFY:              u8 = 0x69;
    pub const OP_RETURN:              u8 = 0x6a;

    /* ------------------- stack ops  ------------------- */
    pub const OP_TOALTSTACK:          u8 = 0x6b;
    pub const OP_FROMALTSTACK:        u8 = 0x6c;
    pub const OP_2DROP:               u8 = 0x6d;
    pub const OP_2DUP:                u8 = 0x6e;
    pub const OP_3DUP:                u8 = 0x6f;
    pub const OP_2OVER:               u8 = 0x70;
    pub const OP_2ROT:                u8 = 0x71;
    pub const OP_2SWAP:               u8 = 0x72;
    pub const OP_IFDUP:               u8 = 0x73;
    pub const OP_DEPTH:               u8 = 0x74;
    pub const OP_DROP:                u8 = 0x75;
    pub const OP_DUP:                 u8 = 0x76;
    pub const OP_NIP:                 u8 = 0x77;
    pub const OP_OVER:                u8 = 0x78;
    pub const OP_PICK:                u8 = 0x79;
    pub const OP_ROLL:                u8 = 0x7a;
    pub const OP_ROT:                 u8 = 0x7b;
    pub const OP_SWAP:                u8 = 0x7c;
    pub const OP_TUCK:                u8 = 0x7d;

    /* ------------------ splice ops  ------------------ */
    pub const OP_CAT:                 u8 = 0x7e;
    pub const OP_SUBSTR:              u8 = 0x7f;
    pub const OP_LEFT:                u8 = 0x80;
    pub const OP_RIGHT:               u8 = 0x81;
    pub const OP_SIZE:                u8 = 0x82;

    /* ------------------- bit logic  ------------------- */
    pub const OP_INVERT:              u8 = 0x83;
    pub const OP_AND:                 u8 = 0x84;
    pub const OP_OR:                  u8 = 0x85;
    pub const OP_XOR:                 u8 = 0x86;
    pub const OP_EQUAL:               u8 = 0x87;
    pub const OP_EQUALVERIFY:         u8 = 0x88;
    pub const OP_RESERVED1:           u8 = 0x89;
    pub const OP_RESERVED2:           u8 = 0x8a;

    /* -------------------- numeric  -------------------- */
    pub const OP_1ADD:                u8 = 0x8b;
    pub const OP_1SUB:                u8 = 0x8c;
    pub const OP_2MUL:                u8 = 0x8d;
    pub const OP_2DIV:                u8 = 0x8e;
    pub const OP_NEGATE:              u8 = 0x8f;
    pub const OP_ABS:                 u8 = 0x90;
    pub const OP_NOT:                 u8 = 0x91;
    pub const OP_0NOTEQUAL:           u8 = 0x92;

    pub const OP_ADD:                 u8 = 0x93;
    pub const OP_SUB:                 u8 = 0x94;
    pub const OP_MUL:                 u8 = 0x95;
    pub const OP_DIV:                 u8 = 0x96;
    pub const OP_MOD:                 u8 = 0x97;
    pub const OP_LSHIFT:              u8 = 0x98;
    pub const OP_RSHIFT:              u8 = 0x99;

    pub const OP_BOOLAND:             u8 = 0x9a;
    pub const OP_BOOLOR:              u8 = 0x9b;
    pub const OP_NUMEQUAL:            u8 = 0x9c;
    pub const OP_NUMEQUALVERIFY:      u8 = 0x9d;
    pub const OP_NUMNOTEQUAL:         u8 = 0x9e;
    pub const OP_LESSTHAN:            u8 = 0x9f;
    pub const OP_GREATERTHAN:         u8 = 0xa0;
    pub const OP_LESSTHANOREQUAL:     u8 = 0xa1;
    pub const OP_GREATERTHANOREQUAL:  u8 = 0xa2;
    pub const OP_MIN:                 u8 = 0xa3;
    pub const OP_MAX:                 u8 = 0xa4;

    pub const OP_WITHIN:              u8 = 0xa5;

    /* -------------------- crypto  -------------------- */
    pub const OP_RIPEMD160:           u8 = 0xa6;
    pub const OP_SHA1:                u8 = 0xa7;
    pub const OP_SHA256:              u8 = 0xa8;
    pub const OP_HASH160:             u8 = 0xa9;
    pub const OP_HASH256:             u8 = 0xaa;
    pub const OP_CODESEPARATOR:       u8 = 0xab;
    pub const OP_CHECKSIG:            u8 = 0xac;
    pub const OP_CHECKSIGVERIFY:      u8 = 0xad;
    pub const OP_CHECKMULTISIG:       u8 = 0xae;
    pub const OP_CHECKMULTISIGVERIFY: u8 = 0xaf;

    /* ------------------- expansion  ------------------- */
    pub const OP_NOP1:                u8 = 0xb0;
    pub const OP_CHECKLOCKTIMEVERIFY: u8 = 0xb1;
    pub const OP_NOP2:                u8 = OP_CHECKLOCKTIMEVERIFY;
    pub const OP_CHECKSEQUENCEVERIFY: u8 = 0xb2;
    pub const OP_NOP3:                u8 = OP_CHECKSEQUENCEVERIFY;
    pub const OP_NOP4:                u8 = 0xb3;
    pub const OP_NOP5:                u8 = 0xb4;
    pub const OP_NOP6:                u8 = 0xb5;
    pub const OP_NOP7:                u8 = 0xb6;
    pub const OP_NOP8:                u8 = 0xb7;
    pub const OP_NOP9:                u8 = 0xb8;
    pub const OP_NOP10:               u8 = 0xb9;

    /* ------ Opcode added by BIP 342 (Tapscript)  ------ */
    pub const OP_CHECKSIGADD:         u8 = 0xba;
    pub const OP_INVALIDOPCODE:       u8 = 0xff;
}

/**
  | Maximum value that an opcode can be
  |
  */
pub const MAX_OPCODE: u32 = opcode_type::OP_NOP10 as u32;

pub struct ScriptNumError { }

impl RuntimeErrorInterface for ScriptNumError {}

impl From<&String> for ScriptNumError {

    fn from(str_: &String) -> Self {
    
        todo!();
        /*


            : std::runtime_error(str)
        */
    }
}

///-------------------------
pub struct ScriptNum {
    value: i64,
}

pub mod script_num {
    pub const N_DEFAULT_MAX_NUM_SIZE: usize = 4;
}

impl PartialEq<i64> for ScriptNum {
    
    fn eq(&self, other: &i64) -> bool {
        todo!();
        /*
            return m_value == rhs;
        */
    }
}

impl PartialOrd<i64> for ScriptNum {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(self.value.cmp(other))
    }
}

impl PartialEq<ScriptNum> for ScriptNum {
    
    fn eq(&self, other: &ScriptNum) -> bool {
        todo!();
        /*
            return operator==(rhs.m_value);
        */
    }
}

impl Eq for ScriptNum {}

impl Ord for ScriptNum {
    
    fn cmp(&self, other: &ScriptNum) -> Ordering {
        todo!();
        /*
            return operator< (rhs.m_value);
        */
    }
}

impl PartialOrd<ScriptNum> for ScriptNum {
    fn partial_cmp(&self, other: &ScriptNum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<i64> for ScriptNum {

    type Output = ScriptNum;
    
    fn add(self, other: i64) -> Self::Output {
        todo!();
        /*
            return CScriptNum(m_value + rhs);
        */
    }
}

impl Sub<i64> for ScriptNum {

    type Output = ScriptNum;
    
    fn sub(self, other: i64) -> Self::Output {
        todo!();
        /*
            return CScriptNum(m_value - rhs);
        */
    }
}

impl Add<&ScriptNum> for ScriptNum {

    type Output = ScriptNum;
    
    fn add(self, other: &ScriptNum) -> Self::Output {
        todo!();
        /*
            return operator+(rhs.m_value);
        */
    }
}

impl Sub<&ScriptNum> for ScriptNum {

    type Output = ScriptNum;
    
    fn sub(self, other: &ScriptNum) -> Self::Output {
        todo!();
        /*
            return operator-(rhs.m_value);
        */
    }
}

impl AddAssign<&ScriptNum> for ScriptNum {
    
    fn add_assign(&mut self, other: &ScriptNum) {
        todo!();
        /*
            return operator+=(rhs.m_value);
        */
    }
}

impl SubAssign<&ScriptNum> for ScriptNum {
    
    fn sub_assign(&mut self, other: &ScriptNum) {
        todo!();
        /*
            return operator-=(rhs.m_value);
        */
    }
}

impl BitAnd<i64> for ScriptNum {
    type Output = ScriptNum;
    
    fn bitand(self, other: i64) -> Self::Output {
        todo!();
        /*
            return CScriptNum(m_value & rhs);
        */
    }
}

impl BitAnd<&ScriptNum> for ScriptNum {
    type Output = ScriptNum;

    fn bitand(self, other: &ScriptNum) -> Self::Output {
        todo!();
        /*
            return operator&(rhs.m_value);
        */
    }
}

impl BitAndAssign<&ScriptNum> for ScriptNum {
    
    fn bitand_assign(&mut self, rhs: &ScriptNum) {
        todo!();
        /*
            return operator&=(rhs.m_value);
        */
    }
}

impl Neg for ScriptNum {
    type Output = Self;
    
    fn neg(self) -> Self::Output {
        todo!();
        /*
            assert(m_value != std::numeric_limits<int64_t>::min());
            return CScriptNum(-m_value);
        */
    }
}

impl ScriptNum {
    
    #[inline] pub fn assign_from(&mut self, rhs: &i64) -> &mut ScriptNum {
        
        todo!();
        /*
            m_value = rhs;
            return *this;
        */
    }
}

impl AddAssign<i64> for ScriptNum {
    
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

impl SubAssign<i64> for ScriptNum {
    
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

impl BitAndAssign<i64> for ScriptNum {
    
    fn bitand_assign(&mut self, rhs: i64) {
        todo!();
        /*
            m_value &= rhs;
            return *this;
        */
    }
}

impl From<i64> for ScriptNum {

    /**
      | Numeric opcodes (OP_1ADD, etc) are
      | restricted to operating on 4-byte integers.
      | 
      | The semantics are subtle, though: operands
      | must be in the range [-2^31 +1...2^31 -1],
      | but results may overflow (and are valid as
      | long as they are not used in a subsequent
      | numeric operation). 
      |
      | CScriptNum enforces those semantics by
      | storing results as an int64 and allowing
      | out-of-range values to be returned as
      | a vector of bytes but throwing an
      | exception if arithmetic is done or the
      | result is interpreted as an integer.
      |
      */
    fn from(n: i64) -> Self {
    
        todo!();
        /*
            m_value = n;
        */
    }
}

impl ScriptNum {

    pub fn new(
        vch:             &Vec<u8>,
        require_minimal: bool,
        n_max_num_size:  Option<usize>) -> Self {

        let n_max_num_size: usize =
                 n_max_num_size.unwrap_or(script_num::N_DEFAULT_MAX_NUM_SIZE);

        todo!();
        /*


            if (vch.size() > nMaxNumSize) {
                throw scriptnum_error("script number overflow");
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
                        throw scriptnum_error("non-minimally encoded script number");
                    }
                }
            }
            m_value = set_vch(vch);
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
            uint64_t absvalue = neg ? ~static_cast<uint64_t>(value) + 1 : static_cast<uint64_t>(value);

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
}

/**
  | We use a prevector for the script to reduce
  | the considerable memory overhead of
  | vectors in cases where they normally
  | contain a small number of small elements.
  | 
  | Tests in October 2015 showed use of this
  | reduced dbcache memory usage by 23%
  | and made an initial sync 13% faster.
  |
  */
pub type ScriptBase     = PreVector<u8,28>;
pub type ScriptIterator = dyn Iterator<Item = u8>;

/**
  | Serialized script, used inside transaction
  | inputs and outputs
  |
  */
#[derive(Default,Clone,Serialize,Deserialize)]
pub struct Script {
    pub base: ScriptBase,
}

impl RecursiveDynamicUsage for Script {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
                return memusage::DynamicUsage(script);
            */
    }
}

impl Deref for Script {

    type Target = ScriptBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Script {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Script {
    pub const fn new() -> Self {
        Self {
            base: TinyVec::Inline(ArrayVec::from_array_empty([0; 28])),
        }
    }

    pub fn empty(&self) -> bool {
        self.base.is_empty()
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CScript, obj) { 
        READWRITEAS(ScriptBase, obj); 
    }
    */
}

impl Write for Script {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        todo!();
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        todo!();
    }
}

impl Read for Script {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        todo!();
    }
}

impl Shl<i64> for Script {
    type Output = Script;
    
    #[inline] fn shl(self, rhs: i64) -> Self::Output {
        todo!();
        /*
            return push_int64(b);
        */
    }
}

impl Shl<OpcodeType> for Script {
    type Output = Script;
    
    #[inline] fn shl(self, rhs: OpcodeType) -> Self::Output {
        todo!();
        /*
            if (opcode < 0 || opcode > 0xff)
                throw std::runtime_error("CScript::operator<<(): invalid opcode");
            insert(end(), (unsigned char)opcode);
            return *this;
        */
    }
}

impl Shl<&ScriptNum> for Script {
    type Output = Script;
    
    #[inline] fn shl(self, rhs: &ScriptNum) -> Self::Output {
        todo!();
        /*
            *this << b.getvch();
            return *this;
        */
    }
}

impl Shl<&Vec<u8>> for Script {
    type Output = Script;
    
    #[inline] fn shl(self, rhs: &Vec<u8>) -> Self::Output {
        todo!();
        /*
            if (b.size() < OP_PUSHDATA1)
            {
                insert(end(), (unsigned char)b.size());
            }
            else if (b.size() <= 0xff)
            {
                insert(end(), OP_PUSHDATA1);
                insert(end(), (unsigned char)b.size());
            }
            else if (b.size() <= 0xffff)
            {
                insert(end(), OP_PUSHDATA2);
                uint8_t _data[2];
                WriteLE16(_data, b.size());
                insert(end(), _data, _data + sizeof(_data));
            }
            else
            {
                insert(end(), OP_PUSHDATA4);
                uint8_t _data[4];
                WriteLE32(_data, b.size());
                insert(end(), _data, _data + sizeof(_data));
            }
            insert(end(), b.begin(), b.end());
            return *this;
        */
    }
}

impl From<i64> for Script {

    fn from(b: i64) -> Self {
    
        todo!();
        /*
            operator<<(b);
        */
    }
}

impl From<OpcodeType> for Script {
    
    fn from(b: OpcodeType) -> Self {
    
        todo!();
        /*
            operator<<(b);
        */
    }
}
    
impl From<&ScriptNum> for Script {

    fn from(b: &ScriptNum) -> Self {
    
        todo!();
        /*
            operator<<(b);
        */
    }
}

impl Script {

    pub fn push_int64(&mut self, n: i64) -> &mut Script {
        
        todo!();
        /*
            if (n == -1 || (n >= 1 && n <= 16))
            {
                push_back(n + (OP_1 - 1));
            }
            else if (n == 0)
            {
                push_back(OP_0);
            }
            else
            {
                *this << CScriptNum::serialize(n);
            }
            return *this;
        */
    }
    
    pub fn new_from_iterator_range(
        pbegin: Box<dyn Iterator<Item = u8>>,
        pend:   Box<dyn Iterator<Item = u8>>) -> Self {
    
        todo!();
        /*
        : script_base(pbegin, pend),
        */
    }
    
    pub fn new_from_pointer_range(
        pbegin: *const u8,
        pend:   *const u8) -> Self {
    
        todo!();
        /*
        : script_base(pbegin, pend),
        */
    }
    
    pub fn get_op<'a>(&self, 
        pc:         &mut Peekable<std::slice::Iter<'a, u8>>,
        opcode_ret: &mut OpcodeType,
        vch_ret:    Option<&mut Vec<u8>>) -> bool {
        
        todo!();
        /*
            return GetScriptOp(pc, end(), opcodeRet, &vchRet);
        */
    }
    
    /**
      | Encode/decode small integers:
      |
      */
    pub fn decodeop_n(opcode: OpcodeType) -> i32 {
        
        todo!();
        /*
            if (opcode == OP_0)
                return 0;
            assert(opcode >= OP_1 && opcode <= OP_16);
            return (int)opcode - (int)(OP_1 - 1);
        */
    }
    
    pub fn encodeop_n(n: i32) -> OpcodeType {
        
        todo!();
        /*
            assert(n >= 0 && n <= 16);
            if (n == 0)
                return OP_0;
            return (opcodetype)(OP_1+n-1);
        */
    }

    /**
      | Returns whether the script is guaranteed
      | to fail at execution, regardless of
      | the initial stack. This allows outputs
      | to be pruned instantly when entering
      | the UTXO set.
      |
      */
    pub fn is_unspendable(&self) -> bool {
        
        todo!();
        /*
            return (size() > 0 && *begin() == OP_RETURN) || (size() > MAX_SCRIPT_SIZE);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            // The default prevector::clear() does not release memory
            ScriptBase::clear();
            shrink_to_fit();
        */
    }
    
    /**
      | Pre-version-0.6, Bitcoin always counted
      | CHECKMULTISIGs as 20 sigops. With pay-to-script-hash,
      | that changed:
      | 
      | CHECKMULTISIGs serialized in scriptSigs
      | are counted more accurately, assuming
      | they are of the form ... OP_N CHECKMULTISIG
      | ...
      |
      */
    pub fn get_sig_op_count(&self, accurate: bool) -> u32 {
        
        todo!();
        /*
            unsigned int n = 0;
        const_iterator pc = begin();
        opcodetype lastOpcode = OP_INVALIDOPCODE;
        while (pc < end())
        {
            opcodetype opcode;
            if (!GetOp(pc, opcode))
                break;
            if (opcode == OP_CHECKSIG || opcode == OP_CHECKSIGVERIFY)
                n++;
            else if (opcode == OP_CHECKMULTISIG || opcode == OP_CHECKMULTISIGVERIFY)
            {
                if (fAccurate && lastOpcode >= OP_1 && lastOpcode <= OP_16)
                    n += DecodeOP_N(lastOpcode);
                else
                    n += MAX_PUBKEYS_PER_MULTISIG;
            }
            lastOpcode = opcode;
        }
        return n;
        */
    }
    
    /**
      | Accurately count sigOps, including
      | sigOps in pay-to-script-hash transactions:
      |
      */
    pub fn get_sig_op_count_with_script_sig(&self, script_sig: &Script) -> u32 {
        
        todo!();
        /*
            if (!IsPayToScriptHash())
            return GetSigOpCount(true);

        // This is a pay-to-script-hash scriptPubKey;
        // get the last item that the scriptSig
        // pushes onto the stack:
        const_iterator pc = scriptSig.begin();
        std::vector<unsigned char> vData;
        while (pc < scriptSig.end())
        {
            opcodetype opcode;
            if (!scriptSig.GetOp(pc, opcode, vData))
                return 0;
            if (opcode > OP_16)
                return 0;
        }

        /// ... and return its opcount:
        CScript subscript(vData.begin(), vData.end());
        return subscript.GetSigOpCount(true);
        */
    }
    
    pub fn is_pay_to_script_hash(&self) -> bool {
        
        todo!();
        /*
            // Extra-fast test for pay-to-script-hash CScripts:
        return (this->size() == 23 &&
                (*this)[0] == OP_HASH160 &&
                (*this)[1] == 0x14 &&
                (*this)[22] == OP_EQUAL);
        */
    }
    
    pub fn is_pay_to_witness_script_hash(&self) -> bool {
        
        todo!();
        /*
            // Extra-fast test for pay-to-witness-script-hash CScripts:
        return (this->size() == 34 &&
                (*this)[0] == OP_0 &&
                (*this)[1] == 0x20);
        */
    }

    /**
      | A witness program is any valid CScript
      | that consists of a 1-byte push opcode
      | followed by a data push between 2 and
      | 40 bytes.
      |
      */
    pub fn is_witness_program(&self, 
        version: &mut i32,
        program: &mut Vec<u8>) -> bool {
        
        todo!();
        /*
            if (this->size() < 4 || this->size() > 42) {
            return false;
        }
        if ((*this)[0] != OP_0 && ((*this)[0] < OP_1 || (*this)[0] > OP_16)) {
            return false;
        }
        if ((size_t)((*this)[1] + 2) == this->size()) {
            version = DecodeOP_N((opcodetype)(*this)[0]);
            program = std::vector<unsigned char>(this->begin() + 2, this->end());
            return true;
        }
        return false;
        */
    }
    
    /**
      | Called by IsStandardTx and P2SH/BIP62
      | VerifyScript (which makes it consensus-critical).
      |
      */
    pub fn is_push_only(&self, pc: Box<ScriptIterator>) -> bool {
        
        todo!();
        /*
            while (pc < end())
        {
            opcodetype opcode;
            if (!GetOp(pc, opcode))
                return false;
            // Note that IsPushOnly() *does* consider OP_RESERVED to be a
            // push-type opcode, however execution of OP_RESERVED fails, so
            // it's not relevant to P2SH/BIP62 as the scriptSig would fail prior to
            // the P2SH special validation code being executed.
            if (opcode > OP_16)
                return false;
        }
        return true;
        */
    }
    
    pub fn is_push_only_from_begin(&self) -> bool {
        
        todo!();
        /*
            return this->IsPushOnly(begin());
        */
    }
    
    /**
      | Check if the script contains valid OP_CODES
      |
      */
    pub fn has_valid_ops(&self) -> bool {
        
        todo!();
        /*
            CScript::const_iterator it = begin();
        while (it < end()) {
            opcodetype opcode;
            std::vector<unsigned char> item;
            if (!GetOp(it, opcode, item) || opcode > MAX_OPCODE || item.size() > MAX_SCRIPT_ELEMENT_SIZE) {
                return false;
            }
        }
        return true;
        */
    }
}

///----------------------
#[derive(Default,Clone,Serialize,Deserialize)]
pub struct ScriptWitness {

    /**
      | @note
      | 
      | this encodes the data elements being
      | pushed, rather than encoding them as
      | a CScript that pushes them.
      |
      */
    pub stack: Vec<Vec<u8>>,
}

impl ScriptWitness {

    pub fn is_null(&self) -> bool {
        
        self.stack.is_empty()
    }
    
    pub fn set_null(&mut self)  {
        
        self.stack.clear();
        self.stack.shrink_to_fit();
    }
    
    pub fn to_string(&self) -> String {
        
        let mut ret: String = "CScriptWitness(".to_string();

        for i in 0..self.stack.len() {

            if i != 0 {
                ret.push_str(", ");
            }

            ret.push_str(&hex_str(&self.stack[i]));
        }

        ret.push_str(")");

        ret
    }
}

//-------------------------------------------[.cpp/bitcoin/src/script/script.cpp]

pub fn get_op_name(opcode: usize) -> String {
    
    todo!();
        /*
            switch (opcode)
        {
        // push value
        case OP_0                      : return "0";
        case OP_PUSHDATA1              : return "OP_PUSHDATA1";
        case OP_PUSHDATA2              : return "OP_PUSHDATA2";
        case OP_PUSHDATA4              : return "OP_PUSHDATA4";
        case OP_1NEGATE                : return "-1";
        case OP_RESERVED               : return "OP_RESERVED";
        case OP_1                      : return "1";
        case OP_2                      : return "2";
        case OP_3                      : return "3";
        case OP_4                      : return "4";
        case OP_5                      : return "5";
        case OP_6                      : return "6";
        case OP_7                      : return "7";
        case OP_8                      : return "8";
        case OP_9                      : return "9";
        case OP_10                     : return "10";
        case OP_11                     : return "11";
        case OP_12                     : return "12";
        case OP_13                     : return "13";
        case OP_14                     : return "14";
        case OP_15                     : return "15";
        case OP_16                     : return "16";

        // control
        case OP_NOP                    : return "OP_NOP";
        case OP_VER                    : return "OP_VER";
        case OP_IF                     : return "OP_IF";
        case OP_NOTIF                  : return "OP_NOTIF";
        case OP_VERIF                  : return "OP_VERIF";
        case OP_VERNOTIF               : return "OP_VERNOTIF";
        case OP_ELSE                   : return "OP_ELSE";
        case OP_ENDIF                  : return "OP_ENDIF";
        case OP_VERIFY                 : return "OP_VERIFY";
        case OP_RETURN                 : return "OP_RETURN";

        // stack ops
        case OP_TOALTSTACK             : return "OP_TOALTSTACK";
        case OP_FROMALTSTACK           : return "OP_FROMALTSTACK";
        case OP_2DROP                  : return "OP_2DROP";
        case OP_2DUP                   : return "OP_2DUP";
        case OP_3DUP                   : return "OP_3DUP";
        case OP_2OVER                  : return "OP_2OVER";
        case OP_2ROT                   : return "OP_2ROT";
        case OP_2SWAP                  : return "OP_2SWAP";
        case OP_IFDUP                  : return "OP_IFDUP";
        case OP_DEPTH                  : return "OP_DEPTH";
        case OP_DROP                   : return "OP_DROP";
        case OP_DUP                    : return "OP_DUP";
        case OP_NIP                    : return "OP_NIP";
        case OP_OVER                   : return "OP_OVER";
        case OP_PICK                   : return "OP_PICK";
        case OP_ROLL                   : return "OP_ROLL";
        case OP_ROT                    : return "OP_ROT";
        case OP_SWAP                   : return "OP_SWAP";
        case OP_TUCK                   : return "OP_TUCK";

        // splice ops
        case OP_CAT                    : return "OP_CAT";
        case OP_SUBSTR                 : return "OP_SUBSTR";
        case OP_LEFT                   : return "OP_LEFT";
        case OP_RIGHT                  : return "OP_RIGHT";
        case OP_SIZE                   : return "OP_SIZE";

        // bit logic
        case OP_INVERT                 : return "OP_INVERT";
        case OP_AND                    : return "OP_AND";
        case OP_OR                     : return "OP_OR";
        case OP_XOR                    : return "OP_XOR";
        case OP_EQUAL                  : return "OP_EQUAL";
        case OP_EQUALVERIFY            : return "OP_EQUALVERIFY";
        case OP_RESERVED1              : return "OP_RESERVED1";
        case OP_RESERVED2              : return "OP_RESERVED2";

        // numeric
        case OP_1ADD                   : return "OP_1ADD";
        case OP_1SUB                   : return "OP_1SUB";
        case OP_2MUL                   : return "OP_2MUL";
        case OP_2DIV                   : return "OP_2DIV";
        case OP_NEGATE                 : return "OP_NEGATE";
        case OP_ABS                    : return "OP_ABS";
        case OP_NOT                    : return "OP_NOT";
        case OP_0NOTEQUAL              : return "OP_0NOTEQUAL";
        case OP_ADD                    : return "OP_ADD";
        case OP_SUB                    : return "OP_SUB";
        case OP_MUL                    : return "OP_MUL";
        case OP_DIV                    : return "OP_DIV";
        case OP_MOD                    : return "OP_MOD";
        case OP_LSHIFT                 : return "OP_LSHIFT";
        case OP_RSHIFT                 : return "OP_RSHIFT";
        case OP_BOOLAND                : return "OP_BOOLAND";
        case OP_BOOLOR                 : return "OP_BOOLOR";
        case OP_NUMEQUAL               : return "OP_NUMEQUAL";
        case OP_NUMEQUALVERIFY         : return "OP_NUMEQUALVERIFY";
        case OP_NUMNOTEQUAL            : return "OP_NUMNOTEQUAL";
        case OP_LESSTHAN               : return "OP_LESSTHAN";
        case OP_GREATERTHAN            : return "OP_GREATERTHAN";
        case OP_LESSTHANOREQUAL        : return "OP_LESSTHANOREQUAL";
        case OP_GREATERTHANOREQUAL     : return "OP_GREATERTHANOREQUAL";
        case OP_MIN                    : return "OP_MIN";
        case OP_MAX                    : return "OP_MAX";
        case OP_WITHIN                 : return "OP_WITHIN";

        // crypto
        case OP_RIPEMD160              : return "OP_RIPEMD160";
        case OP_SHA1                   : return "OP_SHA1";
        case OP_SHA256                 : return "OP_SHA256";
        case OP_HASH160                : return "OP_HASH160";
        case OP_HASH256                : return "OP_HASH256";
        case OP_CODESEPARATOR          : return "OP_CODESEPARATOR";
        case OP_CHECKSIG               : return "OP_CHECKSIG";
        case OP_CHECKSIGVERIFY         : return "OP_CHECKSIGVERIFY";
        case OP_CHECKMULTISIG          : return "OP_CHECKMULTISIG";
        case OP_CHECKMULTISIGVERIFY    : return "OP_CHECKMULTISIGVERIFY";

        // expansion
        case OP_NOP1                   : return "OP_NOP1";
        case OP_CHECKLOCKTIMEVERIFY    : return "OP_CHECKLOCKTIMEVERIFY";
        case OP_CHECKSEQUENCEVERIFY    : return "OP_CHECKSEQUENCEVERIFY";
        case OP_NOP4                   : return "OP_NOP4";
        case OP_NOP5                   : return "OP_NOP5";
        case OP_NOP6                   : return "OP_NOP6";
        case OP_NOP7                   : return "OP_NOP7";
        case OP_NOP8                   : return "OP_NOP8";
        case OP_NOP9                   : return "OP_NOP9";
        case OP_NOP10                  : return "OP_NOP10";

        // Opcode added by BIP 342 (Tapscript)
        case OP_CHECKSIGADD            : return "OP_CHECKSIGADD";

        case OP_INVALIDOPCODE          : return "OP_INVALIDOPCODE";

        default:
            return "OP_UNKNOWN";
        }
        */
}

pub fn get_script_op(
        pc:         &mut Box<ScriptIterator>,
        end:        Box<ScriptIterator>,
        opcode_ret: &mut OpcodeType,
        pvch_ret:   *mut Vec<u8>) -> bool {
    
    todo!();
        /*
            opcodeRet = OP_INVALIDOPCODE;
        if (pvchRet)
            pvchRet->clear();
        if (pc >= end)
            return false;

        // Read instruction
        if (end - pc < 1)
            return false;
        unsigned int opcode = *pc++;

        // Immediate operand
        if (opcode <= OP_PUSHDATA4)
        {
            unsigned int nSize = 0;
            if (opcode < OP_PUSHDATA1)
            {
                nSize = opcode;
            }
            else if (opcode == OP_PUSHDATA1)
            {
                if (end - pc < 1)
                    return false;
                nSize = *pc++;
            }
            else if (opcode == OP_PUSHDATA2)
            {
                if (end - pc < 2)
                    return false;
                nSize = ReadLE16(&pc[0]);
                pc += 2;
            }
            else if (opcode == OP_PUSHDATA4)
            {
                if (end - pc < 4)
                    return false;
                nSize = ReadLE32(&pc[0]);
                pc += 4;
            }
            if (end - pc < 0 || (unsigned int)(end - pc) < nSize)
                return false;
            if (pvchRet)
                pvchRet->assign(pc, pc + nSize);
            pc += nSize;
        }

        opcodeRet = static_cast<opcodetype>(opcode);
        return true;
        */
}

/**
  | Test for OP_SUCCESSx opcodes as defined
  | by BIP342.
  |
  */
pub fn is_op_success(opcode: &OpcodeType) -> bool {
    
    todo!();
        /*
            return opcode == 80 || opcode == 98 || (opcode >= 126 && opcode <= 129) ||
               (opcode >= 131 && opcode <= 134) || (opcode >= 137 && opcode <= 138) ||
               (opcode >= 141 && opcode <= 142) || (opcode >= 149 && opcode <= 153) ||
               (opcode >= 187 && opcode <= 254);
        */
}
