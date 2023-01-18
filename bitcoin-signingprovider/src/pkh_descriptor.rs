crate::ix!();

/**
  | A parsed pkh(P) descriptor.
  |
  */
pub struct PKHDescriptor {
    base: DescriptorImpl,
}

impl From<Box<PubkeyProvider>> for PKHDescriptor {

    fn from(prov: Box<PubkeyProvider>) -> Self {
    
        todo!();
        /*
            : DescriptorImpl(Vector(std::move(prov)), "pkh")
        */
    }
}
    
impl PKHDescriptor {

    pub fn make_scripts(&self, 
        keys: &Vec<crate::PubKey>,
        _1:   &[Script],
        out:  &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            CKeyID id = keys[0].GetID();
            out.pubkeys.emplace(id, keys[0]);
            return Vector(GetScriptForDestination(PKHash(id)));
        */
    }
}

impl GetOutputType for PKHDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            return OutputType::LEGACY;
        */
    }
}

impl IsSingleType for PKHDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

