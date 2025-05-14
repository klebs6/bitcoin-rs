// ---------------- [ File: bitcoin-indexed-chain/src/destination_encoder.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/key_io.h]
//-------------------------------------------[.cpp/bitcoin/src/key_io.cpp]

/**
  | Maximum witness length for Bech32 addresses.
  |
  */
pub const BECH32_WITNESS_PROG_MAX_LEN: usize = 40;

pub struct DestinationEncoder {
    params: Rc<ChainParams>,
}

impl From<ChainParams> for DestinationEncoder {

    fn from(params: ChainParams) -> Self {
        Self {
            params: Rc::new(params)
        }
    }
}

impl DestinationEncoder {
    
    pub fn invoke_with_pkhash(&self, id: &PKHash) -> String {
        
        todo!();
        /*
            std::vector<unsigned char> data = m_params.Base58Prefix(CChainParams::PUBKEY_ADDRESS);
            data.insert(data.end(), id.begin(), id.end());
            return EncodeBase58Check(data);
        */
    }
    
    pub fn invoke_with_scripthash(&self, id: &ScriptHash) -> String {
        
        todo!();
        /*
            std::vector<unsigned char> data = m_params.Base58Prefix(CChainParams::SCRIPT_ADDRESS);
            data.insert(data.end(), id.begin(), id.end());
            return EncodeBase58Check(data);
        */
    }
    
    pub fn invoke_with_witnessv0_keyhash(&self, id: &WitnessV0KeyHash) -> String {
        
        todo!();
        /*
            std::vector<unsigned char> data = {0};
            data.reserve(33);
            ConvertBits<8, 5, true>([&](unsigned char c) { data.push_back(c); }, id.begin(), id.end());
            return bech32::Encode(bech32::Encoding::BECH32, m_params.Bech32HRP(), data);
        */
    }
    
    pub fn invoke_with_witnessv0_scripthash(&self, id: &WitnessV0ScriptHash) -> String {
        
        todo!();
        /*
            std::vector<unsigned char> data = {0};
            data.reserve(53);
            ConvertBits<8, 5, true>([&](unsigned char c) { data.push_back(c); }, id.begin(), id.end());
            return bech32::Encode(bech32::Encoding::BECH32, m_params.Bech32HRP(), data);
        */
    }
    
    pub fn invoke_with_witnessv1_taproot(&self, tap: &WitnessV1Taproot) -> String {
        
        todo!();
        /*
            std::vector<unsigned char> data = {1};
            data.reserve(53);
            ConvertBits<8, 5, true>([&](unsigned char c) { data.push_back(c); }, tap.begin(), tap.end());
            return bech32::Encode(bech32::Encoding::BECH32M, m_params.Bech32HRP(), data);
        */
    }
    
    pub fn invoke_with_witness_unknown(&self, id: &WitnessUnknown) -> String {
        
        todo!();
        /*
            if (id.version < 1 || id.version > 16 || id.length < 2 || id.length > 40) {
                return {};
            }
            std::vector<unsigned char> data = {(unsigned char)id.version};
            data.reserve(1 + (id.length * 8 + 4) / 5);
            ConvertBits<8, 5, true>([&](unsigned char c) { data.push_back(c); }, id.program, id.program + id.length);
            return bech32::Encode(bech32::Encoding::BECH32M, m_params.Bech32HRP(), data);
        */
    }
    
    pub fn invoke_with_no_destination(&self, no: &NoDestination) -> String {
        
        todo!();
        /*
            return {};
        */
    }
}

pub fn decode_secret(str_: &String) -> Key {
    
    todo!();
        /*
            CKey key;
        std::vector<unsigned char> data;
        if (DecodeBase58Check(str, data, 34)) {
            const std::vector<unsigned char>& privkey_prefix = Params().Base58Prefix(CChainParams::SECRET_KEY);
            if ((data.size() == 32 + privkey_prefix.size() || (data.size() == 33 + privkey_prefix.size() && data.back() == 1)) &&
                std::equal(privkey_prefix.begin(), privkey_prefix.end(), data.begin())) {
                bool compressed = data.size() == 33 + privkey_prefix.size();
                key.Set(data.begin() + privkey_prefix.size(), data.begin() + privkey_prefix.size() + 32, compressed);
            }
        }
        if (!data.empty()) {
            memory_cleanse(data.data(), data.size());
        }
        return key;
        */
}

pub fn encode_secret(key: &Key) -> String {
    
    todo!();
        /*
            assert(key.IsValid());
        std::vector<unsigned char> data = Params().Base58Prefix(CChainParams::SECRET_KEY);
        data.insert(data.end(), key.begin(), key.end());
        if (key.IsCompressed()) {
            data.push_back(1);
        }
        std::string ret = EncodeBase58Check(data);
        memory_cleanse(data.data(), data.size());
        return ret;
        */
}

pub fn decode_ext_pub_key(str_: &String) -> ExtPubKey {
    
    todo!();
        /*
            CExtPubKey key;
        std::vector<unsigned char> data;
        if (DecodeBase58Check(str, data, 78)) {
            const std::vector<unsigned char>& prefix = Params().Base58Prefix(CChainParams::EXT_PUBLIC_KEY);
            if (data.size() == BIP32_EXTKEY_SIZE + prefix.size() && std::equal(prefix.begin(), prefix.end(), data.begin())) {
                key.Decode(data.data() + prefix.size());
            }
        }
        return key;
        */
}

pub fn encode_ext_pub_key(key: &ExtPubKey) -> String {
    
    todo!();
        /*
            std::vector<unsigned char> data = Params().Base58Prefix(CChainParams::EXT_PUBLIC_KEY);
        size_t size = data.size();
        data.resize(size + BIP32_EXTKEY_SIZE);
        key.Encode(data.data() + size);
        std::string ret = EncodeBase58Check(data);
        return ret;
        */
}

pub fn decode_ext_key(str_: &String) -> ExtKey {
    
    todo!();
        /*
            CExtKey key;
        std::vector<unsigned char> data;
        if (DecodeBase58Check(str, data, 78)) {
            const std::vector<unsigned char>& prefix = Params().Base58Prefix(CChainParams::EXT_SECRET_KEY);
            if (data.size() == BIP32_EXTKEY_SIZE + prefix.size() && std::equal(prefix.begin(), prefix.end(), data.begin())) {
                key.Decode(data.data() + prefix.size());
            }
        }
        return key;
        */
}

pub fn encode_ext_key(key: &ExtKey) -> String {
    
    todo!();
        /*
            std::vector<unsigned char> data = Params().Base58Prefix(CChainParams::EXT_SECRET_KEY);
        size_t size = data.size();
        data.resize(size + BIP32_EXTKEY_SIZE);
        key.Encode(data.data() + size);
        std::string ret = EncodeBase58Check(data);
        memory_cleanse(data.data(), data.size());
        return ret;
        */
}

pub fn is_valid_destination_string(
        str_:   &String,
        params: Option<&ChainParams>) -> bool {
    
    todo!();
        /*
            std::string error_msg;
        return IsValidDestination(DecodeDestination(str, params, error_msg));
        */
}

/* ----------------- core_read.cpp  ----------------- */

/**
  | Parse a hex string into 256 bits
  | 
  | -----------
  | @param[in] strHex
  | 
  | a hex-formatted, 64-character string
  | ----------
  | @param[out] result
  | 
  | the result of the parsing
  | 
  | -----------
  | @return
  | 
  | true if successful, false if not @see
  | ParseHashV for an RPC-oriented version
  | of this
  |
  */
pub fn parse_hash_str(
        str_hex: &String,
        result:  &mut u256) -> bool {
    
    todo!();
        /*
        
        */
}

pub fn parse_hexuv(
        v:        &UniValue,
        str_name: &String) -> Vec<u8> {
    
    todo!();
        /*
        
        */
}

pub fn parse_sighash_string(sighash: &UniValue) -> i32 {
    
    todo!();
        /*
        
        */
}

/* ---------------- * core_write.cpp  ---------------- */
pub fn value_from_amount(amount: Amount) -> UniValue {
    
    todo!();
        /*
        
        */
}

pub fn sighash_to_str(sighash_type: u8) -> String {
    
    todo!();
        /*
        
        */
}
