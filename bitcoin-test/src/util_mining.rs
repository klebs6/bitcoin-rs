crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/mining.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/mining.cpp]

/**
  | RPC-like helper function, returns
  | the generated coin
  |
  */
pub fn generatetoaddress(
        node:    &NodeContext,
        address: &String) -> TxIn {
    
    todo!();
        /*
            const auto dest = DecodeDestination(address);
        assert(IsValidDestination(dest));
        const auto coinbase_script = GetScriptForDestination(dest);

        return MineBlock(node, coinbase_script);
        */
}

/**
  | Create a blockchain, starting from
  | genesis
  |
  */
pub fn create_block_chain(
        total_height: usize,
        params:       &ChainParams) -> Vec<Arc<Block>> {
    
    todo!();
        /*
            std::vector<std::shared_ptr<CBlock>> ret{total_height};
        auto time{params.GenesisBlock().nTime};
        for (size_t height{0}; height < total_height; ++height) {
            CBlock& block{*(ret.at(height) = std::make_shared<CBlock>())};

            CMutableTransaction coinbase_tx;
            coinbase_tx.vin.resize(1);
            coinbase_tx.vin[0].prevout.SetNull();
            coinbase_tx.vout.resize(1);
            coinbase_tx.vout[0].scriptPubKey = P2WSH_OP_TRUE;
            coinbase_tx.vout[0].nValue = GetBlockSubsidy(height + 1, params.GetConsensus());
            coinbase_tx.vin[0].scriptSig = CScript() << (height + 1) << OP_0;
            block.vtx = {MakeTransactionRef(std::move(coinbase_tx))};

            block.nVersion = VERSIONBITS_LAST_OLD_BLOCK_VERSION;
            block.hashPrevBlock = (height >= 1 ? *ret.at(height - 1) : params.GenesisBlock()).GetHash();
            block.hashMerkleRoot = BlockMerkleRoot(block);
            block.nTime = ++time;
            block.nBits = params.GenesisBlock().nBits;
            block.nNonce = 0;

            while (!CheckProofOfWork(block.GetHash(), block.nBits, params.GetConsensus())) {
                ++block.nNonce;
                assert(block.nNonce);
            }
        }
        return ret;
        */
}

/**
  | Returns the generated coin
  |
  */
pub fn mine_block(
        node:                    &NodeContext,
        coinbase_script_pub_key: &Script) -> TxIn {
    
    todo!();
        /*
            auto block = PrepareBlock(node, coinbase_scriptPubKey);

        while (!CheckProofOfWork(block->GetHash(), block->nBits, Params().GetConsensus())) {
            ++block->nNonce;
            assert(block->nNonce);
        }

        bool processed{Assert(node.chainman)->ProcessNewBlock(Params(), block, true, nullptr)};
        assert(processed);

        return CTxIn{block->vtx[0]->GetHash(), 0};
        */
}

/**
  | Prepare a block to be mined
  |
  */
pub fn prepare_block(
        node:                    &NodeContext,
        coinbase_script_pub_key: &Script) -> Arc<Block> {
    
    todo!();
        /*
            auto block = std::make_shared<CBlock>(
            BlockAssembler{Assert(node.chainman)->ActiveChainstate(), *Assert(node.mempool), Params()}
                .CreateNewBlock(coinbase_scriptPubKey)
                ->block);

        LOCK(cs_main);
        block->nTime = Assert(node.chainman)->ActiveChain().Tip()->GetMedianTimePast() + 1;
        block->hashMerkleRoot = BlockMerkleRoot(*block);

        return block;
        */
}
