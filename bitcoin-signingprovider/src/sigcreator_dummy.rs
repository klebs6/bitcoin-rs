// ---------------- [ File: bitcoin-signingprovider/src/sigcreator_dummy.rs ]
crate::ix!();

///-------------------------------
pub struct DummySignatureCreator {
    r_len: u8, // default = 32
    len:   u8, // default = 32
}

impl BaseSignatureCreator for DummySignatureCreator {

}

impl Checker for DummySignatureCreator {

    fn checker(&self) -> &Box<dyn BaseSignatureChecker> {
        
        todo!();
        /*
            return DUMMY_CHECKER;
        */
    }
}

impl CreateSig for DummySignatureCreator {
    fn create_sig(&self, 
        provider:    &SigningProvider,
        vch_sig:     &mut Vec<u8>,
        keyid:       &KeyID,
        script_code: &Script,
        sigversion:  SigVersion) -> bool {
        
        todo!();
        /*
            // Create a dummy signature that is a valid DER-encoding
            vchSig.assign(m_r_len + m_s_len + 7, '\000');
            vchSig[0] = 0x30;
            vchSig[1] = m_r_len + m_s_len + 4;
            vchSig[2] = 0x02;
            vchSig[3] = m_r_len;
            vchSig[4] = 0x01;
            vchSig[4 + m_r_len] = 0x02;
            vchSig[5 + m_r_len] = m_s_len;
            vchSig[6 + m_r_len] = 0x01;
            vchSig[6 + m_r_len + m_s_len] = SIGHASH_ALL;
            return true;
        */
    }
}

impl CreateSchnorrSig for DummySignatureCreator {
    fn create_schnorr_sig(&self, 
        provider:   &SigningProvider,
        sig:        &mut Vec<u8>,
        pubkey:     &XOnlyPubKey,
        leaf_hash:  *const u256,
        tweak:      *const u256,
        sigversion: SigVersion) -> bool {
        
        todo!();
        /*
            sig.assign(64, '\000');
            return true;
        */
    }
}

impl DummySignatureCreator {

    pub fn new(
        r_len: u8,
        len:   u8) -> Self {
    
        todo!();
        /*
        : r_len(r_len),
        : len(s_len),
        */
    }
}

lazy_static!{

    pub static ref DUMMY_SIGNATURE_CREATOR:         
    Arc<std::sync::Mutex<Box<dyn BaseSignatureCreator>>> 
    = Arc::new(std::sync::Mutex::new(Box::new(DummySignatureCreator::new(32, 32))));

    pub static ref DUMMY_MAXIMUM_SIGNATURE_CREATOR: 
    Arc<std::sync::Mutex<Box<dyn BaseSignatureCreator>>> 
    = Arc::new(std::sync::Mutex::new(Box::new(DummySignatureCreator::new(33, 32))));
}
