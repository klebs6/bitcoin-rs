// ---------------- [ File: bitcoin-tx/src/sighash.rs ]
crate::ix!();

pub fn signature_hash_schnorr<T>(
        hash_out:   &mut u256,
        execdata:   &ScriptExecutionData,
        tx_to:      &T,
        in_pos:     u32,
        hash_type:  u8,
        sigversion: SigVersion,
        cache:      &PrecomputedTransactionData,
        mdb:        MissingDataBehavior) -> bool {

    todo!();
        /*
            uint8_t ext_flag, key_version;
        switch (sigversion) {
        case SigVersion::TAPROOT:
            ext_flag = 0;
            // key_version is not used and left uninitialized.
            break;
        case SigVersion::TAPSCRIPT:
            ext_flag = 1;
            // key_version must be 0 for now, representing the current version of
            // 32-byte public keys in the tapscript signature opcode execution.
            // An upgradable public key version (with a size not 32-byte) may
            // request a different key_version with a new sigversion.
            key_version = 0;
            break;
        default:
            assert(false);
        }
        assert(in_pos < tx_to.vin.size());
        if (!(cache.m_bip341_taproot_ready && cache.m_spent_outputs_ready)) {
            return HandleMissingData(mdb);
        }

        CHashWriter ss = HASHER_TAPSIGHASH;

        // Epoch
        static constexpr uint8_t EPOCH = 0;
        ss << EPOCH;

        // Hash type
        const uint8_t output_type = (hash_type == SIGHASH_DEFAULT) ? SIGHASH_ALL : (hash_type & SIGHASH_OUTPUT_MASK); // Default (no sighash byte) is equivalent to SIGHASH_ALL
        const uint8_t input_type = hash_type & SIGHASH_INPUT_MASK;
        if (!(hash_type <= 0x03 || (hash_type >= 0x81 && hash_type <= 0x83))) return false;
        ss << hash_type;

        // Transaction level data
        ss << tx_to.nVersion;
        ss << tx_to.nLockTime;
        if (input_type != SIGHASH_ANYONECANPAY) {
            ss << cache.m_prevouts_single_hash;
            ss << cache.m_spent_amounts_single_hash;
            ss << cache.m_spent_scripts_single_hash;
            ss << cache.m_sequences_single_hash;
        }
        if (output_type == SIGHASH_ALL) {
            ss << cache.m_outputs_single_hash;
        }

        // Data about the input/prevout being spent
        assert(execdata.m_annex_init);
        const bool have_annex = execdata.m_annex_present;
        const uint8_t spend_type = (ext_flag << 1) + (have_annex ? 1 : 0); // The low bit indicates whether an annex is present.
        ss << spend_type;
        if (input_type == SIGHASH_ANYONECANPAY) {
            ss << tx_to.vin[in_pos].prevout;
            ss << cache.m_spent_outputs[in_pos];
            ss << tx_to.vin[in_pos].nSequence;
        } else {
            ss << in_pos;
        }
        if (have_annex) {
            ss << execdata.m_annex_hash;
        }

        // Data about the output (if only one).
        if (output_type == SIGHASH_SINGLE) {
            if (in_pos >= tx_to.vout.size()) return false;
            CHashWriter sha_single_output(SER_GETHASH, 0);
            sha_single_output << tx_to.vout[in_pos];
            ss << sha_single_output.GetSHA256();
        }

        // Additional data for BIP 342 signatures
        if (sigversion == SigVersion::TAPSCRIPT) {
            assert(execdata.m_tapleaf_hash_init);
            ss << execdata.m_tapleaf_hash;
            ss << key_version;
            assert(execdata.m_codeseparator_pos_init);
            ss << execdata.m_codeseparator_pos;
        }

        hash_out = ss.GetSHA256();
        return true;
        */
}

pub fn signature_hash<T>(
        script_code: &Script,
        tx_to:       &T,
        n_in:        u32,
        n_hash_type: i32,
        amount:      &Amount,
        sigversion:  SigVersion,
        cache:       Option<*const PrecomputedTransactionData>) -> u256 {

    todo!();
        /*
            assert(nIn < txTo.vin.size());

        if (sigversion == SigVersion::WITNESS_V0) {
            uint256 hashPrevouts;
            uint256 hashSequence;
            uint256 hashOutputs;
            const bool cacheready = cache && cache->m_bip143_segwit_ready;

            if (!(nHashType & SIGHASH_ANYONECANPAY)) {
                hashPrevouts = cacheready ? cache->hashPrevouts : SHA256Uint256(GetPrevoutsSHA256(txTo));
            }

            if (!(nHashType & SIGHASH_ANYONECANPAY) && (nHashType & 0x1f) != SIGHASH_SINGLE && (nHashType & 0x1f) != SIGHASH_NONE) {
                hashSequence = cacheready ? cache->hashSequence : SHA256Uint256(GetSequencesSHA256(txTo));
            }

            if ((nHashType & 0x1f) != SIGHASH_SINGLE && (nHashType & 0x1f) != SIGHASH_NONE) {
                hashOutputs = cacheready ? cache->hashOutputs : SHA256Uint256(GetOutputsSHA256(txTo));
            } else if ((nHashType & 0x1f) == SIGHASH_SINGLE && nIn < txTo.vout.size()) {
                CHashWriter ss(SER_GETHASH, 0);
                ss << txTo.vout[nIn];
                hashOutputs = ss.GetHash();
            }

            CHashWriter ss(SER_GETHASH, 0);
            // Version
            ss << txTo.nVersion;
            // Input prevouts/nSequence (none/all, depending on flags)
            ss << hashPrevouts;
            ss << hashSequence;
            // The input being signed (replacing the scriptSig with scriptCode + amount)
            // The prevout may already be contained in hashPrevout, and the nSequence
            // may already be contain in hashSequence.
            ss << txTo.vin[nIn].prevout;
            ss << scriptCode;
            ss << amount;
            ss << txTo.vin[nIn].nSequence;
            // Outputs (none/one/all, depending on flags)
            ss << hashOutputs;
            // Locktime
            ss << txTo.nLockTime;
            // Sighash type
            ss << nHashType;

            return ss.GetHash();
        }

        // Check for invalid use of SIGHASH_SINGLE
        if ((nHashType & 0x1f) == SIGHASH_SINGLE) {
            if (nIn >= txTo.vout.size()) {
                //  nOut out of range
                return uint256::ONE;
            }
        }

        // Wrapper to serialize only the necessary parts of the transaction being signed
        CTransactionSignatureSerializer<T> txTmp(txTo, scriptCode, nIn, nHashType);

        // Serialize and hash
        CHashWriter ss(SER_GETHASH, 0);
        ss << txTmp << nHashType;
        return ss.GetHash();
        */
}
