// ---------------- [ File: bitcoin-scriptpubkeyman/src/is_mine.rs ]
crate::ix!();

/**
  | Value for the first BIP 32 hardened derivation.
  | Can be used as a bit mask and as a value.
  | See BIP 32 for more details.
  |
  */
pub const BIP32_HARDENED_KEY_LIMIT: u32 = 0x80000000;

pub type ValType = Vec<u8>;

/**
  | This is an enum that tracks the execution
  | context of a script, similar to
  | 
  | SigVersion in script/interpreter.
  | It is separate however because we want
  | to distinguish between top-level scriptPubKey
  | execution and P2SH redeemScript execution
  | (a distinction that has no impact on
  | consensus rules).
  |
  */
pub enum IsMineSigVersion
{

    /**
      | scriptPubKey execution
      |
      */
    TOP = 0,        

    /**
      | P2SH redeemScript
      |
      */
    P2SH = 1,       

    /**
      | P2WSH witness script execution
      |
      */
    WITNESS_V0 = 2, 
}

/**
  | This is an internal representation
  | of isminetype + invalidity.
  | 
  | Its order is significant, as we return
  | the max of all explored possibilities.
  |
  */
pub enum IsMineResult
{
    /**
      | Not ours
      |
      */
    NO = 0,         

    /**
      | Included in watch-only balance
      |
      */
    WATCH_ONLY = 1, 

    /**
      | Included in all balances
      |
      */
    SPENDABLE = 2,  

    /**
      | Not spendable by anyone (uncompressed
      | pubkey in segwit, P2SH inside P2SH or
      | witness, witness inside witness)
      |
      */
    INVALID = 3,    
}

pub fn permits_uncompressed(sigversion: IsMineSigVersion) -> bool {
    
    todo!();
        /*
            return sigversion == IsMineSigVersion::TOP || sigversion == IsMineSigVersion::P2SH;
        */
}

/**
  | Recursively solve script and return
  | spendable/watchonly/invalid status.
  | 
  | -----------
  | @param keystore
  | 
  | legacy key and script store
  | ----------
  | @param scriptPubKey
  | 
  | script to solve
  | ----------
  | @param sigversion
  | 
  | script type (top-level / redeemscript
  | / witnessscript)
  | ----------
  | @param recurse_scripthash
  | 
  | whether to recurse into nested p2sh
  | and p2wsh scripts or simply treat any
  | script that has been stored in the keystore
  | as spendable
  |
  */
pub fn is_mine_inner(
        keystore:           &LegacyScriptPubKeyMan,
        script_pub_key:     &Script,
        sigversion:         IsMineSigVersion,
        recurse_scripthash: Option<bool>) -> IsMineResult {
    let recurse_scripthash: bool = recurse_scripthash.unwrap_or(true);

    todo!();
        /*
            IsMineResult ret = IsMineResult::NO;

        std::vector<valtype> vSolutions;
        TxoutType whichType = Solver(scriptPubKey, vSolutions);

        CKeyID keyID;
        switch (whichType) {
        case TxoutType::NONSTANDARD:
        case TxoutType::NULL_DATA:
        case TxoutType::WITNESS_UNKNOWN:
        case TxoutType::WITNESS_V1_TAPROOT:
            break;
        case TxoutType::PUBKEY:
            keyID = CPubKey(vSolutions[0]).GetID();
            if (!PermitsUncompressed(sigversion) && vSolutions[0].size() != 33) {
                return IsMineResult::INVALID;
            }
            if (keystore.HaveKey(keyID)) {
                ret = std::max(ret, IsMineResult::SPENDABLE);
            }
            break;
        case TxoutType::WITNESS_V0_KEYHASH:
        {
            if (sigversion == IsMineSigVersion::WITNESS_V0) {
                // P2WPKH inside P2WSH is invalid.
                return IsMineResult::INVALID;
            }
            if (sigversion == IsMineSigVersion::TOP && !keystore.HaveCScript(CScriptID(CScript() << OP_0 << vSolutions[0]))) {
                // We do not support bare witness outputs unless the P2SH version of it would be
                // acceptable as well. This protects against matching before segwit activates.
                // This also applies to the P2WSH case.
                break;
            }
            ret = std::max(ret, IsMineInner(keystore, GetScriptForDestination(PKHash(u160(vSolutions[0]))), IsMineSigVersion::WITNESS_V0));
            break;
        }
        case TxoutType::PUBKEYHASH:
            keyID = CKeyID(u160(vSolutions[0]));
            if (!PermitsUncompressed(sigversion)) {
                CPubKey pubkey;
                if (keystore.GetPubKey(keyID, pubkey) && !pubkey.IsCompressed()) {
                    return IsMineResult::INVALID;
                }
            }
            if (keystore.HaveKey(keyID)) {
                ret = std::max(ret, IsMineResult::SPENDABLE);
            }
            break;
        case TxoutType::SCRIPTHASH:
        {
            if (sigversion != IsMineSigVersion::TOP) {
                // P2SH inside P2WSH or P2SH is invalid.
                return IsMineResult::INVALID;
            }
            CScriptID scriptID = CScriptID(u160(vSolutions[0]));
            CScript subscript;
            if (keystore.GetCScript(scriptID, subscript)) {
                ret = std::max(ret, recurse_scripthash ? IsMineInner(keystore, subscript, IsMineSigVersion::P2SH) : IsMineResult::SPENDABLE);
            }
            break;
        }
        case TxoutType::WITNESS_V0_SCRIPTHASH:
        {
            if (sigversion == IsMineSigVersion::WITNESS_V0) {
                // P2WSH inside P2WSH is invalid.
                return IsMineResult::INVALID;
            }
            if (sigversion == IsMineSigVersion::TOP && !keystore.HaveCScript(CScriptID(CScript() << OP_0 << vSolutions[0]))) {
                break;
            }
            u160 hash;
            CRIPEMD160().Write(vSolutions[0].data(), vSolutions[0].size()).Finalize(hash.begin());
            CScriptID scriptID = CScriptID(hash);
            CScript subscript;
            if (keystore.GetCScript(scriptID, subscript)) {
                ret = std::max(ret, recurse_scripthash ? IsMineInner(keystore, subscript, IsMineSigVersion::WITNESS_V0) : IsMineResult::SPENDABLE);
            }
            break;
        }

        case TxoutType::MULTISIG:
        {
            // Never treat bare multisig outputs as ours (they can still be made watchonly-though)
            if (sigversion == IsMineSigVersion::TOP) {
                break;
            }

            // Only consider transactions "mine" if we own ALL the
            // keys involved. Multi-signature transactions that are
            // partially owned (somebody else has a key that can spend
            // them) enable spend-out-from-under-you attacks, especially
            // in shared-wallet situations.
            std::vector<valtype> keys(vSolutions.begin()+1, vSolutions.begin()+vSolutions.size()-1);
            if (!PermitsUncompressed(sigversion)) {
                for (size_t i = 0; i < keys.size(); i++) {
                    if (keys[i].size() != 33) {
                        return IsMineResult::INVALID;
                    }
                }
            }
            if (HaveKeys(keys, keystore)) {
                ret = std::max(ret, IsMineResult::SPENDABLE);
            }
            break;
        }
        } // no default case, so the compiler can warn about missing cases

        if (ret == IsMineResult::NO && keystore.HaveWatchOnly(scriptPubKey)) {
            ret = std::max(ret, IsMineResult::WATCH_ONLY);
        }
        return ret;
        */
}
