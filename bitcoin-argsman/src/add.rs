// ---------------- [ File: bitcoin-argsman/src/add.rs ]
crate::ix!();

impl ArgsManagerInner {

    /**
      | Add subcommand
      |
      */
    pub fn add_command(&mut self, 
        cmd:  &str,
        help: &str)  {

        assert!(cmd.find('=') == None);
        assert!(cmd.chars().nth(0) != Some('-'));

        //LOCK(cs_args);
        
        //  latch to false
        self.accept_any_command = false;

        let arg_map: &mut HashMap::<String,ArgsManagerArg> 
        = self.available_args.get_mut(&OptionsCategory::COMMANDS).unwrap();

        let ret = arg_map.insert(
            cmd.to_string(), 
            ArgsManagerArg::new(
                "",
                help,
                ArgsManagerFlags::COMMAND
            )
        );

        //  Fail on duplicate commands
        assert!(ret.is_none());
    }
    
    /**
      | Add argument
      |
      */
    pub fn add_arg(&mut self, x: &ArgDescriptor)  {

        let ArgDescriptor { name, help, flags, category } = x;

        //  use AddCommand
        assert!((*flags & ArgsManagerFlags::COMMAND).bits() == 0);

        //  Split arg name from its help param
        let mut eq_index: usize = match name.find("=") {
            Some(item) => item,
            None       => name.len(),
        };

        let arg_name: String 
        = name[0..eq_index]
            .to_string();

        //LOCK(cs_args);

        let arg_map: &mut HashMap::<String,ArgsManagerArg> 
        = self.available_args.get_mut(category).unwrap();

        let ret = arg_map.insert(
            arg_name.clone(), 
            ArgsManagerArg::new(
                // Help parameter (e.g. "=<chain>") should be the suffix starting at '='
                // or empty if there is no '='.
                &(if eq_index < name.len() { name[eq_index..].to_string() } else { "".to_string() }),
                help,
                *flags
            )
        );

        //  Make sure an insertion actually happened
        assert!(ret.is_none());

        if (*flags & ArgsManagerFlags::NETWORK_ONLY).bits() != 0 {
            self.network_only_args.insert(arg_name);
        }
    }
    
    /**
      | Add many hidden arguments
      |
      */
    pub fn add_hidden_args(&mut self, names: &Vec<&'static str>)  {
        
        for name in names.iter() {
            let desc = ArgDescriptor {
                name:     name,
                help:     "".to_string(),
                flags:    ArgsManagerFlags::ALLOW_ANY, 
                category: OptionsCategory::HIDDEN
            };

            self.add_arg(&desc);
        }
    }

    pub fn setup_cli_args(&mut self) {

        self.setup_help_options();


        self.add_arg(&ARG_VERSION);
        self.add_arg(&ARG_SET_CONF_FILE);
        self.add_arg(&ARG_SET_DATADIR);
        self.add_arg(&ARG_GENERATE);
        self.add_arg(&ARG_ADDRINFO);
        self.add_arg(&ARG_GETINFO);
        self.add_arg(&ARG_NETINFO); 

        self.setup_chain_params_base_options();

        self.add_arg(&ARG_SET_COLOR);
        self.add_arg(&ARG_NAMED);
        self.add_arg(&ARG_SET_RPC_CLIENTTIMEOUT);
        self.add_arg(&ARG_SET_RPC_COOKIEFILE);
        self.add_arg(&ARG_SET_RPC_CONNECT);
        self.add_arg(&ARG_SET_RPC_PASSWORD);
        self.add_arg(&ARG_SET_RPC_PORT);
        self.add_arg(&ARG_SET_RPC_USER);
        self.add_arg(&ARG_SET_RPC_WAIT);
        self.add_arg(&ARG_SET_RPC_WAITTIMEOUT);
        self.add_arg(&ARG_SET_RPC_WALLET);
        self.add_arg(&ARG_STDIN);
        self.add_arg(&ARG_STDINRPCPASS);
        self.add_arg(&ARG_STDINWALLETPASSPHRASE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    static M: OnceLock<Mutex<()>> = OnceLock::new();
    fn lock() -> std::sync::MutexGuard<'static,()> { M.get_or_init(|| Mutex::new(())).lock().unwrap() }

    fn empty_inner() -> ArgsManagerInner {
        let mut inner = ArgsManagerInner::default();
        // Pre-populate categories we’ll use
        for cat in [
            OptionsCategory::OPTIONS,
            OptionsCategory::COMMANDS,
            OptionsCategory::REGISTER_COMMANDS,
            OptionsCategory::CHAINPARAMS,
            OptionsCategory::HIDDEN,
        ] {
            inner.available_args.insert(cat, HashMap::<String,ArgsManagerArg>::new());
        }
        inner
    }

    #[test]
    fn add_command_works_and_rejects_dupes() {
        let _g = lock();
        let mut inner = empty_inner();
        // Accept-any should be turned off after first add_command
        assert!(inner.accept_any_command, "default should be true");
        inner.add_command("delin", "Delete input");
        assert!(!inner.accept_any_command);

        // Duplicate should assert
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            inner.add_command("delin", "Duplicate");
        }));
        assert!(result.is_err());
    }

    #[test]
    fn add_arg_splits_help_param_and_marks_network_only() {
        let _g = lock();
        let mut inner = empty_inner();

        let d = ArgDescriptor {
            name:     "-foo=<bar>",
            help:     "help text".into(),
            flags:    ArgsManagerFlags::ALLOW_ANY | ArgsManagerFlags::NETWORK_ONLY,
            category: OptionsCategory::OPTIONS,
        };
        inner.add_arg(&d);

        let m = inner.available_args.get(&OptionsCategory::OPTIONS).unwrap();
        let a = m.get("-foo").expect("arg inserted");
        assert_eq!(a.help_param, "=<bar>");
        assert_eq!(a.help_text, "help text");
        assert!(inner.network_only_args.contains("-foo"));
    }

    #[test]
    fn add_hidden_args_inserts_as_hidden() {
        let _g = lock();
        let mut inner = empty_inner();
        inner.add_hidden_args(&vec!["-h", "-help"]);
        let m = inner.available_args.get(&OptionsCategory::HIDDEN).unwrap();
        assert!(m.contains_key("-h"));
        assert!(m.contains_key("-help"));
    }

    #[test]
    fn setup_cli_args_registers_core_options() {
        let _g = lock();
        let mut inner = empty_inner();
        inner.setup_cli_args();
        let opts = inner.available_args.get(&OptionsCategory::OPTIONS).unwrap();
        for k in ["-version", "-conf", "-datadir", "-generate", "-getinfo", "-netinfo", "-stdin"] {
            assert!(opts.contains_key(k), "expected option {}", k);
        }
        let chain = inner.available_args.get(&OptionsCategory::CHAINPARAMS).unwrap();
        for k in ["-chain", "-regtest", "-testnet", "-signet"] {
            // "-chain" stored internally as "-chain" even if descriptor contains "=<chain>"
            let key = if k == "-chain" { "-chain" } else { k };
            assert!(chain.contains_key(key), "expected chain option {}", k);
        }
    }
}
