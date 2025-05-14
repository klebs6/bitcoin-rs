// ---------------- [ File: bitcoin-tx/src/sigchecker_caching.rs ]
crate::ix!();

pub struct CachingTransactionSignatureChecker {
    base:  TransactionSignatureChecker,
    store: bool,
}

impl CachingTransactionSignatureChecker {

    pub fn new(
        tx_to_in:  *const Transaction,
        n_in_in:   u32,
        amount_in: &Amount,
        store_in:  bool,
        txdata_in: &mut PrecomputedTransactionData) -> Self {
    
        todo!();
        /*
        : transaction_signature_checker(txToIn, nInIn, amountIn, txdataIn, MissingDataBehavior::ASSERT_FAIL),
        : store(storeIn),
        */
    }
    
    pub fn verify_ecdsa_signature(&self, 
        vch_sig: &Vec<u8>,
        pubkey:  &PubKey,
        sighash: &u256) -> bool {
        
        todo!();
        /*
            uint256 entry;
        signatureCache.ComputeEntryECDSA(entry, sighash, vchSig, pubkey);
        if (signatureCache.Get(entry, !store))
            return true;
        if (!TransactionSignatureChecker::VerifyECDSASignature(vchSig, pubkey, sighash))
            return false;
        if (store)
            signatureCache.Set(entry);
        return true;
        */
    }
    
    pub fn verify_schnorr_signature(&self, 
        sig:     &[u8],
        pubkey:  &XOnlyPubKey,
        sighash: &u256) -> bool {
        
        todo!();
        /*
            uint256 entry;
        signatureCache.ComputeEntrySchnorr(entry, sighash, sig, pubkey);
        if (signatureCache.Get(entry, !store)) return true;
        if (!TransactionSignatureChecker::VerifySchnorrSignature(sig, pubkey, sighash)) return false;
        if (store) signatureCache.Set(entry);
        return true;
        */
    }
}
