// ---------------- [ File: bitcoin-fuzz/src/fuzz_block.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/block.cpp]

pub fn initialize_block()  {
    
    todo!();
        /*
            static const ECCVerifyHandle verify_handle;
        SelectParams(CBaseChainParams::REGTEST);
        */
}

#[fuzz_test(initializer = "initialize_block")]
fn block() {
    todo!();
    /*
    
        DataStream ds(buffer, SER_NETWORK, INIT_PROTO_VERSION);
        CBlock block;
        try {
            int nVersion;
            ds >> nVersion;
            ds.SetVersion(nVersion);
            ds >> block;
        } catch (const std::ios_base::failure&) {
            return;
        }
        const ChainConsensusParams& consensus_params = Params().GetConsensus();
        BlockValidationState validation_state_pow_and_merkle;

        const bool valid_incl_pow_and_merkle = CheckBlock(
            block, 
            validation_state_pow_and_merkle, 
            consensus_params, 
            /* fCheckPOW= */ true, 
            /* fCheckMerkleRoot= */ true);

        assert(validation_state_pow_and_merkle.IsValid() || validation_state_pow_and_merkle.IsInvalid() || validation_state_pow_and_merkle.IsError());
        (c_void)validation_state_pow_and_merkle.Error("");
        BlockValidationState validation_state_pow;
        const bool valid_incl_pow = CheckBlock(block, validation_state_pow, consensus_params, /* fCheckPOW= */ true, /* fCheckMerkleRoot= */ false);
        assert(validation_state_pow.IsValid() || validation_state_pow.IsInvalid() || validation_state_pow.IsError());
        BlockValidationState validation_state_merkle;
        const bool valid_incl_merkle = CheckBlock(block, validation_state_merkle, consensus_params, /* fCheckPOW= */ false, /* fCheckMerkleRoot= */ true);
        assert(validation_state_merkle.IsValid() || validation_state_merkle.IsInvalid() || validation_state_merkle.IsError());
        BlockValidationState validation_state_none;
        const bool valid_incl_none = CheckBlock(block, validation_state_none, consensus_params, /* fCheckPOW= */ false, /* fCheckMerkleRoot= */ false);
        assert(validation_state_none.IsValid() || validation_state_none.IsInvalid() || validation_state_none.IsError());
        if (valid_incl_pow_and_merkle) {
            assert(valid_incl_pow && valid_incl_merkle && valid_incl_none);
        } else if (valid_incl_merkle || valid_incl_pow) {
            assert(valid_incl_none);
        }
        (c_void)block.GetHash();
        (c_void)block.ToString();
        (c_void)BlockMerkleRoot(block);
        if (!block.vtx.empty()) {
            // TODO: Avoid array index out of bounds error in BlockWitnessMerkleRoot
            //       when block.vtx.empty().
            (c_void)BlockWitnessMerkleRoot(block);
        }
        (c_void)GetBlockWeight(block);
        (c_void)GetWitnessCommitmentIndex(block);
        const size_t raw_memory_size = RecursiveDynamicUsage(block);
        const size_t raw_memory_size_as_shared_ptr = RecursiveDynamicUsage(std::make_shared<CBlock>(block));
        assert(raw_memory_size_as_shared_ptr > raw_memory_size);
        CBlock block_copy = block;
        block_copy.SetNull();
        const bool is_null = block_copy.IsNull();
        assert(is_null);

    */
}
