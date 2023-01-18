crate::ix!();

#[derive(Default)]
pub struct NetCleanup {


}

impl Drop for NetCleanup {

    fn drop(&mut self) {
        todo!();
        /*
            #ifdef WIN32
            // Shutdown Windows Sockets
            WSACleanup();
    #endif
        */
    }
}

lazy_static!{
    /*
    static CNetCleanup instance_of_cnetcleanup;
    */
}

