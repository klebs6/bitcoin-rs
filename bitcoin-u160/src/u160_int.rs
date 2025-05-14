// ---------------- [ File: bitcoin-u160/src/u160_int.rs ]
crate::ix!();

/**
  | 160-bit opaque blob.
  | 
  | -----------
  | @note
  | 
  | This type is called u160 for historical
  | reasons only. It is an opaque blob of
  | 160 bits and has no integer operations.
  |
  */
#[derive(Clone,Default,PartialEq,Eq,Hash)]
pub struct u160 {
    pub blob: BaseBlob<160>,
}

impl From<&Vec<u8>> for u160 {
    
    fn from(vch: &Vec<u8>) -> Self {
    
        todo!();
        /*


            : base_blob<160>(vch)
        */
    }
}
