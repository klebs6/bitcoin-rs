// ---------------- [ File: bitcoin-bench/src/bench_rpc_blockchain.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/rpc_blockchain.cpp]

pub struct TestBlockAndIndex {

    /**
       {MakeNoLogFileContext<const TestingSetup>(CBaseChainParams::MAIN)};
      */
    testing_setup: Box<TestingSetup>,

    block:         Block,
    block_hash:    u256,
    blockindex:    BlockIndex,
}

impl Default for TestBlockAndIndex {
    
    fn default() -> Self {
        todo!();
        /*


            DataStream stream(benchmark::data::block413567, SER_NETWORK, PROTOCOL_VERSION);
            char a = '\0';
            stream.write(&a, 1); // Prevent compaction

            stream >> block;

            blockHash = block.GetHash();
            blockindex.phashBlock = &blockHash;
            blockindex.nBits = 403014710;
        */
    }
}

#[bench] fn block_to_json_verbose(b: &mut Bencher)  {
    
    todo!();
        /*
            TestBlockAndIndex data;
        bench.run([&] {
            auto univalue = blockToJSON(data.block, &data.blockindex, &data.blockindex, TxVerbosity::SHOW_DETAILS_AND_PREVOUT);
            ankerl::nanobench::doNotOptimizeAway(univalue);
        });
        */
}

#[bench] fn block_to_json_verbose_write(b: &mut Bencher)  {
    
    todo!();
        /*
            TestBlockAndIndex data;
        auto univalue = blockToJSON(data.block, &data.blockindex, &data.blockindex, TxVerbosity::SHOW_DETAILS_AND_PREVOUT);
        bench.run([&] {
            auto str = univalue.write();
            ankerl::nanobench::doNotOptimizeAway(str);
        });
        */
}
