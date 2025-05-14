// ---------------- [ File: bitcoin-scripting/src/standard.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/standard.h]

pub const DEFAULT_ACCEPT_DATACARRIER: bool = true;

/**
  | A reference to a Script: the Hash160
  | of its serialization (see script.h)
  |
  */
pub struct ScriptID {
    base: BaseHash<u160>,
}

impl Default for ScriptID {
    
    fn default() -> Self {
        todo!();
        /*
        : base_hash(),

        
        */
    }
}

impl From<&u160> for ScriptID {

    fn from(in_: &u160) -> Self {
    
        todo!();
        /*
        : base_hash(in),

        
        */
    }
}

impl From<&Script> for ScriptID {

    fn from(in_: &Script) -> Self {
    
        todo!();
        /*
        : base_hash(Hash160(in)),

        
        */
    }
}
    
impl From<&ScriptHash> for ScriptID {

    fn from(in_: &ScriptHash) -> Self {
    
        todo!();
        /*
            : BaseHash(static_cast<u160>(in))
        */
    }
}

/**
  | Default setting for nMaxDatacarrierBytes.
  | 80 bytes of data, +1 for OP_RETURN, +2
  | for the pushdata opcodes.
  |
  */
pub const MAX_OP_RETURN_RELAY: u32 = 83;

/**
  | A data carrying output is an unspendable
  | output containing data. The script
  | type is designated as TxoutType::NULL_DATA.
  |
  */
lazy_static!{
    /*
    extern bool fAcceptDatacarrier;
    */
}

/**
  | Maximum size of TxoutType::NULL_DATA
  | scripts that this node considers standard.
  |
  */
lazy_static!{
    /*
    extern unsigned nMaxDatacarrierBytes;
    */
}

#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum TxoutType {

    NONSTANDARD,

    /**
      | 'standard' transaction types:
      |
      */
    PUBKEY,
    PUBKEYHASH,
    SCRIPTHASH,
    MULTISIG,

    /**
      | unspendable OP_RETURN script that
      | carries data
      |
      */
    NULL_DATA, 
    WITNESS_V0_SCRIPTHASH,
    WITNESS_V0_KEYHASH,
    WITNESS_V1_TAPROOT,

    /**
      | Only for Witness versions not already
      | defined above
      |
      */
    WITNESS_UNKNOWN, 
}

pub struct NoDestination { }

impl PartialEq<NoDestination> for NoDestination {
    
    #[inline] fn eq(&self, other: &NoDestination) -> bool {
        todo!();
        /*
            return true;
        */
    }
}

impl Eq for NoDestination {}

impl Ord for NoDestination {
    
    #[inline] fn cmp(&self, other: &NoDestination) -> Ordering {
        todo!();
        /*
            return true;
        */
    }
}

impl PartialOrd<NoDestination> for NoDestination {
    #[inline] fn partial_cmp(&self, other: &NoDestination) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

///-----------------------
pub struct PKHash {
    base: BaseHash<u160>,
}

impl Default for PKHash {
    
    fn default() -> Self {
        todo!();
        /*
        : base_hash(),
        */
    }
}

impl From<&u160> for PKHash {
    
    fn from(hash: &u160) -> Self {
    
        todo!();
        /*
        : base_hash(hash),
        */
    }
}

///---------------------
pub struct ScriptHash {
    base: BaseHash<u160>,
}

impl Default for ScriptHash {
    
    fn default() -> Self {
        todo!();
        /*
        : base_hash(),
        */
    }
}

impl From<&u160> for ScriptHash {
    
    fn from(hash: &u160) -> Self {
    
        todo!();
        /*
        : base_hash(hash),
        */
    }
}

///---------------------
pub struct WitnessV0ScriptHash {
    base: BaseHash<u256>,
}

impl Default for WitnessV0ScriptHash {
    
    fn default() -> Self {
        todo!();
        /*
        : base_hash(),
        */
    }
}

impl From<&u256> for WitnessV0ScriptHash {
    
    fn from(hash: &u256) -> Self {
    
        todo!();
        /*
        : base_hash(hash),
        */
    }
}

///---------------------
pub struct WitnessV0KeyHash {
    base: BaseHash<u160>,
}

impl Default for WitnessV0KeyHash {
    
    fn default() -> Self {
        todo!();
        /*
        : base_hash(),
        */
    }
}

impl From<&u160> for WitnessV0KeyHash {
    
    fn from(hash: &u160) -> Self {
    
        todo!();
        /*
        : base_hash(hash),
        */
    }
}

///---------------------
pub struct WitnessV1Taproot {
    base: XOnlyPubKey,
}

impl Default for WitnessV1Taproot {
    
    fn default() -> Self {
        todo!();
        /*
        : x_only_pub_key(),
        */
    }
}

impl From<&XOnlyPubKey> for WitnessV1Taproot {
    
    fn from(xpk: &XOnlyPubKey) -> Self {
    
        todo!();
        /*
        : x_only_pub_key(xpk),
        */
    }
}

/**
  | TxDestination subtype to encode any
  | future Witness version
  |
  */
pub struct WitnessUnknown {
    version: u32,
    length:  u32,
    program: [u8; 40],
}

impl PartialEq<WitnessUnknown> for WitnessUnknown {
    
    #[inline] fn eq(&self, other: &WitnessUnknown) -> bool {
        todo!();
        /*
            if (w1.version != w2.version) return false;
            if (w1.length != w2.length) return false;
            return std::equal(w1.program, w1.program + w1.length, w2.program);
        */
    }
}

impl Eq for WitnessUnknown {}

impl Ord for WitnessUnknown {
    
    #[inline] fn cmp(&self, other: &WitnessUnknown) -> Ordering {
        todo!();
        /*
            if (w1.version < w2.version) return true;
            if (w1.version > w2.version) return false;
            if (w1.length < w2.length) return true;
            if (w1.length > w2.length) return false;
            return std::lexicographical_compare(w1.program, w1.program + w1.length, w2.program, w2.program + w2.length);
        */
    }
}

impl PartialOrd<WitnessUnknown> for WitnessUnknown {
    #[inline] fn partial_cmp(&self, other: &WitnessUnknown) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/**
  | A txout script template with a specific
  | destination. It is either:
  | 
  | - CNoDestination: no destination set
  | 
  | - PKHash: TxoutType::PUBKEYHASH destination
  | (P2PKH)
  | 
  | - ScriptHash: TxoutType::SCRIPTHASH
  | destination (P2SH)
  | 
  | - WitnessV0ScriptHash: TxoutType::WITNESS_V0_SCRIPTHASH
  | destination (P2WSH)
  | 
  | - WitnessV0KeyHash: TxoutType::WITNESS_V0_KEYHASH
  | destination (P2WPKH)
  | 
  | - WitnessV1Taproot: TxoutType::WITNESS_V1_TAPROOT
  | destination (P2TR)
  | 
  | - WitnessUnknown: TxoutType::WITNESS_UNKNOWN
  | destination (P2W???)
  | 
  | A TxDestination is the internal data
  | type encoded in a bitcoin address
  |
  */
pub enum TxDestination {
    NoDestination(NoDestination),
    PKHash(PKHash),
    ScriptHash(ScriptHash),
    WitnessV0ScriptHash(WitnessV0ScriptHash),
    WitnessV0KeyHash(WitnessV0KeyHash),
    WitnessV1Taproot(WitnessV1Taproot),
    WitnessUnknown(WitnessUnknown)
}

pub struct ShortestVectorFirstComparator { }

impl ShortestVectorFirstComparator {
    
    pub fn invoke(&self, 
        a: &Vec<u8>,
        b: &Vec<u8>) -> bool {
        
        todo!();
        /*
            if (a.size() < b.size()) return true;
            if (a.size() > b.size()) return false;
            return a < b;
        */
    }
}

#[derive(Default)]
pub struct TaprootSpendData {

    /**
      | The BIP341 internal key.
      |
      */
    internal_key: XOnlyPubKey,

    /**
      | The Merkle root of the script tree (0
      | if no scripts).
      |
      */
    merkle_root:  u256,

    /**
      | Map from (script, leaf_version) to
      | (sets of) control blocks.
      | 
      | More than one control block for a given
      | script is only possible if it appears
      | in multiple branches of the tree. We
      | keep them all so that inference can reconstruct
      | the full tree. Within each set, the control
      | blocks are sorted by size, so that the
      | signing logic can easily prefer the
      | cheapest one.
      |
      */
    scripts:      HashMap<(Script,i32),HashSet<Vec<u8>,ShortestVectorFirstComparator>>,
}

impl TaprootSpendData {

    /**
      | Merge other TaprootSpendData (for
      | the same scriptPubKey) into this.
      |
      */
    pub fn merge(&mut self, other: TaprootSpendData)  {
        
        todo!();
        /*
            // TODO: figure out how to better deal with conflicting information
        // being merged.
        if (internal_key.IsNull() && !other.internal_key.IsNull()) {
            internal_key = other.internal_key;
        }
        if (merkle_root.IsNull() && !other.merkle_root.IsNull()) {
            merkle_root = other.merkle_root;
        }
        for (auto& [key, control_blocks] : other.scripts) {
            // Once P0083R3 is supported by all our targeted platforms,
            // this loop body can be replaced with:
            // scripts[key].merge(std::move(control_blocks));
            auto& target = scripts[key];
            for (auto& control_block: control_blocks) {
                target.insert(std::move(control_block));
            }
        }
        */
    }
}

/**
  | Utility class to construct Taproot
  | outputs from internal key and script
  | tree.
  |
  */
pub struct TaprootBuilder {

    /**
      | Whether the builder is in a valid state
      | so far.
      |
      */
    valid: bool, // default = true

    /** 
     | The current state of the builder.
     |
     | For each level in the tree, one NodeInfo
     | object may be present. m_branch[0] is
     | information about the root; further values
     | are for deeper subtrees being explored.
     |
     | For every right branch taken to reach the
     | position we're currently working in, there
     | will be a (non-nullopt) entry in m_branch
     | corresponding to the left branch at that
     | level.
     |
     | For example, imagine this tree:
     |
     |       - N0 -
     |      /      \
     |     N1      N2
     |    /  \    /  \
     |   A    B  C   N3
     |              /  \
     |             D    E
     |
     | Initially, m_branch is empty. After
     | processing leaf A, it would become
     | {nullopt, nullopt, A}. When processing leaf
     | B, an entry at level 2 already exists, and
     | it would thus be combined with it to
     | produce a level 1 one, resulting in
     | {nullopt, N1}. Adding C and D takes us to
     | {nullopt, N1, C} and {nullopt, N1, C, D}
     | respectively. When E is processed, it is
     | combined with D, and then C, and then N1,
     | to produce the root, resulting in {N0}.
     |
     | This structure allows processing with just
     | O(log n) overhead if the leaves are
     | computed on the fly.
     |
     | As an invariant, there can never be nullopt
     | entries at the end. There can also not be
     | more than 128 entries (as that would mean
     | more than 128 levels in the tree). The
     | depth of newly added entries will always be
     | at least equal to the current size of
     | m_branch (otherwise it does not correspond
     | to a depth-first traversal of
     | a tree). m_branch is only empty if no
     | entries have ever be processed. m_branch
     | having length 1 corresponds to being done.
     */
    branch:       Vec<Option<taproot_builder::NodeInfo>>,

    /**
      | The internal key, set when finalizing.
      |
      */
    internal_key: XOnlyPubKey,

    /**
      | The output key, computed when finalizing.
      |
      */
    output_key:   XOnlyPubKey,

    /**
      | The tweak parity, computed when finalizing.
      |
      */
    parity:       bool,
}

pub mod taproot_builder {

    use super::*;

    /**
      | Information about a tracked leaf in
      | the Merkle tree.
      |
      */
    pub struct LeafInfo
    {
        /**
          | The script.
          |
          */
        script:        Script,

        /**
          | The leaf version for that 
          | script.
          |
          */
        leaf_version:  i32,

        /**
          | The hashing partners above this 
          | leaf.
          |
          */
        merkle_branch: Vec<u256>,
    }

    /**
      | Information associated with a node
      | in the Merkle tree.
      |
      */
    pub struct NodeInfo {

        /**
          | Merkle hash of this node.
          |
          */
        hash:   u256,


        /**
          | Tracked leaves underneath this node
          | (either from the node itself, or its
          | children).
          | 
          | The merkle_branch field of each is the
          | partners to get to *this* node.
          |
          */
        leaves: Vec<LeafInfo>,
    }
}

impl TaprootBuilder {

    /**
      | Return true if so far all input was valid.
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return m_valid;
        */
    }

    /**
      | Return whether there were either no
      | leaves, or the leaves form a Huffman
      | tree.
      |
      */
    pub fn is_complete(&self) -> bool {
        
        todo!();
        /*
            return m_valid && (m_branch.size() == 0 || (m_branch.size() == 1 && m_branch[0].has_value()));
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/script/standard.cpp]

pub type ValType = Vec<u8>;

lazy_static!{
    /*
    bool fAcceptDatacarrier = DEFAULT_ACCEPT_DATACARRIER;
    unsigned nMaxDatacarrierBytes = MAX_OP_RETURN_RELAY;
    */
}


impl From<&Script> for ScriptHash {

    fn from(in_: &Script) -> Self {
    
        todo!();
        /*
        : base_hash(Hash160(in)),

        
        */
    }
}

impl From<&ScriptID> for ScriptHash {

    fn from(in_: &ScriptID) -> Self {
    
        todo!();
        /*


            : BaseHash(static_cast<u160>(in))
        */
    }
}

impl From<&PubKey> for PKHash {
    
    fn from(pubkey: &PubKey) -> Self {
    
        todo!();
        /*
        : base_hash(pubkey.GetID()),

        
        */
    }
}

impl From<&KeyID> for PKHash {

    fn from(pubkey_id: &KeyID) -> Self {
    
        todo!();
        /*
        : base_hash(pubkey_id),

        
        */
    }
}

impl From<&PubKey> for WitnessV0KeyHash {

    fn from(pubkey: &PubKey) -> Self {
    
        todo!();
        /*
        : base_hash(pubkey.GetID()),

        
        */
    }
}
    
impl From<&PKHash> for WitnessV0KeyHash {
    fn from(pubkey_hash: &PKHash) -> Self {
    
        todo!();
        /*


            : BaseHash(static_cast<u160>(pubkey_hash))
        */
    }
}

impl Into<KeyID> for PKHash {

    fn into(self) -> KeyID {
        
        todo!();
            /*
                return CKeyID{static_cast<u160>(key_hash)};
            */
    }
}

impl Into<KeyID> for WitnessV0KeyHash {

    fn into(self) -> KeyID {
        
        todo!();
            /*
                return CKeyID{static_cast<u160>(key_hash)};
            */
    }
}

impl From<&Script> for WitnessV0ScriptHash {
    
    fn from(in_: &Script) -> Self {
    
        todo!();
        /*
            CSHA256().Write(in.data(), in.size()).Finalize(begin());
        */
    }
}

/**
  | Get the name of a TxoutType as a string
  |
  */
pub fn get_txn_output_type(t: TxoutType) -> String {
    
    todo!();
        /*
            switch (t) {
        case TxoutType::NONSTANDARD: return "nonstandard";
        case TxoutType::PUBKEY: return "pubkey";
        case TxoutType::PUBKEYHASH: return "pubkeyhash";
        case TxoutType::SCRIPTHASH: return "scripthash";
        case TxoutType::MULTISIG: return "multisig";
        case TxoutType::NULL_DATA: return "nulldata";
        case TxoutType::WITNESS_V0_KEYHASH: return "witness_v0_keyhash";
        case TxoutType::WITNESS_V0_SCRIPTHASH: return "witness_v0_scripthash";
        case TxoutType::WITNESS_V1_TAPROOT: return "witness_v1_taproot";
        case TxoutType::WITNESS_UNKNOWN: return "witness_unknown";
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}

pub fn match_pay_to_pubkey(
        script: &Script,
        pubkey: &mut ValType) -> bool {
    
    todo!();
        /*
            if (script.size() == CPubKey::SIZE + 2 && script[0] == CPubKey::SIZE && script.back() == OP_CHECKSIG) {
            pubkey = valtype(script.begin() + 1, script.begin() + CPubKey::SIZE + 1);
            return CPubKey::ValidSize(pubkey);
        }
        if (script.size() == CPubKey::COMPRESSED_SIZE + 2 && script[0] == CPubKey::COMPRESSED_SIZE && script.back() == OP_CHECKSIG) {
            pubkey = valtype(script.begin() + 1, script.begin() + CPubKey::COMPRESSED_SIZE + 1);
            return CPubKey::ValidSize(pubkey);
        }
        return false;
        */
}

pub fn match_pay_to_pubkey_hash(
        script:     &Script,
        pubkeyhash: &mut ValType) -> bool {
    
    todo!();
        /*
            if (script.size() == 25 && script[0] == OP_DUP && script[1] == OP_HASH160 && script[2] == 20 && script[23] == OP_EQUALVERIFY && script[24] == OP_CHECKSIG) {
            pubkeyhash = valtype(script.begin () + 3, script.begin() + 23);
            return true;
        }
        return false;
        */
}

/**
  | Test for "small positive integer" script
  | opcodes - OP_1 through OP_16.
  |
  */
pub fn is_small_integer(opcode: OpcodeType) -> bool {
    
    todo!();
        /*
            return opcode >= OP_1 && opcode <= OP_16;
        */
}

pub fn is_pushdata_op(opcode: OpcodeType) -> bool {
    
    todo!();
        /*
            return opcode > OP_FALSE && opcode <= OP_PUSHDATA4;
        */
}

pub fn is_valid_multisig_key_count(n_keys: i32) -> bool {
    
    todo!();
        /*
            return n_keys > 0 && n_keys <= MAX_PUBKEYS_PER_MULTISIG;
        */
}

pub fn get_multisig_key_count(
        opcode: OpcodeType,
        data:   ValType,
        count:  &mut i32) -> bool {
    
    todo!();
        /*
            if (IsSmallInteger(opcode)) {
            count = CScript::DecodeOP_N(opcode);
            return IsValidMultisigKeyCount(count);
        }

        if (IsPushdataOp(opcode)) {
            if (!CheckMinimalPush(data, opcode)) return false;
            try {
                count = CScriptNum(data, /* fRequireMinimal = */ true).getint();
                return IsValidMultisigKeyCount(count);
            } catch (const scriptnum_error&) {
                return false;
            }
        }

        return false;
        */
}

pub fn match_multisig(
        script:        &Script,
        required_sigs: &mut i32,
        pubkeys:       &mut Vec<ValType>) -> bool {
    
    todo!();
        /*
            opcodetype opcode;
        valtype data;
        int num_keys;

        CScript::const_iterator it = script.begin();
        if (script.size() < 1 || script.back() != OP_CHECKMULTISIG) return false;

        if (!script.GetOp(it, opcode, data) || !GetMultisigKeyCount(opcode, data, required_sigs)) return false;
        while (script.GetOp(it, opcode, data) && CPubKey::ValidSize(data)) {
            pubkeys.emplace_back(std::move(data));
        }
        if (!GetMultisigKeyCount(opcode, data, num_keys)) return false;

        if (pubkeys.size() != static_cast<unsigned long>(num_keys) || num_keys < required_sigs) return false;

        return (it + 1 == script.end());
        */
}

/**
  | Parse a scriptPubKey and identify script
  | type for standard scripts. If successful,
  | returns script type and parsed pubkeys
  | or hashes, depending on the type. For
  | example, for a P2SH script, vSolutionsRet
  | will contain the script hash, for P2PKH
  | it will contain the key hash, etc.
  | 
  | -----------
  | @param[in] scriptPubKey
  | 
  | Script to parse
  | ----------
  | @param[out] vSolutionsRet
  | 
  | Vector of parsed pubkeys and hashes
  | 
  | -----------
  | @return
  | 
  | The script type. TxoutType::NONSTANDARD
  | represents a failed solve.
  |
  */
pub fn solver(
        script_pub_key: &Script,
        solutions_ret:  &mut Vec<Vec<u8>>) -> TxoutType {
    
    todo!();
        /*
            vSolutionsRet.clear();

        // Shortcut for pay-to-script-hash, which are more constrained than the other types:
        // it is always OP_HASH160 20 [20 byte hash] OP_EQUAL
        if (scriptPubKey.IsPayToScriptHash())
        {
            std::vector<unsigned char> hashBytes(scriptPubKey.begin()+2, scriptPubKey.begin()+22);
            vSolutionsRet.push_back(hashBytes);
            return TxoutType::SCRIPTHASH;
        }

        int witnessversion;
        std::vector<unsigned char> witnessprogram;
        if (scriptPubKey.IsWitnessProgram(witnessversion, witnessprogram)) {
            if (witnessversion == 0 && witnessprogram.size() == WITNESS_V0_KEYHASH_SIZE) {
                vSolutionsRet.push_back(std::move(witnessprogram));
                return TxoutType::WITNESS_V0_KEYHASH;
            }
            if (witnessversion == 0 && witnessprogram.size() == WITNESS_V0_SCRIPTHASH_SIZE) {
                vSolutionsRet.push_back(std::move(witnessprogram));
                return TxoutType::WITNESS_V0_SCRIPTHASH;
            }
            if (witnessversion == 1 && witnessprogram.size() == WITNESS_V1_TAPROOT_SIZE) {
                vSolutionsRet.push_back(std::move(witnessprogram));
                return TxoutType::WITNESS_V1_TAPROOT;
            }
            if (witnessversion != 0) {
                vSolutionsRet.push_back(std::vector<unsigned char>{(unsigned char)witnessversion});
                vSolutionsRet.push_back(std::move(witnessprogram));
                return TxoutType::WITNESS_UNKNOWN;
            }
            return TxoutType::NONSTANDARD;
        }

        // Provably prunable, data-carrying output
        //
        // So long as script passes the IsUnspendable() test and all but the first
        // byte passes the IsPushOnly() test we don't care what exactly is in the
        // script.
        if (scriptPubKey.size() >= 1 && scriptPubKey[0] == OP_RETURN && scriptPubKey.IsPushOnly(scriptPubKey.begin()+1)) {
            return TxoutType::NULL_DATA;
        }

        std::vector<unsigned char> data;
        if (MatchPayToPubkey(scriptPubKey, data)) {
            vSolutionsRet.push_back(std::move(data));
            return TxoutType::PUBKEY;
        }

        if (MatchPayToPubkeyHash(scriptPubKey, data)) {
            vSolutionsRet.push_back(std::move(data));
            return TxoutType::PUBKEYHASH;
        }

        int required;
        std::vector<std::vector<unsigned char>> keys;
        if (MatchMultisig(scriptPubKey, required, keys)) {
            vSolutionsRet.push_back({static_cast<unsigned char>(required)}); // safe as required is in range 1..20
            vSolutionsRet.insert(vSolutionsRet.end(), keys.begin(), keys.end());
            vSolutionsRet.push_back({static_cast<unsigned char>(keys.size())}); // safe as size is in range 1..20
            return TxoutType::MULTISIG;
        }

        vSolutionsRet.clear();
        return TxoutType::NONSTANDARD;
        */
}

/**
  | Parse a standard scriptPubKey for the
  | destination address. Assigns result
  | to the addressRet parameter and returns
  | true if successful. Currently only
  | works for P2PK, P2PKH, P2SH, P2WPKH, 
  | and P2WSH scripts.
  |
  */
pub fn extract_destination(
        script_pub_key: &Script,
        address_ret:    &mut TxDestination) -> bool {
    
    todo!();
        /*
            std::vector<valtype> vSolutions;
        TxoutType whichType = Solver(scriptPubKey, vSolutions);

        switch (whichType) {
        case TxoutType::PUBKEY: {
            CPubKey pubKey(vSolutions[0]);
            if (!pubKey.IsValid())
                return false;

            addressRet = PKHash(pubKey);
            return true;
        }
        case TxoutType::PUBKEYHASH: {
            addressRet = PKHash(u160(vSolutions[0]));
            return true;
        }
        case TxoutType::SCRIPTHASH: {
            addressRet = ScriptHash(u160(vSolutions[0]));
            return true;
        }
        case TxoutType::WITNESS_V0_KEYHASH: {
            WitnessV0KeyHash hash;
            std::copy(vSolutions[0].begin(), vSolutions[0].end(), hash.begin());
            addressRet = hash;
            return true;
        }
        case TxoutType::WITNESS_V0_SCRIPTHASH: {
            WitnessV0ScriptHash hash;
            std::copy(vSolutions[0].begin(), vSolutions[0].end(), hash.begin());
            addressRet = hash;
            return true;
        }
        case TxoutType::WITNESS_V1_TAPROOT: {
            WitnessV1Taproot tap;
            std::copy(vSolutions[0].begin(), vSolutions[0].end(), tap.begin());
            addressRet = tap;
            return true;
        }
        case TxoutType::WITNESS_UNKNOWN: {
            WitnessUnknown unk;
            unk.version = vSolutions[0][0];
            std::copy(vSolutions[1].begin(), vSolutions[1].end(), unk.program);
            unk.length = vSolutions[1].size();
            addressRet = unk;
            return true;
        }
        case TxoutType::MULTISIG:
        case TxoutType::NULL_DATA:
        case TxoutType::NONSTANDARD:
            return false;
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}

pub struct ScriptVisitor {

}

pub trait ScriptVisit<T> {
    fn script_visit(&self, item: T) -> Script;
}

impl ScriptVisit<&NoDestination> for ScriptVisitor {
    fn script_visit(&self, dest: &NoDestination) -> Script {
        
        todo!();
        /*
            return CScript();
        */
    }
}

impl ScriptVisit<&PKHash> for ScriptVisitor {
    fn script_visit(&self, keyid: &PKHash) -> Script {
        
        todo!();
        /*
            return CScript() << OP_DUP << OP_HASH160 << ToByteVector(keyID) << OP_EQUALVERIFY << OP_CHECKSIG;
        */
    }
}
    
impl ScriptVisit<&ScriptHash> for ScriptVisitor {
    fn script_visit(&self, scriptid: &ScriptHash) -> Script {
        
        todo!();
        /*
            return CScript() << OP_HASH160 << ToByteVector(scriptID) << OP_EQUAL;
        */
    }
}
    
impl ScriptVisit<&WitnessV0KeyHash> for ScriptVisitor {
    fn script_visit(&self, id: &WitnessV0KeyHash) -> Script {
        
        todo!();
        /*
            return CScript() << OP_0 << ToByteVector(id);
        */
    }
}
    
impl ScriptVisit<&WitnessV0ScriptHash> for ScriptVisitor {
    fn script_visit(&self, id: &WitnessV0ScriptHash) -> Script {
        
        todo!();
        /*
            return CScript() << OP_0 << ToByteVector(id);
        */
    }
}
    
impl ScriptVisit<&WitnessV1Taproot> for ScriptVisitor {
    fn script_visit(&self, tap: &WitnessV1Taproot) -> Script {
        
        todo!();
        /*
            return CScript() << OP_1 << ToByteVector(tap);
        */
    }
}
    
impl ScriptVisit<&WitnessUnknown> for ScriptVisitor {
    fn script_visit(&self, id: &WitnessUnknown) -> Script {
        
        todo!();
        /*
            return CScript() << CScript::EncodeOP_N(id.version) << std::vector<unsigned char>(id.program, id.program + id.length);
        */
    }
}

/**
  | Generate a Bitcoin scriptPubKey for the given
  | TxDestination. 
  |
  | Returns a P2PKH script for a CKeyID
  | destination, a P2SH script for a CScriptID,
  | and an empty script for CNoDestination.
  |
  */
pub fn get_script_for_destination(dest: &TxDestination) -> Script {
    
    todo!();
        /*
            return std::visit(CScriptVisitor(), dest);
        */
}

/**
  | Generate a P2PK script for the given
  | pubkey.
  |
  */
pub fn get_script_for_raw_pub_key(pub_key: &PubKey) -> Script {
    
    todo!();
        /*
            return CScript() << std::vector<unsigned char>(pubKey.begin(), pubKey.end()) << OP_CHECKSIG;
        */
}

/**
  | Generate a multisig script.
  |
  */
pub fn get_script_for_multisig(
        n_required: i32,
        keys:       &Vec<PubKey>) -> Script {
    
    todo!();
        /*
            CScript script;

        script << nRequired;
        for (const CPubKey& key : keys)
            script << ToByteVector(key);
        script << keys.size() << OP_CHECKMULTISIG;

        return script;
        */
}

/**
  | Check whether a TxDestination is a
  | CNoDestination.
  |
  */
pub fn is_valid_destination(dest: &TxDestination) -> bool {
    
    todo!();
        /*
            return dest.index() != 0;
        */
}

impl TaprootBuilder {
    
    /**
      | Combine information about a parent
      | Merkle tree node from its child nodes.
      |
      */
    pub fn combine(&mut self, 
        a: taproot_builder::NodeInfo,
        b: taproot_builder::NodeInfo) -> taproot_builder::NodeInfo {
        
        todo!();
        /*
            NodeInfo ret;
        /* Iterate over all tracked leaves in a, add b's hash to their Merkle branch, and move them to ret. */
        for (auto& leaf : a.leaves) {
            leaf.merkle_branch.push_back(b.hash);
            ret.leaves.emplace_back(std::move(leaf));
        }
        /* Iterate over all tracked leaves in b, add a's hash to their Merkle branch, and move them to ret. */
        for (auto& leaf : b.leaves) {
            leaf.merkle_branch.push_back(a.hash);
            ret.leaves.emplace_back(std::move(leaf));
        }
        /* Lexicographically sort a and b's hash, and compute parent hash. */
        if (a.hash < b.hash) {
            ret.hash = (CHashWriter(HASHER_TAPBRANCH) << a.hash << b.hash).GetSHA256();
        } else {
            ret.hash = (CHashWriter(HASHER_TAPBRANCH) << b.hash << a.hash).GetSHA256();
        }
        return ret;
        */
    }
    
    /**
      | Insert information about a node at a
      | certain depth, and propagate information
      | up.
      |
      */
    pub fn insert(&mut self, 
        node:  taproot_builder::NodeInfo,
        depth: i32)  {
        
        todo!();
        /*
            assert(depth >= 0 && (size_t)depth <= TAPROOT_CONTROL_MAX_NODE_COUNT);
        /* We cannot insert a leaf at a lower depth while a deeper branch is unfinished. Doing
         * so would mean the Add() invocations do not correspond to a DFS traversal of a
         * binary tree. */
        if ((size_t)depth + 1 < m_branch.size()) {
            m_valid = false;
            return;
        }
        /* As long as an entry in the branch exists at the specified depth, combine it and propagate up.
         * The 'node' variable is overwritten here with the newly combined node. */
        while (m_valid && m_branch.size() > (size_t)depth && m_branch[depth].has_value()) {
            node = Combine(std::move(node), std::move(*m_branch[depth]));
            m_branch.pop_back();
            if (depth == 0) m_valid = false; /* Can't propagate further up than the root */
            --depth;
        }
        if (m_valid) {
            /* Make sure the branch is big enough to place the new node. */
            if (m_branch.size() <= (size_t)depth) m_branch.resize((size_t)depth + 1);
            assert(!m_branch[depth].has_value());
            m_branch[depth] = std::move(node);
        }
        */
    }
    
    /**
      | Check if a list of depths is legal (will
      | lead to IsComplete()).
      |
      */
    pub fn valid_depths(&mut self, depths: &Vec<i32>) -> bool {
        
        todo!();
        /*
            std::vector<bool> branch;
        for (int depth : depths) {
            // This inner loop corresponds to effectively the same logic on branch
            // as what Insert() performs on the m_branch variable. Instead of
            // storing a NodeInfo object, just remember whether or not there is one
            // at that depth.
            if (depth < 0 || (size_t)depth > TAPROOT_CONTROL_MAX_NODE_COUNT) return false;
            if ((size_t)depth + 1 < branch.size()) return false;
            while (branch.size() > (size_t)depth && branch[depth]) {
                branch.pop_back();
                if (depth == 0) return false;
                --depth;
            }
            if (branch.size() <= (size_t)depth) branch.resize((size_t)depth + 1);
            assert(!branch[depth]);
            branch[depth] = true;
        }
        // And this check corresponds to the IsComplete() check on m_branch.
        return branch.size() == 0 || (branch.size() == 1 && branch[0]);
        */
    }
    
    /**
      | Add a new script at a certain depth in
      | the tree. Add() operations must be called
      | in depth-first traversal order of binary
      | tree. If track is true, it will be included
      | in the GetSpendData() output.
      |
      */
    pub fn add(&mut self, 
        depth:        i32,
        script:       &Script,
        leaf_version: i32,
        track:        Option<bool>) -> &mut TaprootBuilder {

        let track: bool = track.unwrap_or(true);
        
        todo!();
        /*
            assert((leaf_version & ~TAPROOT_LEAF_MASK) == 0);
        if (!IsValid()) return *this;
        /* Construct NodeInfo object with leaf hash and (if track is true) also leaf information. */
        NodeInfo node;
        node.hash = (CHashWriter{HASHER_TAPLEAF} << uint8_t(leaf_version) << script).GetSHA256();
        if (track) node.leaves.emplace_back(LeafInfo{script, leaf_version, {}});
        /* Insert into the branch. */
        Insert(std::move(node), depth);
        return *this;
        */
    }
    
    /**
      | Like Add(), but for a Merkle node with
      | a given hash to the tree.
      |
      */
    pub fn add_omitted(&mut self, 
        depth: i32,
        hash:  &u256) -> &mut TaprootBuilder {
        
        todo!();
        /*
            if (!IsValid()) return *this;
        /* Construct NodeInfo object with the hash directly, and insert it into the branch. */
        NodeInfo node;
        node.hash = hash;
        Insert(std::move(node), depth);
        return *this;
        */
    }
    
    /**
      | Finalize the construction. Can only
      | be called when IsComplete() is true.
      | internal_key.IsFullyValid() must
      | be true.
      |
      */
    pub fn finalize(&mut self, internal_key: &XOnlyPubKey) -> &mut TaprootBuilder {
        
        todo!();
        /*
            /* Can only call this function when IsComplete() is true. */
        assert(IsComplete());
        m_internal_key = internal_key;
        auto ret = m_internal_key.CreateTapTweak(m_branch.size() == 0 ? nullptr : &m_branch[0]->hash);
        assert(ret.has_value());
        std::tie(m_output_key, m_parity) = *ret;
        return *this;
        */
    }
    
    /**
      | Compute scriptPubKey (after Finalize()).
      |
      */
    pub fn get_output(&mut self) -> WitnessV1Taproot {
        
        todo!();
        /*
            return WitnessV1Taproot{m_output_key};
        */
    }
    
    /**
      | Compute spending data (after Finalize()).
      |
      */
    pub fn get_spend_data(&self) -> TaprootSpendData {
        
        todo!();
        /*
            assert(IsComplete());
        TaprootSpendData spd;
        spd.merkle_root = m_branch.size() == 0 ? uint256() : m_branch[0]->hash;
        spd.internal_key = m_internal_key;
        if (m_branch.size()) {
            // If any script paths exist, they have been combined into the root m_branch[0]
            // by now. Compute the control block for each of its tracked leaves, and put them in
            // spd.scripts.
            for (const auto& leaf : m_branch[0]->leaves) {
                std::vector<unsigned char> control_block;
                control_block.resize(TAPROOT_CONTROL_BASE_SIZE + TAPROOT_CONTROL_NODE_SIZE * leaf.merkle_branch.size());
                control_block[0] = leaf.leaf_version | (m_parity ? 1 : 0);
                std::copy(m_internal_key.begin(), m_internal_key.end(), control_block.begin() + 1);
                if (leaf.merkle_branch.size()) {
                    std::copy(leaf.merkle_branch[0].begin(),
                              leaf.merkle_branch[0].begin() + TAPROOT_CONTROL_NODE_SIZE * leaf.merkle_branch.size(),
                              control_block.begin() + TAPROOT_CONTROL_BASE_SIZE);
                }
                spd.scripts[{leaf.script, leaf.leaf_version}].insert(std::move(control_block));
            }
        }
        return spd;
        */
    }
}

/**
  | Given a TaprootSpendData and the output
  | key, reconstruct its script tree.
  | 
  | If the output doesn't match the spenddata,
  | or if the data in spenddata is incomplete,
  | std::nullopt is returned. Otherwise,
  | a vector of (depth, script, leaf_ver)
  | tuples is returned, corresponding
  | to a depth-first traversal of the script
  | tree.
  |
  */
pub fn infer_taproot_tree(
        spenddata: &TaprootSpendData,
        output:    &XOnlyPubKey) -> Option<Vec<(i32,Script,i32)>> {
    
    todo!();
        /*
            // Verify that the output matches the assumed Merkle root and internal key.
        auto tweak = spenddata.internal_key.CreateTapTweak(spenddata.merkle_root.IsNull() ? nullptr : &spenddata.merkle_root);
        if (!tweak || tweak->first != output) return std::nullopt;
        // If the Merkle root is 0, the tree is empty, and we're done.
        std::vector<std::tuple<int, CScript, int>> ret;
        if (spenddata.merkle_root.IsNull()) return ret;

        /** Data structure to represent the nodes of the tree we're going to build. */
        struct TreeNode {
            /** Hash of this node, if known; 0 otherwise. */
            uint256 hash;
            /** The left and right subtrees (note that their order is irrelevant). */
            std::unique_ptr<TreeNode> sub[2];
            /** If this is known to be a leaf node, a pointer to the (script, leaf_ver) pair.
             *  nullptr otherwise. */
            const std::pair<CScript, int>* leaf = nullptr;
            /** Whether or not this node has been explored (is known to be a leaf, or known to have children). */
            bool explored = false;
            /** Whether or not this node is an inner node (unknown until explored = true). */
            bool inner;
            /** Whether or not we have produced output for this subtree. */
            bool done = false;
        };

        // Build tree from the provided branches.
        TreeNode root;
        root.hash = spenddata.merkle_root;
        for (const auto& [key, control_blocks] : spenddata.scripts) {
            const auto& [script, leaf_ver] = key;
            for (const auto& control : control_blocks) {
                // Skip script records with nonsensical leaf version.
                if (leaf_ver < 0 || leaf_ver >= 0x100 || leaf_ver & 1) continue;
                // Skip script records with invalid control block sizes.
                if (control.size() < TAPROOT_CONTROL_BASE_SIZE || control.size() > TAPROOT_CONTROL_MAX_SIZE ||
                    ((control.size() - TAPROOT_CONTROL_BASE_SIZE) % TAPROOT_CONTROL_NODE_SIZE) != 0) continue;
                // Skip script records that don't match the control block.
                if ((control[0] & TAPROOT_LEAF_MASK) != leaf_ver) continue;
                // Skip script records that don't match the provided Merkle root.
                const uint256 leaf_hash = ComputeTapleafHash(leaf_ver, script);
                const uint256 merkle_root = ComputeTaprootMerkleRoot(control, leaf_hash);
                if (merkle_root != spenddata.merkle_root) continue;

                TreeNode* node = &root;
                size_t levels = (control.size() - TAPROOT_CONTROL_BASE_SIZE) / TAPROOT_CONTROL_NODE_SIZE;
                for (size_t depth = 0; depth < levels; ++depth) {
                    // Can't descend into a node which we already know is a leaf.
                    if (node->explored && !node->inner) return std::nullopt;

                    // Extract partner hash from Merkle branch in control block.
                    uint256 hash;
                    std::copy(control.begin() + TAPROOT_CONTROL_BASE_SIZE + (levels - 1 - depth) * TAPROOT_CONTROL_NODE_SIZE,
                              control.begin() + TAPROOT_CONTROL_BASE_SIZE + (levels - depth) * TAPROOT_CONTROL_NODE_SIZE,
                              hash.begin());

                    if (node->sub[0]) {
                        // Descend into the existing left or right branch.
                        bool desc = false;
                        for (int i = 0; i < 2; ++i) {
                            if (node->sub[i]->hash == hash || (node->sub[i]->hash.IsNull() && node->sub[1-i]->hash != hash)) {
                                node->sub[i]->hash = hash;
                                node = &*node->sub[1-i];
                                desc = true;
                                break;
                            }
                        }
                        if (!desc) return std::nullopt; // This probably requires a hash collision to hit.
                    } else {
                        // We're in an unexplored node. Create subtrees and descend.
                        node->explored = true;
                        node->inner = true;
                        node->sub[0] = std::make_unique<TreeNode>();
                        node->sub[1] = std::make_unique<TreeNode>();
                        node->sub[1]->hash = hash;
                        node = &*node->sub[0];
                    }
                }
                // Cannot turn a known inner node into a leaf.
                if (node->sub[0]) return std::nullopt;
                node->explored = true;
                node->inner = false;
                node->leaf = &key;
                node->hash = leaf_hash;
            }
        }

        // Recursive processing to turn the tree into flattened output. Use an explicit stack here to avoid
        // overflowing the call stack (the tree may be 128 levels deep).
        std::vector<TreeNode*> stack{&root};
        while (!stack.empty()) {
            TreeNode& node = *stack.back();
            if (!node.explored) {
                // Unexplored node, which means the tree is incomplete.
                return std::nullopt;
            } else if (!node.inner) {
                // Leaf node; produce output.
                ret.emplace_back(stack.size() - 1, node.leaf->first, node.leaf->second);
                node.done = true;
                stack.pop_back();
            } else if (node.sub[0]->done && !node.sub[1]->done && !node.sub[1]->explored && !node.sub[1]->hash.IsNull() &&
                       (CHashWriter{HASHER_TAPBRANCH} << node.sub[1]->hash << node.sub[1]->hash).GetSHA256() == node.hash) {
                // Whenever there are nodes with two identical subtrees under it, we run into a problem:
                // the control blocks for the leaves underneath those will be identical as well, and thus
                // they will all be matched to the same path in the tree. The result is that at the location
                // where the duplicate occurred, the left child will contain a normal tree that can be explored
                // and processed, but the right one will remain unexplored.
                //
                // This situation can be detected, by encountering an inner node with unexplored right subtree
                // with known hash, and H_TapBranch(hash, hash) is equal to the parent node (this node)'s hash.
                //
                // To deal with this, simply process the left tree a second time (set its done flag to false;
                // noting that the done flag of its children have already been set to false after processing
                // those). To avoid ending up in an infinite loop, set the done flag of the right (unexplored)
                // subtree to true.
                node.sub[0]->done = false;
                node.sub[1]->done = true;
            } else if (node.sub[0]->done && node.sub[1]->done) {
                // An internal node which we're finished with.
                node.sub[0]->done = false;
                node.sub[1]->done = false;
                node.done = true;
                stack.pop_back();
            } else if (!node.sub[0]->done) {
                // An internal node whose left branch hasn't been processed yet. Do so first.
                stack.push_back(&*node.sub[0]);
            } else if (!node.sub[1]->done) {
                // An internal node whose right branch hasn't been processed yet. Do so first.
                stack.push_back(&*node.sub[1]);
            }
        }

        return ret;
        */
}
