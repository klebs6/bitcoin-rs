crate::ix!();

/**
  | command, total bytes
  |
  */
pub type MapMsgCmdSize = HashMap<String,u64>;

/**
  | Transport protocol agnostic message
  | container.
  | 
  | Ideally it should only contain receive
  | time, payload, command and size.
  |
  */
pub struct NetMessage {

    /**
      | received message data
      |
      */
    pub recv:             DataStream,

    /**
      | time of message receipt
      |
      */
    pub time:             Option<OffsetDateTime>, /* micros */

    /**
      | size of the payload
      |
      */
    pub message_size:     u32, // default = { 0 }

    /**
      | used wire size of the message (including
      | header/checksum)
      |
      */
    pub raw_message_size: u32, // default = { 0 }

    pub command:          String,
}

impl NetMessage {

    pub fn new(recv_in: DataStream) -> Self {
    
        todo!();
        /*
        : recv(std::move(recv_in)),

        
        */
    }
    
    pub fn set_version(&mut self, n_version_in: i32)  {
        
        todo!();
        /*
            m_recv.SetVersion(nVersionIn);
        */
    }
}


