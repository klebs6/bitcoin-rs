crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/merkleblock.h]
//-------------------------------------------[.cpp/bitcoin/src/merkleblock.cpp]

/**
  | Helper functions for serialization.
  |
  */
pub fn bits_to_bytes(bits: &Vec<bool>) -> Vec<u8> {
    
    todo!();
        /*
            std::vector<unsigned char> ret((bits.size() + 7) / 8);
        for (unsigned int p = 0; p < bits.size(); p++) {
            ret[p / 8] |= bits[p] << (p % 8);
        }
        return ret;
        */
}

pub fn bytes_to_bits(bytes: &Vec<u8>) -> Vec<bool> {
    
    todo!();
        /*
            std::vector<bool> ret(bytes.size() * 8);
        for (unsigned int p = 0; p < ret.size(); p++) {
            ret[p] = (bytes[p / 8] & (1 << (p % 8))) != 0;
        }
        return ret;
        */
}

/** 
  | Data structure that represents a partial merkle
  | tree.
  |
  | It represents a subset of the txid's of a known
  | block, in a way that allows recovery of the
  | list of txid's and the merkle root, in an
  | authenticated way.
  |
  | The encoding works as follows: we traverse the
  | tree in depth-first order, storing a bit for
  | each traversed node, signifying whether the
  | node is the parent of at least one matched leaf
  | txid (or a matched txid itself). In case we are
  | at the leaf level, or this bit is 0, its merkle
  | node hash is stored, and its children are not
  | explored further. Otherwise, no hash is stored,
  | but we recurse into both (or the only) child
  | branch. During decoding, the same depth-first
  | traversal is performed, consuming bits and
  | hashes as they written during encoding.
  |
  | The serialization is fixed and provides a hard
  | guarantee about the encoded size:
  |
  |   SIZE <= 10 + ceil(32.25*N)
  |
  | Where N represents the number of leaf nodes of
  | the partial tree. N itself is bounded by:
  |
  |   N <= total_transactions
  |   N <= 1 + matched_transactions*tree_height
  |
  | The serialization format:
  |
  |  - uint32     total_transactions (4 bytes)
  |
  |  - varint     number of hashes   (1-3 bytes)
  |
  |  - uint256[]  hashes in depth-first order (<=
  |               32*N bytes)
  |
  |  - varint     number of bytes of flag bits (1-3
  |               bytes)
  |
  |  - byte[]     flag bits, packed per 8 in
  |               a byte, least significant bit
  |               first (<= 2*N-1 bits)
  |
  | The size constraints follow from this.
  */
pub struct PartialMerkleTree {

    /**
      | the total number of transactions in
      | the block
      |
      */
    n_transactions: u32,

    /**
      | node-is-parent-of-matched-txid
      | bits
      |
      */
    bits:           Vec<bool>,

    /**
      | txids and internal hashes
      |
      */
    hash:           Vec<u256>,

    /**
      | flag set when encountering invalid
      | data
      |
      */
    bad:            bool,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CPartialMerkleTree, obj)
        {
            READWRITE(obj.nTransactions, obj.vHash);
            std::vector<unsigned char> bytes;
            SER_WRITE(obj, bytes = BitsToBytes(obj.vBits));
            READWRITE(bytes);
            SER_READ(obj, obj.vBits = BytesToBits(bytes));
            SER_READ(obj, obj.fBad = false);
        }
    */
}

impl Default for PartialMerkleTree {

    fn default() -> Self {
    
        todo!();
        /*
        : n_transactions(0),
        : bad(true),

        
        */
    }
}

impl PartialMerkleTree {

    /**
      | helper function to efficiently calculate
      | the number of nodes at given height in
      | the merkle tree
      |
      */
    pub fn calc_tree_width(&self, height: i32) -> u32 {
        
        todo!();
        /*
            return (nTransactions+(1 << height)-1) >> height;
        */
    }

    /**
      | Get number of transactions the merkle
      | proof is indicating for cross-reference
      | with local blockchain knowledge.
      |
      */
    pub fn get_num_transactions(&self) -> u32 {
        
        todo!();
        /*
            return nTransactions; }{
        */
    }
    
    /**
      | calculate the hash of a node in the merkle
      | tree (at leaf level: the txid's themselves)
      |
      */
    pub fn calc_hash(&mut self, 
        height: i32,
        pos:    u32,
        txid:   &Vec<u256>) -> u256 {
        
        todo!();
        /*
            //we can never have zero txs in a merkle block, we always need the coinbase tx
        //if we do not have this assert, we can hit a memory access violation when indexing into vTxid
        assert(vTxid.size() != 0);
        if (height == 0) {
            // hash at height 0 is the txids themselves
            return vTxid[pos];
        } else {
            // calculate left hash
            uint256 left = CalcHash(height-1, pos*2, vTxid), right;
            // calculate right hash if not beyond the end of the array - copy left hash otherwise
            if (pos*2+1 < CalcTreeWidth(height-1))
                right = CalcHash(height-1, pos*2+1, vTxid);
            else
                right = left;
            // combine subhashes
            return Hash(left, right);
        }
        */
    }
    
    /**
      | recursive function that traverses
      | tree nodes, storing the data as bits
      | and hashes
      |
      */
    pub fn traverse_and_build(&mut self, 
        height: i32,
        pos:    u32,
        txid:   &Vec<u256>,
        match_: &Vec<bool>)  {
        
        todo!();
        /*
            // determine whether this node is the parent of at least one matched txid
        bool fParentOfMatch = false;
        for (unsigned int p = pos << height; p < (pos+1) << height && p < nTransactions; p++)
            fParentOfMatch |= vMatch[p];
        // store as flag bit
        vBits.push_back(fParentOfMatch);
        if (height==0 || !fParentOfMatch) {
            // if at height 0, or nothing interesting below, store hash and stop
            vHash.push_back(CalcHash(height, pos, vTxid));
        } else {
            // otherwise, don't store any hash, but descend into the subtrees
            TraverseAndBuild(height-1, pos*2, vTxid, vMatch);
            if (pos*2+1 < CalcTreeWidth(height-1))
                TraverseAndBuild(height-1, pos*2+1, vTxid, vMatch);
        }
        */
    }
    
    /**
      | recursive function that traverses
      | tree nodes, consuming the bits and hashes
      | produced by TraverseAndBuild. it returns
      | the hash of the respective node and its
      | respective index.
      |
      */
    pub fn traverse_and_extract(&mut self, 
        height:      i32,
        pos:         u32,
        n_bits_used: &mut u32,
        n_hash_used: &mut u32,
        match_:      &mut Vec<u256>,
        vn_index:    &mut Vec<u32>) -> u256 {
        
        todo!();
        /*
            if (nBitsUsed >= vBits.size()) {
            // overflowed the bits array - failure
            fBad = true;
            return uint256();
        }
        bool fParentOfMatch = vBits[nBitsUsed++];
        if (height==0 || !fParentOfMatch) {
            // if at height 0, or nothing interesting below, use stored hash and do not descend
            if (nHashUsed >= vHash.size()) {
                // overflowed the hash array - failure
                fBad = true;
                return uint256();
            }
            const uint256 &hash = vHash[nHashUsed++];
            if (height==0 && fParentOfMatch) { // in case of height 0, we have a matched txid
                vMatch.push_back(hash);
                vnIndex.push_back(pos);
            }
            return hash;
        } else {
            // otherwise, descend into the subtrees to extract matched txids and hashes
            uint256 left = TraverseAndExtract(height-1, pos*2, nBitsUsed, nHashUsed, vMatch, vnIndex), right;
            if (pos*2+1 < CalcTreeWidth(height-1)) {
                right = TraverseAndExtract(height-1, pos*2+1, nBitsUsed, nHashUsed, vMatch, vnIndex);
                if (right == left) {
                    // The left and right branches should never be identical, as the transaction
                    // hashes covered by them must each be unique.
                    fBad = true;
                }
            } else {
                right = left;
            }
            // and combine them before returning
            return Hash(left, right);
        }
        */
    }
    
    /**
      | Construct a partial merkle tree from
      | a list of transaction ids, and a mask
      | that selects a subset of them
      |
      */
    pub fn new_with_txid_and_match(
        txid:   &Vec<u256>,
        match_: &Vec<bool>) -> Self {
    
        todo!();
        /*
        : n_transactions(vTxid.size()),
        : bad(false),

            // reset state
        vBits.clear();
        vHash.clear();

        // calculate height of tree
        int nHeight = 0;
        while (CalcTreeWidth(nHeight) > 1)
            nHeight++;

        // traverse the partial tree
        TraverseAndBuild(nHeight, 0, vTxid, vMatch);
        */
    }
    
    /**
      | extract the matching txid's represented
      | by this partial merkle tree and their
      | respective indices within the partial
      | tree. returns the merkle root, or 0 in
      | case of failure
      |
      */
    pub fn extract_matches(&mut self, 
        match_:   &mut Vec<u256>,
        vn_index: &mut Vec<u32>) -> u256 {
        
        todo!();
        /*
            vMatch.clear();
        // An empty set will not work
        if (nTransactions == 0)
            return uint256();
        // check for excessively high numbers of transactions
        if (nTransactions > MAX_BLOCK_WEIGHT / MIN_TRANSACTION_WEIGHT)
            return uint256();
        // there can never be more hashes provided than one for every txid
        if (vHash.size() > nTransactions)
            return uint256();
        // there must be at least one bit per node in the partial tree, and at least one node per hash
        if (vBits.size() < vHash.size())
            return uint256();
        // calculate height of tree
        int nHeight = 0;
        while (CalcTreeWidth(nHeight) > 1)
            nHeight++;
        // traverse the partial tree
        unsigned int nBitsUsed = 0, nHashUsed = 0;
        uint256 hashMerkleRoot = TraverseAndExtract(nHeight, 0, nBitsUsed, nHashUsed, vMatch, vnIndex);
        // verify that no problems occurred during the tree traversal
        if (fBad)
            return uint256();
        // verify that all bits were consumed (except for the padding caused by serializing it as a byte sequence)
        if ((nBitsUsed+7)/8 != (vBits.size()+7)/8)
            return uint256();
        // verify that all hashes were consumed
        if (nHashUsed != vHash.size())
            return uint256();
        return hashMerkleRoot;
        */
    }
}

/**
  | Used to relay blocks as header + vector<merkle
  | branch> to filtered nodes.
  | 
  | -----------
  | @note
  | 
  | The class assumes that the given CBlock
  | has *at least* 1 transaction. If the
  | CBlock has 0 txs, it will hit an assertion.
  |
  */
#[derive(Default)]
pub struct MerkleBlock {

    /**
      | Public only for unit testing
      |
      */
    pub header:      BlockHeader,

    pub txn:         PartialMerkleTree,

    /**
      | Public only for unit testing and relay
      | testing (not relayed).
      | 
      | Used only when a bloom filter is specified
      | to allow testing the transactions which
      | matched the bloom filter.
      |
      */
    pub matched_txn: Vec<(u32,u256)>,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CMerkleBlock, obj) { 
        READWRITE(obj.header, obj.txn); 
    }
    */
}

impl MerkleBlock {

    /**
      | Create from a Block, filtering transactions
      | according to filter
      | 
      | -----------
      | @note
      | 
      | this will call IsRelevantAndUpdate
      | on the filter for each transaction,
      | thus the filter will likely be modified.
      |
      */
    pub fn new_with_block_and_filter(
        block:  Amo<Block>,
        filter: &mut BloomFilter) -> Self {
    
        todo!();
        /*


            : CMerkleBlock(block, &filter, nullptr)
        */
    }

    /**
      | Create from a Block, matching the txids
      | in the set
      |
      */
    pub fn new_with_block_and_txids(
        block: &Block,
        txids: &HashSet<u256>) -> Self {
    
        todo!();
        /*


            : CMerkleBlock(block, nullptr, &txids)
        */
    }

    /**
      | Combined constructor to consolidate
      | code
      |
      */
    pub fn new_with_block_filter_and_txids(
        block:  &Block,
        filter: *mut BloomFilter,
        txids:  *const HashSet<u256>) -> Self {
    
        todo!();
        /*


            header = block.GetBlockHeader();

        std::vector<bool> vMatch;
        std::vector<uint256> vHashes;

        vMatch.reserve(block.vtx.size());
        vHashes.reserve(block.vtx.size());

        for (unsigned int i = 0; i < block.vtx.size(); i++)
        {
            const uint256& hash = block.vtx[i]->GetHash();
            if (txids && txids->count(hash)) {
                vMatch.push_back(true);
            } else if (filter && filter->IsRelevantAndUpdate(*block.vtx[i])) {
                vMatch.push_back(true);
                vMatchedTxn.emplace_back(i, hash);
            } else {
                vMatch.push_back(false);
            }
            vHashes.push_back(hash);
        }

        txn = CPartialMerkleTree(vHashes, vMatch);
        */
    }
}
