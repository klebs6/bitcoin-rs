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
                &name[eq_index..name.len() - eq_index].to_string(),
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
