// ---------------- [ File: bitcoin-signingprovider/src/wpkh_descriptor.rs ]
crate::ix!();

/**
  | A parsed wpkh(P) descriptor.
  |
  */
pub struct WPKHDescriptor {
    base: DescriptorImpl,
}

impl From<Box<PubkeyProvider>> for WPKHDescriptor {

    fn from(prov: Box<PubkeyProvider>) -> Self {
    
        todo!();
        /*


            : DescriptorImpl(Vector(std::move(prov)), "wpkh")
        */
    }
}
    
impl WPKHDescriptor {
    
    pub fn make_scripts(&self, 
        keys: &Vec<crate::PubKey>,
        _1:   &[Script],
        out:  &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            CKeyID id = keys[0].GetID();
            out.pubkeys.emplace(id, keys[0]);
            return Vector(GetScriptForDestination(WitnessV0KeyHash(id)));
        */
    }
}

impl GetOutputType for WPKHDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            return OutputType::BECH32;
        */
    }
}

impl IsSingleType for WPKHDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

