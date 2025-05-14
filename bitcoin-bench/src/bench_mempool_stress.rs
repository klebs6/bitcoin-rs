// ---------------- [ File: bitcoin-bench/src/bench_mempool_stress.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/mempool_stress.cpp]

#[EXCLUSIVE_LOCKS_REQUIRED(cs_main, pool.cs)]
pub fn add_tx(
        tx:   &TransactionRef,
        pool: &mut TxMemPool)  {
    
    todo!();
        /*
            int64_t nTime = 0;
        unsigned int nHeight = 1;
        bool spendsCoinbase = false;
        unsigned int sigOpCost = 4;
        LockPoints lp;
        pool.addUnchecked(CTxMemPoolEntry(tx, 1000, nTime, nHeight, spendsCoinbase, sigOpCost, lp));
        */
}

pub struct Available {
    ref_:     TransactionRef,
    vin_left: usize, // default = { 0 }
    tx_count: usize,
}

impl Available {

    pub fn new(
        ref_:     &mut TransactionRef,
        tx_count: usize) -> Self {
    
        todo!();
        /*
        : ref_(ref),
        : tx_count(tx_count),

        
        */
    }
}

pub fn create_ordered_coins(
        det_rand:      &mut FastRandomContext,
        child_txs:     i32,
        min_ancestors: i32) -> Vec<TransactionRef> {
    
    todo!();
        /*
            std::vector<Available> available_coins;
        std::vector<CTransactionRef> ordered_coins;
        // Create some base transactions
        size_t tx_counter = 1;
        for (auto x = 0; x < 100; ++x) {
            CMutableTransaction tx = CMutableTransaction();
            tx.vin.resize(1);
            tx.vin[0].scriptSig = CScript() << CScriptNum(tx_counter);
            tx.vin[0].scriptWitness.stack.push_back(CScriptNum(x).getvch());
            tx.vout.resize(det_rand.randrange(10)+2);
            for (auto& out : tx.vout) {
                out.scriptPubKey = CScript() << CScriptNum(tx_counter) << OP_EQUAL;
                out.nValue = 10 * COIN;
            }
            ordered_coins.emplace_back(MakeTransactionRef(tx));
            available_coins.emplace_back(ordered_coins.back(), tx_counter++);
        }
        for (auto x = 0; x < childTxs && !available_coins.empty(); ++x) {
            CMutableTransaction tx = CMutableTransaction();
            size_t n_ancestors = det_rand.randrange(10)+1;
            for (size_t ancestor = 0; ancestor < n_ancestors && !available_coins.empty(); ++ancestor){
                size_t idx = det_rand.randrange(available_coins.size());
                Available coin = available_coins[idx];
                uint256 hash = coin.ref->GetHash();
                // biased towards taking min_ancestors parents, but maybe more
                size_t n_to_take = det_rand.randrange(2) == 0 ?
                                   min_ancestors :
                                   min_ancestors + det_rand.randrange(coin.ref->vout.size() - coin.vin_left);
                for (size_t i = 0; i < n_to_take; ++i) {
                    tx.vin.emplace_back();
                    tx.vin.back().prevout = OutPoint(hash, coin.vin_left++);
                    tx.vin.back().scriptSig = CScript() << coin.tx_count;
                    tx.vin.back().scriptWitness.stack.push_back(CScriptNum(coin.tx_count).getvch());
                }
                if (coin.vin_left == coin.ref->vin.size()) {
                    coin = available_coins.back();
                    available_coins.pop_back();
                }
                tx.vout.resize(det_rand.randrange(10)+2);
                for (auto& out : tx.vout) {
                    out.scriptPubKey = CScript() << CScriptNum(tx_counter) << OP_EQUAL;
                    out.nValue = 10 * COIN;
                }
            }
            ordered_coins.emplace_back(MakeTransactionRef(tx));
            available_coins.emplace_back(ordered_coins.back(), tx_counter++);
        }
        return ordered_coins;
        */
}

#[bench] fn complex_mem_pool(b: &mut Bencher)  {
    
    todo!();
        /*
            FastRandomContext det_rand{true};
        int childTxs = 800;
        if (bench.complexityN() > 1) {
            childTxs = static_cast<int>(bench.complexityN());
        }
        std::vector<CTransactionRef> ordered_coins = CreateOrderedCoins(det_rand, childTxs, /* min_ancestors */ 1);
        const auto testing_setup = MakeNoLogFileContext<const TestingSetup>(CBaseChainParams::MAIN);
        CTxMemPool pool;
        LOCK2(cs_main, pool.cs);
        bench.run([&]() NO_THREAD_SAFETY_ANALYSIS {
            for (auto& tx : ordered_coins) {
                AddTx(tx, pool);
            }
            pool.TrimToSize(pool.DynamicMemoryUsage() * 3 / 4);
            pool.TrimToSize(GetVirtualTransactionSize(*ordered_coins.front()));
        });
        */
}

#[bench] fn mempool_check(b: &mut Bencher)  {
    
    todo!();
        /*
            FastRandomContext det_rand{true};
        const int childTxs = bench.complexityN() > 1 ? static_cast<int>(bench.complexityN()) : 2000;
        const std::vector<CTransactionRef> ordered_coins = CreateOrderedCoins(det_rand, childTxs, /* min_ancestors */ 5);
        const auto testing_setup = MakeNoLogFileContext<const TestingSetup>(CBaseChainParams::MAIN, {"-checkmempool=1"});
        CTxMemPool pool;
        LOCK2(cs_main, pool.cs);
        const CCoinsViewCache& coins_tip = testing_setup.get()->m_node.chainman->ActiveChainstate().CoinsTip();
        for (auto& tx : ordered_coins) AddTx(tx, pool);

        bench.run([&]() NO_THREAD_SAFETY_ANALYSIS {
            pool.check(coins_tip, /* spendheight */ 2);
        });
        */
}
