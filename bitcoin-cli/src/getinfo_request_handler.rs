crate::ix!();

/**
  | Process getinfo requests
  |
  */
#[derive(Default)]
pub struct GetinfoRequestHandler {

}

pub const GETINFO_REQUEST_HANDLER_ID_NETWORKINFO:    usize = 0;
pub const GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO: usize = 1;
pub const GETINFO_REQUEST_HANDLER_ID_WALLETINFO:     usize = 2;
pub const GETINFO_REQUEST_HANDLER_ID_BALANCES:       usize = 3;

impl BaseRequestHandler for GetinfoRequestHandler {

    /**
      | Create a simulated `getinfo` request.
      |
      */
    fn prepare_request(&mut self, 
        method: &str,
        args:   &Vec<String>) -> Result<UniValue,StdException> {

        if !args.is_empty() {
            return Err(runtime_error("-getinfo takes no arguments"));
        }

        let mut result: UniValue = UniValue::from(uni_value::VType::VARR);

        result.push_back(&jsonrpc_request_obj("getnetworkinfo",    &NULL_UNI_VALUE,&UniValue::from(GETINFO_REQUEST_HANDLER_ID_NETWORKINFO)));
        result.push_back(&jsonrpc_request_obj("getblockchaininfo", &NULL_UNI_VALUE,&UniValue::from(GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO)));
        result.push_back(&jsonrpc_request_obj("getwalletinfo",     &NULL_UNI_VALUE,&UniValue::from(GETINFO_REQUEST_HANDLER_ID_WALLETINFO)));
        result.push_back(&jsonrpc_request_obj("getbalances",       &NULL_UNI_VALUE,&UniValue::from(GETINFO_REQUEST_HANDLER_ID_BALANCES)));

        Ok(result)
    }

    /**
      | Collect values from the batch and form
      | a simulated `getinfo` reply.
      |
      */
    fn process_reply(&mut self, batch_in: &UniValue) -> Result<UniValue,StdException> {
        
        let mut result: UniValue = UniValue::from(uni_value::VType::VOBJ);

        let batch: Vec::<UniValue> = jsonrpc_process_batch_reply(batch_in);

        // Errors in getnetworkinfo() and
        // getblockchaininfo() are fatal, pass
        // them on;
        //
        // getwalletinfo() and getbalances() are
        // allowed to fail if there is no wallet.
        if !batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["error"].is_null() {
            return Ok(batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO].clone());
        }

        if !batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO]["error"].is_null() {
            return Ok(batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO].clone());
        }

        result.pushkv("version",              &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["version"]);
        result.pushkv("blocks",               &batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO]["result"]["blocks"]);
        result.pushkv("headers",              &batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO]["result"]["headers"]);
        result.pushkv("verificationprogress", &batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO]["result"]["verificationprogress"]);
        result.pushkv("timeoffset",           &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["timeoffset"]);

        let mut connections: UniValue = UniValue::from(uni_value::VType::VOBJ);

        connections.pushkv("in",     &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["connections_in"]);
        connections.pushkv("out",    &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["connections_out"]);
        connections.pushkv("total",  &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["connections"]);

        result.pushkv("connections", connections);
        result.pushkv("networks",    &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["networks"]);
        result.pushkv("difficulty",  &batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO]["result"]["difficulty"]);
        result.pushkv("chain",       UniValue::from(batch[GETINFO_REQUEST_HANDLER_ID_BLOCKCHAININFO]["result"]["chain"].clone()));

        if !batch[GETINFO_REQUEST_HANDLER_ID_WALLETINFO]["result"].is_null() {

            result.pushkv("has_wallet",  true);
            result.pushkv("keypoolsize", &batch[GETINFO_REQUEST_HANDLER_ID_WALLETINFO]["result"]["keypoolsize"]);
            result.pushkv("walletname",  &batch[GETINFO_REQUEST_HANDLER_ID_WALLETINFO]["result"]["walletname"]);

            if !batch[GETINFO_REQUEST_HANDLER_ID_WALLETINFO]["result"]["unlocked_until"].is_null() {
                result.pushkv("unlocked_until", &batch[GETINFO_REQUEST_HANDLER_ID_WALLETINFO]["result"]["unlocked_until"]);
            }

            result.pushkv("paytxfee", &batch[GETINFO_REQUEST_HANDLER_ID_WALLETINFO]["result"]["paytxfee"]);
        }

        if !batch[GETINFO_REQUEST_HANDLER_ID_BALANCES]["result"].is_null() {
            result.pushkv("balance", &batch[GETINFO_REQUEST_HANDLER_ID_BALANCES]["result"]["mine"]["trusted"]);
        }

        result.pushkv("relayfee", &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["relayfee"]);
        result.pushkv("warnings", &batch[GETINFO_REQUEST_HANDLER_ID_NETWORKINFO]["result"]["warnings"]);

        Ok(jsonrpc_reply_obj(&result,&NULL_UNI_VALUE,&UniValue::from(1_i32)))
    }
}
