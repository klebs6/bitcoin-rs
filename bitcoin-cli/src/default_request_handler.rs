// ---------------- [ File: bitcoin-cli/src/default_request_handler.rs ]
crate::ix!();

/**
  | Process default single requests
  |
  */
#[derive(Default)]
pub struct DefaultRequestHandler {

}

impl BaseRequestHandler for DefaultRequestHandler {

    fn prepare_request(&mut self, 
        method: &str,
        args:   &Vec<String>) -> Result<UniValue,StdException> {
        
        let mut params = UniValue::default();

        if G_ARGS
            .lock()
            //.unwrap()
            .get_bool_arg("-named", DEFAULT_NAMED) 
        {
            params = rpc_convert_named_values(method,args).unwrap();

        } else {

            params = rpc_convert_values(method,args);
        }

        Ok(jsonrpc_request_obj(method,&params, &UniValue::from(1_i32)))
    }
    
    fn process_reply(&mut self, reply: &UniValue) -> Result<UniValue,StdException> {
        
        Ok(reply.get_obj().clone())
    }
}
