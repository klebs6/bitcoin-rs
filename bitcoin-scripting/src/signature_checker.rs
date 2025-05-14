// ---------------- [ File: bitcoin-scripting/src/signature_checker.rs ]
crate::ix!();

pub trait CheckECDSASignature {
    fn check_ecdsa_signature(&self, 
        script_sig:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool;
}

pub trait CheckSchnorrSignature {
    fn check_schnorr_signature(&self, 
        sig:        &[u8],
        pubkey:     &[u8],
        sigversion: SigVersion,
        execdata:   &ScriptExecutionData,
        serror:     Option<*mut ScriptError>) -> bool;
}

pub trait CheckLockTime {
    fn check_lock_time(&self, n_lock_time: &ScriptNum) -> bool;
}

pub trait CheckSequence {
    fn check_sequence(&self, n_sequence: &ScriptNum) -> bool;
}

pub trait BaseSignatureChecker { }

impl<T: BaseSignatureChecker> CheckECDSASignature for T {

    fn check_ecdsa_signature(&self, 
        script_sig:  &Vec<u8>,
        vch_pub_key: &Vec<u8>,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl<T: BaseSignatureChecker> CheckSchnorrSignature for T {

    fn check_schnorr_signature(&self, 
        sig:        &[u8],
        pubkey:     &[u8],
        sigversion: SigVersion,
        execdata:   &ScriptExecutionData,
        serror:     Option<*mut ScriptError>) -> bool {

        todo!();
        /*
            return false;
        */
    }
}

impl<T: BaseSignatureChecker> CheckLockTime for T {

    fn check_lock_time(&self, n_lock_time: &ScriptNum) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

impl<T: BaseSignatureChecker> CheckSequence for T {

    fn check_sequence(&self, n_sequence: &ScriptNum) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

/**
  | Enum to specify what *TransactionSignatureChecker's
  | behavior should be when dealing with
  | missing transaction data.
  |
  */
pub enum MissingDataBehavior
{
    /**
      | Abort execution through assertion
      | failure (for consensus code)
      |
      */
    ASSERT_FAIL,  

    /**
      | Just act as if the signature was invalid
      |
      */
    FAIL,         
}

pub fn handle_missing_data(mdb: MissingDataBehavior) -> bool {
    
    todo!();
        /*
            switch (mdb) {
        case MissingDataBehavior::ASSERT_FAIL:
            assert(!"Missing data");
            break;
        case MissingDataBehavior::FAIL:
            return false;
        }
        assert(!"Unknown MissingDataBehavior value");
        */
}
