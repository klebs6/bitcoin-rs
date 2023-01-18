crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/sign.h]

/**
  | A signature creator that just produces
  | 71-byte empty signatures.
  |
  */
lazy_static!{
    /*
    extern const BaseSignatureCreator& DUMMY_SIGNATURE_CREATOR;
    */
}

/**
  | A signature creator that just produces
  | 72-byte empty signatures.
  |
  */
lazy_static!{
    /*
    extern const BaseSignatureCreator& DUMMY_MAXIMUM_SIGNATURE_CREATOR;
    */
}

pub type SigPair = (PubKey,Vec<u8>);

/**
  | This struct contains information from
  | a transaction input and also contains
  | signatures for that input.
  |
  | The information contained here can be used to
  | create a signature and is also filled by
  | ProduceSignature in order to construct final
  | scriptSigs and scriptWitnesses.
  */
#[derive(Default)]
pub struct SignatureData {

    /**
      | Stores whether the scriptSig and scriptWitness
      | are complete
      |
      */
    pub complete:               bool, // default = false

    /**
      | Stores whether the input this SigData
      | corresponds to is a witness input
      |
      */
    pub witness:                bool, // default = false

    /**
      | The scriptSig of an input. Contains
      | complete signatures or the traditional
      | partial signatures format
      |
      */
    pub script_sig:             Script,

    /**
      | The redeemScript (if any) for the input
      |
      */
    pub redeem_script:          Script,

    /**
      | The witnessScript (if any) for the input.
      | witnessScripts are used in P2WSH outputs.
      |
      */
    pub witness_script:         Script,

    /**
      | The scriptWitness of an input. Contains
      | complete signatures or the traditional
      | partial signatures format. scriptWitness
      | is part of a transaction input per BIP
      | 144.
      |
      */
    pub script_witness:         ScriptWitness,

    /**
      | Taproot spending data.
      |
      */
    pub tr_spenddata:           TaprootSpendData,

    /**
      | BIP 174 style partial signatures for
      | the input. May contain all signatures
      | necessary for producing a final scriptSig
      | or scriptWitness.
      |
      */
    pub signatures:             HashMap<KeyID,SigPair>,

    pub misc_pubkeys:           HashMap<KeyID,(PubKey,KeyOriginInfo)>,

    /**
      | Schnorr signature for key path spending
      |
      */
    pub taproot_key_path_sig:   Vec<u8>,

    /**
      | (Partial) schnorr signatures, indexed
      | by XOnlyPubKey and leaf_hash.
      |
      */
    pub taproot_script_sigs:    HashMap<(XOnlyPubKey,u256),Vec<u8>>,

    /**
      | KeyIDs of pubkeys which could not be
      | found
      |
      */
    pub missing_pubkeys:        Vec<KeyID>,

    /**
      | KeyIDs of pubkeys for signatures which
      | could not be found
      |
      */
    pub missing_sigs:           Vec<KeyID>,

    /**
      | ScriptID of the missing redeemScript
      | (if any)
      |
      */
    pub missing_redeem_script:  u160,

    /**
      | SHA256 of the missing witnessScript
      | (if any)
      |
      */
    pub missing_witness_script: u256,
}

impl From<&Script> for SignatureData {
    fn from(script: &Script) -> Self {
    
        todo!();
        /*
        : script_sig(script),
        */
    }
}

impl SignatureData {

    pub fn merge_signature_data(&mut self, sigdata: SignatureData)  {
        
        todo!();
        /*
            if (complete) return;
        if (sigdata.complete) {
            *this = std::move(sigdata);
            return;
        }
        if (redeem_script.empty() && !sigdata.redeem_script.empty()) {
            redeem_script = sigdata.redeem_script;
        }
        if (witness_script.empty() && !sigdata.witness_script.empty()) {
            witness_script = sigdata.witness_script;
        }
        signatures.insert(std::make_move_iterator(sigdata.signatures.begin()), std::make_move_iterator(sigdata.signatures.end()));
        */
    }
}

/**
  | Takes a stream and multiple arguments and
  | serializes them as if first serialized into
  | a vector and then into the stream
  |
  | The resulting output into the stream has the
  | total serialized length of all of the objects
  | followed by all objects concatenated with each
  | other.
  */
pub fn serialize_to_vector<Stream, X>(
        s:    &mut Stream,
        args: &X)  {

    todo!();
        /*
            WriteCompactSize(s, GetSerializeSizeMany(s.GetVersion(), args...));
        SerializeMany(s, args...);
        */
}

/**
  | Takes a stream and multiple arguments and
  | unserializes them first as a vector then each
  | object individually in the order provided in
  | the arguments
  */
pub fn unserialize_from_vector<Stream, X>(
        s:    &mut Stream,
        args: &mut X)  {

    todo!();
        /*
            size_t expected_size = ReadCompactSize(s);
        size_t remaining_before = s.size();
        UnserializeMany(s, args...);
        size_t remaining_after = s.size();
        if (remaining_after + expected_size != remaining_before) {
            throw std::ios_base::failure("Size of value was not the stated size");
        }
        */
}

/**
  | Deserialize HD keypaths into a map
  |
  */
pub fn deserialize_hd_keypaths<Stream>(
        s:           &mut Stream,
        key:         &Vec<u8>,
        hd_keypaths: &mut HashMap<PubKey,KeyOriginInfo>)  {

    todo!();
        /*
            // Make sure that the key is the size of pubkey + 1
        if (key.size() != CPubKey::SIZE + 1 && key.size() != CPubKey::COMPRESSED_SIZE + 1) {
            throw std::ios_base::failure("Size of key was not the expected size for the type BIP32 keypath");
        }
        // Read in the pubkey from key
        CPubKey pubkey(key.begin() + 1, key.end());
        if (!pubkey.IsFullyValid()) {
           throw std::ios_base::failure("Invalid pubkey");
        }
        if (hd_keypaths.count(pubkey) > 0) {
            throw std::ios_base::failure("Duplicate Key, pubkey derivation path already provided");
        }

        // Read in key path
        uint64_t value_len = ReadCompactSize(s);
        if (value_len % 4 || value_len == 0) {
            throw std::ios_base::failure("Invalid length for HD key path");
        }

        KeyOriginInfo keypath;
        s >> keypath.fingerprint;
        for (unsigned int i = 4; i < value_len; i += sizeof(uint32_t)) {
            uint32_t index;
            s >> index;
            keypath.path.push_back(index);
        }

        // Add to map
        hd_keypaths.emplace(pubkey, std::move(keypath));
        */
}

/**
  | Serialize HD keypaths to a stream from
  | a map
  |
  */
pub fn serialize_hd_keypaths<Stream>(
        s:           &mut Stream,
        hd_keypaths: &HashMap<PubKey,KeyOriginInfo>,
        ty:          u8)  {

    todo!();
        /*
            for (auto keypath_pair : hd_keypaths) {
            if (!keypath_pair.first.IsValid()) {
                throw std::ios_base::failure("Invalid CPubKey being serialized");
            }
            SerializeToVector(s, type, MakeSpan(keypath_pair.first));
            WriteCompactSize(s, (keypath_pair.second.path.size() + 1) * sizeof(uint32_t));
            s << keypath_pair.second.fingerprint;
            for (const auto& path : keypath_pair.second.path) {
                s << path;
            }
        }
        */
}

//-------------------------------------------[.cpp/bitcoin/src/script/sign.cpp]

pub type ValType = Vec<u8>;

pub fn push_all(values: &Vec<ValType>) -> Script {
    
    todo!();
        /*
            CScript result;
        for (const valtype& v : values) {
            if (v.size() == 0) {
                result << OP_0;
            } else if (v.size() == 1 && v[0] >= 1 && v[0] <= 16) {
                result << CScript::EncodeOP_N(v[0]);
            } else if (v.size() == 1 && v[0] == 0x81) {
                result << OP_1NEGATE;
            } else {
                result << v;
            }
        }
        return result;
        */
}

///-------------------------
pub struct Stacks {
    script:  Vec<ValType>,
    witness: Vec<ValType>,
}

impl Stacks {

    pub fn new(data: &SignatureData) -> Self {
    
        todo!();
        /*
        : witness(data.scriptWitness.stack),

            EvalScript(script, data.scriptSig, SCRIPT_VERIFY_STRICTENC, BaseSignatureChecker(), SigVersion::BASE);
        */
    }
}
