// ---------------- [ File: bitcoin-bench/src/bench_ccoins_caching.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/ccoins_caching.cpp]

/**
  | Microbenchmark for simple accesses to a CCoinsViewCache database. Note from
  | laanwj, "replicating the actual usage patterns of the client is hard though,
  | many times micro-benchmarks of the database showed completely different
  | characteristics than e.g. reindex timings. But that's not a requirement of
  | every benchmark."
  |
  | (https://github.com/bitcoin/bitcoin/issues/7883#issuecomment-224807484)
  */
#[bench]
fn coins_caching(b: &mut Bencher)  {
    
    todo!();
        /*
        const ECCVerifyHandle verify_handle;
        ECC_Start();

        FillableSigningProvider keystore;
        CCoinsView coinsDummy;
        CCoinsViewCache coins(&coinsDummy);
        std::vector<CMutableTransaction> dummyTransactions =
            SetupDummyInputs(keystore, coins, {11 * COIN, 50 * COIN, 21 * COIN, 22 * COIN});

        CMutableTransaction t1;
        t1.vin.resize(3);
        t1.vin[0].prevout.hash = dummyTransactions[0].GetHash();
        t1.vin[0].prevout.n = 1;
        t1.vin[0].scriptSig << std::vector<unsigned char>(65, 0);
        t1.vin[1].prevout.hash = dummyTransactions[1].GetHash();
        t1.vin[1].prevout.n = 0;
        t1.vin[1].scriptSig << std::vector<unsigned char>(65, 0) << std::vector<unsigned char>(33, 4);
        t1.vin[2].prevout.hash = dummyTransactions[1].GetHash();
        t1.vin[2].prevout.n = 1;
        t1.vin[2].scriptSig << std::vector<unsigned char>(65, 0) << std::vector<unsigned char>(33, 4);
        t1.vout.resize(2);
        t1.vout[0].nValue = 90 * COIN;
        t1.vout[0].scriptPubKey << OP_1;

        // Benchmark.
        const CTransaction tx_1(t1);
        bench.run([&] {
            bool success = AreInputsStandard(tx_1, coins, false);
            assert(success);
        });
        ECC_Stop();
        */
}
