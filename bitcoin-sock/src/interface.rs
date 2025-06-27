crate::ix!();

pub trait SockInterface:
    SockGet
    + SockRelease
    + Reset
    + SockSend
    + SockRecv
    + SockConnect
    + SockGetSockOpt
    + SockWait 

    /* Higher level, convenience, methods. These may throw. */
    + SockSendComplete
    + SockRecvUntilTerminator
    + SockIsConnected {}

pub trait SockGet {

    /**
      | Get the value of the contained socket.
      | 
      | 
      | -----------
      | @return
      | 
      | socket or INVALID_SOCKET if empty
      |
      */
    fn get(&self) -> CSocket;
}

pub trait SockRelease {

    /**
      | Get the value of the contained socket
      | and drop ownership. It will not be closed
      | by the destructor after this call.
      | 
      | 
      | -----------
      | @return
      | 
      | socket or INVALID_SOCKET if empty
      |
      */
    fn release(&mut self) -> CSocket;
}

pub trait Reset {

    /**
      | Close if non-empty.
      |
      */
    fn reset(&mut self);
}

pub trait SockSend {

    /**
      | send(2) wrapper. Equivalent to `send(this->Get(),
      | data, len, flags);`. Code that uses
      | this wrapper can be unit tested if this
      | method is overridden by a mock Sock implementation.
      |
      */
    fn send(&self, 
            data:  *const c_void,
            len:   usize,
            flags: i32) -> isize;
}

pub trait SockRecv {

    /**
      | recv(2) wrapper. Equivalent to `recv(this->Get(),
      | buf, len, flags);`. Code that uses this
      | wrapper can be unit tested if this method
      | is overridden by a mock Sock implementation.
      |
      */
    fn recv(&self, 
            buf:   *mut c_void,
            len:   usize,
            flags: i32) -> isize;
}

pub trait SockConnect {

    /**
      | connect(2) wrapper. Equivalent to
      | `connect(this->Get(), addr, addrlen)`.
      | Code that uses this wrapper can be unit
      | tested if this method is overridden
      | by a mock Sock implementation.
      |
      */
    fn connect(&self, 
            addr:     *const SocketAddr,
            addr_len: libc::socklen_t) -> i32;
}

pub trait SockGetSockOpt {

    /**
      | getsockopt(2) wrapper. Equivalent
      | to `getsockopt(this->Get(), level,
      | opt_name, opt_val, opt_len)`. Code
      | that uses this wrapper can be unit tested
      | if this method is overridden by a mock
      | Sock implementation.
      |
      */
    fn get_sock_opt(&self, 
            level:    i32,
            opt_name: i32,
            opt_val:  *mut c_void,
            opt_len:  *mut libc::socklen_t) -> i32;
}

pub trait SockWait {

    /**
      | Wait for readiness for input (recv)
      | or output (send).
      | 
      | -----------
      | @param[in] timeout
      | 
      | Wait this much for at least one of the
      | requested events to occur.
      | ----------
      | @param[in] requested
      | 
      | Wait for those events, bitwise-or of
      | `RECV` and `SEND`.
      | ----------
      | @param[out] occurred
      | 
      | If not nullptr and `true` is returned,
      | then upon return this indicates which
      | of the requested events occurred. A
      | timeout is indicated by return value
      | of `true` and `occurred` being set to
      | 0.
      | 
      | -----------
      | @return
      | 
      | true on success and false otherwise
      |
      */
    fn wait(&self, 
            timeout:   Instant /* millis */,
            requested: SockEvent,
            occurred:  *mut SockEvent) -> bool;
}

pub trait SockSendComplete {

    /**
      | Send the given data, retrying on transient
      | errors.
      | 
      | -----------
      | @param[in] data
      | 
      | Data to send.
      | ----------
      | @param[in] timeout
      | 
      | Timeout for the entire operation.
      | ----------
      | @param[in] interrupt
      | 
      | If this is signaled then the operation
      | is canceled. @throws std::runtime_error
      | if the operation cannot be completed.
      | In this case only some of the data will
      | be written to the socket.
      |
      */
    fn send_complete(&self, 
            data:      &String,
            timeout:   Instant /* millis */,
            interrupt: &mut ThreadInterrupt);
}

pub trait SockRecvUntilTerminator {

    /**
      | Read from socket until a terminator
      | character is encountered. Will never
      | consume bytes past the terminator from
      | the socket.
      | 
      | -----------
      | @param[in] terminator
      | 
      | Character up to which to read from the
      | socket.
      | ----------
      | @param[in] timeout
      | 
      | Timeout for the entire operation.
      | ----------
      | @param[in] interrupt
      | 
      | If this is signaled then the operation
      | is canceled.
      | ----------
      | @param[in] max_data
      | 
      | The maximum amount of data (in bytes)
      | to receive. If this many bytes are received
      | and there is still no terminator, then
      | this method will throw an exception.
      | 
      | -----------
      | @return
      | 
      | The data that has been read, without
      | the terminating character. @throws
      | std::runtime_error if the operation
      | cannot be completed. In this case some
      | bytes may have been consumed from the
      | socket.
      |
      */
    fn recv_until_terminator(&self, 
            terminator: u8,
            timeout:    Instant /* millis */,
            interrupt:  &mut ThreadInterrupt,
            max_data:   usize) -> String;
}

pub trait SockIsConnected {

    /**
      | Check if still connected.
      | 
      | -----------
      | @param[out] errmsg
      | 
      | The error string, if the socket has been
      | disconnected.
      | 
      | -----------
      | @return
      | 
      | true if connected
      |
      */
    fn is_connected(&self, errmsg: &mut String) -> bool;
}
