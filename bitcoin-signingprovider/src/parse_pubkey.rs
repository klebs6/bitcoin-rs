crate::ix!();

pub enum ParseScriptContext {

    /**
      | Top-level context (script goes directly
      | in scriptPubKey)
      |
      */
    TOP,     

    /**
      | Inside sh() (script becomes P2SH redeemScript)
      |
      */
    P2SH,    

    /**
      | Inside wpkh() (no script, pubkey only)
      |
      */
    P2WPKH,  

    /**
      | Inside wsh() (script becomes v0 witness
      | script)
      |
      */
    P2WSH,   

    /**
      | Inside tr() (either internal key, or
      | BIP342 script leaf)
      |
      */
    P2TR,    
}

/**
  | Parse a public key including origin
  | information (if enabled).
  |
  */
pub fn parse_pubkey(
        key_exp_index: u32,
        sp:            &[u8],
        ctx:           ParseScriptContext,
        out:           &mut FlatSigningProvider,
        error:         &mut String) -> Box<PubkeyProvider> {
    
    todo!();
        /*
            using namespace spanparsing;

        auto origin_split = Split(sp, ']');
        if (origin_split.size() > 2) {
            error = "Multiple ']' characters found for a single pubkey";
            return nullptr;
        }
        if (origin_split.size() == 1) return ParsePubkeyInner(key_exp_index, origin_split[0], ctx, out, error);
        if (origin_split[0].empty() || origin_split[0][0] != '[') {
            error = strprintf("Key origin start '[ character expected but not found, got '%c' instead",
                              origin_split[0].empty() ? /** empty, implies split char */ ']' : origin_split[0][0]);
            return nullptr;
        }
        auto slash_split = Split(origin_split[0].subspan(1), '/');
        if (slash_split[0].size() != 8) {
            error = strprintf("Fingerprint is not 4 bytes (%u characters instead of 8 characters)", slash_split[0].size());
            return nullptr;
        }
        std::string fpr_hex = std::string(slash_split[0].begin(), slash_split[0].end());
        if (!IsHex(fpr_hex)) {
            error = strprintf("Fingerprint '%s' is not hex", fpr_hex);
            return nullptr;
        }
        auto fpr_bytes = ParseHex(fpr_hex);
        KeyOriginInfo info;
        const_assert(sizeof(info.fingerprint) == 4, "Fingerprint must be 4 bytes");
        assert(fpr_bytes.size() == 4);
        std::copy(fpr_bytes.begin(), fpr_bytes.end(), info.fingerprint);
        if (!ParseKeyPath(slash_split, info.path, error)) return nullptr;
        auto provider = ParsePubkeyInner(key_exp_index, origin_split[1], ctx, out, error);
        if (!provider) return nullptr;
        return std::make_unique<OriginPubkeyProvider>(key_exp_index, std::move(info), std::move(provider));
        */
}

/**
  | Parse a key path, being passed a split
  | list of elements (the first element
  | is ignored).
  |
  */
pub fn parse_key_path(
        split: &Vec<&[u8]>,
        out:   &mut KeyPath,
        error: &mut String) -> bool {
    
    todo!();
        /*
            for (size_t i = 1; i < split.size(); ++i) {
            Span<const char> elem = split[i];
            bool hardened = false;
            if (elem.size() > 0 && (elem[elem.size() - 1] == '\'' || elem[elem.size() - 1] == 'h')) {
                elem = elem.first(elem.size() - 1);
                hardened = true;
            }
            uint32_t p;
            if (!ParseUInt32(std::string(elem.begin(), elem.end()), &p)) {
                error = strprintf("Key path value '%s' is not a valid uint32", std::string(elem.begin(), elem.end()));
                return false;
            } else if (p > 0x7FFFFFFFUL) {
                error = strprintf("Key path value %u is out of range", p);
                return false;
            }
            out.push_back(p | (((uint32_t)hardened) << 31));
        }
        return true;
        */
}

/**
  | Parse a public key that excludes origin
  | information.
  |
  */
pub fn parse_pubkey_inner(
        key_exp_index: u32,
        sp:            &[u8],
        ctx:           ParseScriptContext,
        out:           &mut FlatSigningProvider,
        error:         &mut String) -> Box<PubkeyProvider> {
    
    todo!();
        /*
            using namespace spanparsing;

        bool permit_uncompressed = ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH;
        auto split = Split(sp, '/');
        std::string str(split[0].begin(), split[0].end());
        if (str.size() == 0) {
            error = "No key provided";
            return nullptr;
        }
        if (split.size() == 1) {
            if (IsHex(str)) {
                std::vector<unsigned char> data = ParseHex(str);
                CPubKey pubkey(data);
                if (pubkey.IsFullyValid()) {
                    if (permit_uncompressed || pubkey.IsCompressed()) {
                        return std::make_unique<ConstPubkeyProvider>(key_exp_index, pubkey, false);
                    } else {
                        error = "Uncompressed keys are not allowed";
                        return nullptr;
                    }
                } else if (data.size() == 32 && ctx == ParseScriptContext::P2TR) {
                    unsigned char fullkey[33] = {0x02};
                    std::copy(data.begin(), data.end(), fullkey + 1);
                    pubkey.Set(std::begin(fullkey), std::end(fullkey));
                    if (pubkey.IsFullyValid()) {
                        return std::make_unique<ConstPubkeyProvider>(key_exp_index, pubkey, true);
                    }
                }
                error = strprintf("Pubkey '%s' is invalid", str);
                return nullptr;
            }
            CKey key = DecodeSecret(str);
            if (key.IsValid()) {
                if (permit_uncompressed || key.IsCompressed()) {
                    CPubKey pubkey = key.GetPubKey();
                    out.keys.emplace(pubkey.GetID(), key);
                    return std::make_unique<ConstPubkeyProvider>(key_exp_index, pubkey, ctx == ParseScriptContext::P2TR);
                } else {
                    error = "Uncompressed keys are not allowed";
                    return nullptr;
                }
            }
        }
        CExtKey extkey = DecodeExtKey(str);
        CExtPubKey extpubkey = DecodeExtPubKey(str);
        if (!extkey.key.IsValid() && !extpubkey.pubkey.IsValid()) {
            error = strprintf("key '%s' is not valid", str);
            return nullptr;
        }
        KeyPath path;
        DeriveType type = DeriveType::NO;
        if (split.back() == MakeSpan("*").first(1)) {
            split.pop_back();
            type = DeriveType::UNHARDENED;
        } else if (split.back() == MakeSpan("*'").first(2) || split.back() == MakeSpan("*h").first(2)) {
            split.pop_back();
            type = DeriveType::HARDENED;
        }
        if (!ParseKeyPath(split, path, error)) return nullptr;
        if (extkey.key.IsValid()) {
            extpubkey = extkey.Neuter();
            out.keys.emplace(extpubkey.pubkey.GetID(), extkey.key);
        }
        return std::make_unique<BIP32PubkeyProvider>(key_exp_index, extpubkey, std::move(path), type);
        */
}
