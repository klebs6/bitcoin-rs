crate::ix!();

// Section names to be recognized in the config file.
lazy_static!{
    static ref AVAILABLE_SECTIONS: std::collections::HashSet<String> = hashset!{
        base_chain_params::REGTEST.to_string(),
        base_chain_params::SIGNET.to_string(),
        base_chain_params::TESTNET.to_string(),
        base_chain_params::MAIN.to_string()
    };
}

impl ArgsManagerInner {
    
    /**
      | Log warnings for unrecognized section
      | names in the config file.
      |
      */
    pub fn get_unrecognized_sections(&self) -> LinkedList<SectionInfo> {
        
        //LOCK(cs_args);

        let mut unrecognized: Vec::<SectionInfo> 
        = self.config_sections
            .iter()
            .cloned()
            .collect();

        unrecognized.retain(|appeared: &SectionInfo| {
            AVAILABLE_SECTIONS.get(appeared.name.as_str()) == None
        });

        unrecognized
            .iter()
            .cloned()
            .collect()
    }
}
