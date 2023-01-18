crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/consensus/merkle.h]
//-------------------------------------------[.cpp/bitcoin/src/consensus/merkle.cpp]

/**     
  | WARNING! If you're reading this because
  | you're learning about crypto and/or
  | designing a new system that will use merkle
  | trees, keep in mind that the following
  | merkle tree algorithm has a serious flaw
  | related to duplicate txids, resulting in
  | a vulnerability (CVE-2012-2459).
  |
  | The reason is that if the number of hashes
  | in the list at a given level is odd, the
  | last one is duplicated before computing the
  | next level (which is unusual in Merkle
  | trees). This results in certain sequences
  | of transactions leading to the same merkle
  | root. For example, these two trees:
  |
  |          A               A
  |        /  \            /   \
  |      B     C         B       C
  |     / \    |        / \     / \
  |    D   E   F       D   E   F   F
  |   / \ / \ / \     / \ / \ / \ / \
  |   1 2 3 4 5 6     1 2 3 4 5 6 5 6
  |
  | for transaction lists [1,2,3,4,5,6] and
  | [1,2,3,4,5,6,5,6] (where 5 and
  | 6 are repeated) result in the same root hash
  | A (because the hash of both of (F) and (F,F) is
  | C).
  |
  | The vulnerability results from being able to send
  | a block with such a transaction list, with the
  | same merkle root, and the same block hash as the
  | original without duplication, resulting in failed
  | validation. If the receiving node proceeds to
  | mark that block as permanently invalid however,
  | it will fail to accept further unmodified (and
  | thus potentially valid) versions of the same
  | block. We defend against this by detecting the
  | case where we would hash two identical hashes at
  | the end of the list together, and treating that
  | identically to the block having an invalid merkle
  | root. Assuming no double-SHA256 collisions, this
  | will detect all known ways of changing the
  | transactions without affecting the merkle root.
  */
pub fn compute_merkle_root(
    mut hashes:  Vec<u256>,
    mutated: &mut bool) -> u256 {
    
    let mut mutation: bool = false;

    while hashes.len() > 1 {

        if *mutated {

            for pos in (0..(hashes.len() - 1)).step_by(2) {

                if hashes[pos] == hashes[pos + 1] {
                    mutation = true;
                }
            }
        }

        if (hashes.len() & 1) != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        sha256d64(
            hashes[0].blob.begin_mut(), 
            hashes[0].blob.begin(), 
            hashes.len() / 2
        );

        hashes.resize(hashes.len() / 2, u256::ZERO);
    }

    if *mutated { 
        *mutated = mutation;
    }

    if hashes.is_empty() {
        return u256::ZERO;
    }

    hashes[0].clone()
}

/**
  | Compute the Merkle root of the transactions
  | in a block. *mutated is set to true if
  | a duplicated subtree was found.
  |
  */
pub fn block_merkle_root(
        block:   &Block,
        mutated: &mut bool) -> u256 {
    
    let mut leaves: Vec<u256> = vec![];

    leaves.resize(block.vtx.len(), u256::ZERO);

    for s in 0..block.vtx.len() {
        leaves[s] = block.vtx[s]
            .get()
            .get_hash()
            .clone();
    }

    compute_merkle_root(leaves, mutated)
}

/**
  | Compute the Merkle root of the witness
  | transactions in a block. *mutated is
  | set to true if a duplicated subtree was
  | found.
  |
  */
pub fn block_witness_merkle_root(
        block:   &Block,
        mutated: &mut bool) -> u256 {
    
    let mut leaves: Vec<u256> = vec![];

    leaves.resize(block.vtx.len(), u256::ZERO);

    /*
      | The witness hash of the coinbase is 0.
      |
      */
    leaves[0].blob.set_null(); 

    for s in 1..block.vtx.len() {

        leaves[s] = block.vtx[s]
            .get()
            .get_witness_hash()
            .clone();
    }

    compute_merkle_root(leaves, mutated)
}
