// ---------------- [ File: bitcoin-signingprovider/src/outputtype.rs ]
crate::ix!();

/**
  | Get a destination of the requested type
  | (if possible) to the specified key.
  | 
  | The caller must make sure LearnRelatedScripts
  | has been called beforehand.
  |
  */
pub fn get_destination_for_key(
    key: &PubKey,
    ty:  OutputType) -> TxDestination {
    
    todo!();
        /*
        switch (type) {
            case OutputType::LEGACY: return PKHash(key);
            case OutputType::P2SH_SEGWIT:
            case OutputType::BECH32: {
                if (!key.IsCompressed()) return PKHash(key);
                TxDestination witdest = WitnessV0KeyHash(key);
                CScript witprog = GetScriptForDestination(witdest);
                if (type == OutputType::P2SH_SEGWIT) {
                    return ScriptHash(witprog);
                } else {
                    return witdest;
                }
            }
            case OutputType::BECH32M: {
                // This function should never be used with BECH32M, so let it assert
            } 
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}

/**
  | Get all destinations (potentially)
  | supported by the wallet for the given
  | key.
  |
  */
pub fn get_all_destinations_for_key(key: &PubKey) -> Vec<TxDestination> {
    
    todo!();
        /*
        PKHash keyid(key);
        TxDestination p2pkh{keyid};
        if (key.IsCompressed()) {
            TxDestination segwit = WitnessV0KeyHash(keyid);
            TxDestination p2sh = ScriptHash(GetScriptForDestination(segwit));
            return Vector(std::move(p2pkh), std::move(p2sh), std::move(segwit));
        } else {
            return Vector(std::move(p2pkh));
        }
        */
}

/**
  | Get a destination of the requested type
  | (if possible) to the specified script.
  | 
  | This function will automatically add
  | the script (and any other necessary
  | scripts) to the keystore.
  |
  */
pub fn add_and_get_destination_for_script<T>(
    keystore: &mut FillableSigningProvider<T>,
    script:   &Script,
    ty:       OutputType) -> TxDestination {
    
    todo!();
        /*
        // Add script to keystore
        keystore.AddCScript(script);
        // Note that scripts over 520 bytes are not yet supported.
        switch (type) {
        case OutputType::LEGACY:
            return ScriptHash(script);
        case OutputType::P2SH_SEGWIT:
        case OutputType::BECH32: {
            TxDestination witdest = WitnessV0ScriptHash(script);
            CScript witprog = GetScriptForDestination(witdest);
            // Check if the resulting program is solvable (i.e. doesn't use an uncompressed key)
            if (!IsSolvable(keystore, witprog)) return ScriptHash(script);
            // Add the redeemscript, so that P2WSH and P2SH-P2WSH outputs are recognized as ours.
            keystore.AddCScript(witprog);
            if (type == OutputType::BECH32) {
                return witdest;
            } else {
                return ScriptHash(witprog);
            }
        }
        case OutputType::BECH32M: {} // This function should not be used for BECH32M, so let it assert
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
}

/**
  | Get the OutputType for a TxDestination
  |
  */
pub fn output_type_from_destination(dest: &TxDestination) -> Option<OutputType> {
    
    todo!();
        /*
            if (std::holds_alternative<PKHash>(dest) ||
            std::holds_alternative<ScriptHash>(dest)) {
            return OutputType::LEGACY;
        }
        if (std::holds_alternative<WitnessV0KeyHash>(dest) ||
            std::holds_alternative<WitnessV0ScriptHash>(dest)) {
            return OutputType::BECH32;
        }
        if (std::holds_alternative<WitnessV1Taproot>(dest) ||
            std::holds_alternative<WitnessUnknown>(dest)) {
            return OutputType::BECH32M;
        }
        return std::nullopt;
        */
}
