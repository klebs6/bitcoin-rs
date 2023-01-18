crate::ix!();

pub struct ConnmanListenSocket {
    pub socket:      CSocket,
    pub permissions: NetPermissionFlags,
}

impl ConnmanListenSocket {

    #[inline] pub fn add_socket_permission_flags(&self, flags: &mut NetPermissionFlags)  {
        
        NetPermissions::add_flag(flags, self.permissions);
    }
    
    pub fn new(
        socket:      CSocket,
        permissions: NetPermissionFlags) -> Self {

        Self {
            socket,
            permissions
        }
    }
}
