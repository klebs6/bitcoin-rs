// ---------------- [ File: bitcoin-cli/src/addr.rs ]
crate::ix!();

/**
  | Call RPC getnewaddress.
  | 
  | 
  | -----------
  | @return
  | 
  | getnewaddress response as a UniValue
  | object.
  |
  */
pub fn get_new_address() -> Result<UniValue,StdException> {

    let mut wallet_name: Option::<String> = None;

    if G_ARGS
        .lock()
        //.unwrap()
        .is_arg_set("-rpcwallet") {

        wallet_name = Some(
            G_ARGS
                .lock()
                //.unwrap()
                .get_arg("-rpcwallet", "")
        );

    }

    let mut rh: Box<dyn BaseRequestHandler> = Box::new(DefaultRequestHandler::default());
    
    connect_and_callrpc(
        &mut rh,
        "getnewaddress",
        /* args=*/ &vec![],
        wallet_name.as_deref()
    )
}

/**
  | Check bounds and set up args for RPC generatetoaddress
  | params: nblocks, address, maxtries.
  | 
  | -----------
  | @param[in] address
  | 
  | Reference to const string address to
  | insert into the args.
  | ----------
  | @param args
  | 
  | Reference to vector of string args to
  | modify.
  |
  */
pub fn set_generate_to_address_args(
        address: &str,
        args:    &mut Vec<String>) -> Result<(), StdException>  {
    

    if args.len() > 2 {
        return Err(
            runtime_error("too many arguments (maximum 2 for nblocks and maxtries)")
        );
    }

    if args.len() == 0 {
        args.push(DEFAULT_NBLOCKS.to_string());

    } else {

        if args[0] == "0" {

            let msg = format!{
                "the first argument (number of blocks to generate, default: {}) must be an integer value greater than zero",
                DEFAULT_NBLOCKS
            };

            return Err(runtime_error(&msg));
        }
    }

    args.insert(1, address.to_string());

    Ok(())
}
