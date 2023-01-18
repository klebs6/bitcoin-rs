crate::ix!();

pub fn infer_pubkey(
        pubkey:   &crate::PubKey,
        _1:       ParseScriptContext,
        provider: &SigningProvider) -> Box<PubkeyProvider> {
    
    todo!();
        /*
            std::unique_ptr<PubkeyProvider> key_provider = std::make_unique<ConstPubkeyProvider>(0, pubkey, false);
        KeyOriginInfo info;
        if (provider.GetKeyOrigin(pubkey.GetID(), info)) {
            return std::make_unique<OriginPubkeyProvider>(0, std::move(info), std::move(key_provider));
        }
        return key_provider;
        */
}

pub fn infer_xonly_pubkey(
        xkey:     &crate::XOnlyPubKey,
        ctx:      ParseScriptContext,
        provider: &SigningProvider) -> Box<PubkeyProvider> {
    
    todo!();
        /*
            unsigned char full_key[CPubKey::COMPRESSED_SIZE] = {0x02};
        std::copy(xkey.begin(), xkey.end(), full_key + 1);
        CPubKey pubkey(full_key);
        std::unique_ptr<PubkeyProvider> key_provider = std::make_unique<ConstPubkeyProvider>(0, pubkey, true);
        KeyOriginInfo info;
        if (provider.GetKeyOriginByXOnly(xkey, info)) {
            return std::make_unique<OriginPubkeyProvider>(0, std::move(info), std::move(key_provider));
        }
        return key_provider;
        */
}

pub fn infer_script(
        script:   &Script,
        ctx:      ParseScriptContext,
        provider: &SigningProvider) -> Box<DescriptorImpl> {
    
    todo!();
        /*
            if (ctx == ParseScriptContext::P2TR && script.size() == 34 && script[0] == 32 && script[33] == OP_CHECKSIG) {
            crate::XOnlyPubKey key{Span<const unsigned char>{script.data() + 1, script.data() + 33}};
            return std::make_unique<PKDescriptor>(InferXOnlyPubkey(key, ctx, provider));
        }

        std::vector<std::vector<unsigned char>> data;
        TxoutType txntype = Solver(script, data);

        if (txntype == TxoutType::PUBKEY && (ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH || ctx == ParseScriptContext::P2WSH)) {
            CPubKey pubkey(data[0]);
            if (pubkey.IsValid()) {
                return std::make_unique<PKDescriptor>(InferPubkey(pubkey, ctx, provider));
            }
        }
        if (txntype == TxoutType::PUBKEYHASH && (ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH || ctx == ParseScriptContext::P2WSH)) {
            u160 hash(data[0]);
            CKeyID keyid(hash);
            CPubKey pubkey;
            if (provider.GetPubKey(keyid, pubkey)) {
                return std::make_unique<PKHDescriptor>(InferPubkey(pubkey, ctx, provider));
            }
        }
        if (txntype == TxoutType::WITNESS_V0_KEYHASH && (ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH)) {
            u160 hash(data[0]);
            CKeyID keyid(hash);
            CPubKey pubkey;
            if (provider.GetPubKey(keyid, pubkey)) {
                return std::make_unique<WPKHDescriptor>(InferPubkey(pubkey, ctx, provider));
            }
        }
        if (txntype == TxoutType::MULTISIG && (ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH || ctx == ParseScriptContext::P2WSH)) {
            std::vector<std::unique_ptr<PubkeyProvider>> providers;
            for (size_t i = 1; i + 1 < data.size(); ++i) {
                CPubKey pubkey(data[i]);
                providers.push_back(InferPubkey(pubkey, ctx, provider));
            }
            return std::make_unique<MultisigDescriptor>((int)data[0][0], std::move(providers));
        }
        if (txntype == TxoutType::SCRIPTHASH && ctx == ParseScriptContext::TOP) {
            u160 hash(data[0]);
            CScriptID scriptid(hash);
            CScript subscript;
            if (provider.GetCScript(scriptid, subscript)) {
                auto sub = InferScript(subscript, ParseScriptContext::P2SH, provider);
                if (sub) return std::make_unique<SHDescriptor>(std::move(sub));
            }
        }
        if (txntype == TxoutType::WITNESS_V0_SCRIPTHASH && (ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH)) {
            CScriptID scriptid;
            CRIPEMD160().Write(data[0].data(), data[0].size()).Finalize(scriptid.begin());
            CScript subscript;
            if (provider.GetCScript(scriptid, subscript)) {
                auto sub = InferScript(subscript, ParseScriptContext::P2WSH, provider);
                if (sub) return std::make_unique<WSHDescriptor>(std::move(sub));
            }
        }
        if (txntype == TxoutType::WITNESS_V1_TAPROOT && ctx == ParseScriptContext::TOP) {
            // Extract x-only pubkey from output.
            crate::XOnlyPubKey pubkey;
            std::copy(data[0].begin(), data[0].end(), pubkey.begin());
            // Request spending data.
            TaprootSpendData tap;
            if (provider.GetTaprootSpendData(pubkey, tap)) {
                // If found, convert it back to tree form.
                auto tree = InferTaprootTree(tap, pubkey);
                if (tree) {
                    // If that works, try to infer subdescriptors for all leaves.
                    bool ok = true;
                    std::vector<std::unique_ptr<DescriptorImpl>> subscripts; /// list of script subexpressions
                    std::vector<int> depths; /// depth in the tree of each subexpression (same length subscripts)
                    for (const auto& [depth, script, leaf_ver] : *tree) {
                        std::unique_ptr<DescriptorImpl> subdesc;
                        if (leaf_ver == TAPROOT_LEAF_TAPSCRIPT) {
                            subdesc = InferScript(script, ParseScriptContext::P2TR, provider);
                        }
                        if (!subdesc) {
                            ok = false;
                            break;
                        } else {
                            subscripts.push_back(std::move(subdesc));
                            depths.push_back(depth);
                        }
                    }
                    if (ok) {
                        auto key = InferXOnlyPubkey(tap.internal_key, ParseScriptContext::P2TR, provider);
                        return std::make_unique<TRDescriptor>(std::move(key), std::move(subscripts), std::move(depths));
                    }
                }
            }
        }

        TxDestination dest;
        if (ExtractDestination(script, dest)) {
            if (GetScriptForDestination(dest) == script) {
                return std::make_unique<AddressDescriptor>(std::move(dest));
            }
        }

        return std::make_unique<RawDescriptor>(script);
        */
}

/**
  | Find a descriptor for the specified
  | `script`, using information from `provider`
  | where possible.
  | 
  | A non-ranged descriptor which only
  | generates the specified script will
  | be returned in all circumstances.
  | 
  | For public keys with key origin information,
  | this information will be preserved
  | in the returned descriptor.
  | 
  | - If all information for solving `script`
  | is present in `provider`, a descriptor
  | will be returned which is IsSolvable()
  | and encapsulates said information.
  | 
  | - Failing that, if `script` corresponds
  | to a known address type, an "addr()"
  | descriptor will be returned (which
  | is not IsSolvable()).
  | 
  | - Failing that, a "raw()" descriptor
  | is returned.
  |
  */
pub fn infer_descriptor(
        script:   &Script,
        provider: &SigningProvider) -> Box<dyn Descriptor> {
    
    todo!();
        /*
            return InferScript(script, ParseScriptContext::TOP, provider);
        */
}
