// ---------------- [ File: bitcoin-blockencoding/src/read_status.rs ]
crate::ix!();

#[derive(PartialEq,Eq,Clone,Debug)]
pub enum ReadStatus
{
    Ok,

    /**
      | Invalid object, peer is sending bogus
      | crap
      |
      */
    Invalid, 

    /**
      | Failed to process object
      |
      */
    Failed, 

    /**
      | Used only by FillBlock to indicate a
      | failure in CheckBlock.
      |
      */
    CheckBlockFailed, 
}
