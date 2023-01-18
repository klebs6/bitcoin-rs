crate::ix!();

/**
  | A parsed addr(A) descriptor.
  |
  */
pub struct AddressDescriptor {
    base:        DescriptorImpl,
    destination: TxDestination,
}

impl From<TxDestination> for AddressDescriptor {

    fn from(destination: TxDestination) -> Self {
    
        todo!();
        /*
            : DescriptorImpl({}, "addr"), m_destination(std::move(destination))
        */
    }
}

impl AddressDescriptor {
    
    pub fn to_string_extra(&self) -> String {
        
        todo!();
        /*
            return EncodeDestination(m_destination);
        */
    }
    
    pub fn make_scripts(&self, 
        _0: &Vec<PubKey>,
        _1: &[Script],
        _2: &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            return Vector(GetScriptForDestination(m_destination));
        */
    }
}

impl GetOutputType for AddressDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            return OutputTypeFromDestination(m_destination);
        */
    }
}

impl IsSolvable for AddressDescriptor {
    fn is_solvable(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
impl IsSingleType for AddressDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
