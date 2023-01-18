crate::ix!();

pub fn callrpc(
        rh:         &mut Box<dyn BaseRequestHandler>,
        str_method: &str,
        args:       &Vec<String>,
        rpcwallet:  Option<&str>) -> Result<UniValue,StdException> {

    let mut host = String::default();

    //  In preference order, we choose the following for the port:
    //      1. -rpcport
    //      2. port in -rpcconnect (ie following : in ipv4 or ]: in ipv6)
    //      3. default port for chain
    let mut port: u16 = base_params().rpc_port();

    split_host_port(
        G_ARGS
            .lock()
            //.unwrap()
            .get_arg("-rpcconnect", DEFAULT_RPCCONNECT), 
        &mut port, 
        &mut host
    );

    port 
        = G_ARGS
            .lock()
            //.unwrap()
            .get_int_arg("-rpcport", port.into()) as u16;

    //  Obtain event base
    let mut base: Box<event_base> = obtain_event_base();;

    let base_raw = Box::into_raw(base);

    //  Synchronously look up hostname
    let mut evcon: Box<evhttp_connection> 
    = obtain_evhttp_connection_base(
        base_raw,
        &host,
        port
    );

    let evcon_raw = Box::into_raw(evcon);

    pub const YEAR_IN_SECONDS: i32 = 0;

    //  Set connection timeout
    {

        let timeout: i32 
        = G_ARGS
            .lock()
            //.unwrap()
            .get_int_arg("-rpcclienttimeout", DEFAULT_HTTP_CLIENT_TIMEOUT.into())
            .try_into()
            .unwrap();

        if timeout > 0 {

            unsafe {
                evhttp_connection_set_timeout(
                    evcon_raw, 
                    timeout
                );
            }

        } else {

            //  Indefinite request timeouts are
            //  not possible in libevent-http, so we set
            //  the timeout to a very long time period
            //  instead.
            {
                //  Average length of year in Gregorian calendar
                pub const YEAR_IN_SECONDS: i32 = 31556952;

                unsafe {
                    evhttp_connection_set_timeout(
                        evcon_raw, 
                        5 * YEAR_IN_SECONDS
                    );
                }
            }
        }
    }

    let mut response = HTTPReply::default();

    let req: Box<evhttp_request> 
    = obtain_evhttp_request(
        http_request_done,
        &mut response as *mut _ as *mut libc::c_void
    );

    let raw_req = Box::into_raw(req);

    if raw_req == std::ptr::null_mut() {
        return Err(runtime_error("create http request failed"));
    }

    #[cfg(LIBEVENT_VERSION_NUMBER_GTE_0x02010300)]
    evhttp_request_set_error_cb(req, http_error_cb);

    //  Get credentials
    let mut str_rpc_user_colon_pass = String::default();;
    let mut failed_to_get_auth_cookie: bool = false;

    if G_ARGS
        .lock()
        //.unwrap()
        .get_arg("-rpcpassword", "") == "" {

        // Try fall back to cookie-based
        // authentication if no password is
        // provided
        if !get_auth_cookie(&mut str_rpc_user_colon_pass) {
            failed_to_get_auth_cookie = true;
        }

    } else {
        str_rpc_user_colon_pass 
            = format!{
                "{}:{}",
                G_ARGS
                    .lock()
                    //.unwrap()
                    .get_arg("-rpcuser", ""),
                G_ARGS
                    .lock()
                    //.unwrap()
                    .get_arg("-rpcpassword", "")
            };
    }

    let output_headers: *mut evkeyvalq 
    = unsafe { 
        evhttp_request_get_output_headers(raw_req)
    };

    assert!(output_headers != null_mut());

    unsafe {
        evhttp_add_header(
            output_headers, 
            "Host".as_ptr() as *const i8, 
            host.as_ptr() as *const i8
        );

        unsafe {
            evhttp_add_header(
                output_headers, 
                "Connection".as_ptr() as *const i8, 
                "close".as_ptr() as *const i8
            );

            evhttp_add_header(
                output_headers, 
                "Content-Type".as_ptr() as *const i8, 
                "application/json".as_ptr() as *const i8
            );
        }
    }

    let authorization_msg 
    = format!(
        "Basic {}", 
        encode_base64(&str_rpc_user_colon_pass)
    );

    unsafe {
        evhttp_add_header(
            output_headers, 
            "Authorization".as_ptr() as *const i8, 
            authorization_msg.as_ptr() as *const i8 
        );
    }

    // Attach request data
    let str_request: String 
    = unsafe { 
        (*rh)
            .prepare_request(str_method, args)
            .unwrap()
            .write(None, None) + "\n"
    };

    let output_buffer: *mut evbuffer = unsafe { 
        evhttp_request_get_output_buffer(raw_req)
    };

    assert!(output_buffer != null_mut());

    unsafe {
        evbuffer_add(
            output_buffer, 
            str_request.as_ptr() as *const c_void, 
            str_request.len().try_into().unwrap()
        );
    }

    // check if we should use a special wallet
    // endpoint
    let mut endpoint: String = "/".to_string();

    if rpcwallet.is_some() {

        //TODO: this may not be correct
        //because rust strings use unicode
        let encoded_uri: *mut i8 = unsafe { 
            evhttp_uriencode(
                rpcwallet.unwrap().as_ptr() as *const i8,
                rpcwallet.unwrap().len().try_into().unwrap(),
                0 /*false*/
            )
        };

        if encoded_uri != null_mut() {

            //TODO: may want a better way to print
            //encoded_uri, which is just a ptr
            endpoint = format!("/wallet/{:?}", encoded_uri);

            unsafe {
                libc::free(encoded_uri as *mut c_void);
            }

        } else {
            return Err(connection_failed("uri-encode failed"));
        }
    }

    // ownership of raw_req moved to evcon in the
    // evhttp_make_request call
    let r: i32 = unsafe { 
        evhttp_make_request(
            evcon_raw,
            raw_req,
            evhttp_cmd_type_EVHTTP_REQ_POST,
            endpoint.as_ptr() as *const i8
        )
    };

    if r != 0 {
        return Err(connection_failed("send http request failed"));
    }

    unsafe {
        event_base_dispatch(base_raw);
    }

    if response.status == 0 {

        let mut response_error_message = String::default();

        if response.error != -1 {
            response_error_message 
                = format!(
                    " (error code {} - \"{}\")",
                    response.error,
                    http_errorstring(response.error)
                );
        }

        let msg = formatdoc!(
            "Could not connect to the server {}:{}{}\n\n
                Make sure the bitcoind server is running and 
                that you are connecting to the correct RPC port.",
            host,
            port,
            response_error_message
        );

        return Err(connection_failed(&msg));

    } else {

        if response.status 
        == HTTPStatusCode::HTTP_UNAUTHORIZED as i32 {

            if failed_to_get_auth_cookie {

                let conf = G_ARGS
                    .lock()
                    //.unwrap()
                    .get_arg("-conf", BITCOIN_CONF_FILENAME);

                let msg = formatdoc!(
                    "Could not locate RPC credentials. 
                        No authentication cookie could be found, and 
                        RPC password is not set.  

                    See -rpcpassword and -stdinrpcpass.  
                    Configuration file: ({})",
                    get_config_file(&conf).to_str().unwrap()
                );

                return Err(runtime_error(&msg));

            } else {

                let msg = "Authorization failed: Incorrect rpcuser or rpcpassword";

                return Err(runtime_error(&msg));
            }

        } else {

            if response.status == HTTPStatusCode::HTTP_SERVICE_UNAVAILABLE as i32 {

                return Err(runtime_error(&format!("Server response: {}",response.body)));

            } else {

                let code = response.status;

                if code >= 400 
                && code != HTTPStatusCode::HTTP_BAD_REQUEST as i32 
                && code != HTTPStatusCode::HTTP_NOT_FOUND as i32
                && code != HTTPStatusCode::HTTP_INTERNAL_SERVER_ERROR as i32 {

                    return Err(runtime_error(&format!("server returned HTTP error {}",response.status)));

                } else {

                    if response.body.is_empty() {
                        return Err(runtime_error("no response from server"));
                    }
                }
            }
        }
    }

    // Parse reply
    let mut val_reply: UniValue = UniValue::new(uni_value::VType::VSTR, None);

    if !val_reply.read(response.body.as_ptr(), response.body.len()) {
        return Err(runtime_error("couldn't parse reply from server"));
    }

    if let Ok(reply) = unsafe { (*rh).process_reply(&val_reply) } {

        Ok(reply)

    } else {

        let msg = "expected reply to have result, error and id properties";

        return Err(runtime_error(msg));
    }
}
