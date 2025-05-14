// ---------------- [ File: bitcoin-cli/src/cli_rpc.rs ]
crate::ix!();

#[inline] pub fn is_switch_char(c: char) -> bool {
    
    todo!();
        /*
    #ifdef WIN32
        return c == '-' || c == '/';
    #else
        return c == '-';
    #endif
        */
}

pub fn command_linerpc(argv: &Vec<String>) -> Result<i32,StdException> {

    let mut argv = argv.clone();

    let argv0 = argv[0].clone();

    argv.remove(0);
    
    let mut str_print = String::default();

    let mut n_ret: i32 = 0;

    let mut try_block = || -> TryBlockResult::<_,StdException> {

        //  Skip switches
        while is_switch_char(argv[0].chars().nth(0).unwrap()) {
            argv.remove(0);
        }

        let mut args = argv.clone();

        let mut rpc_pass = String::default();

        unsafe {

            if G_ARGS
                .lock()
                //.unwrap()
                .get_bool_arg("-stdinrpcpass", false) {

                no_stdin_echo!();

                if !stdin_ready() {
                    libc::fputs("RPC password> ".as_ptr() as *const i8, stderr());
                    libc::fflush(stderr());
                }

                if let Some(ref line) = std::io::stdin().lines().next() {

                    rpc_pass = line.as_ref().unwrap().to_string();

                } else {

                        return TryBlockResult::Err(runtime_error("-stdinrpcpass specified but failed to read from standard input"));
                    }

                if stdin_terminal() {
                    libc::fputc('\n' as i32, stdout());
                }

                G_ARGS
                    .lock()
                    //.unwrap()
                    .force_set_arg("-rpcpassword", &rpc_pass);
            }

            if G_ARGS
                .lock()
                //.unwrap()
                .get_bool_arg("-stdinwalletpassphrase", false) {

                no_stdin_echo!();

                let mut wallet_pass = String::default();

                if args.len() < 1 || &args[0][0..16] != "walletpassphrase" {
                    return TryBlockResult::Err(runtime_error("-stdinwalletpassphrase is only applicable for walletpassphrase(change)"));
                }

                if !stdin_ready() {
                    libc::fputs("Wallet passphrase> ".as_ptr() as *const i8, stderr());
                    libc::fflush(stderr());
                }

                if let Some(ref line) = std::io::stdin().lines().next() {
                    wallet_pass = line.as_ref().unwrap().to_string();

                } else {
                        return TryBlockResult::Err(runtime_error("-stdinwalletpassphrase specified but failed to read from standard input"));
                    }

                if stdin_terminal() {
                    libc::fputc('\n' as i32, stdout());
                }

                args.insert(1, wallet_pass.clone());
            }

            if G_ARGS
                .lock()
                //.unwrap()
                .get_bool_arg("-stdin", false) {

                //  Read one arg per line from stdin and append
                while let Some(ref line) = std::io::stdin().lines().next() {
                    args.push(line.as_ref().unwrap().to_string());
                }

                if stdin_terminal() {
                    libc::fputc('\n' as i32, stdout());
                }
            }
        }

        let mut rh: Option<Box<dyn BaseRequestHandler>> = None;

        let mut method = String::default();

        if G_ARGS
            .lock()
            //.unwrap()
            .is_arg_set("-getinfo") {

            rh = Some(Box::new(GetinfoRequestHandler::default()));

        } else {

            if G_ARGS
                .lock()
                //.unwrap()
                .get_bool_arg("-netinfo", false) {

                if !args.is_empty() && args[0] == "help" {
                    println!("{}\n", NetinfoRequestHandler::default().help_doc);
                    return TryBlockResult::Return(0);
                }

                rh = Some(Box::new(NetinfoRequestHandler::default()));

            } else {

                if G_ARGS
                    .lock()
                    //.unwrap()
                    .get_bool_arg("-generate", false) {

                    let getnewaddress: UniValue = get_new_address().unwrap();

                    let error: &UniValue = find_value(&getnewaddress,"error");

                    if error.is_null() {

                        let addr = find_value(&getnewaddress,"result").get_str();

                        set_generate_to_address_args(addr, &mut args);

                        rh = Some(Box::new(GenerateToAddressRequestHandler::default()));

                    } else {

                        parse_error(error, &mut str_print, &mut n_ret);
                    }

                } else {

                    if G_ARGS
                        .lock()
                        //.unwrap()
                        .get_bool_arg("-addrinfo", false) {

                        rh = Some(Box::new(AddrinfoRequestHandler::default()));

                    } else {

                        rh = Some(Box::new(DefaultRequestHandler::default()));

                        if args.len() < 1 {
                            return TryBlockResult::Err(runtime_error("too few parameters (need at least command)"));
                        }

                        method = args[0].to_string();

                        // Remove trailing method
                        // name from arguments
                        // vector
                        args.remove(0);
                    }
                }
            }
        }

        if n_ret == 0 {

            // Perform RPC call
            let mut wallet_name: Option::<String> = None;

            if G_ARGS
                .lock()
                //.unwrap()
                .is_arg_set("-rpcwallet") 
            {
                wallet_name = Some(
                    G_ARGS
                        .lock()
                        //.unwrap()
                        .get_arg("-rpcwallet", "")
                );
            }

            let mut reply: UniValue = connect_and_callrpc(rh.as_mut().unwrap(),&method,&args,wallet_name.as_deref()).unwrap();

            // Parse reply
            let error:  UniValue = find_value(&reply,"error").clone();
            let result: &mut UniValue = find_value_mut(&mut reply,"result");;

            if error.is_null() {

                if G_ARGS
                    .lock()
                    //.unwrap()
                    .get_bool_arg("-getinfo", false) 
                {
                    if !G_ARGS
                        .lock()
                        //.unwrap()
                        .is_arg_set("-rpcwallet") 
                    {
                        // fetch multiwallet
                        // balances and append to
                        // result
                        get_wallet_balances(result);
                    }

                    parse_get_info_result(result);
                }

                parse_result(&result, &mut str_print);

            } else {

                parse_error(&error, &mut str_print, &mut n_ret);
            }
        }

        TryBlockResult::Success
    };

    match try_block() {
        TryBlockResult::Return(v)  => return Ok(v),
        TryBlockResult::Err(e)  => {
            match e {
                StdException::Default { what: e }  => {
                    str_print = format!{"error: {}", e};
                    n_ret = EXIT_FAILURE;
                }
                _  => {
                    print_exception_continue(None, "CommandLineRPC()");
                    return Err(e);
                }
            }
        },

        TryBlockResult::Break   => { }
        TryBlockResult::Success => { }
    }

    if str_print != "" {

        match n_ret == 0 {
            true  => println!("{}\n",str_print),
            false => eprintln!("{}\n",str_print),
        }
    }

    Ok(n_ret)
}
