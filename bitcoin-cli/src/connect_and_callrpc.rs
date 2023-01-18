crate::ix!();

/**
  | ConnectAndCallRPC wraps CallRPC with
  | -rpcwait and an exception handler.
  | 
  | -----------
  | @param[in] rh
  | 
  | Pointer to RequestHandler.
  | ----------
  | @param[in] strMethod
  | 
  | Reference to const string method to
  | forward to CallRPC.
  | ----------
  | @param[in] rpcwallet
  | 
  | Reference to const optional string
  | wallet name to forward to CallRPC.
  | 
  | -----------
  | @return
  | 
  | the RPC response as a UniValue object.
  | @throws a CConnectionFailed std::runtime_error
  | if connection failed or RPC server still
  | in warmup.
  |
  */
pub fn connect_and_callrpc(
        rh:         &mut Box<dyn BaseRequestHandler>,
        str_method: &str,
        args:       &Vec<String>,
        rpcwallet:  Option<&str>) -> Result<UniValue,StdException> {

    let mut response: UniValue = UniValue::from(uni_value::VType::VOBJ);

    // Execute and handle connection failures with
    // -rpcwait.
    let wait:   bool = 
    G_ARGS
        .lock()
        //.unwrap()
        .get_bool_arg("-rpcwait", false);;

    let timeout: i64 = 
    G_ARGS
        .lock()
        //.unwrap()
        .get_int_arg("-rpcwaittimeout", DEFAULT_WAIT_CLIENT_TIMEOUT.into())
        .try_into()
        .unwrap();

    let deadline = get_time() + Duration::seconds(timeout);
    
    loop {

        let mut try_block = || -> TryBlockResult::<_,StdException> {

            response = callrpc(rh,str_method,args,rpcwallet).unwrap();

            if wait {
                let error: &UniValue = find_value(&response,"error");
                if !error.is_null() && error["code"].get_int() == RPCErrorCode::RPC_IN_WARMUP.bits() {
                    return TryBlockResult::Err(connection_failed("server in warmup"));
                }
            }

            // Connection succeeded, no need to
            // retry.
            TryBlockResult::Break
        };

        match try_block() {
            TryBlockResult::Return(v)  => return v,
            TryBlockResult::Err(e)  => {

                let now = get_time();

                if wait && (timeout <= 0 || now < deadline) {

                    uninterruptible_sleep(Duration::seconds(1));

                } else {

                    let msg = format!("timeout on transient error: {:?}",e);

                    return Err(connection_failed(&msg));
                }
            },
            TryBlockResult::Break    => { break; }
            TryBlockResult::Success  => { }
        }

        if !wait {
            break;
        }
    }

    Ok(response)
}
