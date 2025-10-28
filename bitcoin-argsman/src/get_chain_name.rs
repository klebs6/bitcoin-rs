// ---------------- [ File: bitcoin-argsman/src/get_chain_name.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Returns the appropriate chain name
      | from the program arguments.
      | 
      | 
      | -----------
      | @return
      | 
      | CBaseChainParams::MAIN by default;
      | raises runtime error if an invalid combination
      | is given.
      |
      */
    pub fn get_chain_name(&mut self) -> Result<String,StdException> {
        
        let ignore_default_section_config = false;
        let get_chain_name                = true;
        let section = "";

        let mut get_net = |arg: &str| {

            let value: SettingsValue = get_setting(
                &self.settings,
                section,
                &setting_name(arg),
                ignore_default_section_config,
                get_chain_name
            );

            match value.0.is_null() {
                true   => false,
                false  => match value.0.is_bool() {
                    true   => value.0.get_bool(),
                    false  => interpret_bool(value.0.get_str())
                }
            }
        };

        let reg_test:         bool = get_net("-regtest");
        let sig_net:          bool = get_net("-signet");
        let test_net:         bool = get_net("-testnet");
        let is_chain_arg_set: bool = self.is_arg_set("-chain");

        if is_chain_arg_set as i32 + reg_test as i32 + sig_net as i32 + test_net as i32 > 1 {
            return Err(runtime_error("Invalid combination of -regtest, -signet, -testnet and -chain. Can use at most one."));
        }

        if reg_test {
            return Ok(base_chain_params::REGTEST.to_string());
        }

        if sig_net {
            return Ok(base_chain_params::SIGNET.to_string());
        }

        if test_net {
            return Ok(base_chain_params::TESTNET.to_string());
        }

        Ok(self.get_arg("-chain",base_chain_params::MAIN))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn inner_for_chain() -> ArgsManagerInner {
        let mut inner = ArgsManagerInner::default();
        inner.available_args.insert(OptionsCategory::CHAINPARAMS, HashMap::new());
        // Register the chain options so get_setting will find flags if needed
        inner.add_arg(&crate::args::base_options::ARG_TESTNET);
        inner.add_arg(&crate::args::base_options::ARG_SIGNET);
        inner.add_arg(&crate::args::base_options::ARG_REGTEST);
        inner.add_arg(&crate::args::base_options::ARG_SET_CHAIN);
        inner
    }

    #[test]
    fn chain_name_selection_prefers_flags_in_order() {
        let mut inner = inner_for_chain();
        // Simulate -testnet
        inner.force_set_arg("-testnet", "1");
        assert_eq!(inner.get_chain_name().unwrap(), base_chain_params::TESTNET);

        let mut inner = inner_for_chain();
        inner.force_set_arg("-signet", "1");
        assert_eq!(inner.get_chain_name().unwrap(), base_chain_params::SIGNET);

        let mut inner = inner_for_chain();
        inner.force_set_arg("-regtest", "1");
        assert_eq!(inner.get_chain_name().unwrap(), base_chain_params::REGTEST);

        let mut inner = inner_for_chain();
        // Default is main if nothing set
        assert_eq!(inner.get_chain_name().unwrap(), base_chain_params::MAIN);

        let mut inner = inner_for_chain();
        // -chain overrides default
        inner.force_set_arg("-chain", base_chain_params::SIGNET);
        assert_eq!(inner.get_chain_name().unwrap(), base_chain_params::SIGNET);
    }

    #[test]
    fn invalid_combo_is_error() {
        let mut inner = inner_for_chain();
        inner.force_set_arg("-testnet", "1");
        inner.force_set_arg("-regtest", "1");
        assert!(inner.get_chain_name().is_err());
    }
}
