// ---------------- [ File: bitcoin-signingprovider/src/multisig_descriptor.rs ]
crate::ix!();

/**
  | A parsed multi(...) or sortedmulti(...)
  | descriptor
  |
  */
pub struct MultisigDescriptor {
    base:      DescriptorImpl,
    threshold: i32,
    sorted:    bool,
}

impl MultisigDescriptor {
    
    pub fn to_string_extra(&self) -> String {
        
        todo!();
        /*
            return strprintf("%i", m_threshold);
        */
    }
    
    pub fn make_scripts(&self, 
        keys: &Vec<crate::PubKey>,
        _1:   &[Script],
        _2:   &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            if (m_sorted) {
                std::vector<CPubKey> sorted_keys(keys);
                std::sort(sorted_keys.begin(), sorted_keys.end());
                return Vector(GetScriptForMultisig(m_threshold, sorted_keys));
            }
            return Vector(GetScriptForMultisig(m_threshold, keys));
        */
    }
    
    pub fn new(
        threshold: i32,
        providers: Vec<Box<PubkeyProvider>>,
        sorted:    Option<bool>) -> Self {

        let sorted: bool = sorted.unwrap_or(false);

        todo!();

        /*
            : DescriptorImpl(std::move(providers), sorted ? "sortedmulti" : "multi"), m_threshold(threshold), m_sorted(sorted)
        */
    }
}

impl IsSingleType for MultisigDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
