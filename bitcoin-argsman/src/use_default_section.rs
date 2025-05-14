// ---------------- [ File: bitcoin-argsman/src/use_default_section.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Returns true if settings values from
      | the default section should be used,
      | depending on the current network and
      | whether the setting is network-specific.
      |
      */
    pub fn use_default_section(&self, arg: &str) -> bool {
        
        self.network == Some(base_chain_params::MAIN.to_string()) 
        || !self.network_only_args.contains(arg)
    }
}
