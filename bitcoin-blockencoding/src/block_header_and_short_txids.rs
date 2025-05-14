// ---------------- [ File: bitcoin-blockencoding/src/block_header_and_short_txids.rs ]
crate::ix!();

///-----------------------
#[derive(Default)]
pub struct BlockHeaderAndShortTxIDs {

    pub shorttxidk0:  RefCell<u64>,
    pub shorttxidk1:  RefCell<u64>,
    pub nonce:        u64,
    pub shorttxids:   Vec<u64>,
    pub prefilledtxn: Vec<PrefilledTransaction>,
    pub header:       BlockHeader,
}

unsafe impl Send for BlockHeaderAndShortTxIDs {}
unsafe impl Sync for BlockHeaderAndShortTxIDs {}

const SHORTTXIDS_LENGTH: i32 = 6;

lazy_static!{
    /*
    SERIALIZE_METHODS(CBlockHeaderAndShortTxIDs, obj)
        {
            READWRITE(obj.header, obj.nonce, Using<VectorFormatter<CustomUintFormatter<SHORTTXIDS_LENGTH>>>(obj.shorttxids), obj.prefilledtxn);
            if (ser_action.ForRead()) {
                if (obj.BlockTxCount() > std::numeric_limits<uint16_t>::max()) {
                    throw std::ios_base::failure("indexes overflowed 16 bits");
                }
                obj.FillShortTxIDSelector();
            }
        }
    */
}

//-------------------------------------------[.cpp/bitcoin/src/blockencodings.cpp]
impl BlockHeaderAndShortTxIDs {

    pub fn block_tx_count(&self) -> usize {
        
        self.shorttxids.len() + self.prefilledtxn.len()
    }

    pub fn new(
        block:    Amo<Block>,
        usewtxid: bool) -> Self {
    
        todo!();
        /*
            :
            nonce(GetRand(std::numeric_limits<uint64_t>::max())),
            shorttxids(block.vtx.size() - 1), prefilledtxn(1), header(block) 

        FillShortTxIDSelector();
        //TODO: Use our mempool prior to block acceptance to predictively fill more than just the coinbase
        prefilledtxn[0] = {0, block.vtx[0]};
        for (size_t i = 1; i < block.vtx.size(); i++) {
            const CTransaction& tx = *block.vtx[i];
            shorttxids[i - 1] = GetShortID(fUseWTXID ? tx.GetWitnessHash() : tx.GetHash());
        }
        */
    }
    
    pub fn fill_short_tx_id_selector(&self)  {
        
        let mut stream: DataStream 
        = DataStream::new(SER_NETWORK, PROTOCOL_VERSION);

        stream.stream(&self.header);
        stream.stream(&self.nonce);

        let mut hasher = Sha256::default();

        hasher.write_from_iterator(
            stream.begin(), 
            stream.size()
        );

        let mut shorttxidhash = u256::default();

        hasher.finalize(
            unsafe { 
                std::slice::from_raw_parts(
                    shorttxidhash.blob.begin(), 
                    SHA256_OUTPUT_SIZE
                ).try_into().unwrap()
            }
        );

        *self.shorttxidk0.borrow_mut() = shorttxidhash.blob.get_u64(0);
        *self.shorttxidk1.borrow_mut() = shorttxidhash.blob.get_u64(1);
    }
    
    pub fn get_shortid(&self, txhash: &u256) -> u64 {
        
        //shorttxids calculation assumes 6-byte shorttxids
        const_assert![SHORTTXIDS_LENGTH == 6];

        sip_hash_uint256(
            *self.shorttxidk0.borrow(),
            *self.shorttxidk1.borrow(),
            txhash) 
        & 0xffffffffffff
    }
}
