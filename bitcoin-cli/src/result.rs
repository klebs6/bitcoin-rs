crate::ix!();

/**
  | Parse UniValue result to update the
  | message to print to std::cout.
  |
  */
pub fn parse_result(
        result:    &UniValue,
        str_print: &mut String)  {
    
    if result.is_null() {
        return;
    }

    *str_print = match result.is_str() {
        true   => result.get_str().to_string(),
        false  => {

            let pretty_indent = Some(2);

            result.write(pretty_indent,None)
        }
    };
}

/**
  | Parse UniValue error to update the message
  | to print to std::cerr and the code to
  | return.
  |
  */
pub fn parse_error(
        error:     &UniValue,
        str_print: &mut String,
        n_ret:     &mut i32)  {

    if error.is_object() {

        let err_code: &UniValue = find_value(error,"code");
        let err_msg:  &UniValue = find_value(error,"message");

        if !err_code.is_null() {
            *str_print = format!{
                "error code: {}\n", err_code.get_val_str()
            };
        }

        if err_msg.is_str() {

            *str_print += format!(
                "error message:\n{}", 
                err_msg.get_str()
            ).as_str();
        }

        if err_code.is_num() 
        && err_code.get_int() == RPCErrorCode::RPC_WALLET_NOT_SPECIFIED.bits()
        {
            *str_print += format!{
                "\nTry adding \"-rpcwallet=<filename>\" option to bitcoin-cli command line."
            }.as_str();
        }

    } else {
        *str_print = format!{"error: {}", error.write(None,None)};
    }

    *n_ret = unsafe {
        abs(error["code"].get_int()) 
    };
}
