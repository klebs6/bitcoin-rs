// ---------------- [ File: bitcoin-scriptpubkeyman/src/access.rs ]
crate::ix!();

pub fn get_oldest_key_time_in_pool(
        set_key_pool: &HashSet<i64>,
        batch:        &mut WalletBatch) -> i64 {
    
    todo!();
        /*
            if (setKeyPool.empty()) {
            return GetTime();
        }

        CKeyPool keypool;
        int64_t nIndex = *(setKeyPool.begin());
        if (!batch.ReadPool(nIndex, keypool)) {
            throw std::runtime_error(std::string(__func__) + ": read oldest key in keypool failed");
        }
        assert(keypool.vchPubKey.IsValid());
        return keypool.nTime;
        */
}

pub fn extract_pub_key(
        dest:        &Script,
        pub_key_out: &mut PubKey) -> bool {
    
    todo!();
        /*
            std::vector<std::vector<unsigned char>> solutions;
        return Solver(dest, solutions) == TxoutType::PUBKEY &&
            (pubKeyOut = CPubKey(solutions[0])).IsFullyValid();
        */
}

pub fn get_affected_keys(
        spk:      &Script,
        provider: &SigningProvider) -> Vec<KeyID> {
    
    todo!();
        /*
            std::vector<CScript> dummy;
        FlatSigningProvider out;
        InferDescriptor(spk, provider)->Expand(0, DUMMY_SIGNING_PROVIDER, dummy, out);
        std::vector<CKeyID> ret;
        for (const auto& entry : out.pubkeys) {
            ret.push_back(entry.first);
        }
        return ret;
        */
}

/**
  | OutputTypes supported by the LegacyScriptPubKeyMan
  |
  */
lazy_static!{
    /*
    static const std::unordered_set<OutputType> LEGACY_OUTPUT_TYPES {
        OutputType::LEGACY,
        OutputType::P2SH_SEGWIT,
        OutputType::BECH32,
    };
    */
}

pub fn have_keys(
        pubkeys:  &Vec<ValType>,
        keystore: &LegacyScriptPubKeyMan) -> bool {
    
    todo!();
        /*
            for (const valtype& pubkey : pubkeys) {
            CKeyID keyID = CPubKey(pubkey).GetID();
            if (!keystore.HaveKey(keyID)) return false;
        }
        return true;
        */
}
