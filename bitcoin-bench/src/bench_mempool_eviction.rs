// ---------------- [ File: bitcoin-bench/src/bench_mempool_eviction.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/mempool_eviction.cpp]

#[EXCLUSIVE_LOCKS_REQUIRED(cs_main, pool.cs)]
pub fn add_tx(
        tx:    &TransactionRef,
        n_fee: &Amount,
        pool:  &mut TxMemPool)  {
    
    todo!();
        /*
            int64_t nTime = 0;
        unsigned int nHeight = 1;
        bool spendsCoinbase = false;
        unsigned int sigOpCost = 4;
        LockPoints lp;
        pool.addUnchecked(CTxMemPoolEntry(
            tx, nFee, nTime, nHeight,
            spendsCoinbase, sigOpCost, lp));
        */
}

/**
  | Right now this is only testing eviction
  | performance in an extremely small mempool. Code
  | needs to be written to generate a much wider
  | variety of unique transactions for a more
  | meaningful performance measurement.
  */
#[bench] fn mempool_eviction(b: &mut Bencher)  {
    
    todo!();
        /*
            const auto testing_setup = MakeNoLogFileContext<const TestingSetup>();

        CMutableTransaction tx1 = CMutableTransaction();
        tx1.vin.resize(1);
        tx1.vin[0].scriptSig = CScript() << OP_1;
        tx1.vin[0].scriptWitness.stack.push_back({1});
        tx1.vout.resize(1);
        tx1.vout[0].scriptPubKey = CScript() << OP_1 << OP_EQUAL;
        tx1.vout[0].nValue = 10 * COIN;

        CMutableTransaction tx2 = CMutableTransaction();
        tx2.vin.resize(1);
        tx2.vin[0].scriptSig = CScript() << OP_2;
        tx2.vin[0].scriptWitness.stack.push_back({2});
        tx2.vout.resize(1);
        tx2.vout[0].scriptPubKey = CScript() << OP_2 << OP_EQUAL;
        tx2.vout[0].nValue = 10 * COIN;

        CMutableTransaction tx3 = CMutableTransaction();
        tx3.vin.resize(1);
        tx3.vin[0].prevout = OutPoint(tx2.GetHash(), 0);
        tx3.vin[0].scriptSig = CScript() << OP_2;
        tx3.vin[0].scriptWitness.stack.push_back({3});
        tx3.vout.resize(1);
        tx3.vout[0].scriptPubKey = CScript() << OP_3 << OP_EQUAL;
        tx3.vout[0].nValue = 10 * COIN;

        CMutableTransaction tx4 = CMutableTransaction();
        tx4.vin.resize(2);
        tx4.vin[0].prevout.SetNull();
        tx4.vin[0].scriptSig = CScript() << OP_4;
        tx4.vin[0].scriptWitness.stack.push_back({4});
        tx4.vin[1].prevout.SetNull();
        tx4.vin[1].scriptSig = CScript() << OP_4;
        tx4.vin[1].scriptWitness.stack.push_back({4});
        tx4.vout.resize(2);
        tx4.vout[0].scriptPubKey = CScript() << OP_4 << OP_EQUAL;
        tx4.vout[0].nValue = 10 * COIN;
        tx4.vout[1].scriptPubKey = CScript() << OP_4 << OP_EQUAL;
        tx4.vout[1].nValue = 10 * COIN;

        CMutableTransaction tx5 = CMutableTransaction();
        tx5.vin.resize(2);
        tx5.vin[0].prevout = OutPoint(tx4.GetHash(), 0);
        tx5.vin[0].scriptSig = CScript() << OP_4;
        tx5.vin[0].scriptWitness.stack.push_back({4});
        tx5.vin[1].prevout.SetNull();
        tx5.vin[1].scriptSig = CScript() << OP_5;
        tx5.vin[1].scriptWitness.stack.push_back({5});
        tx5.vout.resize(2);
        tx5.vout[0].scriptPubKey = CScript() << OP_5 << OP_EQUAL;
        tx5.vout[0].nValue = 10 * COIN;
        tx5.vout[1].scriptPubKey = CScript() << OP_5 << OP_EQUAL;
        tx5.vout[1].nValue = 10 * COIN;

        CMutableTransaction tx6 = CMutableTransaction();
        tx6.vin.resize(2);
        tx6.vin[0].prevout = OutPoint(tx4.GetHash(), 1);
        tx6.vin[0].scriptSig = CScript() << OP_4;
        tx6.vin[0].scriptWitness.stack.push_back({4});
        tx6.vin[1].prevout.SetNull();
        tx6.vin[1].scriptSig = CScript() << OP_6;
        tx6.vin[1].scriptWitness.stack.push_back({6});
        tx6.vout.resize(2);
        tx6.vout[0].scriptPubKey = CScript() << OP_6 << OP_EQUAL;
        tx6.vout[0].nValue = 10 * COIN;
        tx6.vout[1].scriptPubKey = CScript() << OP_6 << OP_EQUAL;
        tx6.vout[1].nValue = 10 * COIN;

        CMutableTransaction tx7 = CMutableTransaction();
        tx7.vin.resize(2);
        tx7.vin[0].prevout = OutPoint(tx5.GetHash(), 0);
        tx7.vin[0].scriptSig = CScript() << OP_5;
        tx7.vin[0].scriptWitness.stack.push_back({5});
        tx7.vin[1].prevout = OutPoint(tx6.GetHash(), 0);
        tx7.vin[1].scriptSig = CScript() << OP_6;
        tx7.vin[1].scriptWitness.stack.push_back({6});
        tx7.vout.resize(2);
        tx7.vout[0].scriptPubKey = CScript() << OP_7 << OP_EQUAL;
        tx7.vout[0].nValue = 10 * COIN;
        tx7.vout[1].scriptPubKey = CScript() << OP_7 << OP_EQUAL;
        tx7.vout[1].nValue = 10 * COIN;

        CTxMemPool pool;
        LOCK2(cs_main, pool.cs);
        // Create transaction references outside the "hot loop"
        const CTransactionRef tx1_r{MakeTransactionRef(tx1)};
        const CTransactionRef tx2_r{MakeTransactionRef(tx2)};
        const CTransactionRef tx3_r{MakeTransactionRef(tx3)};
        const CTransactionRef tx4_r{MakeTransactionRef(tx4)};
        const CTransactionRef tx5_r{MakeTransactionRef(tx5)};
        const CTransactionRef tx6_r{MakeTransactionRef(tx6)};
        const CTransactionRef tx7_r{MakeTransactionRef(tx7)};

        bench.run([&]() NO_THREAD_SAFETY_ANALYSIS {
            AddTx(tx1_r, 10000LL, pool);
            AddTx(tx2_r, 5000LL, pool);
            AddTx(tx3_r, 20000LL, pool);
            AddTx(tx4_r, 7000LL, pool);
            AddTx(tx5_r, 1000LL, pool);
            AddTx(tx6_r, 1100LL, pool);
            AddTx(tx7_r, 9000LL, pool);
            pool.TrimToSize(pool.DynamicMemoryUsage() * 3 / 4);
            pool.TrimToSize(GetVirtualTransactionSize(*tx1_r));
        });
        */
}
