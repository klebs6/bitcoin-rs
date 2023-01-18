crate::ix!();

/**
  | This function returns either one of EXIT_ codes
  | when it's expected to stop the process or
  | CONTINUE_EXECUTION when it's expected to
  | continue further.
  |
  */
pub fn app_init_rpc(argv: &Vec<String>) -> i32 {

    let argc = argv.len();

    G_ARGS
        .lock()
        //.unwrap()
        .setup_cli_args();

    let mut error = String::default();

    if !G_ARGS
        .lock()
        //.unwrap()
        .parse_parameters(argv, &mut error) 
    {
        eprintln!("Error parsing command line arguments: {}\n", error);
        return EXIT_FAILURE;
    }

    let help_requested 
    = G_ARGS
        .lock()
        //.unwrap()
        .help_requested();

    let is_version_arg_set 
    = G_ARGS
        .lock()
        //.unwrap()
        .is_arg_set("-version");

    if argc < 2 || help_requested || is_version_arg_set {

        let mut str_usage: String = format!{"{} RPC client version {}\n", PACKAGE_NAME, format_full_version()};

        if !is_version_arg_set {

            let help_message 
            = G_ARGS
                .lock()
                //.unwrap()
                .get_help_message();

            str_usage.push_str("\n");
            str_usage += format!{"Usage:  bitcoin-cli [options] <command> [params]  Send command to {}\n",PACKAGE_NAME}.as_str();
            str_usage += format!{"or:     bitcoin-cli [options] -named <command> [name=value]...  Send command to {} (with named arguments)\n",PACKAGE_NAME}.as_str();
            str_usage += format!{"or:     bitcoin-cli [options] help                List commands\n"}.as_str();
            str_usage += format!{"or:     bitcoin-cli [options] help <command>      Get help for a command\n"}.as_str();
            str_usage += format!{"\n{}", help_message}.as_str();
        }

        println!{"{}", str_usage};

        if argc < 2 {
            eprintln!{"Error: too few parameters\n"};
            return EXIT_FAILURE;
        }

        return EXIT_SUCCESS;
    }

    if !check_data_dir_option() {

        eprintln!{
            "Error: Specified data directory \"{}\" does not exist.\n", 
            G_ARGS
                .lock()
                //.unwrap()
                .get_arg("-datadir", "")
        };

        return EXIT_FAILURE;
    }

    if !G_ARGS
        .lock()
        //.unwrap()
        .read_config_files(&mut error, Some(true)) {

        eprintln!{
            "Error reading configuration file: {}\n", 
            error
        };

        return EXIT_FAILURE;
    }

    // Check for chain settings (BaseParams()
    // calls are only valid after this clause)
    let try_block = || -> TryBlockResult::<_,&'static str> {
        select_base_params(
            &G_ARGS
                .lock()
                //.unwrap()
                .get_chain_name()
                .unwrap()
        );
        TryBlockResult::Success
    };

    match try_block() {
        TryBlockResult::Return(v)  => return v,
        TryBlockResult::Err(e)  => {

            eprintln!{
                "Error: {}\n", 
                e
            };

            return EXIT_FAILURE;
        },

        TryBlockResult::Break   => { }
        TryBlockResult::Success => { }
    }

    CONTINUE_EXECUTION
}
