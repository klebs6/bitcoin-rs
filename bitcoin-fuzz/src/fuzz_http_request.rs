// ---------------- [ File: bitcoin-fuzz/src/fuzz_http_request.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/http_request.cpp]

/**
  | workaround for libevent versions before
  | 2.1.1, when internal functions didn't
  | have underscores at the end
  |
  */
#[cfg(LIBEVENT_VERSION_NUMBER_LT_0x02010100)]
extern "C" {
    #[inline] pub fn evhttp_parse_firstline(
            r: *mut libevent_sys::ev_http_request,
            b: *mut libevent_sys::evbuffer) -> i32 {
        
        todo!();
            /*
                return evhttp_parse_firstline(r, b);
            */
    }

    #[inline] pub fn evhttp_parse_headers(
            r: *mut libevent_sys::evhttp_request,
            b: *mut libevent_sys::evbuffer) -> i32 {
        
        todo!();
            /*
                return evhttp_parse_headers(r, b);
            */
    }
}

pub fn request_method_string(m: http_request::RequestMethod) -> String {
    
    todo!();
        /*
        
        */
}

#[fuzz_test] fn http_request() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        evhttp_request* evreq = evhttp_request_new(nullptr, nullptr);
        assert(evreq != nullptr);
        evreq->kind = EVHTTP_REQUEST;
        evbuffer* evbuf = evbuffer_new();
        assert(evbuf != nullptr);
        const std::vector<uint8_t> http_buffer = ConsumeRandomLengthByteVector(fuzzed_data_provider, 4096);
        evbuffer_add(evbuf, http_buffer.data(), http_buffer.size());
        // Avoid constructing requests that will be interpreted by libevent as PROXY requests to avoid triggering
        // a nullptr dereference. The dereference (req->evcon->http_server) takes place in evhttp_parse_request_line
        // and is a consequence of our hacky but necessary use of the internal function evhttp_parse_firstline_ in
        // this fuzzing harness. The workaround is not aesthetically pleasing, but it successfully avoids the troublesome
        // code path. " http:// HTTP/1.1\n" was a crashing input prior to this workaround.
        const std::string http_buffer_str = ToLower({http_buffer.begin(), http_buffer.end()});
        if (http_buffer_str.find(" http://") != std::string::npos || http_buffer_str.find(" https://") != std::string::npos ||
            evhttp_parse_firstline_(evreq, evbuf) != 1 || evhttp_parse_headers_(evreq, evbuf) != 1) {
            evbuffer_free(evbuf);
            evhttp_request_free(evreq);
            return;
        }

        HTTPRequest http_request{evreq, true};
        const HTTPRequest::RequestMethod request_method = http_request.GetRequestMethod();
        (c_void)RequestMethodString(request_method);
        (c_void)http_request.GetURI();
        (c_void)http_request.GetHeader("Host");
        const std::string header = fuzzed_data_provider.ConsumeRandomLengthString(16);
        (c_void)http_request.GetHeader(header);
        (c_void)http_request.WriteHeader(header, fuzzed_data_provider.ConsumeRandomLengthString(16));
        (c_void)http_request.GetHeader(header);
        const std::string body = http_request.ReadBody();
        assert(body.empty());
        const CService service = http_request.GetPeer();
        assert(service.ToString() == "[::]:0");

        evbuffer_free(evbuf);
        evhttp_request_free(evreq);

    */
}
