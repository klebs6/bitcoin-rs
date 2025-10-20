// ---------------- [ File: bitcoin-remote/src/json_request.rs ]
crate::ix!();

pub enum JSONRPCRequestMode { 
    EXECUTE, 
    GET_HELP, 
    GET_ARGS 
}

//-------------------------------------------[.cpp/bitcoin/src/rpc/request.h]

pub struct JSONRPCRequest {
    id:         UniValue,
    str_method: String,
    params:     UniValue,
    mode:       JSONRPCRequestMode, // default = EXECUTE
    uri:        String,
    auth_user:  String,
    peer_addr:  String,
    context:    Box<dyn Any>,
}

impl JSONRPCRequest {
    
    pub fn parse(&mut self, val_request: &UniValue)  {
        
        todo!();
        /*
            // Parse request
        if (!valRequest.isObject())
            throw JSONRPCError(RPC_INVALID_REQUEST, "Invalid Request object");
        const UniValue& request = valRequest.get_obj();

        // Parse id now so errors from here on will have the id
        id = find_value(request, "id");

        // Parse method
        UniValue valMethod = find_value(request, "method");
        if (valMethod.isNull())
            throw JSONRPCError(RPC_INVALID_REQUEST, "Missing method");
        if (!valMethod.isStr())
            throw JSONRPCError(RPC_INVALID_REQUEST, "Method must be a string");
        strMethod = valMethod.get_str();
        if (fLogIPs)
            LogPrint(LogFlags::RPC, "ThreadRPCServer method=%s user=%s peeraddr=%s\n", SanitizeString(strMethod),
                this->authUser, this->peerAddr);
        else
            LogPrint(LogFlags::RPC, "ThreadRPCServer method=%s user=%s\n", SanitizeString(strMethod), this->authUser);

        // Parse params
        UniValue valParams = find_value(request, "params");
        if (valParams.isArray() || valParams.isObject())
            params = valParams;
        else if (valParams.isNull())
            params = UniValue(UniValue::VARR);
        else
            throw JSONRPCError(RPC_INVALID_REQUEST, "Params must be an array or object");
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/rpc/request.cpp]

/**
  | JSON-RPC protocol.
  | 
  | Bitcoin speaks version 1.0 for maximum
  | compatibility, but uses JSON-RPC 1.1/2.0
  | standards for parts of the 1.0 standard
  | that were unspecified (HTTP errors
  | and contents of 'error').
  | 
  | 1.0 spec: http://json-rpc.org/wiki/specification
  | 
  | 1.2 spec: http://jsonrpc.org/historical/json-rpc-over-http.html
  |
  */
pub fn jsonrpc_request_obj(
        str_method: &str,
        params:     &UniValue,
        id:         &UniValue) -> UniValue {
    
    todo!();
        /*
            UniValue request(UniValue::VOBJ);
        request.pushKV("method", strMethod);
        request.pushKV("params", params);
        request.pushKV("id", id);
        return request;
        */
}

pub fn jsonrpc_reply_obj(
        result: &UniValue,
        error:  &UniValue,
        id:     &UniValue) -> UniValue {
    
    todo!();
        /*
            UniValue reply(UniValue::VOBJ);
        if (!error.isNull())
            reply.pushKV("result", NullUniValue);
        else
            reply.pushKV("result", result);
        reply.pushKV("error", error);
        reply.pushKV("id", id);
        return reply;
        */
}

pub fn jsonrpc_reply(
        result: &UniValue,
        error:  &UniValue,
        id:     &UniValue) -> String {
    
    todo!();
        /*
            UniValue reply = JSONRPCReplyObj(result, error, id);
        return reply.write() + "\n";
        */
}

pub fn jsonrpc_error(
        code:    i32,
        message: &str) -> UniValue {
    
    todo!();
        /*
            UniValue error(UniValue::VOBJ);
        error.pushKV("code", code);
        error.pushKV("message", message);
        return error;
        */
}

/**
  | Parse JSON-RPC batch reply into a vector
  |
  */
pub fn jsonrpc_process_batch_reply(in_: &UniValue) -> Vec<UniValue> {
    
    todo!();
        /*
            if (!in.isArray()) {
            throw std::runtime_error("Batch must be an array");
        }
        const size_t num {in.size()};
        std::vector<UniValue> batch(num);
        for (const UniValue& rec : in.getValues()) {
            if (!rec.isObject()) {
                throw std::runtime_error("Batch member must be an object");
            }
            size_t id = rec["id"].get_int();
            if (id >= num) {
                throw std::runtime_error("Batch member id is larger than batch size");
            }
            batch[id] = rec;
        }
        return batch;
        */
}
