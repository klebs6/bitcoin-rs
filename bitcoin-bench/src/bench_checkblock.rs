// ---------------- [ File: bitcoin-bench/src/bench_checkblock.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/checkblock.cpp]

/**
   | These are the two major time-sinks which happen
   | after we have fully received a block off the
   | wire, but before we can relay the block on to
   | peers using compact block relay.
  */
#[bench] fn deserialize_block_test(b: &mut Bencher)  {
    
    todo!();
        /*
            DataStream stream(typename benchmark::data::block413567, SER_NETWORK, PROTOCOL_VERSION);
        char a = '\0';
        stream.write(&a, 1); // Prevent compaction

        bench.unit("block").run([&] {
            CBlock block;
            stream >> block;
            bool rewound = stream.Rewind(typename benchmark::data::block413567.size());
            assert(rewound);
        });
        */
}

#[bench] fn deserialize_and_check_block_test(b: &mut Bencher)  {
    
    todo!();
        /*
        DataStream stream(typename benchmark::data::block413567, SER_NETWORK, PROTOCOL_VERSION);
        char a = '\0';
        stream.write(&a, 1); // Prevent compaction

        ArgsManager bench_args;
        const auto chainParams = CreateChainParams(bench_args, CBaseChainParams::MAIN);

        bench.unit("block").run([&] {
            CBlock block; // Note that CBlock caches its checked state, so we need to recreate it here
            stream >> block;
            bool rewound = stream.Rewind(benchmark::data::block413567.size());
            assert(rewound);

            BlockValidationState validationState;
            bool checked = CheckBlock(block, validationState, chainParams->GetConsensus());
            assert(checked);
        });
        */
}
