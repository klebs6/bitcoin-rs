// ---------------- [ File: bitcoin-argsman/src/get_unsuitable_section_only_args.rs ]
crate::ix!();

impl ArgsManager {

    /**
      | Log warnings for options in m_section_only_args
      | when they are specified in the default
      | section but not overridden on the command
      | line or in a network-specific section
      | in the config file.
      |
      */
    pub fn get_unsuitable_section_only_args(&self) -> HashSet<String> {
        
        let mut unsuitables = HashSet::<String>::new();

        let guard = self.cs_args.lock();

        /*
          | if there's no section selected, don't
          | worry
          |
          */
        if guard.network.is_none() {
            return unsuitables;
        }

        /*
          | if it's okay to use the default section
          | for this network, don't worry
          |
          */
        if guard.network == Some(base_chain_params::MAIN.to_string()) {
            return unsuitables;
        }

        for arg in guard.network_only_args.iter() {

            if only_has_default_section_setting(
                &guard.settings, 
                &guard.network.as_ref().unwrap().as_str(), 
                &setting_name(arg)) 
            {
                unsuitables.insert(arg.to_string());
            }
        }

        unsuitables
    }
}
