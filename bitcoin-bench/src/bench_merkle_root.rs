// ---------------- [ File: bitcoin-bench/src/bench_merkle_root.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/merkle_root.cpp]

#[bench] fn merkle_root(b: &mut Bencher)  {
    
    todo!();
        /*
            FastRandomContext rng(true);
        std::vector<uint256> leaves;
        leaves.resize(9001);
        for (auto& item : leaves) {
            item = rng.rand256();
        }
        bench.batch(leaves.size()).unit("leaf").run([&] {
            bool mutation = false;
            uint256 hash = ComputeMerkleRoot(std::vector<uint256>(leaves), &mutation);
            leaves[mutation] = hash;
        });
        */
}
