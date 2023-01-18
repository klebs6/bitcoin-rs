crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/httpserver.h]

pub const DEFAULT_HTTP_THREADS:        i32 = 4;
pub const DEFAULT_HTTP_WORKQUEUE:      i32 = 16;
pub const DEFAULT_HTTP_SERVER_TIMEOUT: i32 = 30;

/**
  | Handler for requests to a certain HTTP
  | path
  |
  */
pub type HTTPRequestHandler = fn(req: *mut HTTPRequest, _1: &String) -> bool;

/**
  | In-flight HTTP request.
  | 
  | Thin C++ wrapper around evhttp_request.
  |
  */
pub struct HTTPRequest {
    req:        *mut libevent_sys::evhttp_request,
    reply_sent: bool,
}

pub mod http_request {

    pub enum RequestMethod {
        UNKNOWN,
        GET,
        POST,
        HEAD,
        PUT
    }
}

/**
  | libevent_sys::event handler closure.
  |
  */
pub trait HTTPClosure {
    fn invoke(&mut self);
}

/**
  | libevent_sys::event class. This can be used either
  | as a cross-thread trigger or as a timer.
  |
  */
pub struct HTTPEvent {
    delete_when_triggered: bool,
    handler:               fn() -> (),
    ev:                    *mut libevent_sys::event,
}

impl HTTPEvent {

    /**
      | Create a new event. deleteWhenTriggered
      | deletes this event object after the
      | event is triggered (and the handler
      | called) handler is the handler to call
      | when the event is triggered.
      |
      */
    pub fn new(
        base:                  *mut libevent_sys::event_base,
        delete_when_triggered: bool,
        handler:               &fn() -> ()) -> Self {
    
        todo!();
        /*
        : delete_when_triggered(_deleteWhenTriggered),
        : handler(_handler),

            ev = event_new(base, -1, 0, httpevent_callback_fn, this);
        assert(ev);
        */
    }
    
    /**
      | Trigger the event. If tv is 0, trigger
      | it immediately. Otherwise trigger
      | it after the given time has elapsed.
      |
      */
    pub fn trigger(&mut self, tv: *mut libc::timeval)  {
        
        todo!();
        /*
            if (tv == nullptr)
            event_active(ev, 0, 0); // immediately trigger event in main thread
        else
            evtimer_add(ev, tv); // trigger after timeval passed
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/httpserver.cpp]

/**
  | Maximum size of http request (request
  | line + headers)
  |
  */
pub const MAX_HEADERS_SIZE: usize = 8192;

/**
  | HTTP request work item
  |
  */
pub struct HTTPWorkItem {
    req:  Box<HTTPRequest>,
    path: String,
    func: HTTPRequestHandler,
}

impl HTTPClosure for HTTPWorkItem {
    fn invoke(&mut self)  {
        
        todo!();
        /*
            func(req.get(), path);
        */
    }
}

impl HTTPWorkItem {

    pub fn new(
        req:  Box<HTTPRequest>,
        path: &String,
        func: &HTTPRequestHandler) -> Self {
    
        todo!();
        /*
        : req(std::move(_req)),
        : path(_path),
        : func(_func),

        
        */
    }
}

/**
  | Simple work queue for distributing
  | work over multiple threads.
  | 
  | Work items are simply callable objects.
  |
  */
pub struct WorkQueue<WorkItem> {
    cs:        Mutex<WorkQueueInner<WorkItem>>,
    max_depth: usize,
}

pub struct WorkQueueInner<WorkItem> {
    cond:      std::sync::Condvar,
    queue:     VecDeque<Box<WorkItem>>,
    running:   bool,
}

impl<WorkItem> Drop for WorkQueue<WorkItem> {

    /**
      | Precondition: worker threads have
      | all stopped (they have been joined).
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
        
        */
    }
}

impl<WorkItem> WorkQueue<WorkItem> {

    pub fn new(max_depth: usize) -> Self {
    
        todo!();
        /*
        : running(true),
        : max_depth(_maxDepth),
        */
    }

    /**
      | Enqueue a work item
      |
      */
    pub fn enqueue(&mut self, item: *mut WorkItem) -> bool {
        
        todo!();
        /*
            LOCK(cs);
            if (!running || queue.size() >= maxDepth) {
                return false;
            }
            queue.emplace_back(std::unique_ptr<WorkItem>(item));
            cond.notify_one();
            return true;
        */
    }

    /**
      | Thread function
      |
      */
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (true) {
                std::unique_ptr<WorkItem> i;
                {
                    WAIT_LOCK(cs, lock);
                    while (running && queue.empty())
                        cond.wait(lock);
                    if (!running && queue.empty())
                        break;
                    i = std::move(queue.front());
                    queue.pop_front();
                }
                (*i)();
            }
        */
    }

    /**
      | Interrupt and exit loops
      |
      */
    pub fn interrupt(&mut self)  {
        
        todo!();
        /*
            LOCK(cs);
            running = false;
            cond.notify_all();
        */
    }
}

///-------------------------
pub struct HTTPPathHandler {
    prefix:      String,
    exact_match: bool,
    handler:     HTTPRequestHandler,
}

impl HTTPPathHandler {

    pub fn new(
        prefix:      String,
        exact_match: bool,
        handler:     HTTPRequestHandler) -> Self {
    
        todo!();
        /*
        : prefix(_prefix),
        : exact_match(_exactMatch),
        : handler(_handler),

        
        */
    }
}

/* --------------- HTTP module state  --------------- */

/**
   libevent event loop
  */
lazy_static!{
    /*
    static struct event_base* eventBase = nullptr;
    */
}

/**
   HTTP server
  */
lazy_static!{
    /*
    static struct evhttp* eventHTTP = nullptr;
    */
}

/**
   List of subnets to allow RPC connections from
  */
lazy_static!{
    /*
    static std::vector<CSubNet> rpc_allow_subnets;
    */
}

/**
   Work queue for handling longer requests off the event loop thread
  */
lazy_static!{
    /*
    static std::unique_ptr<WorkQueue<HTTPClosure>> g_work_queue{nullptr};
    */
}

/**
   Handlers for (sub)paths
  */
lazy_static!{
    /*
    static std::vector<HTTPPathHandler> pathHandlers;
    */
}

/**
   Bound listening sockets
  */
lazy_static!{
    /*
    static std::vector<evhttp_bound_socket *> boundSockets;
    */
}

/**
  | Check if a network address is allowed
  | to access the HTTP server
  |
  */
pub fn client_allowed(netaddr: &NetAddr) -> bool {
    
    todo!();
        /*
            if (!netaddr.IsValid())
            return false;
        for(const CSubNet& subnet : rpc_allow_subnets)
            if (subnet.Match(netaddr))
                return true;
        return false;
        */
}

/**
  | Initialize ACL list for HTTP server
  |
  */
pub fn init_http_allow_list() -> bool {
    
    todo!();
        /*
            rpc_allow_subnets.clear();
        CNetAddr localv4;
        CNetAddr localv6;
        LookupHost("127.0.0.1", localv4, false);
        LookupHost("::1", localv6, false);
        rpc_allow_subnets.push_back(CSubNet(localv4, 8));      // always allow IPv4 local subnet
        rpc_allow_subnets.push_back(CSubNet(localv6));         // always allow IPv6 localhost
        for (const std::string& strAllow : gArgs.GetArgs("-rpcallowip")) {
            CSubNet subnet;
            LookupSubNet(strAllow, subnet);
            if (!subnet.IsValid()) {
                uiInterface.ThreadSafeMessageBox(
                    strprintf(Untranslated("Invalid -rpcallowip subnet specification: %s. Valid are a single IP (e.g. 1.2.3.4), a network/netmask (e.g. 1.2.3.4/255.255.255.0) or a network/CIDR (e.g. 1.2.3.4/24)."), strAllow),
                    "", CClientUIInterface::MSG_ERROR);
                return false;
            }
            rpc_allow_subnets.push_back(subnet);
        }
        std::string strAllowed;
        for (const CSubNet& subnet : rpc_allow_subnets)
            strAllowed += subnet.ToString() + " ";
        LogPrint(BCLog::HTTP, "Allowing HTTP connections from: %s\n", strAllowed);
        return true;
        */
}

/**
  | HTTP request method as string - use for
  | logging only
  |
  */
pub fn request_method_string(m: http_request::RequestMethod) -> String {
    
    todo!();
        /*
            switch (m) {
        case HTTPRequest::GET:
            return "GET";
            break;
        case HTTPRequest::POST:
            return "POST";
            break;
        case HTTPRequest::HEAD:
            return "HEAD";
            break;
        case HTTPRequest::PUT:
            return "PUT";
            break;
        default:
            return "unknown";
        }
        */
}

/**
  | HTTP request callback
  |
  */
pub fn http_request_cb(
        req: *mut libevent_sys::evhttp_request,
        arg: *mut c_void)  {
    
    todo!();
        /*
            // Disable reading to work around a libevent bug, fixed in 2.2.0.
        if (event_get_version_number() >= 0x02010600 && event_get_version_number() < 0x02020001) {
            evhttp_connection* conn = evhttp_request_get_connection(req);
            if (conn) {
                bufferevent* bev = evhttp_connection_get_bufferevent(conn);
                if (bev) {
                    bufferevent_disable(bev, EV_READ);
                }
            }
        }
        std::unique_ptr<HTTPRequest> hreq(new HTTPRequest(req));

        // Early address-based allow check
        if (!ClientAllowed(hreq->GetPeer())) {
            LogPrint(BCLog::HTTP, "HTTP request from %s rejected: Client network is not allowed RPC access\n",
                     hreq->GetPeer().ToString());
            hreq->WriteReply(HTTP_FORBIDDEN);
            return;
        }

        // Early reject unknown HTTP methods
        if (hreq->GetRequestMethod() == HTTPRequest::UNKNOWN) {
            LogPrint(BCLog::HTTP, "HTTP request from %s rejected: Unknown HTTP request method\n",
                     hreq->GetPeer().ToString());
            hreq->WriteReply(HTTP_BAD_METHOD);
            return;
        }

        LogPrint(BCLog::HTTP, "Received a %s request for %s from %s\n",
                 RequestMethodString(hreq->GetRequestMethod()), SanitizeString(hreq->GetURI(), SAFE_CHARS_URI).substr(0, 100), hreq->GetPeer().ToString());

        // Find registered handler for prefix
        std::string strURI = hreq->GetURI();
        std::string path;
        std::vector<HTTPPathHandler>::const_iterator i = pathHandlers.begin();
        std::vector<HTTPPathHandler>::const_iterator iend = pathHandlers.end();
        for (; i != iend; ++i) {
            bool match = false;
            if (i->exactMatch)
                match = (strURI == i->prefix);
            else
                match = (strURI.substr(0, i->prefix.size()) == i->prefix);
            if (match) {
                path = strURI.substr(i->prefix.size());
                break;
            }
        }

        // Dispatch to worker thread
        if (i != iend) {
            std::unique_ptr<HTTPWorkItem> item(new HTTPWorkItem(std::move(hreq), path, i->handler));
            assert(g_work_queue);
            if (g_work_queue->Enqueue(item.get())) {
                item.release(); /* if true, queue took ownership */
            } else {
                LogPrintf("WARNING: request rejected because http work queue depth exceeded, it can be increased with the -rpcworkqueue= setting\n");
                item->req->WriteReply(HTTP_SERVICE_UNAVAILABLE, "Work queue depth exceeded");
            }
        } else {
            hreq->WriteReply(HTTP_NOT_FOUND);
        }
        */
}

/**
  | Callback to reject HTTP requests after
  | shutdown.
  |
  */
pub fn http_reject_request_cb(
        req: *mut libevent_sys::evhttp_request,
        _1:  *mut c_void)  {
    
    todo!();
        /*
            LogPrint(BCLog::HTTP, "Rejecting request while shutting down\n");
        evhttp_send_error(req, HTTP_SERVUNAVAIL, nullptr);
        */
}

/**
  | libevent_sys::event dispatcher thread
  |
  */
pub fn threadhttp(base: *mut libevent_sys::event_base) -> bool {
    
    todo!();
        /*
            util::ThreadRename("http");
        SetSyscallSandboxPolicy(SyscallSandboxPolicy::NET_HTTP_SERVER);
        LogPrint(BCLog::HTTP, "Entering http event loop\n");
        event_base_dispatch(base);
        // libevent_sys::event loop will be interrupted by InterruptHTTPServer()
        LogPrint(BCLog::HTTP, "Exited http event loop\n");
        return event_base_got_break(base) == 0;
        */
}

/**
  | Bind HTTP server to specified addresses
  |
  */
pub fn http_bind_addresses(http: *mut libevent_sys::evhttp) -> bool {
    
    todo!();
        /*
            uint16_t http_port{static_cast<uint16_t>(gArgs.GetIntArg("-rpcport", BaseParams().RPCPort()))};
        std::vector<std::pair<std::string, uint16_t>> endpoints;

        // Determine what addresses to bind to
        if (!(gArgs.IsArgSet("-rpcallowip") && gArgs.IsArgSet("-rpcbind"))) { // Default to loopback if not allowing external IPs
            endpoints.push_back(std::make_pair("::1", http_port));
            endpoints.push_back(std::make_pair("127.0.0.1", http_port));
            if (gArgs.IsArgSet("-rpcallowip")) {
                LogPrintf("WARNING: option -rpcallowip was specified without -rpcbind; this doesn't usually make sense\n");
            }
            if (gArgs.IsArgSet("-rpcbind")) {
                LogPrintf("WARNING: option -rpcbind was ignored because -rpcallowip was not specified, refusing to allow everyone to connect\n");
            }
        } else if (gArgs.IsArgSet("-rpcbind")) { // Specific bind address
            for (const std::string& strRPCBind : gArgs.GetArgs("-rpcbind")) {
                uint16_t port{http_port};
                std::string host;
                SplitHostPort(strRPCBind, port, host);
                endpoints.push_back(std::make_pair(host, port));
            }
        }

        // Bind addresses
        for (std::vector<std::pair<std::string, uint16_t> >::iterator i = endpoints.begin(); i != endpoints.end(); ++i) {
            LogPrint(BCLog::HTTP, "Binding RPC on address %s port %i\n", i->first, i->second);
            evhttp_bound_socket *bind_handle = evhttp_bind_socket_with_handle(http, i->first.empty() ? nullptr : i->first.c_str(), i->second);
            if (bind_handle) {
                CNetAddr addr;
                if (i->first.empty() || (LookupHost(i->first, addr, false) && addr.IsBindAny())) {
                    LogPrintf("WARNING: the RPC server is not safe to expose to untrusted networks such as the public internet\n");
                }
                boundSockets.push_back(bind_handle);
            } else {
                LogPrintf("Binding RPC on address %s port %i failed.\n", i->first, i->second);
            }
        }
        return !boundSockets.empty();
        */
}

/**
  | Simple wrapper to set thread name and
  | run work queue
  |
  */
pub fn http_work_queue_run(
        queue:      *mut WorkQueue<Box<dyn HTTPClosure>>,
        worker_num: i32)  {
    
    todo!();
        /*
            util::ThreadRename(strprintf("httpworker.%i", worker_num));
        SetSyscallSandboxPolicy(SyscallSandboxPolicy::NET_HTTP_SERVER_WORKER);
        queue->Run();
        */
}

/**
  | libevent event log callback
  |
  */
pub fn libevent_log_cb(
        severity: i32,
        msg:      *const u8)  {
    
    todo!();
        /*
            if (severity >= EVENT_LOG_WARN) // Log warn messages and higher without debug category
            LogPrintf("libevent: %s\n", msg);
        else
            LogPrint(BCLog::LIBEVENT, "libevent: %s\n", msg);
        */
}

/**
  | Initialize HTTP server.
  | 
  | Call this before RegisterHTTPHandler
  | or libevent_sys::event_base().
  |
  */
pub fn init_http_server() -> bool {
    
    todo!();
        /*
            if (!InitHTTPAllowList())
            return false;

        // Redirect libevent's logging to our own log
        event_set_log_callback(&libevent_log_cb);
        // Update libevent's log handling. Returns false if our version of
        // libevent doesn't support debug logging, in which case we should
        // clear the BCLog::LIBEVENT flag.
        if (!UpdateHTTPServerLogging(LogInstance().WillLogCategory(BCLog::LIBEVENT))) {
            LogInstance().DisableCategory(BCLog::LIBEVENT);
        }

    #ifdef WIN32
        evthread_use_windows_threads();
    #else
        evthread_use_pthreads();
    #endif

        raii_event_base base_ctr = obtain_event_base();

        /* Create a new evhttp object to handle requests. */
        raii_evhttp http_ctr = obtain_evhttp(base_ctr.get());
        struct evhttp* http = http_ctr.get();
        if (!http) {
            LogPrintf("couldn't create evhttp. Exiting.\n");
            return false;
        }

        evhttp_set_timeout(http, gArgs.GetIntArg("-rpcservertimeout", DEFAULT_HTTP_SERVER_TIMEOUT));
        evhttp_set_max_headers_size(http, MAX_HEADERS_SIZE);
        evhttp_set_max_body_size(http, MAX_SIZE);
        evhttp_set_gencb(http, http_request_cb, nullptr);

        if (!HTTPBindAddresses(http)) {
            LogPrintf("Unable to bind any endpoint for RPC server\n");
            return false;
        }

        LogPrint(BCLog::HTTP, "Initialized HTTP server\n");
        int workQueueDepth = std::max((long)gArgs.GetIntArg("-rpcworkqueue", DEFAULT_HTTP_WORKQUEUE), 1L);
        LogPrintf("HTTP: creating work queue of depth %d\n", workQueueDepth);

        g_work_queue = std::make_unique<WorkQueue<HTTPClosure>>(workQueueDepth);
        // transfer ownership to eventBase/HTTP via .release()
        eventBase = base_ctr.release();
        eventHTTP = http_ctr.release();
        return true;
        */
}

/**
  | Change logging level for libevent.
  | Removes BCLog::LIBEVENT from log categories
  | if libevent doesn't support debug logging.
  |
  */
pub fn update_http_server_logging(enable: bool) -> bool {
    
    todo!();
        /*
            #if LIBEVENT_VERSION_NUMBER >= 0x02010100
        if (enable) {
            event_enable_debug_logging(EVENT_DBG_ALL);
        } else {
            event_enable_debug_logging(EVENT_DBG_NONE);
        }
        return true;
    #else
        // Can't update libevent logging if version < 02010100
        return false;
    #endif
        */
}

lazy_static!{
    /*
    static std::thread g_thread_http;
    static std::vector<std::thread> g_thread_http_workers;
    */
}

/**
  | Start HTTP server.
  | 
  | This is separate from InitHTTPServer
  | to give users race-condition-free
  | time to register their handlers between
  | InitHTTPServer and StartHTTPServer.
  |
  */
pub fn start_http_server()  {
    
    todo!();
        /*
            LogPrint(BCLog::HTTP, "Starting HTTP server\n");
        int rpcThreads = std::max((long)gArgs.GetIntArg("-rpcthreads", DEFAULT_HTTP_THREADS), 1L);
        LogPrintf("HTTP: starting %d worker threads\n", rpcThreads);
        g_thread_http = std::thread(ThreadHTTP, eventBase);

        for (int i = 0; i < rpcThreads; i++) {
            g_thread_http_workers.emplace_back(HTTPWorkQueueRun, g_work_queue.get(), i);
        }
        */
}

/**
  | Interrupt HTTP server threads
  |
  */
pub fn interrupt_http_server()  {
    
    todo!();
        /*
            LogPrint(BCLog::HTTP, "Interrupting HTTP server\n");
        if (eventHTTP) {
            // Reject requests on current connections
            evhttp_set_gencb(eventHTTP, http_reject_request_cb, nullptr);
        }
        if (g_work_queue) {
            g_work_queue->Interrupt();
        }
        */
}

/**
  | Stop HTTP server
  |
  */
pub fn stop_http_server()  {
    
    todo!();
        /*
            LogPrint(BCLog::HTTP, "Stopping HTTP server\n");
        if (g_work_queue) {
            LogPrint(BCLog::HTTP, "Waiting for HTTP worker threads to exit\n");
            for (auto& thread : g_thread_http_workers) {
                thread.join();
            }
            g_thread_http_workers.clear();
        }
        // Unlisten sockets, these are what make the event loop running, which means
        // that after this and all connections are closed the event loop will quit.
        for (evhttp_bound_socket *socket : boundSockets) {
            evhttp_del_accept_socket(eventHTTP, socket);
        }
        boundSockets.clear();
        if (eventBase) {
            LogPrint(BCLog::HTTP, "Waiting for HTTP event thread to exit\n");
            if (g_thread_http.joinable()) g_thread_http.join();
        }
        if (eventHTTP) {
            evhttp_free(eventHTTP);
            eventHTTP = nullptr;
        }
        if (eventBase) {
            event_base_free(eventBase);
            eventBase = nullptr;
        }
        g_work_queue.reset();
        LogPrint(BCLog::HTTP, "Stopped HTTP server\n");
        */
}

/**
  | Return evhttp event base. This can be
  | used by submodules to queue timers or
  | custom events.
  |
  */
pub fn event_base() -> *mut libevent_sys::event_base {
    
    todo!();
        /*
            return eventBase;
        */
}

pub fn httpevent_callback_fn(
        _0:   libevent::EvutilSocket,
        _1:   i16,
        data: *mut c_void)  {
    
    todo!();
        /*
            // Static handler: simply call inner handler
        HTTPEvent *self = static_cast<HTTPEvent*>(data);
        self->handler();
        if (self->deleteWhenTriggered)
            delete self;
        */
}

impl Drop for HTTPEvent {
    fn drop(&mut self) {
        todo!();
        /*
            event_free(ev);
        */
    }
}

///--------------------------
impl Drop for HTTPRequest {
    fn drop(&mut self) {
        todo!();
        /*
            if (!replySent) {
            // Keep track of whether reply was sent to avoid request leaks
            LogPrintf("%s: Unhandled request\n", __func__);
            WriteReply(HTTP_INTERNAL_SERVER_ERROR, "Unhandled request");
        }
        // evhttpd cleans up the request, as long as a reply was sent.
        */
    }
}

impl HTTPRequest {

    pub fn new(
        req:        *mut libevent_sys::evhttp_request,
        reply_sent: Option<bool>) -> Self {

        let reply_sent: bool = reply_sent.unwrap_or(false);
    
        todo!();
        /*
        : req(_req),
        : reply_sent(_replySent),
        */
    }
    
    /**
      | Get the request header specified by
      | hdr, or an empty string.
      | 
      | Return a pair (isPresent,string).
      |
      */
    pub fn get_header(&self, hdr: &String) -> (bool,String) {
        
        todo!();
        /*
            const struct evkeyvalq* headers = evhttp_request_get_input_headers(req);
        assert(headers);
        const char* val = evhttp_find_header(headers, hdr.c_str());
        if (val)
            return std::make_pair(true, val);
        else
            return std::make_pair(false, "");
        */
    }
    
    /**
      | Read request body.
      | 
      | -----------
      | @note
      | 
      | As this consumes the underlying buffer,
      | call this only once.
      | 
      | Repeated calls will return an empty
      | string.
      |
      */
    pub fn read_body(&mut self) -> String {
        
        todo!();
        /*
            struct evbuffer* buf = evhttp_request_get_input_buffer(req);
        if (!buf)
            return "";
        size_t size = evbuffer_get_length(buf);
        /** Trivial implementation: if this is ever a performance bottleneck,
         * internal copying can be avoided in multi-segment buffers by using
         * evbuffer_peek and an awkward loop. Though in that case, it'd be even
         * better to not copy into an intermediate string but use a stream
         * abstraction to consume the evbuffer on the fly in the parsing algorithm.
         */
        const char* data = (const char*)evbuffer_pullup(buf, size);
        if (!data) // returns nullptr in case of empty buffer
            return "";
        std::string rv(data, size);
        evbuffer_drain(buf, size);
        return rv;
        */
    }
    
    /**
      | Write output header.
      | 
      | -----------
      | @note
      | 
      | call this before calling WriteErrorReply
      | or Reply.
      |
      */
    pub fn write_header(&mut self, 
        hdr:   &String,
        value: &String)  {
        
        todo!();
        /*
            struct evkeyvalq* headers = evhttp_request_get_output_headers(req);
        assert(headers);
        evhttp_add_header(headers, hdr.c_str(), value.c_str());
        */
    }

    /**
      | Closure sent to main thread to request
      | a reply to be sent to a HTTP request.
      | 
      | Replies must be sent in the main loop
      | in the main http thread, this cannot
      | be done from worker threads.
      |
      ------------------------
      | Write HTTP reply. nStatus is the HTTP
      | status code to send. strReply is the
      | body of the reply. Keep it empty to send
      | a standard message.
      | 
      | -----------
      | @note
      | 
      | Can be called only once. As this will
      | give the request back to the main thread,
      | do not call any other HTTPRequest methods
      | after calling this.
      |
      */
    pub fn write_reply(&mut self, 
        n_status:  i32,
        str_reply: Option<&str>)  {

        let str_reply: &str = str_reply.unwrap_or("");
        
        todo!();
        /*
            assert(!replySent && req);
        if (ShutdownRequested()) {
            WriteHeader("Connection", "close");
        }
        // Send event to main http thread to send reply message
        struct evbuffer* evb = evhttp_request_get_output_buffer(req);
        assert(evb);
        evbuffer_add(evb, strReply.data(), strReply.size());
        auto req_copy = req;
        HTTPEvent* ev = new HTTPEvent(eventBase, true, [req_copy, nStatus]{
            evhttp_send_reply(req_copy, nStatus, nullptr, nullptr);
            // Re-enable reading from the socket. This is the second part of the libevent
            // workaround above.
            if (event_get_version_number() >= 0x02010600 && event_get_version_number() < 0x02020001) {
                evhttp_connection* conn = evhttp_request_get_connection(req_copy);
                if (conn) {
                    bufferevent* bev = evhttp_connection_get_bufferevent(conn);
                    if (bev) {
                        bufferevent_enable(bev, EV_READ | EV_WRITE);
                    }
                }
            }
        });
        ev->trigger(nullptr);
        replySent = true;
        req = nullptr; // transferred back to main thread
        */
    }
    
    /**
      | Get CService (address:ip) for the origin
      | of the http request.
      |
      */
    pub fn get_peer(&self) -> Service {
        
        todo!();
        /*
            evhttp_connection* con = evhttp_request_get_connection(req);
        CService peer;
        if (con) {
            // evhttp retains ownership over returned address string
            const char* address = "";
            uint16_t port = 0;
            evhttp_connection_get_peer(con, (char**)&address, &port);
            peer = LookupNumeric(address, port);
        }
        return peer;
        */
    }
    
    /**
      | Get requested URI.
      |
      */
    pub fn geturi(&self) -> String {
        
        todo!();
        /*
            return evhttp_request_get_uri(req);
        */
    }
    
    /**
      | Get request method.
      |
      */
    pub fn get_request_method(&self) -> http_request::RequestMethod {
        
        todo!();
        /*
            switch (evhttp_request_get_command(req)) {
        case EVHTTP_REQ_GET:
            return GET;
            break;
        case EVHTTP_REQ_POST:
            return POST;
            break;
        case EVHTTP_REQ_HEAD:
            return HEAD;
            break;
        case EVHTTP_REQ_PUT:
            return PUT;
            break;
        default:
            return UNKNOWN;
            break;
        }
        */
    }
}

/**
  | Register handler for prefix.
  | 
  | If multiple handlers match a prefix,
  | the first-registered one will be invoked.
  |
  */
pub fn register_http_handler(
        prefix:      &String,
        exact_match: bool,
        handler:     &HTTPRequestHandler)  {
    
    todo!();
        /*
            LogPrint(BCLog::HTTP, "Registering HTTP handler for %s (exactmatch %d)\n", prefix, exactMatch);
        pathHandlers.push_back(HTTPPathHandler(prefix, exactMatch, handler));
        */
}

/**
  | Unregister handler for prefix
  |
  */
pub fn unregister_http_handler(
        prefix:      &String,
        exact_match: bool)  {
    
    todo!();
        /*
            std::vector<HTTPPathHandler>::iterator i = pathHandlers.begin();
        std::vector<HTTPPathHandler>::iterator iend = pathHandlers.end();
        for (; i != iend; ++i)
            if (i->prefix == prefix && i->exactMatch == exactMatch)
                break;
        if (i != iend)
        {
            LogPrint(BCLog::HTTP, "Unregistering HTTP handler for %s (exactmatch %d)\n", prefix, exactMatch);
            pathHandlers.erase(i);
        }
        */
}
