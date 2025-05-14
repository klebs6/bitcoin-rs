// ---------------- [ File: bitcoin-tx/src/sigchecker_dummy.rs ]
crate::ix!();

/**
  | Dummy signature checker which accepts
  | all signatures.
  |
  */
#[derive(Default)]
pub struct DummySignatureChecker {

}

impl BaseSignatureChecker for DummySignatureChecker {

}

impl DummySignatureChecker {

    pub fn check_ecdsa_signature(&self, 
        script_sig:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn check_schnorr_signature(&self, 
        sig:        &[u8],
        pubkey:     &[u8],
        sigversion: SigVersion,
        execdata:   &ScriptExecutionData,
        serror:     *mut ScriptError) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

pub const DUMMY_CHECKER: DummySignatureChecker = DummySignatureChecker {};
