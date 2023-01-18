crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/events.h]

macro_rules! make_raii {
    ($type:ident) => {
        /*
        
        /* deleter */
        struct type##_deleter {
            c_void operator()(struct type* ob) {
                type##_free(ob);
            }
        };
        /* unique ptr typedef */
        typedef std::unique_ptr<struct type, type##_deleter> raii_##type
        */
    }
}

make_raii!{ event_base }
make_raii!{ event }
make_raii!{ evhttp }
make_raii!{ evhttp_request }
make_raii!{ evhttp_connection }

/// TODO: remove the generic.
///
/// just here to placate the compiler
///
#[inline] pub fn obtain_event_base<event_base>() -> Box<event_base> {
    
    todo!();
        /*
            auto result = raii_event_base(event_base_new());
        if (!result.get())
            throw std::runtime_error("cannot create event_base");
        return result;
        */
}

/// TODO: remove the generic.
///
/// just here to placate the compiler
///
#[inline] pub fn obtain_event<event_base,event_callback_fn>(
        _base:   *mut event_base,
        _s:      EvutilSocket,
        _events: i16,
        _cb:     event_callback_fn,
        _arg:    *mut c_void) -> Box<event> {
    
    todo!();
        /*
            return raii_event(event_new(base, s, events, cb, arg));
        */
}

#[inline] pub fn obtain_evhttp<event_base, evhttp>(_base: *mut event_base) -> Box<evhttp> {
    
    todo!();
        /*
            return raii_evhttp(evhttp_new(base));
        */
}

#[inline] pub fn obtain_evhttp_request<evhttp_request>(
        _cb:  fn(_0: *mut evhttp_request, _1: *mut c_void) -> (),
        _arg: *mut c_void) -> Box<evhttp_request> {
    
    todo!();
        /*
            return raii_evhttp_request(evhttp_request_new(cb, arg));
        */
}

#[inline] pub fn obtain_evhttp_connection_base<event_base,evhttp_connection>(
        _base: *mut event_base,
        _host: &str,
        _port: u16) -> Box<evhttp_connection> {
    
    todo!();
        /*
            auto result = raii_evhttp_connection(evhttp_connection_base_new(base, nullptr, host.c_str(), port));
        if (!result.get())
            throw std::runtime_error("create connection failed");
        return result;
        */
}
