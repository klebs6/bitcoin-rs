crate::ix!();

/**
  | Non-RFC4627 JSON parser, accepts internal
  | values (such as numbers, true, false,
  | null) as well as objects and arrays.
  |
  */
pub fn parse_non_rfcjson_value(str_val: &str) -> Result<UniValue,StdException> {
    
    let mut j_val = UniValue::default();

    let msg = format!("[{}]", str_val);

    if !j_val.read(msg.as_ptr() as *const u8, msg.len()) 
    || !j_val.is_array() 
    || j_val.size() != 1 {

        let msg = format!("Error parsing JSON: {}", str_val);

        return Err(runtime_error(&msg));
    }

    Ok(j_val[0].clone())
}

/**
  | Convert positional arguments to command-specific
  | RPC representation
  |
  */
pub fn rpc_convert_values(
        str_method: &str,
        str_params: &Vec<String>) -> UniValue {
    
    let mut params: UniValue = UniValue::from(uni_value::VType::VARR);

    for idx in 0..str_params.len() {

        let str_val: &str = &str_params[idx];

        if !RPC_CVT_TABLE
            .lock()
            //.unwrap()
            .convert_with_method_and_idx(
                str_method, 
                idx.try_into().unwrap()
            ) 
        {
            // insert string value directly
            params.push_back(str_val);

        } else {

            // parse string as JSON, insert
            // bool/number/object/etc. value
            params.push_back(&parse_non_rfcjson_value(str_val));
        }
    }

    params
}

/**
  | Convert named arguments to command-specific
  | RPC representation
  |
  */
pub fn rpc_convert_named_values(
        str_method: &str,
        str_params: &Vec<String>) -> Result<UniValue,StdException> {
    
    let mut params: UniValue 
    = UniValue::from(uni_value::VType::VOBJ);

    for s in str_params.iter() {

        if let Some(pos) = s.find('=') {

            let name:  String = s[0..pos].to_string();
            let value: String = s[pos + 1..].to_string();

            if !RPC_CVT_TABLE
                .lock()
                //.unwrap()
                .convert_with_method_and_name(str_method, &name) 
            {

                // insert string value directly
                params.pushkv(name, value);

            } else {

                // parse string as JSON, insert
                // bool/number/object/etc. value
                params.pushkv(name, parse_non_rfcjson_value(&value));
            }

        } else {

            let msg = format!{
                "No '=' in named argument '{}', this needs to be present for every argument (even if it is empty)",
                s
            };

            return Err(runtime_error(&msg));
        }
    }

    Ok(params)
}

/**
  | Class that handles the conversion from
  | a command-line to a JSON-RPC request,
  | as well as converting back to a JSON object
  | that can be shown as result.
  |
  */
pub trait BaseRequestHandler {

    fn prepare_request(&mut self, 
            method: &str,
            args:   &Vec<String>) -> Result<UniValue,StdException>;

    fn process_reply(&mut self, batch_in: &UniValue) -> Result<UniValue,StdException>;
}

/**
  | Process addrinfo requests
  |
  */
#[derive(Default)]
pub struct AddrinfoRequestHandler {

}

impl BaseRequestHandler for AddrinfoRequestHandler {

    fn prepare_request(&mut self, 
        method: &str,
        args:   &Vec<String>) -> Result<UniValue,StdException> {

        if !args.is_empty() {
            return Err(runtime_error("-addrinfo takes no arguments"));
        }

        let params: UniValue = {

            let params = vec!["0".to_string()];

            rpc_convert_values("getnodeaddresses",&params)
        };

        let result = jsonrpc_request_obj(
            "getnodeaddresses",
            &params,
            &UniValue::from(1_i32)
        );

        Ok(result)
    }
    
    fn process_reply(&mut self, reply: &UniValue) -> Result<UniValue,StdException> {

        if !reply["error"].is_null() {
            return Ok(reply.clone());
        }

        let nodes: &Vec::<UniValue> 
        = reply["result"]
            .get_values()
            .unwrap();

        if !nodes.is_empty() && nodes[0]["network"].is_null() {
            return Err(runtime_error("-addrinfo requires bitcoind server to be running v22.0 and up"));
        }

        //TODO: need to fix the parser on this
        //one, though I have worked around the
        //issue below
        todo!();
        /*
            array<uint64_t,m_networks_size> counts{{}};
        */

        // Count the number of peers known to our node, by network.
        let mut counts: Vec<u64> = Vec::with_capacity(NETINFO_REQUEST_HANDLER_NETWORKS.len());

        for node in nodes.iter() {

            let network_name: String = String::from(node["network"].get_str());
            let network_id: i8 = self.network_string_to_id(&network_name);

            if network_id == UNKNOWN_NETWORK {
                continue;
            }

            let idx: usize = network_id.try_into().unwrap();

            counts[idx] += 1;
        }

        // Prepare result to return to user.
        let result:    UniValue = UniValue::from(uni_value::VType::VOBJ);;
        let addresses: UniValue = UniValue::from(uni_value::VType::VOBJ);;

        // Total address count
        let total: u64 = 0;

        for i in 0..NETINFO_REQUEST_HANDLER_NETWORKS.len() {

            addresses.pushkv(
                NETINFO_REQUEST_HANDLER_NETWORKS[i], 
                counts[i]
            );

            total += counts[i];
        }

        addresses.pushkv("total", total);

        result.pushkv("addresses_known", ADDRESSES);

        Ok(
            jsonrpc_reply_obj(
                &result,
                &NULL_UNI_VALUE,
                &UniValue::from(1_i32)
            )
        )
    }
}
