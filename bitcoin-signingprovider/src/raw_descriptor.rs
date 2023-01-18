crate::ix!();

/**
  | A parsed raw(H) descriptor.
  |
  */
pub struct RawDescriptor {
    base:   DescriptorImpl,
    script: Script,
}

impl From<Script> for RawDescriptor {

    fn from(script: Script) -> Self {
    
        todo!();
        /*


            : DescriptorImpl({}, "raw"), m_script(std::move(script))
        */
    }
}
    

impl RawDescriptor {

    pub fn to_string_extra(&self) -> String {
        
        todo!();
        /*
            return HexStr(m_script);
        */
    }
    
    pub fn make_scripts(&self, 
        _0: &Vec<crate::PubKey>,
        _1: &[Script],
        _2: &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            return Vector(m_script);
        */
    }
}

impl GetOutputType for RawDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            TxDestination dest;
            ExtractDestination(m_script, dest);
            return OutputTypeFromDestination(dest);
        */
    }
}

impl IsSingleType for RawDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}

impl IsSolvable for RawDescriptor {
    fn is_solvable(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

