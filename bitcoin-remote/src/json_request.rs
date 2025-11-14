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
    /// Parse a single JSON-RPC request object into this `JSONRPCRequest`
    /// using **UniValue** exclusively (no `serde_json`).
    pub fn parse(&mut self, val_request: &UniValue) {
        trace!("JSONRPCRequest::parse: begin");

        // Parse request
        if !val_request.is_object() {
            error!("JSONRPCRequest::parse: invalid top-level type (expected object)");
            let err = jsonrpc_error(RPCErrorCode::RPC_INVALID_REQUEST.bits(), "Invalid Request object");
            panic!("{}", err.write(None, None));
        }

        // Parse id now so errors from here on will have the id
        self.id = find_value(val_request, "id").clone();
        debug!(id = ?self.id, "JSONRPCRequest::parse: captured id");

        // Parse method
        let val_method = find_value(val_request, "method").clone();
        if val_method.is_null() {
            error!("JSONRPCRequest::parse: missing method");
            let err = jsonrpc_error(RPCErrorCode::RPC_INVALID_REQUEST.bits(), "Missing method");
            panic!("{}", err.write(None, None));
        }
        if !val_method.is_str() {
            error!("JSONRPCRequest::parse: method not a string");
            let err = jsonrpc_error(RPCErrorCode::RPC_INVALID_REQUEST.bits(), "Method must be a string");
            panic!("{}", err.write(None, None));
        }
        self.str_method = val_method.get_str().to_owned();

        // Parity with upstream logging (include peer address when available)
        if self.peer_addr.is_empty() {
            info!(
                method = %self.str_method,
                user = %self.auth_user,
                "ThreadRPCServer method log (no peer address)"
            );
        } else {
            info!(
                method = %self.str_method,
                user = %self.auth_user,
                peeraddr = %self.peer_addr,
                "ThreadRPCServer method log"
            );
        }

        // Parse params
        let val_params = find_value(val_request, "params").clone();
        if val_params.is_array() || val_params.is_object() {
            self.params = val_params;
        } else if val_params.is_null() {
            self.params = UniValue::empty_array();
        } else {
            error!("JSONRPCRequest::parse: params wrong type (must be array or object)");
            let err = jsonrpc_error(
                RPCErrorCode::RPC_INVALID_REQUEST.bits(),
                "Params must be an array or object",
            );
            panic!("{}", err.write(None, None));
        }

        trace!(
            method = %self.str_method,
            params_kind = %if self.params.is_array() { "array" } else if self.params.is_object() { "object" } else { "null" },
            "JSONRPCRequest::parse: end"
        );
    }
}

//-------------------------------------------[.cpp/bitcoin/src/rpc/request.cpp]

/// JSON-RPC protocol.
/// 
/// Bitcoin speaks version 1.0 for maximum compatibility, but uses JSON-RPC
/// 1.1/2.0 standards for parts of the 1.0 standard that were unspecified (HTTP
/// errors and contents of 'error').
/// 
/// 1.0 spec: http://json-rpc.org/wiki/specification
/// 
/// 1.2 spec: http://jsonrpc.org/historical/json-rpc-over-http.html
///
/// Build a JSON-RPC request object using **UniValue**:
///
/// { "method": <strMethod>, "params": <params>, "id": <id> }
pub fn jsonrpc_request_obj(
    str_method: &str,
    params: &UniValue,
    id: &UniValue,
) -> UniValue {
    trace!(method = str_method, "jsonrpc_request_obj: building request");

    let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
    assert!(obj.pushkv("method", str_method));
    assert!(obj.pushkv("params", params.clone()));
    assert!(obj.pushkv("id", id.clone()));

    debug!(request = ?obj, "jsonrpc_request_obj: built");
    obj
}

/// Build a JSON-RPC reply object (UniValue):
///
/// If `error` is not null, `"result"` is forced to `null`.
///
/// {
///   "result": <result or null>,
///   "error":  <error>,
///   "id":     <id>
/// }
pub fn jsonrpc_reply_obj(
    result: &UniValue,
    error:  &UniValue,
    id:     &UniValue,
) -> UniValue {
    let mut reply = UniValue::new(uni_value::VType::VOBJ, None);

    if !error.is_null() {
        assert!(reply.pushkv("result", UniValue::null()));
    } else {
        assert!(reply.pushkv("result", result.clone()));
    }
    assert!(reply.pushkv("error",  error.clone()));
    assert!(reply.pushkv("id",     id.clone()));

    trace!(reply = ?reply, "jsonrpc_reply_obj: built");
    reply
}

/// Serialize a JSON-RPC reply object to a compact JSON string with a trailing
/// newline, mirroring upstream's `write() + "\n"`.
pub fn jsonrpc_reply(
    result: &UniValue,
    error:  &UniValue,
    id:     &UniValue,
) -> String {
    let reply_obj = jsonrpc_reply_obj(result, error, id);
    let mut s = reply_obj.write(None, None);
    s.push('\n');
    trace!(len = s.len(), "jsonrpc_reply: serialized");
    s
}

/// Construct a JSON-RPC error object:
///
/// { "code": <code>, "message": <message> }
pub fn jsonrpc_error(
    code:    i32,
    message: &str,
) -> UniValue {
    let mut err = UniValue::new(uni_value::VType::VOBJ, None);
    assert!(err.pushkv("code", code));
    assert!(err.pushkv("message", message));
    debug!(code, message, err = ?err, "jsonrpc_error: built");
    err
}

/// Parse a JSON-RPC *batch* reply into a vector whose indices correspond to the
/// numeric `"id"` of each member.
///
/// Structural violations panic, mirroring the original C++ exceptions.
///
/// Errors:
/// - *"Batch must be an array"* if `in_` is not an array
/// - *"Batch member must be an object"* for non-object members
/// - *"Batch member id is larger than batch size"* if an `id` exceeds the batch bound
///
pub fn jsonrpc_process_batch_reply(in_: &UniValue) -> Vec<UniValue> {
    trace!("jsonrpc_process_batch_reply: begin");

    if !in_.is_array() {
        error!("jsonrpc_process_batch_reply: not an array");
        panic!("Batch must be an array");
    }

    let num = in_.size();
    let mut batch: Vec<UniValue> = vec![UniValue::null(); num];

    for rec in in_.get_values().iter() {
        if !rec.is_object() {
            error!(record = ?rec, "jsonrpc_process_batch_reply: member not an object");
            panic!("Batch member must be an object");
        }

        let id_uv = &rec["id"];
        let id_i64 = if id_uv.is_num() {
            id_uv.get_int64()
        } else {
            error!(id = ?id_uv, "jsonrpc_process_batch_reply: non-integer id");
            panic!("Batch member id is larger than batch size");
        };

        if id_i64 < 0 {
            error!(id = id_i64, "jsonrpc_process_batch_reply: negative id");
            panic!("Batch member id is larger than batch size");
        }

        let id_u = id_i64 as usize;
        if id_u >= num {
            error!(id = id_u, num, "jsonrpc_process_batch_reply: id out of bounds");
            panic!("Batch member id is larger than batch size");
        }

        trace!(id = id_u, "jsonrpc_process_batch_reply: placing record");
        batch[id_u] = rec.clone();
    }

    debug!(count = batch.len(), "jsonrpc_process_batch_reply: end");
    batch
}

#[cfg(test)]
mod tests_jsonrpc_request_parsing {
    use super::*;

    fn make_request(id: UniValue, method: UniValue, params: Option<UniValue>) -> UniValue {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(obj.pushkv("id", id));
        assert!(obj.pushkv("method", method));
        if let Some(p) = params {
            assert!(obj.pushkv("params", p));
        }
        obj
    }

    fn empty_array() -> UniValue {
        UniValue::new(uni_value::VType::VARR, None)
    }

    fn blank_req() -> JSONRPCRequest {
        JSONRPCRequest {
            id: UniValue::null(),
            str_method: String::new(),
            params: UniValue::null(),
            mode: JSONRPCRequestMode::EXECUTE,
            uri: String::new(),
            auth_user: String::new(),
            peer_addr: String::new(),
            context: Box::new(()),
        }
    }

    #[traced_test]
    fn parses_valid_minimal_request_object() {
        let req_val = make_request(7i64.into(), "echo".into(), Some(empty_array()));
        let mut req = blank_req();
        req.parse(&req_val);

        debug!(id = ?req.id, method = %req.str_method, params = ?req.params, "post-parse state");
        assert_eq!(req.id.get_int64(), 7);
        assert_eq!(req.str_method, "echo");
        assert!(req.params.is_array());
        assert_eq!(req.params.size(), 0);
    }

    #[traced_test]
    fn params_defaults_to_empty_array_when_missing() {
        let req_val = make_request(1i64.into(), "getinfo".into(), None);
        let mut req = blank_req();
        req.parse(&req_val);
        assert!(req.params.is_array());
        assert_eq!(req.params.size(), 0);
    }

    #[traced_test]
    fn params_defaults_to_empty_array_when_null() {
        let req_val = {
            let mut o = UniValue::new(uni_value::VType::VOBJ, None);
            assert!(o.pushkv("id", 1i64));
            assert!(o.pushkv("method", "getinfo"));
            assert!(o.pushkv("params", UniValue::null()));
            o
        };
        let mut req = blank_req();
        req.parse(&req_val);
        assert!(req.params.is_array());
        assert_eq!(req.params.size(), 0);
    }

    #[traced_test]
    fn parse_panics_on_missing_method() {
        let mut obj = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(obj.pushkv("id", 1i64));
        let mut req = blank_req();
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| req.parse(&obj)));
        assert!(got.is_err(), "expected panic for missing method");
    }

    #[traced_test]
    fn parse_panics_on_non_string_method() {
        let req_val = make_request(1i64.into(), 123i64.into(), Some(empty_array()));
        let mut req = blank_req();
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| req.parse(&req_val)));
        assert!(got.is_err(), "expected panic for non-string method");
    }

    #[traced_test]
    fn parse_panics_on_bad_params_type() {
        let req_val = make_request(1i64.into(), "getinfo".into(), Some("not-array-or-object".into()));
        let mut req = blank_req();
        let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| req.parse(&req_val)));
        assert!(got.is_err(), "expected panic for bad params type");
    }
}

#[cfg(test)]
mod tests_jsonrpc_protocol_builders {
    use super::*;

    fn array_123() -> UniValue {
        let mut a = UniValue::new(uni_value::VType::VARR, None);
        a.values_mut().extend([1i64.into(), 2i64.into(), 3i64.into()]);
        a
    }

    #[traced_test]
    fn request_object_contains_method_params_id() {
        let params = array_123();
        let id: UniValue = 42i64.into();
        let obj = jsonrpc_request_obj("sum", &params, &id);

        assert!(obj.is_object());
        assert_eq!(obj["method"].get_str(), "sum");

        let p = &obj["params"];
        assert!(p.is_array());
        assert_eq!(p.size(), 3);
        assert_eq!(p.get_values()[0].get_int64(), 1);

        assert_eq!(obj["id"].get_int64(), 42);
    }

    #[traced_test]
    fn reply_object_uses_result_when_error_is_null() {
        let mut result = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(result.pushkv("ok", true));
        let error  = UniValue::null();
        let id     = 1i64.into();
        let reply  = jsonrpc_reply_obj(&result, &error, &id);

        assert_eq!(reply["result"]["ok"].get_bool(), true);
        assert!(reply["error"].is_null());
        assert_eq!(reply["id"].get_int64(), 1);
    }

    #[traced_test]
    fn reply_object_nulls_result_when_error_present() {
        let mut result = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(result.pushkv("ok", true));
        let error  = jsonrpc_error(-1, "boom");
        let id     = 1i64.into();
        let reply  = jsonrpc_reply_obj(&result, &error, &id);

        assert!(reply["result"].is_null());
        assert_eq!(reply["error"]["code"].get_int(), -1);
        assert_eq!(reply["error"]["message"].get_str(), "boom");
        assert_eq!(reply["id"].get_int64(), 1);
    }

    #[traced_test]
    fn reply_string_is_json_with_trailing_newline() {
        let mut result = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(result.pushkv("data", array_123()));
        let error  = UniValue::null();
        let id     = 99i64.into();
        let s      = jsonrpc_reply(&result, &error, &id);

        assert!(s.ends_with('\n'), "must end with newline");
        let trimmed = s.trim_end_matches('\n');

        let mut parsed = UniValue::default();
        assert!(parsed.read(trimmed.as_ptr(), trimmed.len()));
        assert_eq!(parsed["id"].get_int64(), 99);
        assert!(parsed["error"].is_null());

        let arr = &parsed["result"]["data"];
        assert!(arr.is_array());
        assert_eq!(arr.size(), 3);
    }

    #[traced_test]
    fn error_object_contains_code_and_message() {
        let e = jsonrpc_error(-32600, "Invalid Request object");
        assert_eq!(e["code"].get_int(), -32600);
        assert_eq!(e["message"].get_str(), "Invalid Request object");
    }
}

#[cfg(test)]
mod tests_jsonrpc_batch_processing {
    use super::*;

    fn obj_with_id_result(id: i64, key: &str, val: i64) -> UniValue {
        let mut r = UniValue::new(uni_value::VType::VOBJ, None);
        let mut res = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(res.pushkv(key, val));
        assert!(r.pushkv("id", id));
        assert!(r.pushkv("result", res));
        assert!(r.pushkv("error", UniValue::null()));
        r
    }

    fn batch_ok() -> UniValue {
        let mut arr = UniValue::new(uni_value::VType::VARR, None);
        arr.values_mut().push(obj_with_id_result(0, "a", 1));
        arr.values_mut().push(obj_with_id_result(1, "b", 2));
        arr
    }

    #[traced_test]
    fn processes_simple_batch_into_indexed_vector() {
        let v = jsonrpc_process_batch_reply(&batch_ok());
        assert_eq!(v.len(), 2);
        assert_eq!(v[0]["id"].get_int(), 0);
        assert_eq!(v[1]["id"].get_int(), 1);
    }

    #[traced_test]
    fn panics_when_input_is_not_array() {
        let mut not_array = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(not_array.pushkv("id", 0i64));
        let got = std::panic::catch_unwind(|| jsonrpc_process_batch_reply(&not_array));
        assert!(got.is_err(), "expected panic for non-array batch");
    }

    #[traced_test]
    fn panics_when_member_is_not_object() {
        let mut arr = UniValue::new(uni_value::VType::VARR, None);
        arr.values_mut().extend([1i64.into(), 2i64.into(), 3i64.into()]);
        let got = std::panic::catch_unwind(|| jsonrpc_process_batch_reply(&arr));
        assert!(got.is_err(), "expected panic for non-object member");
    }

    #[traced_test]
    fn panics_when_id_is_out_of_bounds() {
        let mut arr = UniValue::new(uni_value::VType::VARR, None);
        let mut r = UniValue::new(uni_value::VType::VOBJ, None);
        assert!(r.pushkv("id", 5i64));
        assert!(r.pushkv("result", UniValue::null()));
        assert!(r.pushkv("error", UniValue::null()));
        arr.values_mut().push(r);

        let got = std::panic::catch_unwind(|| jsonrpc_process_batch_reply(&arr));
        assert!(got.is_err(), "expected panic for id >= batch size");
    }

    #[traced_test]
    fn duplicate_ids_last_write_wins() {
        let mut arr = UniValue::new(uni_value::VType::VARR, None);

        // first record
        arr.values_mut().push({
            let mut r = UniValue::new(uni_value::VType::VOBJ, None);
            let mut res = UniValue::new(uni_value::VType::VOBJ, None);
            assert!(res.pushkv("v", 1i64));
            assert!(r.pushkv("id", 0i64));
            assert!(r.pushkv("result", res));
            assert!(r.pushkv("error", UniValue::null()));
            r
        });

        // second record with same id
        arr.values_mut().push({
            let mut r = UniValue::new(uni_value::VType::VOBJ, None);
            let mut res = UniValue::new(uni_value::VType::VOBJ, None);
            assert!(res.pushkv("v", 2i64));
            assert!(r.pushkv("id", 0i64));
            assert!(r.pushkv("result", res));
            assert!(r.pushkv("error", UniValue::null()));
            r
        });

        let v = jsonrpc_process_batch_reply(&arr);
        let last = &v[0]["result"]["v"];
        assert_eq!(last.get_int64(), 2);
    }
}
