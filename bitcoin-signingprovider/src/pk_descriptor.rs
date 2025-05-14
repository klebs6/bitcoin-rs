// ---------------- [ File: bitcoin-signingprovider/src/pk_descriptor.rs ]
crate::ix!();

/**
  | A parsed pk(P) descriptor.
  |
  */
pub struct PKDescriptor {
    base:  DescriptorImpl,
    xonly: bool,
}

impl PKDescriptor {

    pub fn make_scripts(&self, 
        keys: &Vec<crate::PubKey>,
        _1:   &[Script],
        _2:   &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            if (m_xonly) {
                CScript script = CScript() << ToByteVector(crate::XOnlyPubKey(keys[0])) << OP_CHECKSIG;
                return Vector(std::move(script));
            } else {
                return Vector(GetScriptForRawPubKey(keys[0]));
            }
        */
    }
    
    pub fn new(
        prov:  Box<PubkeyProvider>,
        xonly: Option<bool>) -> Self {
        let xonly: bool = xonly.unwrap_or(false);
        todo!();
        /*
            : DescriptorImpl(Vector(std::move(prov)), "pk"), m_xonly(xonly)
        */
    }
    
}

impl IsSingleType for PKDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
