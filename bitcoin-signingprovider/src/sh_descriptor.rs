// ---------------- [ File: bitcoin-signingprovider/src/sh_descriptor.rs ]
crate::ix!();

/**
  | A parsed sh(...) descriptor.
  |
  */
pub struct SHDescriptor {
    base: DescriptorImpl,
}

impl From<Box<DescriptorImpl>> for SHDescriptor {

    fn from(desc: Box<DescriptorImpl>) -> Self {
    
        todo!();
        /*


            : DescriptorImpl({}, std::move(desc), "sh")
        */
    }
}

impl SHDescriptor {
    
    pub fn make_scripts(&self, 
        _0:      &Vec<crate::PubKey>,
        scripts: &[Script],
        out:     &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            auto ret = Vector(GetScriptForDestination(ScriptHash(scripts[0])));
            if (ret.size()) out.scripts.emplace(CScriptID(scripts[0]), scripts[0]);
            return ret;
        */
    }
}

impl GetOutputType for SHDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            assert(m_subdescriptor_args.size() == 1);
            if (m_subdescriptor_args[0]->GetOutputType() == OutputType::BECH32) return OutputType::P2SH_SEGWIT;
            return OutputType::LEGACY;
        */
    }
}

impl IsSingleType for SHDescriptor {

    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
