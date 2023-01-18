crate::ix!();

/**
  | Reply structure for request_done to
  | fill in
  |
  */
pub struct HTTPReply {
    pub status: i32,
    pub error:  i32,
    pub body:   String,
}

impl Default for HTTPReply {
    
    fn default() -> Self {
        Self {
            status:  0,
            error:  -1,
            body:   String::new(),
        }
    }
}

pub fn http_errorstring(code: i32) -> &'static str {

    #[cfg(not(LIBEVENT_VERSION_NUMBER_GTE_0x02010300))]
    {
        return "unknown";
    }

    match code {
        EVREQ_HTTP_TIMEOUT        => { return "timeout reached"; },
        EVREQ_HTTP_EOF            => { return "EOF reached"; },
        EVREQ_HTTP_INVALID_HEADER => { return "error while reading header, or invalid header"; },
        EVREQ_HTTP_BUFFER_ERROR   => { return "error encountered while reading or writing"; },
        EVREQ_HTTP_REQUEST_CANCEL => { return "request was canceled"; },
        EVREQ_HTTP_DATA_TOO_LONG  => { return "response body is larger than allowed"; },
        _                         => { return "unknown"; },
    }
}

pub fn http_request_done(
        req: *mut evhttp_request,
        ctx: *mut c_void)  {
    
    let reply: *mut HTTPReply = ctx as *mut HTTPReply;

    if req == std::ptr::null_mut() {

        /*
          | If req is nullptr, it means an error occurred
          | while connecting: the error code will
          | have been passed to http_error_cb.
          |
          */
        unsafe {
            (*reply).status = 0;
        }

        return;
    }

    unsafe {
        (*reply).status = 
            evhttp_request_get_response_code(req)
    };

    let buf: *mut evbuffer = unsafe {
        evhttp_request_get_input_buffer(req)
    };

    if buf != null_mut() {

        let size: usize = unsafe { 
            evbuffer_get_length(buf)
                .try_into()
                .unwrap()
        };

        let data: *mut i8 
        = unsafe { 
            evbuffer_pullup(
                buf,
                size.try_into().unwrap()
            ) as *mut i8
        };

        if data != null_mut() {

            unsafe { 

                (*reply).body = {

                    let slice : &[u8] = unsafe{ std::slice::from_raw_parts(data as *const u8, size) };

                    std::ffi::CStr::from_bytes_with_nul(slice)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                };
            }
        }

        unsafe {
            evbuffer_drain(buf, size.try_into().unwrap());
        }
    }
}

#[cfg(LIBEVENT_VERSION_NUMBER_GTE_0x02010300)]
pub fn http_error_cb(
        err: HttpRequestError,
        ctx: *mut c_void)  {
    
    let reply: *mut HTTPReply = ctx as *mut HTTPReply;

    (*reply).error = err;
}
