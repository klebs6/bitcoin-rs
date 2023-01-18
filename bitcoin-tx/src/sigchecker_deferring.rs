crate::ix!();

///-----------------------
pub struct DeferringSignatureChecker {
    checker: Rc<RefCell<Box<dyn BaseSignatureChecker>>>,
}

impl BaseSignatureChecker for DeferringSignatureChecker {

}

impl From<&mut Box<dyn BaseSignatureChecker>> for DeferringSignatureChecker {

    fn from(checker: &mut Box<dyn BaseSignatureChecker>) -> Self {
    
        todo!();
        /*
        : checker(checker),
        */
    }
}

impl DeferringSignatureChecker {
    
    pub fn check_ecdsa_signature(&self, 
        script_sig:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            return m_checker.CheckECDSASignature(scriptSig, vchPubKey, scriptCode, sigversion);
        */
    }
    
    pub fn check_schnorr_signature(&self, 
        sig:        &[u8],
        pubkey:     &[u8],
        sigversion: SigVersion,
        execdata:   &ScriptExecutionData,
        serror:     Option<*mut ScriptError>) -> bool {

        todo!();
        /*
            return m_checker.CheckSchnorrSignature(sig, pubkey, sigversion, execdata, serror);
        */
    }
    
    pub fn check_lock_time(&self, n_lock_time: &ScriptNum) -> bool {
        
        todo!();
        /*
            return m_checker.CheckLockTime(nLockTime);
        */
    }
    
    pub fn check_sequence(&self, n_sequence: &ScriptNum) -> bool {
        
        todo!();
        /*
            return m_checker.CheckSequence(nSequence);
        */
    }
}

