crate::ix!();

#[cfg(windows)]
impl Drop for TokenPipe {
    fn drop(&mut self) {
        todo!();
        /*
            Close();
        */
    }
}

#[cfg(windows)]
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
