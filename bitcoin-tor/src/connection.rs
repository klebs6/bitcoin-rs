crate::ix!();

/****** Low-level TorControlConnection ********/

/**
  | Low-level handling for Tor control
  | connection.
  | 
  | Speaks the SMTP-like protocol as defined
  | in torspec/control-spec.txt
  |
  */
pub struct TorControlConnection {

    /**
      | Response handlers for async replies
      |
      */
    async_handler:  Signal<fn(_0: &mut TorControlConnection, _1: &TorControlReply) -> ()>,

    /**
      | Callback when ready for use
      |
      */
    connected:      fn(_0: &mut TorControlConnection) -> (),

    /**
      | Callback when connection lost
      |
      */
    disconnected:   fn(_0: &mut TorControlConnection) -> (),

    /**
      | Libevent event base
      |
      */
    base:           *mut libevent_sys::event_base,

    /**
      | Connection to control socket
      |
      */
    conn:           *mut libevent_sys::bufferevent,

    /**
      | Message being received
      |
      */
    message:        TorControlReply,

    /**
      | Response handlers
      |
      */
    reply_handlers: VecDeque<tor_control_connection::ReplyHandlerCB>,
}

pub mod tor_control_connection {

    use super::*;

    pub type ConnectionCB   = fn(_0: &mut TorControlConnection) -> ();
    pub type ReplyHandlerCB = fn(_0: &mut TorControlConnection, _1: &TorControlReply) -> ();
}

impl Drop for TorControlConnection {

    fn drop(&mut self) {
        todo!();
        /*
            if (b_conn)
            bufferevent_free(b_conn);
        */
    }
}

impl TorControlConnection {

    /**
      | Create a new TorControlConnection.
      |
      */
    pub fn new(base: *mut libevent_sys::event_base) -> Self {
    
        todo!();
        /*
        : base(_base),
        : conn(nullptr),
        */
    }
    
    /**
      | Libevent handlers: internal
      |
      */
    pub fn readcb(&mut self, 
        bev: *mut libevent_sys::bufferevent,
        ctx: *mut c_void)  {
        
        todo!();
        /*
            TorControlConnection *self = static_cast<TorControlConnection*>(ctx);
        struct evbuffer *input = bufferevent_get_input(bev);
        size_t n_read_out = 0;
        char *line;
        assert(input);
        //  If there is not a whole line to read, evbuffer_readln returns nullptr
        while((line = evbuffer_readln(input, &n_read_out, EVBUFFER_EOL_CRLF)) != nullptr)
        {
            std::string s(line, n_read_out);
            free(line);
            if (s.size() < 4) // Short line
                continue;
            // <status>(-|+| )<data><CRLF>
            self->message.code = LocaleIndependentAtoi<int>(s.substr(0,3));
            self->message.lines.push_back(s.substr(4));
            char ch = s[3]; // '-','+' or ' '
            if (ch == ' ') {
                // Final line, dispatch reply and clean up
                if (self->message.code >= 600) {
                    // Dispatch async notifications to async handler
                    // Synchronous and asynchronous messages are never interleaved
                    self->async_handler(*self, self->message);
                } else {
                    if (!self->reply_handlers.empty()) {
                        // Invoke reply handler with message
                        self->reply_handlers.front()(*self, self->message);
                        self->reply_handlers.pop_front();
                    } else {
                        LogPrint(BCLog::TOR, "tor: Received unexpected sync reply %i\n", self->message.code);
                    }
                }
                self->message.Clear();
            }
        }
        //  Check for size of buffer - protect against memory exhaustion with very long lines
        //  Do this after evbuffer_readln to make sure all full lines have been
        //  removed from the buffer. Everything left is an incomplete line.
        if (evbuffer_get_length(input) > MAX_LINE_LENGTH) {
            LogPrintf("tor: Disconnecting because MAX_LINE_LENGTH exceeded\n");
            self->Disconnect();
        }
        */
    }
    
    pub fn eventcb(&mut self, 
        bev:  *mut libevent_sys::bufferevent,
        what: i16,
        ctx:  *mut c_void)  {
        
        todo!();
        /*
            TorControlConnection *self = static_cast<TorControlConnection*>(ctx);
        if (what & BEV_EVENT_CONNECTED) {
            LogPrint(BCLog::TOR, "tor: Successfully connected!\n");
            self->connected(*self);
        } else if (what & (BEV_EVENT_EOF|BEV_EVENT_ERROR)) {
            if (what & BEV_EVENT_ERROR) {
                LogPrint(BCLog::TOR, "tor: Error connecting to Tor control socket\n");
            } else {
                LogPrint(BCLog::TOR, "tor: End of stream\n");
            }
            self->Disconnect();
            self->disconnected(*self);
        }
        */
    }
    
    /**
      | Connect to a Tor control port. tor_control_center
      | is address of the form host:port. connected
      | is the handler that is called when connection
      | is successfully established. disconnected
      | is a handler that is called when the connection
      | is broken.
      | 
      | Return true on success.
      |
      */
    pub fn connect(&mut self, 
        tor_control_center: &String,
        connected:          &tor_control_connection::ConnectionCB,
        disconnected:       &tor_control_connection::ConnectionCB) -> bool {
        
        todo!();
        /*
            if (b_conn) {
            Disconnect();
        }

        CService control_service;
        if (!Lookup(tor_control_center, control_service, 9051, fNameLookup)) {
            LogPrintf("tor: Failed to look up control center %s\n", tor_control_center);
            return false;
        }

        struct sockaddr_storage control_address;
        socklen_t control_address_len = sizeof(control_address);
        if (!control_service.GetSockAddr(reinterpret_cast<struct sockaddr*>(&control_address), &control_address_len)) {
            LogPrintf("tor: Error parsing socket address %s\n", tor_control_center);
            return false;
        }

        // Create a new socket, set up callbacks and enable notification bits
        b_conn = bufferevent_socket_new(base, -1, BEV_OPT_CLOSE_ON_FREE);
        if (!b_conn) {
            return false;
        }
        bufferevent_setcb(b_conn, TorControlConnection::readcb, nullptr, TorControlConnection::eventcb, this);
        bufferevent_enable(b_conn, EV_READ|EV_WRITE);
        this->connected = _connected;
        this->disconnected = _disconnected;

        // Finally, connect to tor_control_center
        if (bufferevent_socket_connect(b_conn, reinterpret_cast<struct sockaddr*>(&control_address), control_address_len) < 0) {
            LogPrintf("tor: Error connecting to address %s\n", tor_control_center);
            return false;
        }
        return true;
        */
    }
    
    /**
      | Disconnect from Tor control port.
      |
      */
    pub fn disconnect(&mut self)  {
        
        todo!();
        /*
            if (b_conn)
            bufferevent_free(b_conn);
        b_conn = nullptr;
        */
    }
    
    /**
      | Send a command, register a handler for
      | the reply.
      | 
      | A trailing CRLF is automatically added.
      | 
      | Return true on success.
      |
      */
    pub fn command(&mut self, 
        cmd:           &String,
        reply_handler: &tor_control_connection::ReplyHandlerCB) -> bool {
        
        todo!();
        /*
            if (!b_conn)
            return false;
        struct evbuffer *buf = bufferevent_get_output(b_conn);
        if (!buf)
            return false;
        evbuffer_add(buf, cmd.data(), cmd.size());
        evbuffer_add(buf, "\r\n", 2);
        reply_handlers.push_back(reply_handler);
        return true;
        */
    }
}
