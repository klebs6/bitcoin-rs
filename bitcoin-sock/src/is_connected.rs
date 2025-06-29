crate::ix!();

impl Sock {
    
    pub fn is_connected(&self, errmsg: &mut String) -> bool {
        
        todo!();
        /*
            if (m_socket == INVALID_SOCKET) {
            errmsg = "not connected";
            return false;
        }

        char c;
        switch (Recv(&c, sizeof(c), MSG_PEEK)) {
        case -1: {
            const int err = WSAGetLastError();
            if (IOErrorIsPermanent(err)) {
                errmsg = NetworkErrorString(err);
                return false;
            }
            return true;
        }
        case 0:
            errmsg = "closed";
            return false;
        default:
            return true;
        }
        */
    }
}
