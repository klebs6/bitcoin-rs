// ---------------- [ File: bitcoin-net/src/interface.rs ]
crate::ix!();

pub trait Complete {

    /**
      | returns true if the current deserialization
      | is complete
      |
      */
    fn complete(&self) -> bool;
}

pub trait SetVersion {

    /**
      | set the serialization context version
      |
      */
    fn set_version(&mut self, version: i32);
}

pub trait ReadData {

    /**
      | read and deserialize data, advances
      | msg_bytes data pointer
      |
      */
    fn read(&mut self, msg_bytes: &mut [u8]) -> i32;
}

pub trait GetMessage {

    /**
      | decomposes a message from the context
      |
      */
    fn get_message(&mut self, 
            time:    Instant /* microseconds */,
            out_err: &mut u32) -> Option<NetMessage>;

}

/**
  | The TransportDeserializer takes care
  | of holding and deserializing the network
  | receive buffer. It can deserialize
  | the network buffer into a transport
  | protocol agnostic CNetMessage (command
  | & payload)
  |
  */
pub trait TransportDeserializer: 
Complete 
+ SetVersion 
+ ReadData 
+ GetMessage { }

/**
  | The TransportSerializer prepares
  | messages for the network transport
  |
  */
pub trait TransportSerializer: PrepareForTransport { }

pub trait PrepareForTransport {

    /**
      | prepare message for transport (header
      | construction, error-correction computation,
      | payload encryption, etc.)
      |
      */
    fn prepare_for_transport(&mut self, 
            msg:    &mut SerializedNetMsg,
            header: &mut Vec<u8>);

}
