crate::ix!();

///---------------------
pub type ScopedConnection = Broken;

pub struct HandlerImpl {
    connection: ScopedConnection,
}

impl Handler for HandlerImpl {}

impl Disconnect for HandlerImpl {
    fn disconnect(&mut self)  {
        
        todo!();
        /*
            m_connection.disconnect();
        */
    }
}

impl HandlerImpl {

    pub fn new(connection: Connection) -> Self {
    
        todo!();
        /*
        : connection(std::move(connection)),

        
        */
    }
}

///---------------------
pub struct CleanupHandler {
    cleanup: fn() -> (),
}

impl Handler for CleanupHandler {}

impl Disconnect for CleanupHandler {
    fn disconnect(&mut self)  {
        
        todo!();
        /*
            if (!m_cleanup) return; m_cleanup(); m_cleanup = nullptr;
        */
    }
}

impl Drop for CleanupHandler {
    fn drop(&mut self) {
        todo!();
        /*
            if (!m_cleanup) return; m_cleanup(); m_cleanup = nullptr;
        */
    }
}

impl CleanupHandler {

    pub fn new(cleanup: fn() -> ()) -> Self {
    
        todo!();
        /*
        : cleanup(std::move(cleanup)),

        
        */
    }
}

