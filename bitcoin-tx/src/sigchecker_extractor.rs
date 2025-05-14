// ---------------- [ File: bitcoin-tx/src/sigchecker_extractor.rs ]
crate::ix!();

///---------------------------
pub struct SignatureExtractorChecker {
    base:    DeferringSignatureChecker,
    sigdata: Rc<RefCell<SignatureData>>,
}

impl SignatureExtractorChecker {

    pub fn new(
        sigdata: &mut SignatureData,
        checker: &mut Box<dyn BaseSignatureChecker>) -> Self {
    
        todo!();
        /*
        : deferring_signature_checker(checker),
        : sigdata(sigdata),
        */
    }
    
    pub fn check_ecdsa_signature(&self, 
        script_sig:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            if (m_checker.CheckECDSASignature(scriptSig, vchPubKey, scriptCode, sigversion)) {
                CPubKey pubkey(vchPubKey);
                sigdata.signatures.emplace(pubkey.GetID(), SigPair(pubkey, scriptSig));
                return true;
            }
            return false;
        */
    }
}
