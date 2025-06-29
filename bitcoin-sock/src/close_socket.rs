crate::ix!();

/**
  | Close socket and set hSocket to INVALID_SOCKET
  |
  */
pub fn close_socket(h_socket: &mut CSocket) -> bool {
    
    todo!();
        /*
            if (hSocket == INVALID_SOCKET)
            return false;

    #ifdef WIN32
        int ret = closesocket(hSocket);
    #else
        int ret = close(hSocket);
    #endif

        if (ret) {
            LogPrintf("Socket close failed: %d. Error: %s\n", hSocket, NetworkErrorString(WSAGetLastError()));
        }
        hSocket = INVALID_SOCKET;
        return ret != SOCKET_ERROR;
        */
}
