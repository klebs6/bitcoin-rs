crate::ix!();

/**
  | Process RPC generatetoaddress request.
  |
  */
#[derive(Default)]
pub struct GenerateToAddressRequestHandler {
    address_str: String,
}

impl BaseRequestHandler for GenerateToAddressRequestHandler {
    
    fn prepare_request(&mut self, 
        method: &str,
        args:   &Vec<String>) -> Result<UniValue,StdException> {
        
        self.address_str = args[1].to_string();

        let params: UniValue = UniValue::from(
            rpc_convert_values(
                "generatetoaddress",
                args
            )
        );

        Ok(
            jsonrpc_request_obj(
                "generatetoaddress",
                &params,
                &UniValue::from(1_i32)
            )
        )
    }
    
    fn process_reply(&mut self, reply: &UniValue) -> Result<UniValue,StdException> {
        
        let mut result: UniValue = UniValue::from(uni_value::VType::VOBJ);

        result.pushkv("address", &self.address_str);
        result.pushkv("blocks",  &reply.get_obj()["result"]);

        Ok(jsonrpc_reply_obj(
            &result,
            &NULL_UNI_VALUE,
            &UniValue::from(1_i32)
        ))
    }
}
