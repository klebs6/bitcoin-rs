crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/rpc_mempool.cpp]

#[EXCLUSIVE_LOCKS_REQUIRED(cs_main, pool.cs)]
pub fn add_tx(
        tx:   &TransactionRef,
        fee:  &Amount,
        pool: &mut TxMemPool)  {
    
    todo!();
        /*
            LockPoints lp;
        pool.addUnchecked(CTxMemPoolEntry(tx, fee, /* time */ 0, /* height */ 1, /* spendsCoinbase */ false, /* sigOpCost */ 4, lp));
        */
}

#[bench] fn rpc_mempool(b: &mut Bencher)  {
    
    todo!();
        /*
            CTxMemPool pool;
        LOCK2(cs_main, pool.cs);

        for (int i = 0; i < 1000; ++i) {
            CMutableTransaction tx = CMutableTransaction();
            tx.vin.resize(1);
            tx.vin[0].scriptSig = CScript() << OP_1;
            tx.vin[0].scriptWitness.stack.push_back({1});
            tx.vout.resize(1);
            tx.vout[0].scriptPubKey = CScript() << OP_1 << OP_EQUAL;
            tx.vout[0].nValue = i;
            const CTransactionRef tx_r{MakeTransactionRef(tx)};
            AddTx(tx_r, /* fee */ i, pool);
        }

        bench.run([&] {
            (c_void)MempoolToJSON(pool, /*verbose*/ true);
        });
        */
}
