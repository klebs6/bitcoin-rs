// ---------------- [ File: bitcoin-tx/src/sigchecker.rs ]
crate::ix!();

///--------------------
pub struct GenericTransactionSignatureChecker<T> {
    tx_to:  *const T,
    mdb:    MissingDataBehavior,
    n_in:   u32,
    amount: Amount,
    txdata: *const PrecomputedTransactionData,
}

impl<T> BaseSignatureChecker for GenericTransactionSignatureChecker<T> {

}

impl<T> VerifyECDSASignature for GenericTransactionSignatureChecker<T> {

    fn verify_ecdsa_signature(&self, 
        vch_sig:     &Vec<u8>,
        vch_pub_key: &PubKey,
        sighash:     &u256) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl<T> VerifySchnorrSignature for GenericTransactionSignatureChecker<T> {

    fn verify_schnorr_signature(&self, 
        sig:     &[u8],
        pubkey:  &XOnlyPubKey,
        sighash: &u256) -> bool {
        
        todo!();
        /*
        
        */
    }
}

impl<T> GenericTransactionSignatureChecker<T> {

    pub fn new(
        tx_to_in:  *const T,
        n_in_in:   u32,
        amount_in: &Amount,
        mdb:       MissingDataBehavior) -> Self {
    
        todo!();
        /*
        : tx_to(txToIn),
        : mdb(mdb),
        : n_in(nInIn),
        : amount(amountIn),
        : txdata(nullptr),
        */
    }
    
    pub fn new_with_txdata_in(
        tx_to_in:  *const T,
        n_in_in:   u32,
        amount_in: &Amount,
        txdata_in: &PrecomputedTransactionData,
        mdb:       MissingDataBehavior) -> Self {
    
        todo!();
        /*
        : tx_to(txToIn),
        : mdb(mdb),
        : n_in(nInIn),
        : amount(amountIn),
        : txdata(&txdataIn),
        */
    }
    
    pub fn verify_ecdsa_signature(&self, 
        vch_sig: &Vec<u8>,
        pubkey:  &PubKey,
        sighash: &u256) -> bool {
        
        todo!();
        /*
            return pubkey.Verify(sighash, vchSig);
        */
    }
    
    pub fn verify_schnorr_signature(&self, 
        sig:     &[u8],
        pubkey:  &XOnlyPubKey,
        sighash: &u256) -> bool {
        
        todo!();
        /*
            return pubkey.VerifySchnorr(sighash, sig);
        */
    }
    
    pub fn check_ecdsa_signature(&self, 
        vch_sig_in:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            CPubKey pubkey(vchPubKey);
        if (!pubkey.IsValid())
            return false;

        // Hash type is one byte tacked on to the end of the signature
        std::vector<unsigned char> vchSig(vchSigIn);
        if (vchSig.empty())
            return false;
        int nHashType = vchSig.back();
        vchSig.pop_back();

        // Witness sighashes need the amount.
        if (sigversion == SigVersion::WITNESS_V0 && amount < 0) return HandleMissingData(m_mdb);

        uint256 sighash = SignatureHash(scriptCode, *txTo, nIn, nHashType, amount, sigversion, this->txdata);

        if (!VerifyECDSASignature(vchSig, pubkey, sighash))
            return false;

        return true;
        */
    }
    
    pub fn check_schnorr_signature(&self, 
        sig:        &[u8],
        pubkey_in:  &[u8],
        sigversion: SigVersion,
        execdata:   &ScriptExecutionData,
        serror:     *mut ScriptError) -> bool {
        
        todo!();
        /*
            assert(sigversion == SigVersion::TAPROOT || sigversion == SigVersion::TAPSCRIPT);
        // Schnorr signatures have 32-byte public keys. The caller is responsible for enforcing this.
        assert(pubkey_in.size() == 32);
        // Note that in Tapscript evaluation, empty signatures are treated specially (invalid signature that does not
        // abort script execution). This is implemented in EvalChecksigTapscript, which won't invoke
        // CheckSchnorrSignature in that case. In other contexts, they are invalid like every other signature with
        // size different from 64 or 65.
        if (sig.size() != 64 && sig.size() != 65) return set_error(serror, SCRIPT_ERR_SCHNORR_SIG_SIZE);

        XOnlyPubKey pubkey{pubkey_in};

        uint8_t hashtype = SIGHASH_DEFAULT;
        if (sig.size() == 65) {
            hashtype = SpanPopBack(sig);
            if (hashtype == SIGHASH_DEFAULT) return set_error(serror, SCRIPT_ERR_SCHNORR_SIG_HASHTYPE);
        }
        uint256 sighash;
        if (!this->txdata) return HandleMissingData(m_mdb);
        if (!SignatureHashSchnorr(sighash, execdata, *txTo, nIn, hashtype, sigversion, *this->txdata, m_mdb)) {
            return set_error(serror, SCRIPT_ERR_SCHNORR_SIG_HASHTYPE);
        }
        if (!VerifySchnorrSignature(sig, pubkey, sighash)) return set_error(serror, SCRIPT_ERR_SCHNORR_SIG);
        return true;
        */
    }
    
    pub fn check_lock_time(&self, n_lock_time: &ScriptNum) -> bool {
        
        todo!();
        /*
            // There are two kinds of nLockTime: lock-by-blockheight
        // and lock-by-blocktime, distinguished by whether
        // nLockTime < LOCKTIME_THRESHOLD.
        //
        // We want to compare apples to apples, so fail the script
        // unless the type of nLockTime being tested is the same as
        // the nLockTime in the transaction.
        if (!(
            (txTo->nLockTime <  LOCKTIME_THRESHOLD && nLockTime <  LOCKTIME_THRESHOLD) ||
            (txTo->nLockTime >= LOCKTIME_THRESHOLD && nLockTime >= LOCKTIME_THRESHOLD)
        ))
            return false;

        // Now that we know we're comparing apples-to-apples, the
        // comparison is a simple numeric one.
        if (nLockTime > (int64_t)txTo->nLockTime)
            return false;

        // Finally the nLockTime feature can be disabled in IsFinalTx()
        // and thus CHECKLOCKTIMEVERIFY bypassed if every txin has
        // been finalized by setting nSequence to maxint. The
        // transaction would be allowed into the blockchain, making
        // the opcode ineffective.
        //
        // Testing if this vin is not final is sufficient to
        // prevent this condition. Alternatively we could test all
        // inputs, but testing just this input minimizes the data
        // required to prove correct CHECKLOCKTIMEVERIFY execution.
        if (CTxIn::SEQUENCE_FINAL == txTo->vin[nIn].nSequence)
            return false;

        return true;
        */
    }
    
    pub fn check_sequence(&self, n_sequence: &ScriptNum) -> bool {
        
        todo!();
        /*
            // Relative lock times are supported by comparing the passed
        // in operand to the sequence number of the input.
        const int64_t txToSequence = (int64_t)txTo->vin[nIn].nSequence;

        // Fail if the transaction's version number is not set high
        // enough to trigger BIP 68 rules.
        if (static_cast<uint32_t>(txTo->nVersion) < 2)
            return false;

        // Sequence numbers with their most significant bit set are not
        // consensus constrained. Testing that the transaction's sequence
        // number do not have this bit set prevents using this property
        // to get around a CHECKSEQUENCEVERIFY check.
        if (txToSequence & CTxIn::SEQUENCE_LOCKTIME_DISABLE_FLAG)
            return false;

        // Mask off any bits that do not have consensus-enforced meaning
        // before doing the integer comparisons
        const uint32_t nLockTimeMask = CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG | CTxIn::SEQUENCE_LOCKTIME_MASK;
        const int64_t txToSequenceMasked = txToSequence & nLockTimeMask;
        const CScriptNum nSequenceMasked = nSequence & nLockTimeMask;

        // There are two kinds of nSequence: lock-by-blockheight
        // and lock-by-blocktime, distinguished by whether
        // nSequenceMasked < CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG.
        //
        // We want to compare apples to apples, so fail the script
        // unless the type of nSequenceMasked being tested is the same as
        // the nSequenceMasked in the transaction.
        if (!(
            (txToSequenceMasked <  CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG && nSequenceMasked <  CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG) ||
            (txToSequenceMasked >= CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG && nSequenceMasked >= CTxIn::SEQUENCE_LOCKTIME_TYPE_FLAG)
        )) {
            return false;
        }

        // Now that we know we're comparing apples-to-apples, the
        // comparison is a simple numeric one.
        if (nSequenceMasked > txToSequenceMasked)
            return false;

        return true;
        */
    }
}

pub type TransactionSignatureChecker        = GenericTransactionSignatureChecker<Transaction>;
pub type MutableTransactionSignatureChecker = GenericTransactionSignatureChecker<MutableTransaction>;
