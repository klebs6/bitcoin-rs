// ---------------- [ File: bitcoin-crypter/src/masterkey.rs ]
crate::ix!();

/**
  | Master key for wallet encryption
  |
  */
pub struct MasterKey {

    vch_crypted_key:                 Vec<u8>,
    vch_salt:                        Vec<u8>,

    /**
      | 0 = EVP_sha512() 
      | 1 = scrypt()
      |
      */
    n_derivation_method:             u32,

    n_derive_iterations:             u32,

    /**
      | Use this for more parameters to key derivation,
      | such as the various parameters to scrypt
      |
      */
    vch_other_derivation_parameters: Vec<u8>,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CMasterKey, obj)
        {
            READWRITE(obj.vchCryptedKey, obj.vchSalt, obj.nDerivationMethod, obj.nDeriveIterations, obj.vchOtherDerivationParameters);
        }
    */
}

impl Default for MasterKey {
    
    fn default() -> Self {
        todo!();
        /*

            // 25000 rounds is just under 0.1 seconds on a 1.86 GHz Pentium M
            // ie slightly lower than the lowest hardware we need bother supporting
            nDeriveIterations = 25000;
            nDerivationMethod = 0;
            vchOtherDerivationParameters = std::vector<unsigned char>(0);
        */
    }
}
