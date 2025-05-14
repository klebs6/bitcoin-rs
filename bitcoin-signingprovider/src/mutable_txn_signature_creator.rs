// ---------------- [ File: bitcoin-signingprovider/src/mutable_txn_signature_creator.rs ]
crate::ix!();

/**
  | A signature creator for transactions.
  |
  */
pub struct MutableTransactionSignatureCreator {
    pub tx_to:       *const MutableTransaction,
    pub n_in:        u32,
    pub n_hash_type: i32,
    pub amount:      Amount,
    pub checker:     MutableTransactionSignatureChecker,
    pub txdata:      *const PrecomputedTransactionData,
}

unsafe impl Send for MutableTransactionSignatureCreator {}
unsafe impl Sync for MutableTransactionSignatureCreator {}

impl BaseSignatureCreator for MutableTransactionSignatureCreator {

}

impl Checker for MutableTransactionSignatureCreator {

    fn checker(&self) -> &Box<dyn BaseSignatureChecker> {
        
        todo!();
        /*
            return checker;
        */
    }
}

impl MutableTransactionSignatureCreator {
    
    pub fn new(
        tx_to_in:       *const MutableTransaction,
        n_in_in:        u32,
        amount_in:      &Amount,
        n_hash_type_in: i32) -> Self {
    
        todo!();
        /*
            : txTo(txToIn), nIn(nInIn), nHashType(nHashTypeIn), amount(amountIn), checker(txTo, nIn, amountIn, MissingDataBehavior::FAIL),
          m_txdata(nullptr)
        */
    }
    
    pub fn new_with_txdata(
        tx_to_in:       *const MutableTransaction,
        n_in_in:        u32,
        amount_in:      &Amount,
        txdata:         *const PrecomputedTransactionData,
        n_hash_type_in: i32) -> Self {
    
        todo!();
        /*
            : txTo(txToIn), nIn(nInIn), nHashType(nHashTypeIn), amount(amountIn),
          checker(txdata ? MutableTransactionSignatureChecker(txTo, nIn, amount, *txdata, MissingDataBehavior::FAIL) :
              MutableTransactionSignatureChecker(txTo, nIn, amount, MissingDataBehavior::FAIL)),
          m_txdata(txdata)
        */
    }
}

impl CreateSig for MutableTransactionSignatureCreator {

    fn create_sig(&self, 
        provider:    &SigningProvider,
        vch_sig:     &mut Vec<u8>,
        address:     &KeyID,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            assert(sigversion == SigVersion::BASE || sigversion == SigVersion::WITNESS_V0);

        CKey key;
        if (!provider.GetKey(address, key))
            return false;

        // Signing with uncompressed keys is disabled in witness scripts
        if (sigversion == SigVersion::WITNESS_V0 && !key.IsCompressed())
            return false;

        // Signing without known amount does not work in witness scripts.
        if (sigversion == SigVersion::WITNESS_V0 && !MoneyRange(amount)) return false;

        // BASE/WITNESS_V0 signatures don't support explicit SIGHASH_DEFAULT, use SIGHASH_ALL instead.
        const int hashtype = nHashType == SIGHASH_DEFAULT ? SIGHASH_ALL : nHashType;

        uint256 hash = SignatureHash(scriptCode, *txTo, nIn, hashtype, amount, sigversion, m_txdata);
        if (!key.Sign(hash, vchSig))
            return false;
        vchSig.push_back((unsigned char)hashtype);
        return true;
        */
    }
}

impl CreateSchnorrSig for MutableTransactionSignatureCreator {
    
    fn create_schnorr_sig(&self, 
        provider:    &SigningProvider,
        sig:         &mut Vec<u8>,
        pubkey:      &XOnlyPubKey,
        leaf_hash:   *const u256,
        merkle_root: *const u256,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            assert(sigversion == SigVersion::TAPROOT || sigversion == SigVersion::TAPSCRIPT);

        CKey key;
        if (!provider.GetKeyByXOnly(pubkey, key)) return false;

        // BIP341/BIP342 signing needs lots of precomputed transaction data. While some
        // (non-SIGHASH_DEFAULT) sighash modes exist that can work with just some subset
        // of data present, for now, only support signing when everything is provided.
        if (!m_txdata || !m_txdata->m_bip341_taproot_ready || !m_txdata->m_spent_outputs_ready) return false;

        ScriptExecutionData execdata;
        execdata.m_annex_init = true;
        execdata.m_annex_present = false; // Only support annex-less signing for now.
        if (sigversion == SigVersion::TAPSCRIPT) {
            execdata.m_codeseparator_pos_init = true;
            execdata.m_codeseparator_pos = 0xFFFFFFFF; // Only support non-OP_CODESEPARATOR BIP342 signing for now.
            if (!leaf_hash) return false; // BIP342 signing needs leaf hash.
            execdata.m_tapleaf_hash_init = true;
            execdata.m_tapleaf_hash = *leaf_hash;
        }
        uint256 hash;
        if (!SignatureHashSchnorr(hash, execdata, *txTo, nIn, nHashType, sigversion, *m_txdata, MissingDataBehavior::FAIL)) return false;
        sig.resize(64);
        if (!key.SignSchnorr(hash, sig, merkle_root, nullptr)) return false;
        if (nHashType) sig.push_back(nHashType);
        return true;
        */
    }
}
