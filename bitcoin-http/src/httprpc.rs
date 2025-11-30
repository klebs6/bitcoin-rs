// ---------------- [ File: bitcoin-http/src/httprpc.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/httprpc.h]

/**
  | Start HTTP RPC subsystem.
  | 
  | Precondition; HTTP and RPC has been
  | started.
  |
  */
pub fn starthttprpc(context: &dyn Any) -> bool {
    
    todo!();
        /*
        
        */
}

/**
  | Stop HTTP RPC subsystem.
  | 
  | Precondition; HTTP and RPC has been
  | stopped.
  |
  */
pub fn stophttprpc()  {
    
    todo!();
        /*
        
        */
}

/**
  | Start HTTP REST subsystem.
  | 
  | Precondition; HTTP and RPC has been
  | started.
  |
  */
pub fn startrest(context: &dyn Any)  {
    
    todo!();
        /*
        
        */
}

/**
  | Interrupt RPC REST subsystem.
  |
  */
pub fn interruptrest()  {
    
    todo!();
        /*
        
        */
}

/**
  | Stop HTTP REST subsystem.
  | 
  | Precondition; HTTP and RPC has been
  | stopped.
  |
  */
pub fn stoprest()  {
    
    todo!();
        /*
        
        */
}

//-------------------------------------------[.cpp/bitcoin/src/httprpc.cpp]

/**
  | WWW-Authenticate to present with 401
  | Unauthorized response
  |
  */
pub const WWW_AUTH_HEADER_DATA: &'static str = "Basic realm=\"jsonrpc\"";

/**
  | Simple one-shot callback timer to be
  | used by the RPC mechanism to e.g. re-lock
  | the wallet.
  |
  */
pub struct HTTPRPCTimer {
    ev:   HTTPEvent,
}

impl RPCTimerBase for HTTPRPCTimer {

}

impl HTTPRPCTimer {

    pub fn new(
        event_base: *mut libevent_sys::event_base,
        func:       &mut fn() -> (),
        millis:     i64) -> Self {
    
        todo!();
        /*
        : ev(eventBase, false, func),

            struct timeval tv;
            tv.tv_sec = millis/1000;
            tv.tv_usec = (millis%1000)*1000;
            ev.trigger(&tv);
        */
    }
}

///-----------------
pub struct HTTPRPCTimerInterface {
    base: *mut libevent_sys::event_base,
}

impl RPCTimerInterface for HTTPRPCTimerInterface {

}

impl Name for HTTPRPCTimerInterface {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("HTTP".to_string())
    }
}

impl HTTPRPCTimerInterface {

    pub fn new(base: *mut libevent_sys::event_base) -> Self {
    
        todo!();
        /*
        : base(_base),

        
        */
    }
    
    pub fn new_timer(&mut self, 
        func:   &mut fn() -> (),
        millis: i64) -> Rc<RefCell<dyn RPCTimerBase>> {
        
        todo!();
        /*
            return new HTTPRPCTimer(base, func, millis);
        */
    }
}

/**
  | Pre-base64-encoded authentication
  | token
  |
  */
lazy_static!{
    /*
    static std::string strRPCUserColonPass;
    */
}

/**
  | Stored RPC timer interface (for unregistration)
  |
  */
lazy_static!{
    /*
    static std::unique_ptr<HTTPRPCTimerInterface> httpRPCTimerInterface;
    */
}

/**
  | List of -rpcauth values
  |
  */
lazy_static!{
    /*
    static std::vector<std::vector<std::string>> g_rpcauth;
    */
}

/**
  | RPC Auth Whitelist
  |
  */
lazy_static!{
    /*
    static std::map<std::string, std::set<std::string>> g_rpc_whitelist;
    static bool g_rpc_whitelist_default = false;
    */
}

pub fn json_error_reply(
        req:       *mut HTTPRequest,
        obj_error: &UniValue,
        id:        &UniValue)  {
    
    todo!();
        /*
            // Send error reply from json-rpc error object
        int nStatus = HTTP_INTERNAL_SERVER_ERROR;
        int code = find_value(objError, "code").get_int();

        if (code == RPC_INVALID_REQUEST)
            nStatus = HTTP_BAD_REQUEST;
        else if (code == RPC_METHOD_NOT_FOUND)
            nStatus = HTTP_NOT_FOUND;

        std::string strReply = JSONRPCReply(NullUniValue, objError, id);

        req->WriteHeader("Content-Type", "application/json");
        req->WriteReply(nStatus, strReply);
        */
}

/**
  | This function checks username and password
  | against -rpcauth entries from config file.
  */
pub fn multi_user_authorized(str_user_pass: String) -> bool {
    
    todo!();
        /*
            if (strUserPass.find(':') == std::string::npos) {
            return false;
        }
        std::string strUser = strUserPass.substr(0, strUserPass.find(':'));
        std::string strPass = strUserPass.substr(strUserPass.find(':') + 1);

        for (const auto& vFields : g_rpcauth) {
            std::string strName = vFields[0];
            if (!TimingResistantEqual(strName, strUser)) {
                continue;
            }

            std::string strSalt = vFields[1];
            std::string strHash = vFields[2];

            static const unsigned int KEY_SIZE = 32;
            unsigned char out[KEY_SIZE];

            CHMAC_SHA256(reinterpret_cast<const unsigned char*>(strSalt.data()), strSalt.size()).Write(reinterpret_cast<const unsigned char*>(strPass.data()), strPass.size()).Finalize(out);
            std::vector<unsigned char> hexvec(out, out+KEY_SIZE);
            std::string strHashFromPass = HexStr(hexvec);

            if (TimingResistantEqual(strHashFromPass, strHash)) {
                return true;
            }
        }
        return false;
        */
}

pub fn rpc_authorized(
        str_auth:              &String,
        str_auth_username_out: &mut String) -> bool {
    
    todo!();
        /*
            if (strRPCUserColonPass.empty()) // Belt-and-suspenders measure if InitRPCAuthentication was not called
            return false;
        if (strAuth.substr(0, 6) != "Basic ")
            return false;
        std::string strUserPass64 = TrimString(strAuth.substr(6));
        std::string strUserPass = DecodeBase64(strUserPass64);

        if (strUserPass.find(':') != std::string::npos)
            strAuthUsernameOut = strUserPass.substr(0, strUserPass.find(':'));

        //Check if authorized under single-user field
        if (TimingResistantEqual(strUserPass, strRPCUserColonPass)) {
            return true;
        }
        return multiUserAuthorized(strUserPass);
        */
}

pub fn http_req_jsonrpc(
        context: &dyn Any,
        req:     *mut HTTPRequest) -> bool {
    
    todo!();
        /*
            // JSONRPC handles only POST
        if (req->GetRequestMethod() != HTTPRequest::POST) {
            req->WriteReply(HTTP_BAD_METHOD, "JSONRPC server handles only POST requests");
            return false;
        }
        // Check authorization
        std::pair<bool, std::string> authHeader = req->GetHeader("authorization");
        if (!authHeader.first) {
            req->WriteHeader("WWW-Authenticate", WWW_AUTH_HEADER_DATA);
            req->WriteReply(HTTP_UNAUTHORIZED);
            return false;
        }

        JSONRPCRequest jreq;
        jreq.context = context;
        jreq.peerAddr = req->GetPeer().ToString();
        if (!RPCAuthorized(authHeader.second, jreq.authUser)) {
            LogPrintf("ThreadRPCServer incorrect password attempt from %s\n", jreq.peerAddr);

            /* Deter brute-forcing
               If this results in a DoS the user really
               shouldn't have their RPC port exposed. */
            UninterruptibleSleep(std::chrono::milliseconds{250});

            req->WriteHeader("WWW-Authenticate", WWW_AUTH_HEADER_DATA);
            req->WriteReply(HTTP_UNAUTHORIZED);
            return false;
        }

        try {
            // Parse request
            UniValue valRequest;
            if (!valRequest.read(req->ReadBody()))
                throw JSONRPCError(RPC_PARSE_ERROR, "Parse error");

            // Set the URI
            jreq.URI = req->GetURI();

            std::string strReply;
            bool user_has_whitelist = g_rpc_whitelist.count(jreq.authUser);
            if (!user_has_whitelist && g_rpc_whitelist_default) {
                LogPrintf("RPC User %s not allowed to call any methods\n", jreq.authUser);
                req->WriteReply(HTTP_FORBIDDEN);
                return false;

            // singleton request
            } else if (valRequest.isObject()) {
                jreq.parse(valRequest);
                if (user_has_whitelist && !g_rpc_whitelist[jreq.authUser].count(jreq.strMethod)) {
                    LogPrintf("RPC User %s not allowed to call method %s\n", jreq.authUser, jreq.strMethod);
                    req->WriteReply(HTTP_FORBIDDEN);
                    return false;
                }
                UniValue result = tableRPC.execute(jreq);

                // Send reply
                strReply = JSONRPCReply(result, NullUniValue, jreq.id);

            // array of requests
            } else if (valRequest.isArray()) {
                if (user_has_whitelist) {
                    for (unsigned int reqIdx = 0; reqIdx < valRequest.size(); reqIdx++) {
                        if (!valRequest[reqIdx].isObject()) {
                            throw JSONRPCError(RPC_INVALID_REQUEST, "Invalid Request object");
                        } else {
                            const UniValue& request = valRequest[reqIdx].get_obj();
                            // Parse method
                            std::string strMethod = find_value(request, "method").get_str();
                            if (!g_rpc_whitelist[jreq.authUser].count(strMethod)) {
                                LogPrintf("RPC User %s not allowed to call method %s\n", jreq.authUser, strMethod);
                                req->WriteReply(HTTP_FORBIDDEN);
                                return false;
                            }
                        }
                    }
                }
                strReply = JSONRPCExecBatch(jreq, valRequest.get_array());
            }
            else
                throw JSONRPCError(RPC_PARSE_ERROR, "Top-level object parse error");

            req->WriteHeader("Content-Type", "application/json");
            req->WriteReply(HTTP_OK, strReply);
        } catch (const UniValue& objError) {
            JSONErrorReply(req, objError, jreq.id);
            return false;
        } catch (const std::exception& e) {
            JSONErrorReply(req, JSONRPCError(RPC_PARSE_ERROR, e.what()), jreq.id);
            return false;
        }
        return true;
        */
}

pub fn init_rpc_authentication() -> bool {
    
    todo!();
        /*
            if (gArgs.GetArg("-rpcpassword", "") == "")
        {
            LogPrintf("Using random cookie authentication.\n");
            if (!GenerateAuthCookie(&strRPCUserColonPass)) {
                return false;
            }
        } else {
            LogPrintf("Config options rpcuser and rpcpassword will soon be deprecated. Locally-run instances may remove rpcuser to use cookie-based auth, or may be replaced with rpcauth. Please see share/rpcauth for rpcauth auth generation.\n");
            strRPCUserColonPass = gArgs.GetArg("-rpcuser", "") + ":" + gArgs.GetArg("-rpcpassword", "");
        }
        if (gArgs.GetArg("-rpcauth","") != "")
        {
            LogPrintf("Using rpcauth authentication.\n");
            for (const std::string& rpcauth : gArgs.GetArgs("-rpcauth")) {
                std::vector<std::string> fields;
                boost::split(fields, rpcauth, boost::is_any_of(":$"));
                if (fields.size() == 3) {
                    g_rpcauth.push_back(fields);
                } else {
                    LogPrintf("Invalid -rpcauth argument.\n");
                    return false;
                }
            }
        }

        g_rpc_whitelist_default = gArgs.GetBoolArg("-rpcwhitelistdefault", gArgs.IsArgSet("-rpcwhitelist"));
        for (const std::string& strRPCWhitelist : gArgs.GetArgs("-rpcwhitelist")) {
            auto pos = strRPCWhitelist.find(':');
            std::string strUser = strRPCWhitelist.substr(0, pos);
            bool intersect = g_rpc_whitelist.count(strUser);
            std::set<std::string>& whitelist = g_rpc_whitelist[strUser];
            if (pos != std::string::npos) {
                std::string strWhitelist = strRPCWhitelist.substr(pos + 1);
                std::set<std::string> new_whitelist;
                boost::split(new_whitelist, strWhitelist, boost::is_any_of(", "));
                if (intersect) {
                    std::set<std::string> tmp_whitelist;
                    std::set_intersection(new_whitelist.begin(), new_whitelist.end(),
                           whitelist.begin(), whitelist.end(), std::inserter(tmp_whitelist, tmp_whitelist.end()));
                    new_whitelist = std::move(tmp_whitelist);
                }
                whitelist = std::move(new_whitelist);
            }
        }

        return true;
        */
}

pub fn start_http_rpc(context: &dyn Any) -> bool {
    
    todo!();
        /*
            LogPrint(LogFlags::RPC, "Starting HTTP RPC server\n");
        if (!InitRPCAuthentication())
            return false;

        auto handle_rpc = [context](HTTPRequest* req, const std::string&) { return HTTPReq_JSONRPC(context, req); };
        RegisterHTTPHandler("/", true, handle_rpc);
        if (g_wallet_init_interface.HasWalletSupport()) {
            RegisterHTTPHandler("/wallet/", false, handle_rpc);
        }
        struct event_base* eventBase = libevent_sys::event_base();
        assert(eventBase);
        httpRPCTimerInterface = std::make_unique<HTTPRPCTimerInterface>(eventBase);
        RPCSetTimerInterface(httpRPCTimerInterface.get());
        return true;
        */
}

/**
  | Interrupt HTTP RPC subsystem.
  |
  */
pub fn interrupt_http_rpc()  {
    
    todo!();
        /*
            LogPrint(LogFlags::RPC, "Interrupting HTTP RPC server\n");
        */
}

pub fn stop_http_rpc()  {
    
    todo!();
        /*
            LogPrint(LogFlags::RPC, "Stopping HTTP RPC server\n");
        UnregisterHTTPHandler("/", true);
        if (g_wallet_init_interface.HasWalletSupport()) {
            UnregisterHTTPHandler("/wallet/", false);
        }
        if (httpRPCTimerInterface) {
            RPCUnsetTimerInterface(httpRPCTimerInterface.get());
            httpRPCTimerInterface.reset();
        }
        */
}
