crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/tokenpipe.h]

/**
  | One end of a token pipe.
  |
  */
#[cfg(not(WIN32))]
pub struct TokenPipeEnd {
    fd: i32, // default = -1
}

/**
  | Return value constants for TokenWrite
  | and TokenRead.
  |
  */
#[cfg(not(WIN32))]
pub enum TokenPipeEndStatus {

    /**
      | I/O error
      |
      */
    TS_ERR = -1, 

    /**
      | Unexpected end of stream
      |
      */
    TS_EOS = -2, 
}

#[cfg(not(WIN32))]
impl TokenPipeEnd {

    pub fn new(fd: Option<i32>) -> Self {
        let fd: i32 = fd.unwrap_or(-1);
        todo!();
        /*


        
        */
    }

    /**
      | Write token to endpoint.
      | 
      | 
      | -----------
      | @return
      | 
      | 0 If successful. <0 if error:
      | 
      | TS_ERR If an error happened.
      | 
      | TS_EOS If end of stream happened.
      |
      */
    pub fn token_write(&mut self, token: u8) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Read token from endpoint.
      | 
      | 
      | -----------
      | @return
      | 
      | >=0 Token value, if successful. <0 if
      | error:
      | 
      | TS_ERR If an error happened.
      | 
      | TS_EOS If end of stream happened.
      |
      */
    pub fn token_read(&mut self) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Explicit close function.
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Return whether endpoint is open.
      |
      */
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return m_fd != -1;
        */
    }

    /**
      | Move-only class.
      |
      */
    pub fn new_from_other(other: TokenPipeEnd) -> Self {
    
        todo!();
        /*


            m_fd = other.m_fd;
            other.m_fd = -1;
        */
    }
    
    pub fn assign_from(&mut self, other: TokenPipeEnd) -> &mut TokenPipeEnd {
        
        todo!();
        /*
            Close();
            m_fd = other.m_fd;
            other.m_fd = -1;
            return *this;
        */
    }
}

/**
  | An interprocess or interthread pipe
  | for sending tokens (one-byte values)
  | over.
  |
  */
#[cfg(not(WIN32))]
pub struct TokenPipe {
    fds: [i32; 2], // default = {-1, -1}
}

#[cfg(not(WIN32))]
impl TokenPipe {
    
    pub fn new(fds: [i32; 2]) -> Self {
    
        todo!();
        /*


            : m_fds{fds[0], fds[1]}
        */
    }

    /**
      | Create a new pipe.
      | 
      | -----------
      | @return
      | 
      | The created TokenPipe, or an empty std::nullopt
      | in case of error.
      |
      */
    pub fn make() -> Option<TokenPipe> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Take the read end of this pipe. This can
      | only be called once, as the object will
      | be moved out.
      |
      */
    pub fn take_read_end(&mut self) -> TokenPipeEnd {
        
        todo!();
        /*
        
        */
    }

    /**
      | Take the write end of this pipe. This
      | should only be called once, as the object
      | will be moved out.
      |
      */
    pub fn take_write_end(&mut self) -> TokenPipeEnd {
        
        todo!();
        /*
        
        */
    }

    /**
      | Close and end of the pipe that hasn't
      | been moved out.
      |
      */
    pub fn close(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Move-only class.
      |
      */
    pub fn new_from_other(other: TokenPipe) -> Self {
    
        todo!();
        /*


            for (int i = 0; i < 2; ++i) {
                m_fds[i] = other.m_fds[i];
                other.m_fds[i] = -1;
            }
        */
    }
    
    pub fn assign_from(&mut self, other: TokenPipe) -> &mut TokenPipe {
        
        todo!();
        /*
            Close();
            for (int i = 0; i < 2; ++i) {
                m_fds[i] = other.m_fds[i];
                other.m_fds[i] = -1;
            }
            return *this;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/util/tokenpipe.cpp]
#[cfg(WIN32)]
impl TokenPipe {

    pub fn take_read_end(&mut self) -> TokenPipeEnd {
        
        todo!();
        /*
            TokenPipeEnd res(m_fds[0]);
        m_fds[0] = -1;
        return res;
        */
    }
    
    pub fn take_write_end(&mut self) -> TokenPipeEnd {
        
        todo!();
        /*
            TokenPipeEnd res(m_fds[1]);
        m_fds[1] = -1;
        return res;
        */
    }
}

///---------------------------
#[cfg(WIN32)]
impl Drop for TokenPipeEnd {
    fn drop(&mut self) {
        todo!();
        /*
            Close();
        */
    }
}

#[cfg(WIN32)]
impl TokenPipeEnd {
    
    pub fn new(fd: i32) -> Self {
    
        todo!();
        /*
        : fd(fd),

        
        */
    }
    
    pub fn token_write(&mut self, token: u8) -> i32 {
        
        todo!();
        /*
        while (true) {
            ssize_t result = write(m_fd, &token, 1);
            if (result < 0) {
                // Failure. It's possible that the write was interrupted by a signal,
                // in that case retry.
                if (errno != EINTR) {
                    return TS_ERR;
                }
            } else if (result == 0) {
                return TS_EOS;
            } else { // ==1
                return 0;
            }
        }
        */
    }
    
    pub fn token_read(&mut self) -> i32 {
        
        todo!();
        /*
            uint8_t token;
        while (true) {
            ssize_t result = read(m_fd, &token, 1);
            if (result < 0) {
                // Failure. Check if the read was interrupted by a signal,
                // in that case retry.
                if (errno != EINTR) {
                    return TS_ERR;
                }
            } else if (result == 0) {
                return TS_EOS;
            } else { // ==1
                return token;
            }
        }
        return token;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (m_fd != -1) close(m_fd);
        m_fd = -1;
        */
    }
}

#[cfg(WIN32)]
impl Drop for TokenPipe {
    fn drop(&mut self) {
        todo!();
        /*
            Close();
        */
    }
}

#[cfg(WIN32)]
impl TokenPipe {
    
    pub fn make(&mut self) -> Option<TokenPipe> {
        
        todo!();
        /*
            int fds[2] = {-1, -1};
    #if HAVE_O_CLOEXEC && HAVE_DECL_PIPE2
        if (pipe2(fds, O_CLOEXEC) != 0) {
            return std::nullopt;
        }
    #else
        if (pipe(fds) != 0) {
            return std::nullopt;
        }
    #endif
        return TokenPipe(fds);
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            if (m_fds[0] != -1) close(m_fds[0]);
        if (m_fds[1] != -1) close(m_fds[1]);
        m_fds[0] = m_fds[1] = -1;
        */
    }
}

