// ---------------- [ File: bitcoin-netmsg/src/netmessagemaker.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/netmessagemaker.h]

#[derive(Default)]
#[no_copy]
pub struct SerializedNetMsg {
    pub data: Vec<u8>,
    pub ty:   String,
}

pub struct NetMsgMaker {
    n_version: i32,
}

impl NetMsgMaker {

    pub fn new(n_version_in: i32) -> Self {
    
        todo!();
        /*
        : n_version(nVersionIn),

        
        */
    }
    
    pub fn make_with_flags(&self, 
        n_flags:  i32,
        msg_type: &str,
        args:     &[&dyn Any]) -> SerializedNetMsg {
    
        todo!();
        /*
            CSerializedNetMsg msg;
            msg.m_type = std::move(msg_type);
            CVectorWriter{ SER_NETWORK, nFlags | nVersion, msg.data, 0, std::forward<Args>(args)... };
            return msg;
        */
    }
    
    pub fn make(&self, 
        msg_type: &str,
        args:     &[&dyn Any]) -> SerializedNetMsg {
    
        todo!();
        /*
            return Make(0, std::move(msg_type), std::forward<Args>(args)...);
        */
    }
}
