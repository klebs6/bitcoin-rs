crate::ix!();

pub trait VerifyECDSASignature {
    fn verify_ecdsa_signature(&self, 
        vch_sig:     &Vec<u8>,
        vch_pub_key: &crate::PubKey,
        sighash:     &u256) -> bool;
}

pub trait VerifySchnorrSignature {
    fn verify_schnorr_signature(&self, 
        sig:     &[u8],
        pubkey:  &crate::XOnlyPubKey,
        sighash: &u256) -> bool;
}

//-------------------------------------------[.cpp/bitcoin/src/script/interpreter.h]

/*
  | Signature hash types/flags
  |
  */

pub const SIGHASH_ALL:          usize = 1;
pub const SIGHASH_NONE:         usize = 2;
pub const SIGHASH_SINGLE:       usize = 3;
pub const SIGHASH_ANYONECANPAY: usize = 0x80;

/**
  | Taproot only; implied when sighash
  | byte is missing, and equivalent to SIGHASH_ALL
  |
  */
pub const SIGHASH_DEFAULT:      usize = 0;
pub const SIGHASH_OUTPUT_MASK:  usize = 3;
pub const SIGHASH_INPUT_MASK:   usize = 0x80;

/**
  | Script verification flags.
  | 
  | All flags are intended to be soft forks:
  | the set of acceptable scripts under
  | flags (A | B) is a subset of the acceptable
  | scripts under flag (A).
  |
  */
bitflags!{
    pub struct ScriptVerificationFlags: u32 {

        const SCRIPT_VERIFY_NONE      = 0;

        /*
          | Evaluate P2SH subscripts (BIP16).
          |
          */
        const SCRIPT_VERIFY_P2SH      = 1 << 0;

        /*
          | Passing a non-strict-DER signature or one
          | with undefined hashtype to a checksig
          | operation causes script failure.
          |
          | Evaluating a pubkey that is not (0x04 + 64
          | bytes) or (0x02 or 0x03 + 32 bytes) by
          | checksig causes script failure.  (not used
          | or intended as a consensus rule).
          */
        const SCRIPT_VERIFY_STRICTENC = 1 << 1;

        /*
          | Passing a non-strict-DER signature
          | to a checksig operation causes script
          | failure (BIP62 rule 1)
          |
          */
        const SCRIPT_VERIFY_DERSIG    = 1 << 2;

        /*
          | Passing a non-strict-DER signature
          | or one with S > order/2 to a checksig operation
          | causes script failure (BIP62 rule 5).
          |
          */
        const SCRIPT_VERIFY_LOW_S     = 1 << 3;

        /*
          | verify dummy stack item consumed by
          | 
          | CHECKMULTISIG is of zero-length (BIP62
          | rule 7).
          |
          */
        const SCRIPT_VERIFY_NULLDUMMY = 1 << 4;

        /*
          | Using a non-push operator in the scriptSig
          | causes script failure (BIP62 rule 2).
          |
          */
        const SCRIPT_VERIFY_SIGPUSHONLY = 1 << 5;

        /*
          | Require minimal encodings for all push
          | operations (OP_0... OP_16, OP_1NEGATE where
          | possible, direct pushes up to 75 bytes,
          | OP_PUSHDATA up to 255 bytes, OP_PUSHDATA2
          | for anything larger). Evaluating any other
          | push causes the script to fail (BIP62 rule
          | 3).
          |
          | In addition, whenever a stack element is
          | interpreted as a number, it must be of
          | minimal length (BIP62 rule 4).
          */
        const SCRIPT_VERIFY_MINIMALDATA = 1 << 6;

        /*
          | Discourage use of NOPs reserved for
          | upgrades (NOP1-10)
          |
          | Provided so that nodes can avoid accepting
          | or mining transactions containing executed
          | NOP's whose meaning may change after
          | a soft-fork, thus rendering the script
          | invalid; with this flag set executing
          | discouraged NOPs fails the script.
          |
          | This verification flag will never be
          | a mandatory flag applied to scripts in
          | a block. NOPs that are not executed, e.g.
          | within an unexecuted IF ENDIF block, are
          | *not* rejected.  NOPs that have associated
          | forks to give them new meaning (CLTV, CSV)
          | are not subject to this rule.
          */
        const SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_NOPS  = 1 << 7;

        /*
          | Require that only a single stack element
          | remains after evaluation. This changes the
          | success criterion from
          |
          | "At least one stack element must remain,
          | and when interpreted as a boolean, it must
          | be true" to
          |
          | "Exactly one stack element must remain, and
          | when interpreted as a boolean, it must be
          | true".
          |
          | (BIP62 rule 6)
          |
          | Note: CLEANSTACK should never be used
          |       without P2SH or WITNESS.
          |
          | Note: WITNESS_V0 and TAPSCRIPT script
          |       execution have behavior similar to
          |       CLEANSTACK as part of their consensus
          |       rules. It is automatic there and does
          |       not need this flag.
          */
        const SCRIPT_VERIFY_CLEANSTACK = 1 << 8;

        /*
          | Verify CHECKLOCKTIMEVERIFY
          | 
          | See BIP65 for details.
          |
          */
        const SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY = 1 << 9;

        /*
          | support CHECKSEQUENCEVERIFY opcode
          | 
          | See BIP112 for details
          |
          */
        const SCRIPT_VERIFY_CHECKSEQUENCEVERIFY = 1 << 10;

        /*
          | Support segregated witness
          |
          */
        const SCRIPT_VERIFY_WITNESS = 1 << 11;

        /*
          | Making v1-v16 witness program non-standard
          |
          */
        const SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM = 1 << 12;

        /*
          | Segwit script only: Require the argument of
          | OP_IF/NOTIF to be exactly 0x01 or empty
          | vector
          |
          | Note: TAPSCRIPT script execution has
          |       behavior similar to MINIMALIF as part
          |       of its consensus rules. It is
          |       automatic there and does not depend
          |       on this flag.
          */
        const SCRIPT_VERIFY_MINIMALIF = 1 << 13;

        /*
          | Signature(s) must be empty vector if
          | a CHECK(MULTI)SIG operation failed
          |
          */
        const SCRIPT_VERIFY_NULLFAIL = 1 << 14;

        /*
          | Public keys in segregated witness scripts
          | must be compressed
          |
          */
        const SCRIPT_VERIFY_WITNESS_PUBKEYTYPE = 1 << 15;

        /*
          | Making OP_CODESEPARATOR and FindAndDelete
          | fail any non-segwit scripts
          |
          */
        const SCRIPT_VERIFY_CONST_SCRIPTCODE = 1 << 16;

        /*
          | Taproot/Tapscript validation (BIPs
          | 341 & 342)
          |
          */
        const SCRIPT_VERIFY_TAPROOT = 1 << 17;

        /*
          | Making unknown Taproot leaf versions
          | non-standard
          |
          */
        const SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_TAPROOT_VERSION = 1 << 18;

        /*
          | Making unknown OP_SUCCESS non-standard
          |
          */
        const SCRIPT_VERIFY_DISCOURAGE_OP_SUCCESS = 1 << 19;

        /*
          | Making unknown public key versions
          | (in BIP 342 scripts) non-standard
          |
          */
        const SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_PUBKEYTYPE = 1 << 20;

        /*
          | Constants to point to the highest flag
          | in use. Add new flags above this line.
          |
          */
        const SCRIPT_VERIFY_END_MARKER = 1 << 21;

        /*
           | Mandatory script verification flags
           | that all new blocks must comply with
           | for them to be valid. (but old blocks
           | may not comply with) Currently just
           | P2SH, but in the future other flags may
           | be added.
           | 
           | Failing one of these tests may trigger
           | a DoS ban - see CheckInputScripts()
           | for details.
           |
           */
        const MANDATORY_SCRIPT_VERIFY_FLAGS = Self::SCRIPT_VERIFY_P2SH.bits;

        /*
           | Standard script verification flags
           | that standard transactions will comply
           | with. However scripts violating these
           | flags may still be present in valid blocks
           | and we must accept those blocks.
           |
           */
        const STANDARD_SCRIPT_VERIFY_FLAGS = 
            Self::MANDATORY_SCRIPT_VERIFY_FLAGS.bits 
            | Self::SCRIPT_VERIFY_DERSIG.bits 
            | Self::SCRIPT_VERIFY_STRICTENC.bits 
            | Self::SCRIPT_VERIFY_MINIMALDATA.bits 
            | Self::SCRIPT_VERIFY_NULLDUMMY.bits 
            | Self::SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_NOPS.bits 
            | Self::SCRIPT_VERIFY_CLEANSTACK.bits 
            | Self::SCRIPT_VERIFY_MINIMALIF.bits 
            | Self::SCRIPT_VERIFY_NULLFAIL.bits 
            | Self::SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY.bits 
            | Self::SCRIPT_VERIFY_CHECKSEQUENCEVERIFY.bits 
            | Self::SCRIPT_VERIFY_LOW_S.bits 
            | Self::SCRIPT_VERIFY_WITNESS.bits 
            | Self::SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM.bits 
            | Self::SCRIPT_VERIFY_WITNESS_PUBKEYTYPE.bits 
            | Self::SCRIPT_VERIFY_CONST_SCRIPTCODE.bits 
            | Self::SCRIPT_VERIFY_TAPROOT.bits 
            | Self::SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_TAPROOT_VERSION.bits 
            | Self::SCRIPT_VERIFY_DISCOURAGE_OP_SUCCESS.bits 
            | Self::SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_PUBKEYTYPE.bits;

        /*
          | For convenience, standard but not mandatory
          | verify flags.
          |
          */
        const STANDARD_NOT_MANDATORY_VERIFY_FLAGS = 
            Self::STANDARD_SCRIPT_VERIFY_FLAGS.bits 
            & !Self::MANDATORY_SCRIPT_VERIFY_FLAGS.bits;
    }
}

pub enum SigVersion
{
    /**
      | Bare scripts and BIP16 P2SH-wrapped
      | redeemscripts
      |
      */
    BASE = 0,        

    /**
      | Witness v0 (P2WPKH and P2WSH); see BIP
      | 141
      |
      */
    WITNESS_V0 = 1,  

    /**
      | Witness v1 with 32-byte program, not
      | BIP16 P2SH-wrapped, key path spending;
      | see BIP 341
      |
      */
    TAPROOT = 2,     

    /**
      | Witness v1 with 32-byte program, not
      | BIP16 P2SH-wrapped, script path spending,
      | leaf version 0xc0; see BIP 342
      |
      */
    TAPSCRIPT = 3,   
}

///---------------------------
pub struct ScriptExecutionData {

    /**
      | Whether m_tapleaf_hash is initialized.
      |
      */
    tapleaf_hash_init:           bool, // default = false

    /**
      | The tapleaf hash.
      |
      */
    tapleaf_hash:                u256,

    /**
      | Whether m_codeseparator_pos is initialized.
      |
      */
    codeseparator_pos_init:      bool, // default = false

    /**
      | Opcode position of the last executed
      | OP_CODESEPARATOR (or 0xFFFFFFFF if
      | none executed).
      |
      */
    codeseparator_pos:           u32,

    /**
      | Whether m_annex_present and (when
      | needed) m_annex_hash are initialized.
      |
      */
    annex_init:                  bool, // default = false

    /**
      | Whether an annex is present.
      |
      */
    annex_present:               bool,

    /**
      | Hash of the annex data.
      |
      */
    annex_hash:                  u256,

    /**
      | Whether m_validation_weight_left
      | is initialized.
      |
      */
    validation_weight_left_init: bool, // default = false

    /**
      | How much validation weight is left (decremented
      | for every successful non-empty signature
      | check).
      |
      */
    validation_weight_left:      i64,
}

/**
  | Signature hash sizes
  |
  */
pub const WITNESS_V0_SCRIPTHASH_SIZE:       usize = 32;
pub const WITNESS_V0_KEYHASH_SIZE:          usize = 20;
pub const WITNESS_V1_TAPROOT_SIZE:          usize = 32;
pub const TAPROOT_LEAF_MASK:                u8    = 0xfe;
pub const TAPROOT_LEAF_TAPSCRIPT:           u8    = 0xc0;
pub const TAPROOT_CONTROL_BASE_SIZE:        usize = 33;
pub const TAPROOT_CONTROL_NODE_SIZE:        usize = 32;
pub const TAPROOT_CONTROL_MAX_NODE_COUNT:   usize = 128;
pub const TAPROOT_CONTROL_MAX_SIZE:         usize = TAPROOT_CONTROL_BASE_SIZE + TAPROOT_CONTROL_NODE_SIZE * TAPROOT_CONTROL_MAX_NODE_COUNT;

//-------------------------------------------[.cpp/bitcoin/src/script/interpreter.cpp]

pub type valtype = Vec<u8>;

#[inline] pub fn set_success(ret: *mut ScriptError) -> bool {
    
    todo!();
        /*
            if (ret)
            *ret = SCRIPT_ERR_OK;
        return true;
        */
}

#[inline] pub fn set_error(
        ret:    *mut ScriptError,
        serror: ScriptError) -> bool {
    
    todo!();
        /*
            if (ret)
            *ret = serror;
        return false;
        */
}

pub fn cast_to_bool(vch: &ValType) -> bool {
    
    todo!();
        /*
            for (unsigned int i = 0; i < vch.size(); i++)
        {
            if (vch[i] != 0)
            {
                // Can be negative zero
                if (i == vch.size()-1 && vch[i] == 0x80)
                    return false;
                return true;
            }
        }
        return false;
        */
}

/**
  | Script is a stack machine (like Forth)
  | that evaluates a predicate returning
  | a bool indicating valid or not. There
  | are no loops.
  |
  */
macro_rules! stacktop {
    ($i:ident) => {
        /*
                (stack.at(stack.size()+(i)))
        */
    }
}

macro_rules! altstacktop {
    ($i:ident) => {
        /*
                (altstack.at(altstack.size()+(i)))
        */
    }
}

#[inline] pub fn popstack(stack: &mut Vec<ValType>)  {
    
    todo!();
        /*
            if (stack.empty())
            throw std::runtime_error("popstack(): stack empty");
        stack.pop_back();
        */
}

pub fn is_compressed_or_uncompressed_pub_key(vch_pub_key: &ValType) -> bool {
    
    todo!();
        /*
            if (vchPubKey.size() < CPubKey::COMPRESSED_SIZE) {
            //  Non-canonical public key: too short
            return false;
        }
        if (vchPubKey[0] == 0x04) {
            if (vchPubKey.size() != CPubKey::SIZE) {
                //  Non-canonical public key: invalid length for uncompressed key
                return false;
            }
        } else if (vchPubKey[0] == 0x02 || vchPubKey[0] == 0x03) {
            if (vchPubKey.size() != CPubKey::COMPRESSED_SIZE) {
                //  Non-canonical public key: invalid length for compressed key
                return false;
            }
        } else {
            //  Non-canonical public key: neither compressed nor uncompressed
            return false;
        }
        return true;
        */
}

pub fn is_compressed_pub_key(vch_pub_key: &ValType) -> bool {
    
    todo!();
        /*
            if (vchPubKey.size() != CPubKey::COMPRESSED_SIZE) {
            //  Non-canonical public key: invalid length for compressed key
            return false;
        }
        if (vchPubKey[0] != 0x02 && vchPubKey[0] != 0x03) {
            //  Non-canonical public key: invalid prefix for compressed key
            return false;
        }
        return true;
        */
}

/**
  | A canonical signature exists of: <30>
  | <total len> <02> <len R> <R> <02> <len
  | S> <S> <hashtype>
  | 
  | Where R and S are not negative (their
  | first byte has its highest bit not set),
  | and not excessively padded (do not start
  | with a 0 byte, unless an otherwise negative
  | number follows, in which case a single
  | 0 byte is necessary and even required).
  | 
  | See https://bitcointalk.org/index.php?topic=8392.msg127623#msg127623
  | 
  | This function is consensus-critical
  | since BIP66.
  |
  */
pub fn is_valid_signature_encoding(sig: &Vec<u8>) -> bool {
    
    todo!();
        /*
            // Format: 0x30 [total-length] 0x02 [R-length] [R] 0x02 [S-length] [S] [sighash]
        // * total-length: 1-byte length descriptor of everything that follows,
        //   excluding the sighash byte.
        // * R-length: 1-byte length descriptor of the R value that follows.
        // * R: arbitrary-length big-endian encoded R value. It must use the shortest
        //   possible encoding for a positive integer (which means no null bytes at
        //   the start, except a single one when the next byte has its highest bit set).
        // * S-length: 1-byte length descriptor of the S value that follows.
        // * S: arbitrary-length big-endian encoded S value. The same rules apply.
        // * sighash: 1-byte value indicating what data is hashed (not part of the DER
        //   signature)

        // Minimum and maximum size constraints.
        if (sig.size() < 9) return false;
        if (sig.size() > 73) return false;

        // A signature is of type 0x30 (compound).
        if (sig[0] != 0x30) return false;

        // Make sure the length covers the entire signature.
        if (sig[1] != sig.size() - 3) return false;

        // Extract the length of the R element.
        unsigned int lenR = sig[3];

        // Make sure the length of the S element is still inside the signature.
        if (5 + lenR >= sig.size()) return false;

        // Extract the length of the S element.
        unsigned int lenS = sig[5 + lenR];

        // Verify that the length of the signature matches the sum of the length
        // of the elements.
        if ((size_t)(lenR + lenS + 7) != sig.size()) return false;

        // Check whether the R element is an integer.
        if (sig[2] != 0x02) return false;

        // Zero-length integers are not allowed for R.
        if (lenR == 0) return false;

        // Negative numbers are not allowed for R.
        if (sig[4] & 0x80) return false;

        // Null bytes at the start of R are not allowed, unless R would
        // otherwise be interpreted as a negative number.
        if (lenR > 1 && (sig[4] == 0x00) && !(sig[5] & 0x80)) return false;

        // Check whether the S element is an integer.
        if (sig[lenR + 4] != 0x02) return false;

        // Zero-length integers are not allowed for S.
        if (lenS == 0) return false;

        // Negative numbers are not allowed for S.
        if (sig[lenR + 6] & 0x80) return false;

        // Null bytes at the start of S are not allowed, unless S would otherwise be
        // interpreted as a negative number.
        if (lenS > 1 && (sig[lenR + 6] == 0x00) && !(sig[lenR + 7] & 0x80)) return false;

        return true;
        */
}

pub fn is_low_der_signature(
        vch_sig: &ValType,
        serror:  *mut ScriptError) -> bool {
    
    todo!();
        /*
            if (!IsValidSignatureEncoding(vchSig)) {
            return set_error(serror, SCRIPT_ERR_SIG_DER);
        }
        // https://bitcoin.stackexchange.com/a/12556:
        //     Also note that inside transaction signatures, an extra hashtype byte
        //     follows the actual signature data.
        std::vector<unsigned char> vchSigCopy(vchSig.begin(), vchSig.begin() + vchSig.size() - 1);
        // If the S value is above the order of the curve divided by two, its
        // complement modulo the order could have been used instead, which is
        // one byte shorter when encoded correctly.
        if (!CPubKey::CheckLowS(vchSigCopy)) {
            return set_error(serror, SCRIPT_ERR_SIG_HIGH_S);
        }
        return true;
        */
}

pub fn is_defined_hashtype_signature(vch_sig: &ValType) -> bool {
    
    todo!();
        /*
            if (vchSig.size() == 0) {
            return false;
        }
        unsigned char nHashType = vchSig[vchSig.size() - 1] & (~(SIGHASH_ANYONECANPAY));
        if (nHashType < SIGHASH_ALL || nHashType > SIGHASH_SINGLE)
            return false;

        return true;
        */
}

pub fn check_signature_encoding(
        vch_sig: &Vec<u8>,
        flags:   u32,
        serror:  *mut ScriptError) -> bool {
    
    todo!();
        /*
            // Empty signature. Not strictly DER encoded, but allowed to provide a
        // compact way to provide an invalid signature for use with CHECK(MULTI)SIG
        if (vchSig.size() == 0) {
            return true;
        }
        if ((flags & (SCRIPT_VERIFY_DERSIG | SCRIPT_VERIFY_LOW_S | SCRIPT_VERIFY_STRICTENC)) != 0 && !IsValidSignatureEncoding(vchSig)) {
            return set_error(serror, SCRIPT_ERR_SIG_DER);
        } else if ((flags & SCRIPT_VERIFY_LOW_S) != 0 && !IsLowDERSignature(vchSig, serror)) {
            // serror is set
            return false;
        } else if ((flags & SCRIPT_VERIFY_STRICTENC) != 0 && !IsDefinedHashtypeSignature(vchSig)) {
            return set_error(serror, SCRIPT_ERR_SIG_HASHTYPE);
        }
        return true;
        */
}

pub fn check_pub_key_encoding(
        vch_pub_key: &ValType,
        flags:       u32,
        sigversion:  &SigVersion,
        serror:      *mut ScriptError) -> bool {
    
    todo!();
        /*
            if ((flags & SCRIPT_VERIFY_STRICTENC) != 0 && !IsCompressedOrUncompressedPubKey(vchPubKey)) {
            return set_error(serror, SCRIPT_ERR_PUBKEYTYPE);
        }
        // Only compressed keys are accepted in segwit
        if ((flags & SCRIPT_VERIFY_WITNESS_PUBKEYTYPE) != 0 && sigversion == SigVersion::WITNESS_V0 && !IsCompressedPubKey(vchPubKey)) {
            return set_error(serror, SCRIPT_ERR_WITNESS_PUBKEYTYPE);
        }
        return true;
        */
}

pub fn check_minimal_push(
        data:   &ValType,
        opcode: OpcodeType) -> bool {
    
    todo!();
        /*
            // Excludes OP_1NEGATE, OP_1-16 since they are by definition minimal
        assert(0 <= opcode && opcode <= OP_PUSHDATA4);
        if (data.size() == 0) {
            // Should have used OP_0.
            return opcode == OP_0;
        } else if (data.size() == 1 && data[0] >= 1 && data[0] <= 16) {
            // Should have used OP_1 .. OP_16.
            return false;
        } else if (data.size() == 1 && data[0] == 0x81) {
            // Should have used OP_1NEGATE.
            return false;
        } else if (data.size() <= 75) {
            // Must have used a direct push (opcode indicating number of bytes pushed + those bytes).
            return opcode == data.size();
        } else if (data.size() <= 255) {
            // Must have used OP_PUSHDATA.
            return opcode == OP_PUSHDATA1;
        } else if (data.size() <= 65535) {
            // Must have used OP_PUSHDATA2.
            return opcode == OP_PUSHDATA2;
        }
        return true;
        */
}

pub fn find_and_delete(
        script: &mut Script,
        b:      &Script) -> i32 {
    
    todo!();
        /*
            int nFound = 0;
        if (b.empty())
            return nFound;
        Script result;
        Script::const_iterator pc = script.begin(), pc2 = script.begin(), end = script.end();
        opcodetype opcode;
        do
        {
            result.insert(result.end(), pc2, pc);
            while (static_cast<size_t>(end - pc) >= b.size() && std::equal(b.begin(), b.end(), pc))
            {
                pc = pc + b.size();
                ++nFound;
            }
            pc2 = pc;
        }
        while (script.GetOp(pc, opcode));

        if (nFound > 0) {
            result.insert(result.end(), pc2, end);
            script = std::move(result);
        }

        return nFound;
        */
}

/**
  | A data type to abstract out the condition
  | stack during script execution.
  | 
  | Conceptually it acts like a vector of
  | booleans, one for each level of nested
  | 
  | IF/THEN/ELSE, indicating whether
  | we're in the active or inactive branch
  | of each.
  | 
  | The elements on the stack cannot be observed
  | individually; we only need to expose
  | whether the stack is empty and whether
  | or not any false values are present at
  | all. To implement OP_ELSE, a toggle_top
  | modifier is added, which flips the last
  | value without returning it.
  | 
  | This uses an optimized implementation
  | that does not materialize the actual
  | stack. Instead, it just stores the size
  | of the would-be stack, and the position
  | of the first false value in it.
  |
  */
pub struct ConditionStack {

    /**
      | The size of the implied stack.
      |
      */
    stack_size:      u32, // default = 0

    /**
      | The position of the first false value
      | on the implied stack, or NO_FALSE if
      | all true.
      |
      */
    first_false_pos: u32, // default = NO_FALSE
}

pub mod condition_stack {

    /**
      | A constant for m_first_false_pos to
      | indicate there are no falses.
      |
      */
    pub const NO_FALSE: u32 = u32::MAX;
}

impl ConditionStack {

    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return m_stack_size == 0;
        */
    }
    
    pub fn all_true(&self) -> bool {
        
        todo!();
        /*
            return m_first_false_pos == NO_FALSE;
        */
    }
    
    pub fn push_back(&mut self, f: bool)  {
        
        todo!();
        /*
            if (m_first_false_pos == NO_FALSE && !f) {
                // The stack consists of all true values, and a false is added.
                // The first false value will appear at the current size.
                m_first_false_pos = m_stack_size;
            }
            ++m_stack_size;
        */
    }
    
    pub fn pop_back(&mut self)  {
        
        todo!();
        /*
            assert(m_stack_size > 0);
            --m_stack_size;
            if (m_first_false_pos == m_stack_size) {
                // When popping off the first false value, everything becomes true.
                m_first_false_pos = NO_FALSE;
            }
        */
    }
    
    pub fn toggle_top(&mut self)  {
        
        todo!();
        /*
            assert(m_stack_size > 0);
            if (m_first_false_pos == NO_FALSE) {
                // The current stack is all true values; the first false will be the top.
                m_first_false_pos = m_stack_size - 1;
            } else if (m_first_false_pos == m_stack_size - 1) {
                // The top is the first false value; toggling it will make everything true.
                m_first_false_pos = NO_FALSE;
            } else {
                // There is a false value, but not on top. No action is needed as toggling
                // anything but the first false value is unobservable.
            }
        */
    }
}

pub fn eval_checksig_pre_tapscript(
        vch_sig:        &ValType,
        vch_pub_key:    &ValType,
        pbegincodehash: Box<ScriptIterator>,
        pend:           Box<ScriptIterator>,
        flags:          u32,
        checker:        &Box<dyn BaseSignatureChecker>,
        sigversion:     SigVersion,
        serror:         *mut ScriptError,
        success:        &mut bool) -> bool {
    
    todo!();
        /*
            assert(sigversion == SigVersion::BASE || sigversion == SigVersion::WITNESS_V0);

        // Subset of script starting at the most recent codeseparator
        Script scriptCode(pbegincodehash, pend);

        // Drop the signature in pre-segwit scripts but not segwit scripts
        if (sigversion == SigVersion::BASE) {
            int found = FindAndDelete(scriptCode, Script() << vchSig);
            if (found > 0 && (flags & SCRIPT_VERIFY_CONST_SCRIPTCODE))
                return set_error(serror, SCRIPT_ERR_SIG_FINDANDDELETE);
        }

        if (!CheckSignatureEncoding(vchSig, flags, serror) || !CheckPubKeyEncoding(vchPubKey, flags, sigversion, serror)) {
            //serror is set
            return false;
        }
        fSuccess = checker.CheckECDSASignature(vchSig, vchPubKey, scriptCode, sigversion);

        if (!fSuccess && (flags & SCRIPT_VERIFY_NULLFAIL) && vchSig.size())
            return set_error(serror, SCRIPT_ERR_SIG_NULLFAIL);

        return true;
        */
}

pub fn eval_checksig_tapscript(
        sig:        &ValType,
        pubkey:     &ValType,
        execdata:   &mut ScriptExecutionData,
        flags:      u32,
        checker:    &Box<dyn BaseSignatureChecker>,
        sigversion: SigVersion,
        serror:     *mut ScriptError,
        success:    &mut bool) -> bool {
    
    todo!();
        /*
            assert(sigversion == SigVersion::TAPSCRIPT);

        /*
         *  The following validation sequence is consensus critical. Please note how --
         *    upgradable public key versions precede other rules;
         *    the script execution fails when using empty signature with invalid public key;
         *    the script execution fails when using non-empty invalid signature.
         */
        success = !sig.empty();
        if (success) {
            // Implement the sigops/witnesssize ratio test.
            // Passing with an upgradable public key version is also counted.
            assert(execdata.m_validation_weight_left_init);
            execdata.m_validation_weight_left -= VALIDATION_WEIGHT_PER_SIGOP_PASSED;
            if (execdata.m_validation_weight_left < 0) {
                return set_error(serror, SCRIPT_ERR_TAPSCRIPT_VALIDATION_WEIGHT);
            }
        }
        if (pubkey.size() == 0) {
            return set_error(serror, SCRIPT_ERR_PUBKEYTYPE);
        } else if (pubkey.size() == 32) {
            if (success && !checker.CheckSchnorrSignature(sig, pubkey, sigversion, execdata, serror)) {
                return false; // serror is set
            }
        } else {
            /*
             *  New public key version softforks should be defined before this `else` block.
             *  Generally, the new code should not do anything but failing the script execution. To avoid
             *  consensus bugs, it should not modify any existing values (including `success`).
             */
            if ((flags & SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_PUBKEYTYPE) != 0) {
                return set_error(serror, SCRIPT_ERR_DISCOURAGE_UPGRADABLE_PUBKEYTYPE);
            }
        }

        return true;
        */
}

/**
  | Helper for OP_CHECKSIG, OP_CHECKSIGVERIFY,
  | and (in Tapscript) OP_CHECKSIGADD.
  | 
  | A return value of false means the script
  | fails entirely. When true is returned,
  | the success variable indicates whether
  | the signature check itself succeeded.
  |
  */
pub fn eval_checksig(
        sig:            &ValType,
        pubkey:         &ValType,
        pbegincodehash: Box<ScriptIterator>,
        pend:           Box<ScriptIterator>,
        execdata:       &mut ScriptExecutionData,
        flags:          u32,
        checker:        &Box<dyn BaseSignatureChecker>,
        sigversion:     SigVersion,
        serror:         *mut ScriptError,
        success:        &mut bool) -> bool {
    
    todo!();
        /*
            switch (sigversion) {
        case SigVersion::BASE:
        case SigVersion::WITNESS_V0:
            return EvalChecksigPreTapscript(sig, pubkey, pbegincodehash, pend, flags, checker, sigversion, serror, success);
        case SigVersion::TAPSCRIPT:
            return EvalChecksigTapscript(sig, pubkey, execdata, flags, checker, sigversion, serror, success);
        case SigVersion::TAPROOT:
            // Key path spending in Taproot has no script, so this is unreachable.
            break;
        }
        assert(false);
        */
}

pub fn eval_script_with_execdata(
        stack:      &mut Vec<Vec<u8>>,
        script:     &Script,
        flags:      u32,
        checker:    &Box<dyn BaseSignatureChecker>,
        sigversion: SigVersion,
        execdata:   &mut ScriptExecutionData,
        serror:     Option<*mut ScriptError>) -> bool {
    
    todo!();
        /*
            static const CScriptNum bnZero(0);
        static const CScriptNum bnOne(1);
        // static const CScriptNum bnFalse(0);
        // static const CScriptNum bnTrue(1);
        static const valtype vchFalse(0);
        // static const valtype vchZero(0);
        static const valtype vchTrue(1, 1);

        // sigversion cannot be TAPROOT here, as it admits no script execution.
        assert(sigversion == SigVersion::BASE || sigversion == SigVersion::WITNESS_V0 || sigversion == SigVersion::TAPSCRIPT);

        Script::const_iterator pc = script.begin();
        Script::const_iterator pend = script.end();
        Script::const_iterator pbegincodehash = script.begin();
        opcodetype opcode;
        valtype vchPushValue;
        ConditionStack vfExec;
        std::vector<valtype> altstack;
        set_error(serror, SCRIPT_ERR_UNKNOWN_ERROR);
        if ((sigversion == SigVersion::BASE || sigversion == SigVersion::WITNESS_V0) && script.size() > MAX_SCRIPT_SIZE) {
            return set_error(serror, SCRIPT_ERR_SCRIPT_SIZE);
        }
        int nOpCount = 0;
        bool fRequireMinimal = (flags & SCRIPT_VERIFY_MINIMALDATA) != 0;
        uint32_t opcode_pos = 0;
        execdata.m_codeseparator_pos = 0xFFFFFFFFUL;
        execdata.m_codeseparator_pos_init = true;

        try
        {
            for (; pc < pend; ++opcode_pos) {
                bool fExec = vfExec.all_true();

                //
                // Read instruction
                //
                if (!script.GetOp(pc, opcode, vchPushValue))
                    return set_error(serror, SCRIPT_ERR_BAD_OPCODE);
                if (vchPushValue.size() > MAX_SCRIPT_ELEMENT_SIZE)
                    return set_error(serror, SCRIPT_ERR_PUSH_SIZE);

                if (sigversion == SigVersion::BASE || sigversion == SigVersion::WITNESS_V0) {
                    // Note how OP_RESERVED does not count towards the opcode limit.
                    if (opcode > OP_16 && ++nOpCount > MAX_OPS_PER_SCRIPT) {
                        return set_error(serror, SCRIPT_ERR_OP_COUNT);
                    }
                }

                if (opcode == OP_CAT ||
                    opcode == OP_SUBSTR ||
                    opcode == OP_LEFT ||
                    opcode == OP_RIGHT ||
                    opcode == OP_INVERT ||
                    opcode == OP_AND ||
                    opcode == OP_OR ||
                    opcode == OP_XOR ||
                    opcode == OP_2MUL ||
                    opcode == OP_2DIV ||
                    opcode == OP_MUL ||
                    opcode == OP_DIV ||
                    opcode == OP_MOD ||
                    opcode == OP_LSHIFT ||
                    opcode == OP_RSHIFT)
                    return set_error(serror, SCRIPT_ERR_DISABLED_OPCODE); // Disabled opcodes (CVE-2010-5137).

                // With SCRIPT_VERIFY_CONST_SCRIPTCODE, OP_CODESEPARATOR in non-segwit script is rejected even in an unexecuted branch
                if (opcode == OP_CODESEPARATOR && sigversion == SigVersion::BASE && (flags & SCRIPT_VERIFY_CONST_SCRIPTCODE))
                    return set_error(serror, SCRIPT_ERR_OP_CODESEPARATOR);

                if (fExec && 0 <= opcode && opcode <= OP_PUSHDATA4) {
                    if (fRequireMinimal && !CheckMinimalPush(vchPushValue, opcode)) {
                        return set_error(serror, SCRIPT_ERR_MINIMALDATA);
                    }
                    stack.push_back(vchPushValue);
                } else if (fExec || (OP_IF <= opcode && opcode <= OP_ENDIF))
                switch (opcode)
                {
                    //
                    // Push value
                    //
                    case OP_1NEGATE:
                    case OP_1:
                    case OP_2:
                    case OP_3:
                    case OP_4:
                    case OP_5:
                    case OP_6:
                    case OP_7:
                    case OP_8:
                    case OP_9:
                    case OP_10:
                    case OP_11:
                    case OP_12:
                    case OP_13:
                    case OP_14:
                    case OP_15:
                    case OP_16:
                    {
                        // ( -- value)
                        CScriptNum bn((int)opcode - (int)(OP_1 - 1));
                        stack.push_back(bn.getvch());
                        // The result of these opcodes should always be the minimal way to push the data
                        // they push, so no need for a CheckMinimalPush here.
                    }
                    break;

                    //
                    // Control
                    //
                    case OP_NOP:
                        break;

                    case OP_CHECKLOCKTIMEVERIFY:
                    {
                        if (!(flags & SCRIPT_VERIFY_CHECKLOCKTIMEVERIFY)) {
                            // not enabled; treat as a NOP2
                            break;
                        }

                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        // Note that elsewhere numeric opcodes are limited to
                        // operands in the range -2**31+1 to 2**31-1, however it is
                        // legal for opcodes to produce results exceeding that
                        // range. This limitation is implemented by CScriptNum's
                        // default 4-byte limit.
                        //
                        // If we kept to that limit we'd have a year 2038 problem,
                        // even though the nLockTime field in transactions
                        // themselves is uint32 which only becomes meaningless
                        // after the year 2106.
                        //
                        // Thus as a special case we tell CScriptNum to accept up
                        // to 5-byte bignums, which are good until 2**39-1, well
                        // beyond the 2**32-1 limit of the nLockTime field itself.
                        const CScriptNum nLockTime(stacktop(-1), fRequireMinimal, 5);

                        // In the rare event that the argument may be < 0 due to
                        // some arithmetic being done first, you can always use
                        // 0 MAX CHECKLOCKTIMEVERIFY.
                        if (nLockTime < 0)
                            return set_error(serror, SCRIPT_ERR_NEGATIVE_LOCKTIME);

                        // Actually compare the specified lock time with the transaction.
                        if (!checker.CheckLockTime(nLockTime))
                            return set_error(serror, SCRIPT_ERR_UNSATISFIED_LOCKTIME);

                        break;
                    }

                    case OP_CHECKSEQUENCEVERIFY:
                    {
                        if (!(flags & SCRIPT_VERIFY_CHECKSEQUENCEVERIFY)) {
                            // not enabled; treat as a NOP3
                            break;
                        }

                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        // nSequence, like nLockTime, is a 32-bit unsigned integer
                        // field. See the comment in CHECKLOCKTIMEVERIFY regarding
                        // 5-byte numeric operands.
                        const CScriptNum nSequence(stacktop(-1), fRequireMinimal, 5);

                        // In the rare event that the argument may be < 0 due to
                        // some arithmetic being done first, you can always use
                        // 0 MAX CHECKSEQUENCEVERIFY.
                        if (nSequence < 0)
                            return set_error(serror, SCRIPT_ERR_NEGATIVE_LOCKTIME);

                        // To provide for future soft-fork extensibility, if the
                        // operand has the disabled lock-time flag set,
                        // CHECKSEQUENCEVERIFY behaves as a NOP.
                        if ((nSequence & CTxIn::SEQUENCE_LOCKTIME_DISABLE_FLAG) != 0)
                            break;

                        // Compare the specified sequence number with the input.
                        if (!checker.CheckSequence(nSequence))
                            return set_error(serror, SCRIPT_ERR_UNSATISFIED_LOCKTIME);

                        break;
                    }

                    case OP_NOP1: case OP_NOP4: case OP_NOP5:
                    case OP_NOP6: case OP_NOP7: case OP_NOP8: case OP_NOP9: case OP_NOP10:
                    {
                        if (flags & SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_NOPS)
                            return set_error(serror, SCRIPT_ERR_DISCOURAGE_UPGRADABLE_NOPS);
                    }
                    break;

                    case OP_IF:
                    case OP_NOTIF:
                    {
                        // <expression> if [statements] [else [statements]] endif
                        bool fValue = false;
                        if (fExec)
                        {
                            if (stack.size() < 1)
                                return set_error(serror, SCRIPT_ERR_UNBALANCED_CONDITIONAL);
                            valtype& vch = stacktop(-1);
                            // Tapscript requires minimal IF/NOTIF inputs as a consensus rule.
                            if (sigversion == SigVersion::TAPSCRIPT) {
                                // The input argument to the OP_IF and OP_NOTIF opcodes must be either
                                // exactly 0 (the empty vector) or exactly 1 (the one-byte vector with value 1).
                                if (vch.size() > 1 || (vch.size() == 1 && vch[0] != 1)) {
                                    return set_error(serror, SCRIPT_ERR_TAPSCRIPT_MINIMALIF);
                                }
                            }
                            // Under witness v0 rules it is only a policy rule, enabled through SCRIPT_VERIFY_MINIMALIF.
                            if (sigversion == SigVersion::WITNESS_V0 && (flags & SCRIPT_VERIFY_MINIMALIF)) {
                                if (vch.size() > 1)
                                    return set_error(serror, SCRIPT_ERR_MINIMALIF);
                                if (vch.size() == 1 && vch[0] != 1)
                                    return set_error(serror, SCRIPT_ERR_MINIMALIF);
                            }
                            fValue = CastToBool(vch);
                            if (opcode == OP_NOTIF)
                                fValue = !fValue;
                            popstack(stack);
                        }
                        vfExec.push_back(fValue);
                    }
                    break;

                    case OP_ELSE:
                    {
                        if (vfExec.empty())
                            return set_error(serror, SCRIPT_ERR_UNBALANCED_CONDITIONAL);
                        vfExec.toggle_top();
                    }
                    break;

                    case OP_ENDIF:
                    {
                        if (vfExec.empty())
                            return set_error(serror, SCRIPT_ERR_UNBALANCED_CONDITIONAL);
                        vfExec.pop_back();
                    }
                    break;

                    case OP_VERIFY:
                    {
                        // (true -- ) or
                        // (false -- false) and return
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        bool fValue = CastToBool(stacktop(-1));
                        if (fValue)
                            popstack(stack);
                        else
                            return set_error(serror, SCRIPT_ERR_VERIFY);
                    }
                    break;

                    case OP_RETURN:
                    {
                        return set_error(serror, SCRIPT_ERR_OP_RETURN);
                    }
                    break;

                    //
                    // Stack ops
                    //
                    case OP_TOALTSTACK:
                    {
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        altstack.push_back(stacktop(-1));
                        popstack(stack);
                    }
                    break;

                    case OP_FROMALTSTACK:
                    {
                        if (altstack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_ALTSTACK_OPERATION);
                        stack.push_back(altstacktop(-1));
                        popstack(altstack);
                    }
                    break;

                    case OP_2DROP:
                    {
                        // (x1 x2 -- )
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        popstack(stack);
                        popstack(stack);
                    }
                    break;

                    case OP_2DUP:
                    {
                        // (x1 x2 -- x1 x2 x1 x2)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch1 = stacktop(-2);
                        valtype vch2 = stacktop(-1);
                        stack.push_back(vch1);
                        stack.push_back(vch2);
                    }
                    break;

                    case OP_3DUP:
                    {
                        // (x1 x2 x3 -- x1 x2 x3 x1 x2 x3)
                        if (stack.size() < 3)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch1 = stacktop(-3);
                        valtype vch2 = stacktop(-2);
                        valtype vch3 = stacktop(-1);
                        stack.push_back(vch1);
                        stack.push_back(vch2);
                        stack.push_back(vch3);
                    }
                    break;

                    case OP_2OVER:
                    {
                        // (x1 x2 x3 x4 -- x1 x2 x3 x4 x1 x2)
                        if (stack.size() < 4)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch1 = stacktop(-4);
                        valtype vch2 = stacktop(-3);
                        stack.push_back(vch1);
                        stack.push_back(vch2);
                    }
                    break;

                    case OP_2ROT:
                    {
                        // (x1 x2 x3 x4 x5 x6 -- x3 x4 x5 x6 x1 x2)
                        if (stack.size() < 6)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch1 = stacktop(-6);
                        valtype vch2 = stacktop(-5);
                        stack.erase(stack.end()-6, stack.end()-4);
                        stack.push_back(vch1);
                        stack.push_back(vch2);
                    }
                    break;

                    case OP_2SWAP:
                    {
                        // (x1 x2 x3 x4 -- x3 x4 x1 x2)
                        if (stack.size() < 4)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        swap(stacktop(-4), stacktop(-2));
                        swap(stacktop(-3), stacktop(-1));
                    }
                    break;

                    case OP_IFDUP:
                    {
                        // (x - 0 | x x)
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch = stacktop(-1);
                        if (CastToBool(vch))
                            stack.push_back(vch);
                    }
                    break;

                    case OP_DEPTH:
                    {
                        // -- stacksize
                        CScriptNum bn(stack.size());
                        stack.push_back(bn.getvch());
                    }
                    break;

                    case OP_DROP:
                    {
                        // (x -- )
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        popstack(stack);
                    }
                    break;

                    case OP_DUP:
                    {
                        // (x -- x x)
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch = stacktop(-1);
                        stack.push_back(vch);
                    }
                    break;

                    case OP_NIP:
                    {
                        // (x1 x2 -- x2)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        stack.erase(stack.end() - 2);
                    }
                    break;

                    case OP_OVER:
                    {
                        // (x1 x2 -- x1 x2 x1)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch = stacktop(-2);
                        stack.push_back(vch);
                    }
                    break;

                    case OP_PICK:
                    case OP_ROLL:
                    {
                        // (xn ... x2 x1 x0 n - xn ... x2 x1 x0 xn)
                        // (xn ... x2 x1 x0 n - ... x2 x1 x0 xn)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        int n = CScriptNum(stacktop(-1), fRequireMinimal).getint();
                        popstack(stack);
                        if (n < 0 || n >= (int)stack.size())
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch = stacktop(-n-1);
                        if (opcode == OP_ROLL)
                            stack.erase(stack.end()-n-1);
                        stack.push_back(vch);
                    }
                    break;

                    case OP_ROT:
                    {
                        // (x1 x2 x3 -- x2 x3 x1)
                        //  x2 x1 x3  after first swap
                        //  x2 x3 x1  after second swap
                        if (stack.size() < 3)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        swap(stacktop(-3), stacktop(-2));
                        swap(stacktop(-2), stacktop(-1));
                    }
                    break;

                    case OP_SWAP:
                    {
                        // (x1 x2 -- x2 x1)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        swap(stacktop(-2), stacktop(-1));
                    }
                    break;

                    case OP_TUCK:
                    {
                        // (x1 x2 -- x2 x1 x2)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype vch = stacktop(-1);
                        stack.insert(stack.end()-2, vch);
                    }
                    break;

                    case OP_SIZE:
                    {
                        // (in -- in size)
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        CScriptNum bn(stacktop(-1).size());
                        stack.push_back(bn.getvch());
                    }
                    break;

                    //
                    // Bitwise logic
                    //
                    case OP_EQUAL:
                    case OP_EQUALVERIFY:
                    //case OP_NOTEQUAL: // use OP_NUMNOTEQUAL
                    {
                        // (x1 x2 - bool)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype& vch1 = stacktop(-2);
                        valtype& vch2 = stacktop(-1);
                        bool fEqual = (vch1 == vch2);
                        // OP_NOTEQUAL is disabled because it would be too easy to say
                        // something like n != 1 and have some wiseguy pass in 1 with extra
                        // zero bytes after it (numerically, 0x01 == 0x0001 == 0x000001)
                        //if (opcode == OP_NOTEQUAL)
                        //    fEqual = !fEqual;
                        popstack(stack);
                        popstack(stack);
                        stack.push_back(fEqual ? vchTrue : vchFalse);
                        if (opcode == OP_EQUALVERIFY)
                        {
                            if (fEqual)
                                popstack(stack);
                            else
                                return set_error(serror, SCRIPT_ERR_EQUALVERIFY);
                        }
                    }
                    break;

                    //
                    // Numeric
                    //
                    case OP_1ADD:
                    case OP_1SUB:
                    case OP_NEGATE:
                    case OP_ABS:
                    case OP_NOT:
                    case OP_0NOTEQUAL:
                    {
                        // (in -- out)
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        CScriptNum bn(stacktop(-1), fRequireMinimal);
                        switch (opcode)
                        {
                        case OP_1ADD:       bn += bnOne; break;
                        case OP_1SUB:       bn -= bnOne; break;
                        case OP_NEGATE:     bn = -bn; break;
                        case OP_ABS:        if (bn < bnZero) bn = -bn; break;
                        case OP_NOT:        bn = (bn == bnZero); break;
                        case OP_0NOTEQUAL:  bn = (bn != bnZero); break;
                        default:            assert(!"invalid opcode"); break;
                        }
                        popstack(stack);
                        stack.push_back(bn.getvch());
                    }
                    break;

                    case OP_ADD:
                    case OP_SUB:
                    case OP_BOOLAND:
                    case OP_BOOLOR:
                    case OP_NUMEQUAL:
                    case OP_NUMEQUALVERIFY:
                    case OP_NUMNOTEQUAL:
                    case OP_LESSTHAN:
                    case OP_GREATERTHAN:
                    case OP_LESSTHANOREQUAL:
                    case OP_GREATERTHANOREQUAL:
                    case OP_MIN:
                    case OP_MAX:
                    {
                        // (x1 x2 -- out)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        CScriptNum bn1(stacktop(-2), fRequireMinimal);
                        CScriptNum bn2(stacktop(-1), fRequireMinimal);
                        CScriptNum bn(0);
                        switch (opcode)
                        {
                        case OP_ADD:
                            bn = bn1 + bn2;
                            break;

                        case OP_SUB:
                            bn = bn1 - bn2;
                            break;

                        case OP_BOOLAND:             bn = (bn1 != bnZero && bn2 != bnZero); break;
                        case OP_BOOLOR:              bn = (bn1 != bnZero || bn2 != bnZero); break;
                        case OP_NUMEQUAL:            bn = (bn1 == bn2); break;
                        case OP_NUMEQUALVERIFY:      bn = (bn1 == bn2); break;
                        case OP_NUMNOTEQUAL:         bn = (bn1 != bn2); break;
                        case OP_LESSTHAN:            bn = (bn1 < bn2); break;
                        case OP_GREATERTHAN:         bn = (bn1 > bn2); break;
                        case OP_LESSTHANOREQUAL:     bn = (bn1 <= bn2); break;
                        case OP_GREATERTHANOREQUAL:  bn = (bn1 >= bn2); break;
                        case OP_MIN:                 bn = (bn1 < bn2 ? bn1 : bn2); break;
                        case OP_MAX:                 bn = (bn1 > bn2 ? bn1 : bn2); break;
                        default:                     assert(!"invalid opcode"); break;
                        }
                        popstack(stack);
                        popstack(stack);
                        stack.push_back(bn.getvch());

                        if (opcode == OP_NUMEQUALVERIFY)
                        {
                            if (CastToBool(stacktop(-1)))
                                popstack(stack);
                            else
                                return set_error(serror, SCRIPT_ERR_NUMEQUALVERIFY);
                        }
                    }
                    break;

                    case OP_WITHIN:
                    {
                        // (x min max -- out)
                        if (stack.size() < 3)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        CScriptNum bn1(stacktop(-3), fRequireMinimal);
                        CScriptNum bn2(stacktop(-2), fRequireMinimal);
                        CScriptNum bn3(stacktop(-1), fRequireMinimal);
                        bool fValue = (bn2 <= bn1 && bn1 < bn3);
                        popstack(stack);
                        popstack(stack);
                        popstack(stack);
                        stack.push_back(fValue ? vchTrue : vchFalse);
                    }
                    break;

                    //
                    // Crypto
                    //
                    case OP_RIPEMD160:
                    case OP_SHA1:
                    case OP_SHA256:
                    case OP_HASH160:
                    case OP_HASH256:
                    {
                        // (in -- hash)
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        valtype& vch = stacktop(-1);
                        valtype vchHash((opcode == OP_RIPEMD160 || opcode == OP_SHA1 || opcode == OP_HASH160) ? 20 : 32);
                        if (opcode == OP_RIPEMD160)
                            CRIPEMD160().Write(vch.data(), vch.size()).Finalize(vchHash.data());
                        else if (opcode == OP_SHA1)
                            CSHA1().Write(vch.data(), vch.size()).Finalize(vchHash.data());
                        else if (opcode == OP_SHA256)
                            CSHA256().Write(vch.data(), vch.size()).Finalize(vchHash.data());
                        else if (opcode == OP_HASH160)
                            CHash160().Write(vch).Finalize(vchHash);
                        else if (opcode == OP_HASH256)
                            CHash256().Write(vch).Finalize(vchHash);
                        popstack(stack);
                        stack.push_back(vchHash);
                    }
                    break;

                    case OP_CODESEPARATOR:
                    {
                        // If SCRIPT_VERIFY_CONST_SCRIPTCODE flag is set, use of OP_CODESEPARATOR is rejected in pre-segwit
                        // script, even in an unexecuted branch (this is checked above the opcode case statement).

                        // Hash starts after the code separator
                        pbegincodehash = pc;
                        execdata.m_codeseparator_pos = opcode_pos;
                    }
                    break;

                    case OP_CHECKSIG:
                    case OP_CHECKSIGVERIFY:
                    {
                        // (sig pubkey -- bool)
                        if (stack.size() < 2)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        valtype& vchSig    = stacktop(-2);
                        valtype& vchPubKey = stacktop(-1);

                        bool fSuccess = true;
                        if (!EvalChecksig(vchSig, vchPubKey, pbegincodehash, pend, execdata, flags, checker, sigversion, serror, fSuccess)) return false;
                        popstack(stack);
                        popstack(stack);
                        stack.push_back(fSuccess ? vchTrue : vchFalse);
                        if (opcode == OP_CHECKSIGVERIFY)
                        {
                            if (fSuccess)
                                popstack(stack);
                            else
                                return set_error(serror, SCRIPT_ERR_CHECKSIGVERIFY);
                        }
                    }
                    break;

                    case OP_CHECKSIGADD:
                    {
                        // OP_CHECKSIGADD is only available in Tapscript
                        if (sigversion == SigVersion::BASE || sigversion == SigVersion::WITNESS_V0) return set_error(serror, SCRIPT_ERR_BAD_OPCODE);

                        // (sig num pubkey -- num)
                        if (stack.size() < 3) return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        const valtype& sig = stacktop(-3);
                        const CScriptNum num(stacktop(-2), fRequireMinimal);
                        const valtype& pubkey = stacktop(-1);

                        bool success = true;
                        if (!EvalChecksig(sig, pubkey, pbegincodehash, pend, execdata, flags, checker, sigversion, serror, success)) return false;
                        popstack(stack);
                        popstack(stack);
                        popstack(stack);
                        stack.push_back((num + (success ? 1 : 0)).getvch());
                    }
                    break;

                    case OP_CHECKMULTISIG:
                    case OP_CHECKMULTISIGVERIFY:
                    {
                        if (sigversion == SigVersion::TAPSCRIPT) return set_error(serror, SCRIPT_ERR_TAPSCRIPT_CHECKMULTISIG);

                        // ([sig ...] num_of_signatures [pubkey ...] num_of_pubkeys -- bool)

                        int i = 1;
                        if ((int)stack.size() < i)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        int nKeysCount = CScriptNum(stacktop(-i), fRequireMinimal).getint();
                        if (nKeysCount < 0 || nKeysCount > MAX_PUBKEYS_PER_MULTISIG)
                            return set_error(serror, SCRIPT_ERR_PUBKEY_COUNT);
                        nOpCount += nKeysCount;
                        if (nOpCount > MAX_OPS_PER_SCRIPT)
                            return set_error(serror, SCRIPT_ERR_OP_COUNT);
                        int ikey = ++i;
                        // ikey2 is the position of last non-signature item in the stack. Top stack item = 1.
                        // With SCRIPT_VERIFY_NULLFAIL, this is used for cleanup if operation fails.
                        int ikey2 = nKeysCount + 2;
                        i += nKeysCount;
                        if ((int)stack.size() < i)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        int nSigsCount = CScriptNum(stacktop(-i), fRequireMinimal).getint();
                        if (nSigsCount < 0 || nSigsCount > nKeysCount)
                            return set_error(serror, SCRIPT_ERR_SIG_COUNT);
                        int isig = ++i;
                        i += nSigsCount;
                        if ((int)stack.size() < i)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);

                        // Subset of script starting at the most recent codeseparator
                        Script scriptCode(pbegincodehash, pend);

                        // Drop the signature in pre-segwit scripts but not segwit scripts
                        for (int k = 0; k < nSigsCount; k++)
                        {
                            valtype& vchSig = stacktop(-isig-k);
                            if (sigversion == SigVersion::BASE) {
                                int found = FindAndDelete(scriptCode, Script() << vchSig);
                                if (found > 0 && (flags & SCRIPT_VERIFY_CONST_SCRIPTCODE))
                                    return set_error(serror, SCRIPT_ERR_SIG_FINDANDDELETE);
                            }
                        }

                        bool fSuccess = true;
                        while (fSuccess && nSigsCount > 0)
                        {
                            valtype& vchSig    = stacktop(-isig);
                            valtype& vchPubKey = stacktop(-ikey);

                            // Note how this makes the exact order of pubkey/signature evaluation
                            // distinguishable by CHECKMULTISIG NOT if the STRICTENC flag is set.
                            // See the script_(in)valid tests for details.
                            if (!CheckSignatureEncoding(vchSig, flags, serror) || !CheckPubKeyEncoding(vchPubKey, flags, sigversion, serror)) {
                                // serror is set
                                return false;
                            }

                            // Check signature
                            bool fOk = checker.CheckECDSASignature(vchSig, vchPubKey, scriptCode, sigversion);

                            if (fOk) {
                                isig++;
                                nSigsCount--;
                            }
                            ikey++;
                            nKeysCount--;

                            // If there are more signatures left than keys left,
                            // then too many signatures have failed. Exit early,
                            // without checking any further signatures.
                            if (nSigsCount > nKeysCount)
                                fSuccess = false;
                        }

                        // Clean up stack of actual arguments
                        while (i-- > 1) {
                            // If the operation failed, we require that all signatures must be empty vector
                            if (!fSuccess && (flags & SCRIPT_VERIFY_NULLFAIL) && !ikey2 && stacktop(-1).size())
                                return set_error(serror, SCRIPT_ERR_SIG_NULLFAIL);
                            if (ikey2 > 0)
                                ikey2--;
                            popstack(stack);
                        }

                        // A bug causes CHECKMULTISIG to consume one extra argument
                        // whose contents were not checked in any way.
                        //
                        // Unfortunately this is a potential source of mutability,
                        // so optionally verify it is exactly equal to zero prior
                        // to removing it from the stack.
                        if (stack.size() < 1)
                            return set_error(serror, SCRIPT_ERR_INVALID_STACK_OPERATION);
                        if ((flags & SCRIPT_VERIFY_NULLDUMMY) && stacktop(-1).size())
                            return set_error(serror, SCRIPT_ERR_SIG_NULLDUMMY);
                        popstack(stack);

                        stack.push_back(fSuccess ? vchTrue : vchFalse);

                        if (opcode == OP_CHECKMULTISIGVERIFY)
                        {
                            if (fSuccess)
                                popstack(stack);
                            else
                                return set_error(serror, SCRIPT_ERR_CHECKMULTISIGVERIFY);
                        }
                    }
                    break;

                    default:
                        return set_error(serror, SCRIPT_ERR_BAD_OPCODE);
                }

                // Size limits
                if (stack.size() + altstack.size() > MAX_STACK_SIZE)
                    return set_error(serror, SCRIPT_ERR_STACK_SIZE);
            }
        }
        catch (...)
        {
            return set_error(serror, SCRIPT_ERR_UNKNOWN_ERROR);
        }

        if (!vfExec.empty())
            return set_error(serror, SCRIPT_ERR_UNBALANCED_CONDITIONAL);

        return set_success(serror);
        */
}

pub fn eval_script(
        stack:      &mut Vec<Vec<u8>>,
        script:     &Script,
        flags:      u32,
        checker:    &Box<dyn BaseSignatureChecker>,
        sigversion: SigVersion,
        serror:     Option<*mut ScriptError>) -> bool {
    
    todo!();
        /*
            ScriptExecutionData execdata;
        return EvalScript(stack, script, flags, checker, sigversion, execdata, serror);
        */
}

/**
  | Compute the (single) SHA256 of the concatenation
  | of all prevouts of a tx.
  |
  */
pub fn get_prevoutssha256<T>(tx_to: &T) -> u256 {

    todo!();
        /*
            CHashWriter ss(SER_GETHASH, 0);
        for (const auto& txin : txTo.vin) {
            ss << txin.prevout;
        }
        return ss.GetSHA256();
        */
}

/**
  | Compute the (single) SHA256 of the concatenation
  | of all nSequences of a tx.
  |
  */

pub fn get_sequencessha256<T>(tx_to: &T) -> u256 {

    todo!();
        /*
            CHashWriter ss(SER_GETHASH, 0);
        for (const auto& txin : txTo.vin) {
            ss << txin.nSequence;
        }
        return ss.GetSHA256();
        */
}

/**
  | Compute the (single) SHA256 of the concatenation
  | of all txouts of a tx.
  |
  */

pub fn get_outputssha256<T>(tx_to: &T) -> u256 {

    todo!();
        /*
            CHashWriter ss(SER_GETHASH, 0);
        for (const auto& txout : txTo.vout) {
            ss << txout;
        }
        return ss.GetSHA256();
        */
}

pub fn execute_witness_script(
        stack_span:  &[ValType],
        exec_script: &Script,
        flags:       u32,
        sigversion:  SigVersion,
        checker:     &Box<dyn BaseSignatureChecker>,
        execdata:    &mut ScriptExecutionData,
        serror:      *mut ScriptError) -> bool {
    
    todo!();
        /*
            std::vector<valtype> stack{stack_span.begin(), stack_span.end()};

        if (sigversion == SigVersion::TAPSCRIPT) {
            // OP_SUCCESSx processing overrides everything, including stack element size limits
            Script::const_iterator pc = exec_script.begin();
            while (pc < exec_script.end()) {
                opcodetype opcode;
                if (!exec_script.GetOp(pc, opcode)) {
                    // Note how this condition would not be reached if an unknown OP_SUCCESSx was found
                    return set_error(serror, SCRIPT_ERR_BAD_OPCODE);
                }
                // New opcodes will be listed here. May use a different sigversion to modify existing opcodes.
                if (IsOpSuccess(opcode)) {
                    if (flags & SCRIPT_VERIFY_DISCOURAGE_OP_SUCCESS) {
                        return set_error(serror, SCRIPT_ERR_DISCOURAGE_OP_SUCCESS);
                    }
                    return set_success(serror);
                }
            }

            // Tapscript enforces initial stack size limits (altstack is empty here)
            if (stack.size() > MAX_STACK_SIZE) return set_error(serror, SCRIPT_ERR_STACK_SIZE);
        }

        // Disallow stack item size > MAX_SCRIPT_ELEMENT_SIZE in witness stack
        for (const valtype& elem : stack) {
            if (elem.size() > MAX_SCRIPT_ELEMENT_SIZE) return set_error(serror, SCRIPT_ERR_PUSH_SIZE);
        }

        // Run the script interpreter.
        if (!EvalScript(stack, exec_script, flags, checker, sigversion, execdata, serror)) return false;

        // Scripts inside witness implicitly require cleanstack behaviour
        if (stack.size() != 1) return set_error(serror, SCRIPT_ERR_CLEANSTACK);
        if (!CastToBool(stack.back())) return set_error(serror, SCRIPT_ERR_EVAL_FALSE);
        return true;
        */
}

/**
  | Compute the BIP341 tapleaf hash from
  | leaf version & script.
  |
  */
pub fn compute_tapleaf_hash(
        leaf_version: u8,
        script:       &Script) -> u256 {
    
    todo!();
        /*
            return (CHashWriter(HASHER_TAPLEAF) << leaf_version << script).GetSHA256();
        */
}

/**
  | Compute the BIP341 taproot script tree
  | Merkle root from control block and leaf
  | hash.
  | 
  | Requires control block to have valid
  | length (33 + k*32, with k in {0,1,..,128}).
  |
  */
pub fn compute_taproot_merkle_root(
        control:      &[u8],
        tapleaf_hash: &u256) -> u256 {
    
    todo!();
        /*
            const int path_len = (control.size() - TAPROOT_CONTROL_BASE_SIZE) / TAPROOT_CONTROL_NODE_SIZE;
        uint256 k = tapleaf_hash;
        for (int i = 0; i < path_len; ++i) {
            CHashWriter ss_branch{HASHER_TAPBRANCH};
            Span<const unsigned char> node(control.data() + TAPROOT_CONTROL_BASE_SIZE + TAPROOT_CONTROL_NODE_SIZE * i, TAPROOT_CONTROL_NODE_SIZE);
            if (std::lexicographical_compare(k.begin(), k.end(), node.begin(), node.end())) {
                ss_branch << k << node;
            } else {
                ss_branch << node << k;
            }
            k = ss_branch.GetSHA256();
        }
        return k;
        */
}

pub fn verify_taproot_commitment(
        control:      &Vec<u8>,
        program:      &Vec<u8>,
        tapleaf_hash: &u256) -> bool {
    
    todo!();
        /*
            assert(control.size() >= TAPROOT_CONTROL_BASE_SIZE);
        assert(program.size() >= uint256::size());
        /// The internal pubkey (x-only, so no Y coordinate parity).
        const crate::XOnlyPubKey p{Span<const unsigned char>{control.data() + 1, control.data() + TAPROOT_CONTROL_BASE_SIZE}};
        /// The output pubkey (taken from the scriptPubKey).
        const crate::XOnlyPubKey q{program};
        // Compute the Merkle root from the leaf and the provided path.
        const uint256 merkle_root = ComputeTaprootMerkleRoot(control, tapleaf_hash);
        // Verify that the output pubkey matches the tweaked internal pubkey, after correcting for parity.
        return q.CheckTapTweak(p, merkle_root, control[0] & 1);
        */
}

pub fn verify_witness_program(
        witness:    &ScriptWitness,
        witversion: i32,
        program:    &Vec<u8>,
        flags:      u32,
        checker:    &Box<dyn BaseSignatureChecker>,
        serror:     *mut ScriptError,
        is_p2sh:    bool) -> bool {
    
    todo!();
        /*
            Script exec_script; /// Actually executed script (last stack item in P2WSH; implied P2PKH script in P2WPKH; leaf script in P2TR)
        Span<const valtype> stack{witness.stack};
        ScriptExecutionData execdata;

        if (witversion == 0) {
            if (program.size() == WITNESS_V0_SCRIPTHASH_SIZE) {
                // BIP141 P2WSH: 32-byte witness v0 program (which encodes SHA256(script))
                if (stack.size() == 0) {
                    return set_error(serror, SCRIPT_ERR_WITNESS_PROGRAM_WITNESS_EMPTY);
                }
                const valtype& script_bytes = SpanPopBack(stack);
                exec_script = Script(script_bytes.begin(), script_bytes.end());
                uint256 hash_exec_script;
                CSHA256().Write(exec_script.data(), exec_script.size()).Finalize(hash_exec_script.begin());
                if (memcmp(hash_exec_script.begin(), program.data(), 32)) {
                    return set_error(serror, SCRIPT_ERR_WITNESS_PROGRAM_MISMATCH);
                }
                return ExecuteWitnessScript(stack, exec_script, flags, SigVersion::WITNESS_V0, checker, execdata, serror);
            } else if (program.size() == WITNESS_V0_KEYHASH_SIZE) {
                // BIP141 P2WPKH: 20-byte witness v0 program (which encodes Hash160(pubkey))
                if (stack.size() != 2) {
                    return set_error(serror, SCRIPT_ERR_WITNESS_PROGRAM_MISMATCH); // 2 items in witness
                }
                exec_script << OP_DUP << OP_HASH160 << program << OP_EQUALVERIFY << OP_CHECKSIG;
                return ExecuteWitnessScript(stack, exec_script, flags, SigVersion::WITNESS_V0, checker, execdata, serror);
            } else {
                return set_error(serror, SCRIPT_ERR_WITNESS_PROGRAM_WRONG_LENGTH);
            }
        } else if (witversion == 1 && program.size() == WITNESS_V1_TAPROOT_SIZE && !is_p2sh) {
            // BIP341 Taproot: 32-byte non-P2SH witness v1 program (which encodes a P2C-tweaked pubkey)
            if (!(flags & SCRIPT_VERIFY_TAPROOT)) return set_success(serror);
            if (stack.size() == 0) return set_error(serror, SCRIPT_ERR_WITNESS_PROGRAM_WITNESS_EMPTY);
            if (stack.size() >= 2 && !stack.back().empty() && stack.back()[0] == ANNEX_TAG) {
                // Drop annex (this is non-standard; see IsWitnessStandard)
                const valtype& annex = SpanPopBack(stack);
                execdata.m_annex_hash = (CHashWriter(SER_GETHASH, 0) << annex).GetSHA256();
                execdata.m_annex_present = true;
            } else {
                execdata.m_annex_present = false;
            }
            execdata.m_annex_init = true;
            if (stack.size() == 1) {
                // Key path spending (stack size is 1 after removing optional annex)
                if (!checker.CheckSchnorrSignature(stack.front(), program, SigVersion::TAPROOT, execdata, serror)) {
                    return false; // serror is set
                }
                return set_success(serror);
            } else {
                // Script path spending (stack size is >1 after removing optional annex)
                const valtype& control = SpanPopBack(stack);
                const valtype& script_bytes = SpanPopBack(stack);
                exec_script = Script(script_bytes.begin(), script_bytes.end());
                if (control.size() < TAPROOT_CONTROL_BASE_SIZE || control.size() > TAPROOT_CONTROL_MAX_SIZE || ((control.size() - TAPROOT_CONTROL_BASE_SIZE) % TAPROOT_CONTROL_NODE_SIZE) != 0) {
                    return set_error(serror, SCRIPT_ERR_TAPROOT_WRONG_CONTROL_SIZE);
                }
                execdata.m_tapleaf_hash = ComputeTapleafHash(control[0] & TAPROOT_LEAF_MASK, exec_script);
                if (!VerifyTaprootCommitment(control, program, execdata.m_tapleaf_hash)) {
                    return set_error(serror, SCRIPT_ERR_WITNESS_PROGRAM_MISMATCH);
                }
                execdata.m_tapleaf_hash_init = true;
                if ((control[0] & TAPROOT_LEAF_MASK) == TAPROOT_LEAF_TAPSCRIPT) {
                    // Tapscript (leaf version 0xc0)
                    execdata.m_validation_weight_left = ::GetSerializeSize(witness.stack, PROTOCOL_VERSION) + VALIDATION_WEIGHT_OFFSET;
                    execdata.m_validation_weight_left_init = true;
                    return ExecuteWitnessScript(stack, exec_script, flags, SigVersion::TAPSCRIPT, checker, execdata, serror);
                }
                if (flags & SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_TAPROOT_VERSION) {
                    return set_error(serror, SCRIPT_ERR_DISCOURAGE_UPGRADABLE_TAPROOT_VERSION);
                }
                return set_success(serror);
            }
        } else {
            if (flags & SCRIPT_VERIFY_DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM) {
                return set_error(serror, SCRIPT_ERR_DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM);
            }
            // Other version/size/p2sh combinations return true for future softfork compatibility
            return true;
        }
        // There is intentionally no return statement here, to be able to use "control reaches end of non-c_void function" warnings to detect gaps in the logic above.
        */
}

pub fn verify_script_with_checker<C: BaseSignatureChecker>(
        script_sig:     &Script,
        script_pub_key: &Script,
        witness:        *const ScriptWitness,
        flags:          u32,
        checker:        &C,
        serror:         Option<*mut ScriptError>) -> bool {
    
    todo!();
        /*
            static const CScriptWitness emptyWitness;
        if (witness == nullptr) {
            witness = &emptyWitness;
        }
        bool hadWitness = false;

        set_error(serror, SCRIPT_ERR_UNKNOWN_ERROR);

        if ((flags & SCRIPT_VERIFY_SIGPUSHONLY) != 0 && !scriptSig.IsPushOnly()) {
            return set_error(serror, SCRIPT_ERR_SIG_PUSHONLY);
        }

        // scriptSig and scriptPubKey must be evaluated sequentially on the same stack
        // rather than being simply concatenated (see CVE-2010-5141)
        std::vector<std::vector<unsigned char> > stack, stackCopy;
        if (!EvalScript(stack, scriptSig, flags, checker, SigVersion::BASE, serror))
            // serror is set
            return false;
        if (flags & SCRIPT_VERIFY_P2SH)
            stackCopy = stack;
        if (!EvalScript(stack, scriptPubKey, flags, checker, SigVersion::BASE, serror))
            // serror is set
            return false;
        if (stack.empty())
            return set_error(serror, SCRIPT_ERR_EVAL_FALSE);
        if (CastToBool(stack.back()) == false)
            return set_error(serror, SCRIPT_ERR_EVAL_FALSE);

        // Bare witness programs
        int witnessversion;
        std::vector<unsigned char> witnessprogram;
        if (flags & SCRIPT_VERIFY_WITNESS) {
            if (scriptPubKey.IsWitnessProgram(witnessversion, witnessprogram)) {
                hadWitness = true;
                if (scriptSig.size() != 0) {
                    // The scriptSig must be _exactly_ Script(), otherwise we reintroduce malleability.
                    return set_error(serror, SCRIPT_ERR_WITNESS_MALLEATED);
                }
                if (!VerifyWitnessProgram(*witness, witnessversion, witnessprogram, flags, checker, serror, /* is_p2sh */ false)) {
                    return false;
                }
                // Bypass the cleanstack check at the end. The actual stack is obviously not clean
                // for witness programs.
                stack.resize(1);
            }
        }

        // Additional validation for spend-to-script-hash transactions:
        if ((flags & SCRIPT_VERIFY_P2SH) && scriptPubKey.IsPayToScriptHash())
        {
            // scriptSig must be literals-only or validation fails
            if (!scriptSig.IsPushOnly())
                return set_error(serror, SCRIPT_ERR_SIG_PUSHONLY);

            // Restore stack.
            swap(stack, stackCopy);

            // stack cannot be empty here, because if it was the
            // P2SH  HASH <> EQUAL  scriptPubKey would be evaluated with
            // an empty stack and the EvalScript above would return false.
            assert(!stack.empty());

            const valtype& pubKeySerialized = stack.back();
            Script pubKey2(pubKeySerialized.begin(), pubKeySerialized.end());
            popstack(stack);

            if (!EvalScript(stack, pubKey2, flags, checker, SigVersion::BASE, serror))
                // serror is set
                return false;
            if (stack.empty())
                return set_error(serror, SCRIPT_ERR_EVAL_FALSE);
            if (!CastToBool(stack.back()))
                return set_error(serror, SCRIPT_ERR_EVAL_FALSE);

            // P2SH witness program
            if (flags & SCRIPT_VERIFY_WITNESS) {
                if (pubKey2.IsWitnessProgram(witnessversion, witnessprogram)) {
                    hadWitness = true;
                    if (scriptSig != Script() << std::vector<unsigned char>(pubKey2.begin(), pubKey2.end())) {
                        // The scriptSig must be _exactly_ a single push of the redeemScript. Otherwise we
                        // reintroduce malleability.
                        return set_error(serror, SCRIPT_ERR_WITNESS_MALLEATED_P2SH);
                    }
                    if (!VerifyWitnessProgram(*witness, witnessversion, witnessprogram, flags, checker, serror, /* is_p2sh */ true)) {
                        return false;
                    }
                    // Bypass the cleanstack check at the end. The actual stack is obviously not clean
                    // for witness programs.
                    stack.resize(1);
                }
            }
        }

        // The CLEANSTACK check is only performed after potential P2SH evaluation,
        // as the non-P2SH evaluation of a P2SH script will obviously not result in
        // a clean stack (the P2SH inputs remain). The same holds for witness evaluation.
        if ((flags & SCRIPT_VERIFY_CLEANSTACK) != 0) {
            // Disallow CLEANSTACK without P2SH, as otherwise a switch CLEANSTACK->P2SH+CLEANSTACK
            // would be possible, which is not a softfork (and P2SH should be one).
            assert((flags & SCRIPT_VERIFY_P2SH) != 0);
            assert((flags & SCRIPT_VERIFY_WITNESS) != 0);
            if (stack.size() != 1) {
                return set_error(serror, SCRIPT_ERR_CLEANSTACK);
            }
        }

        if (flags & SCRIPT_VERIFY_WITNESS) {
            // We can't check for correct unexpected witness data if P2SH was off, so require
            // that WITNESS implies P2SH. Otherwise, going from WITNESS->P2SH+WITNESS would be
            // possible, which is not a softfork.
            assert((flags & SCRIPT_VERIFY_P2SH) != 0);
            if (!hadWitness && !witness->IsNull()) {
                return set_error(serror, SCRIPT_ERR_WITNESS_UNEXPECTED);
            }
        }

        return set_success(serror);
        */
}

pub fn witness_sig_ops(
        witversion: i32,
        witprogram: &Vec<u8>,
        witness:    &ScriptWitness) -> usize {
    
    todo!();
        /*
            if (witversion == 0) {
            if (witprogram.size() == WITNESS_V0_KEYHASH_SIZE)
                return 1;

            if (witprogram.size() == WITNESS_V0_SCRIPTHASH_SIZE && witness.stack.size() > 0) {
                Script subscript(witness.stack.back().begin(), witness.stack.back().end());
                return subscript.GetSigOpCount(true);
            }
        }

        // Future flags may be implemented here.
        return 0;
        */
}

pub fn count_witness_sig_ops(
        script_sig:     &Script,
        script_pub_key: &Script,
        witness:        *const ScriptWitness,
        flags:          u32) -> usize {
    
    todo!();
        /*
            static const CScriptWitness witnessEmpty;

        if ((flags & SCRIPT_VERIFY_WITNESS) == 0) {
            return 0;
        }
        assert((flags & SCRIPT_VERIFY_P2SH) != 0);

        int witnessversion;
        std::vector<unsigned char> witnessprogram;
        if (scriptPubKey.IsWitnessProgram(witnessversion, witnessprogram)) {
            return WitnessSigOps(witnessversion, witnessprogram, witness ? *witness : witnessEmpty);
        }

        if (scriptPubKey.IsPayToScriptHash() && scriptSig.IsPushOnly()) {
            Script::const_iterator pc = scriptSig.begin();
            std::vector<unsigned char> data;
            while (pc < scriptSig.end()) {
                opcodetype opcode;
                scriptSig.GetOp(pc, opcode, data);
            }
            Script subscript(data.begin(), data.end());
            if (subscript.IsWitnessProgram(witnessversion, witnessprogram)) {
                return WitnessSigOps(witnessversion, witnessprogram, witness ? *witness : witnessEmpty);
            }
        }

        return 0;
        */
}
