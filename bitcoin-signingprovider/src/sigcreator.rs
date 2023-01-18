crate::ix!();

pub type ValType = Vec<u8>;

/**
  | Interface for signature creators.
  |
  */
pub trait BaseSignatureCreator: 
Checker 
+ CreateSig 
+ CreateSchnorrSig
+ Send
+ Sync { }

pub trait Checker {
    fn checker(&self) -> &Box<dyn BaseSignatureChecker>;
}

pub trait CreateSig {

    /**
      | Create a singular (non-script) signature.
      |
      */
    fn create_sig(&self, 
            provider:    &SigningProvider,
            vch_sig:     &mut Vec<u8>,
            keyid:       &KeyID,
            script_code: &Script,
            sigversion:  SigVersion) -> bool;
}

pub trait CreateSchnorrSig {

    fn create_schnorr_sig(&self, 
            provider:    &SigningProvider,
            sig:         &mut Vec<u8>,
            pubkey:      &XOnlyPubKey,
            leaf_hash:   *const u256,
            merkle_root: *const u256,
            sigversion:  SigVersion) -> bool;
}


/**
  | Produce a script signature using a generic
  | signature creator.
  |
  */
pub fn produce_signature(
        provider:     &SigningProvider,
        creator:      &dyn BaseSignatureCreator,
        from_pub_key: &Script,
        sigdata:      &mut SignatureData) -> bool {
    
    todo!();
        /*
            if (sigdata.complete) return true;

        std::vector<valtype> result;
        TxoutType whichType;
        bool solved = SignStep(provider, creator, fromPubKey, result, whichType, SigVersion::BASE, sigdata);
        bool P2SH = false;
        CScript subscript;

        if (solved && whichType == TxoutType::SCRIPTHASH)
        {
            // Solver returns the subscript that needs to be evaluated;
            // the final scriptSig is the signatures from that
            // and then the serialized subscript:
            subscript = CScript(result[0].begin(), result[0].end());
            sigdata.redeem_script = subscript;
            solved = solved && SignStep(provider, creator, subscript, result, whichType, SigVersion::BASE, sigdata) && whichType != TxoutType::SCRIPTHASH;
            P2SH = true;
        }

        if (solved && whichType == TxoutType::WITNESS_V0_KEYHASH)
        {
            CScript witnessscript;
            witnessscript << OP_DUP << OP_HASH160 << ToByteVector(result[0]) << OP_EQUALVERIFY << OP_CHECKSIG;
            TxoutType subType;
            solved = solved && SignStep(provider, creator, witnessscript, result, subType, SigVersion::WITNESS_V0, sigdata);
            sigdata.scriptWitness.stack = result;
            sigdata.witness = true;
            result.clear();
        }
        else if (solved && whichType == TxoutType::WITNESS_V0_SCRIPTHASH)
        {
            CScript witnessscript(result[0].begin(), result[0].end());
            sigdata.witness_script = witnessscript;
            TxoutType subType;
            solved = solved && SignStep(provider, creator, witnessscript, result, subType, SigVersion::WITNESS_V0, sigdata) && subType != TxoutType::SCRIPTHASH && subType != TxoutType::WITNESS_V0_SCRIPTHASH && subType != TxoutType::WITNESS_V0_KEYHASH;
            result.push_back(std::vector<unsigned char>(witnessscript.begin(), witnessscript.end()));
            sigdata.scriptWitness.stack = result;
            sigdata.witness = true;
            result.clear();
        } else if (whichType == TxoutType::WITNESS_V1_TAPROOT && !P2SH) {
            sigdata.witness = true;
            if (solved) {
                sigdata.scriptWitness.stack = std::move(result);
            }
            result.clear();
        } else if (solved && whichType == TxoutType::WITNESS_UNKNOWN) {
            sigdata.witness = true;
        }

        if (!sigdata.witness) sigdata.scriptWitness.stack.clear();
        if (P2SH) {
            result.push_back(std::vector<unsigned char>(subscript.begin(), subscript.end()));
        }
        sigdata.scriptSig = PushAll(result);

        // Test solution
        sigdata.complete = solved && VerifyScript(sigdata.scriptSig, fromPubKey, &sigdata.scriptWitness, STANDARD_SCRIPT_VERIFY_FLAGS, creator.Checker());
        return sigdata.complete;
        */
}

/**
  | Sign scriptPubKey using signature
  | made with creator.
  | 
  | Signatures are returned in scriptSigRet
  | (or returns false if scriptPubKey can't
  | be signed), unless whichTypeRet is
  | TxoutType::SCRIPTHASH, in which case
  | scriptSigRet is the redemption script.
  | 
  | Returns false if scriptPubKey could
  | not be completely satisfied.
  |
  */
pub fn sign_step(
        provider:       &SigningProvider,
        creator:        &dyn BaseSignatureCreator,
        script_pub_key: &Script,
        ret:            &mut Vec<ValType>,
        which_type_ret: &mut TxoutType,
        sigversion:     SigVersion,
        sigdata:        &mut SignatureData) -> bool {
    
    todo!();
        /*
            CScript scriptRet;
        u160 h160;
        ret.clear();
        std::vector<unsigned char> sig;

        std::vector<valtype> vSolutions;
        whichTypeRet = Solver(scriptPubKey, vSolutions);

        switch (whichTypeRet) {
        case TxoutType::NONSTANDARD:
        case TxoutType::NULL_DATA:
        case TxoutType::WITNESS_UNKNOWN:
            return false;
        case TxoutType::PUBKEY:
            if (!CreateSig(creator, sigdata, provider, sig, CPubKey(vSolutions[0]), scriptPubKey, sigversion)) return false;
            ret.push_back(std::move(sig));
            return true;
        case TxoutType::PUBKEYHASH: {
            CKeyID keyID = CKeyID(u160(vSolutions[0]));
            CPubKey pubkey;
            if (!GetPubKey(provider, sigdata, keyID, pubkey)) {
                // Pubkey could not be found, add to missing
                sigdata.missing_pubkeys.push_back(keyID);
                return false;
            }
            if (!CreateSig(creator, sigdata, provider, sig, pubkey, scriptPubKey, sigversion)) return false;
            ret.push_back(std::move(sig));
            ret.push_back(ToByteVector(pubkey));
            return true;
        }
        case TxoutType::SCRIPTHASH:
            h160 = u160(vSolutions[0]);
            if (GetCScript(provider, sigdata, CScriptID{h160}, scriptRet)) {
                ret.push_back(std::vector<unsigned char>(scriptRet.begin(), scriptRet.end()));
                return true;
            }
            // Could not find redeemScript, add to missing
            sigdata.missing_redeem_script = h160;
            return false;

        case TxoutType::MULTISIG: {
            size_t required = vSolutions.front()[0];
            ret.push_back(valtype()); // workaround CHECKMULTISIG bug
            for (size_t i = 1; i < vSolutions.size() - 1; ++i) {
                CPubKey pubkey = CPubKey(vSolutions[i]);
                // We need to always call CreateSig in order to fill sigdata with all
                // possible signatures that we can create. This will allow further PSBT
                // processing to work as it needs all possible signature and pubkey pairs
                if (CreateSig(creator, sigdata, provider, sig, pubkey, scriptPubKey, sigversion)) {
                    if (ret.size() < required + 1) {
                        ret.push_back(std::move(sig));
                    }
                }
            }
            bool ok = ret.size() == required + 1;
            for (size_t i = 0; i + ret.size() < required + 1; ++i) {
                ret.push_back(valtype());
            }
            return ok;
        }
        case TxoutType::WITNESS_V0_KEYHASH:
            ret.push_back(vSolutions[0]);
            return true;

        case TxoutType::WITNESS_V0_SCRIPTHASH:
            CRIPEMD160().Write(vSolutions[0].data(), vSolutions[0].size()).Finalize(h160.begin());
            if (GetCScript(provider, sigdata, CScriptID{h160}, scriptRet)) {
                ret.push_back(std::vector<unsigned char>(scriptRet.begin(), scriptRet.end()));
                return true;
            }
            // Could not find witnessScript, add to missing
            sigdata.missing_witness_script = uint256(vSolutions[0]);
            return false;

        case TxoutType::WITNESS_V1_TAPROOT:
            return SignTaproot(provider, creator, WitnessV1Taproot(XOnlyPubKey{vSolutions[0]}), sigdata, ret);
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}

pub fn sign_taproot(
        provider: &SigningProvider,
        creator:  &dyn BaseSignatureCreator,
        output:   &WitnessV1Taproot,
        sigdata:  &mut SignatureData,
        result:   &mut Vec<ValType>) -> bool {
    
    todo!();
        /*
            TaprootSpendData spenddata;

        // Gather information about this output.
        if (provider.GetTaprootSpendData(output, spenddata)) {
            sigdata.tr_spenddata.Merge(spenddata);
        }

        // Try key path spending.
        {
            std::vector<unsigned char> sig;
            if (sigdata.taproot_key_path_sig.size() == 0) {
                if (creator.CreateSchnorrSig(provider, sig, spenddata.internal_key, nullptr, &spenddata.merkle_root, SigVersion::TAPROOT)) {
                    sigdata.taproot_key_path_sig = sig;
                }
            }
            if (sigdata.taproot_key_path_sig.size()) {
                result = Vector(sigdata.taproot_key_path_sig);
                return true;
            }
        }

        // Try script path spending.
        std::vector<std::vector<unsigned char>> smallest_result_stack;
        for (const auto& [key, control_blocks] : sigdata.tr_spenddata.scripts) {
            const auto& [script, leaf_ver] = key;
            std::vector<std::vector<unsigned char>> result_stack;
            if (SignTaprootScript(provider, creator, sigdata, leaf_ver, script, result_stack)) {
                result_stack.emplace_back(std::begin(script), std::end(script)); // Push the script
                result_stack.push_back(*control_blocks.begin()); // Push the smallest control block
                if (smallest_result_stack.size() == 0 ||
                    GetSerializeSize(result_stack, PROTOCOL_VERSION) < GetSerializeSize(smallest_result_stack, PROTOCOL_VERSION)) {
                    smallest_result_stack = std::move(result_stack);
                }
            }
        }
        if (smallest_result_stack.size() != 0) {
            result = std::move(smallest_result_stack);
            return true;
        }

        return false;
        */
}

pub fn create_sig(
        creator:    &dyn BaseSignatureCreator,
        sigdata:    &mut SignatureData,
        provider:   &SigningProvider,
        sig_out:    &mut Vec<u8>,
        pubkey:     &PubKey,
        scriptcode: &Script,
        sigversion: SigVersion) -> bool {
    
    todo!();
        /*
            CKeyID keyid = pubkey.GetID();
        const auto it = sigdata.signatures.find(keyid);
        if (it != sigdata.signatures.end()) {
            sig_out = it->second.second;
            return true;
        }
        KeyOriginInfo info;
        if (provider.GetKeyOrigin(keyid, info)) {
            sigdata.misc_pubkeys.emplace(keyid, std::make_pair(pubkey, std::move(info)));
        }
        if (creator.CreateSig(provider, sig_out, keyid, scriptcode, sigversion)) {
            auto i = sigdata.signatures.emplace(keyid, SigPair(pubkey, sig_out));
            assert(i.second);
            return true;
        }
        // Could not make signature or signature not found, add keyid to missing
        sigdata.missing_sigs.push_back(keyid);
        return false;
        */
}

pub fn create_taproot_script_sig(
        creator:    &dyn BaseSignatureCreator,
        sigdata:    &mut SignatureData,
        provider:   &SigningProvider,
        sig_out:    &mut Vec<u8>,
        pubkey:     &XOnlyPubKey,
        leaf_hash:  &u256,
        sigversion: SigVersion) -> bool {
    
    todo!();
        /*
            auto lookup_key = std::make_pair(pubkey, leaf_hash);
        auto it = sigdata.taproot_script_sigs.find(lookup_key);
        if (it != sigdata.taproot_script_sigs.end()) {
            sig_out = it->second;
        }
        if (creator.CreateSchnorrSig(provider, sig_out, pubkey, &leaf_hash, nullptr, sigversion)) {
            sigdata.taproot_script_sigs[lookup_key] = sig_out;
            return true;
        }
        return false;
        */
}

pub fn sign_taproot_script(
        provider:     &SigningProvider,
        creator:      &dyn BaseSignatureCreator,
        sigdata:      &mut SignatureData,
        leaf_version: i32,
        script:       &Script,
        result:       &mut Vec<ValType>) -> bool {
    
    todo!();
        /*
            // Only BIP342 tapscript signing is supported for now.
        if (leaf_version != TAPROOT_LEAF_TAPSCRIPT) return false;
        SigVersion sigversion = SigVersion::TAPSCRIPT;

        uint256 leaf_hash = (CHashWriter(HASHER_TAPLEAF) << uint8_t(leaf_version) << script).GetSHA256();

        // <xonly pubkey> OP_CHECKSIG
        if (script.size() == 34 && script[33] == OP_CHECKSIG && script[0] == 0x20) {
            XOnlyPubKey pubkey(MakeSpan(script).subspan(1, 32));
            std::vector<unsigned char> sig;
            if (CreateTaprootScriptSig(creator, sigdata, provider, sig, pubkey, leaf_hash, sigversion)) {
                result = Vector(std::move(sig));
                return true;
            }
        }

        return false;
        */
}



