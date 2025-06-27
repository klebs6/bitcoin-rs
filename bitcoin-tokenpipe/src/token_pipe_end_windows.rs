crate::ix!();

#[cfg(windows)]
impl TokenPipeEnd {
    
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
