// ---------------- [ File: bitcoin-signingprovider/src/wsh_descriptor.rs ]
crate::ix!();

/**
  | A parsed wsh(...) descriptor.
  |
  */
pub struct WSHDescriptor {
    base: DescriptorImpl,
}

impl From<Box<DescriptorImpl>> for WSHDescriptor {

    fn from(desc: Box<DescriptorImpl>) -> Self {
    
        todo!();
        /*


            : DescriptorImpl({}, std::move(desc), "wsh")
        */
    }
}

impl WSHDescriptor {
    
    pub fn make_scripts(&self, 
        _0:      &Vec<crate::PubKey>,
        scripts: &[Script],
        out:     &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            auto ret = Vector(GetScriptForDestination(WitnessV0ScriptHash(scripts[0])));
            if (ret.size()) out.scripts.emplace(CScriptID(scripts[0]), scripts[0]);
            return ret;
        */
    }
}

impl GetOutputType for WSHDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            return OutputType::BECH32;
        */
    }
}

impl IsSingleType for WSHDescriptor {

    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
