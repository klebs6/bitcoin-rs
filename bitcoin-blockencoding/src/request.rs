// ---------------- [ File: bitcoin-blockencoding/src/request.rs ]
crate::ix!();

#[derive(Default)]
pub struct BlockTransactionsRequest {

    /**
      | A BlockTransactionsRequest message
      |
      */
    pub blockhash: u256,

    pub indexes:   Vec<u16>,

}

lazy_static!{
    /*
    SERIALIZE_METHODS(BlockTransactionsRequest, obj)
        {
            READWRITE(obj.blockhash, Using<VectorFormatter<DifferenceFormatter>>(obj.indexes));
        }
    */
}
